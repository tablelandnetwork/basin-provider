use basin_common::errors::Result;
use basin_worker::domain::Cid;
use multibase::Base;
use serde::Serialize;
use sqlx::Row;
use sqlx::{types::chrono::NaiveDateTime, PgPool};

#[derive(Serialize, Debug)]
pub struct UnfinishedJob {
    pub vault: String,
    pub cid: String,
    // pub activated:  NaiveDateTime,
    pub timestamp: Option<i64>,
    // pub cache_path: String,
    // pub expires_at: NaiveDateTime,
}

impl UnfinishedJob {
    pub fn new(vault: String, cid: Vec<u8>, timestamp: Option<i64>) -> UnfinishedJob {
        let cid_str = multibase::encode::<Vec<u8>>(Base::Base32Lower, cid);

        UnfinishedJob {
            vault,
            cid: cid_str,
            timestamp,
        }
    }
}

/// Creates a namespace for owner.
/// Returns whether or not the namespace was created.
pub async fn unfinished_jobs(pool: &PgPool) -> Result<Vec<UnfinishedJob>> {
    let jobs = sqlx::query(
        "
            SELECT namespaces.name || '.' || jobs.relation as vault, jobs.cid, jobs.timestamp
            FROM namespaces, jobs
            WHERE namespaces.id = jobs.ns_id and activated is NULL
        ",
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| {
        UnfinishedJob::new(
            row.get("vault"),
            row.get("cid"),
            row.try_get("timestamp").ok(),
        )
    })
    .collect();

    Ok(jobs)
}

// Updates the job status in the DB.
pub async fn update_job_status(pool: &PgPool, cid: Cid, activation: NaiveDateTime) -> Result<()> {
    let sql = format!(
        "UPDATE jobs SET activated = '{}' WHERE cid = '\\x{}'",
        activation,
        hex::encode(cid.as_ref())
    );

    sqlx::query(sql.as_str()).execute(pool).await?;

    Ok(())
}
