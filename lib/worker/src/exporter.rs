use log::{info, warn};
use sqlx::PgPool;
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};

/// Starts a repeated job that exports new publication data to Basin Storage.
pub async fn start_exporter(
    pg_pool: PgPool,
    sink: String,
    interval: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    let scheduler = JobScheduler::new().await?;
    let job = Job::new_repeated_async(interval, move |_uuid, _l| {
        let p = pg_pool.clone();
        let s = sink.clone();
        Box::pin(async move {
            info!("storage job started");
            crate::db::export_into(&p, s)
                .await
                .unwrap_or_else(|err| warn!("{err}"));
            info!("storage job ended");
        })
    })?;
    scheduler.add(job).await?;
    // scheduler.shutdown_on_ctrl_c();
    // scheduler.set_shutdown_handler(Box::new(|| {
    //     Box::pin(async move {
    //         info!("storage scheduler shutdown");
    //     })
    // }));
    scheduler.start().await?;
    info!("storage scheduler started");
    Ok(())
}
