use basin_common::errors::Result;
use futures::TryStreamExt;
use log::{error, info};
use sqlx::{postgres::PgPool, Executor, Row};
use std::error::Error;
use tokio_cron_scheduler::{Job, JobScheduler};

/// Internal type used during export.
#[derive(Debug)]
struct Ns {
    id: i64,
    name: String,
    last_export: chrono::DateTime<chrono::Utc>,
    rels: Vec<String>,
}

/// Starts a repeated job that exports new publication data to Basin Storage.
pub async fn start(
    pg_pool: PgPool,
    bucket: String,
    creds: String,
    schedule: &str,
) -> Result<(), Box<dyn Error>> {
    let mut scheduler = JobScheduler::new().await?;
    let job = Job::new_async(schedule, move |_uuid, _l| {
        let p = pg_pool.clone();
        let b = bucket.clone();
        let c = creds.clone();
        Box::pin(async move {
            info!("export job started");
            export(&p, &b, &c)
                .await
                .unwrap_or_else(|err| error!("{err}"));
            info!("export job ended");
        })
    })?;
    scheduler.add(job).await?;
    scheduler.shutdown_on_ctrl_c();
    scheduler.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            info!("exporter shutdown");
        })
    }));
    scheduler.start().await?;
    info!("exporter started");
    Ok(())
}

/// Exports new publication data to sink.
async fn export(pool: &PgPool, bucket: &str, creds: &str) -> Result<()> {
    let now_res = sqlx::query("SELECT now()").fetch_one(pool).await?;
    let now: chrono::DateTime<chrono::Utc> = now_res.try_get("now")?;

    let mut nss: Vec<Ns> = Vec::new();
    let mut ns_res = sqlx::query("SELECT * FROM namespaces").fetch(pool);
    while let Some(row) = ns_res.try_next().await? {
        let mut ns = Ns {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            last_export: row
                .try_get("last_export")
                .unwrap_or(chrono::DateTime::default()),
            rels: Vec::new(),
        };
        // Get all tables in the namespace's schema
        let mut rels_res = sqlx::query("SELECT tablename FROM pg_tables WHERE schemaname=$1")
            .bind(ns.name.clone())
            .fetch(pool);
        while let Some(row) = rels_res.try_next().await? {
            let rel: String = row.try_get("tablename")?;
            ns.rels.push(rel);
        }
        nss.push(ns);
    }

    for ns in nss {
        for rel in ns.rels {
            info!(
                "preparing export for ns {} ({}.{rel}); last export: {}",
                ns.id, ns.name, ns.last_export
            );

            let schema = ns.name.clone();
            // Hack to get all the column names except for __basin_created
            // We don't want to export this internal column
            let select_res = sqlx::query(&format!(
                "SELECT \
                    'SELECT ' || STRING_AGG('t.' || column_name, ', ') || ' FROM {schema}.{rel} AS t' AS q \
                    FROM information_schema.columns \
                    WHERE table_schema = '{schema}' \
                    AND table_name = '{rel}' \
                    AND column_name NOT IN ('__basin_created')"
            ))
                .fetch_one(pool)
                .await?;
            let select: String = select_res.try_get("q")?;

            let window = format!(
                "__basin_created < '{now}'::timestamp AND __basin_created >= '{}'::timestamp",
                ns.last_export
            );
            let count_res = sqlx::query(&format!(
                "SELECT COUNT(*) FROM {schema}.{rel} WHERE {window}"
            ))
            .fetch_one(pool)
            .await?;
            let count: i64 = count_res.try_get(0)?;
            info!(
                "exporting {count} rows for ns {} ({}.{rel})",
                ns.id, ns.name
            );

            if count > 0 {
                let dest = format!(
                    "gs://{bucket}/{}/{rel}?AUTH=specified&CREDENTIALS={creds}",
                    ns.name
                );
                // Unprepared query required for EXPORT
                pool.execute(
                    format!("EXPORT INTO PARQUET '{dest}' FROM {select} WHERE {window}").as_str(),
                )
                .await?;
            }
        }
        sqlx::query(&format!(
            "UPDATE namespaces SET last_export='{now}'::timestamp WHERE id={}",
            ns.id
        ))
        .execute(pool)
        .await?;
    }
    Ok(())
}
