use crate::db::Result;
use ethers::types::Address;
use futures::TryStreamExt;
use log::info;
use sqlx::postgres::{PgPool, PgQueryResult};
use sqlx::{Executor, Row};

/// Creates a namespace for owner.
pub async fn namespace_create(pool: &PgPool, ns: String, owner: Address) -> Result<()> {
    // Insert a new namespace for owner
    sqlx::query!(
        "INSERT INTO namespaces (name, owner) VALUES ($1, $2) ON CONFLICT (name) DO NOTHING",
        ns.clone(),
        owner.as_bytes()
    )
    .execute(pool)
    .await?;

    // Create schema for the namespace
    sqlx::query(&format!("CREATE SCHEMA IF NOT EXISTS {ns}"))
        .execute(pool)
        .await?;
    Ok(())
}

/// Returns whether or not the namespace is owned by `owner`.
pub async fn is_namespace_owner(pool: &PgPool, ns: String, owner: Address) -> Result<bool> {
    let res = sqlx::query("SELECT id FROM namespaces WHERE name=$1 AND owner=$2")
        .bind(ns)
        .bind(owner.as_bytes())
        .fetch_one(pool)
        .await?;
    Ok(!res.is_empty())
}

/// Creates a data table and scheduled changefeed for pub.
pub async fn pub_table_create(pool: &PgPool, table_stmt: &str) -> Result<()> {
    sqlx::query(table_stmt).execute(pool).await?;
    Ok(())
}

/// Inserts data into a pub table.
pub async fn pub_table_insert(pool: &PgPool, stmts: Vec<String>) -> Result<()> {
    let mut txn = pool.begin().await?;
    for s in stmts {
        txn_execute(&mut txn, &s).await?;
    }
    Ok(txn.commit().await?)
}

/// Runs sqlx query within a database transaction.
async fn txn_execute(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    stmt: &str,
) -> sqlx::Result<PgQueryResult> {
    sqlx::query(stmt).execute(&mut **txn).await
}

/// Internal type used during export.
#[derive(Debug)]
struct Ns {
    id: i64,
    name: String,
    last_export: chrono::DateTime<chrono::Utc>,
    rels: Vec<String>,
}

/// Exports new publication data to sink.
pub async fn export_into(pool: &PgPool, sink: String) -> Result<()> {
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

            // This conditional isn't needed, but as long as we're logging count, may as well use it
            if count > 0 {
                // Unprepared query required for EXPORT
                pool.execute(
                    format!("EXPORT INTO PARQUET '{sink}' FROM {select} WHERE {window}").as_str(),
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
