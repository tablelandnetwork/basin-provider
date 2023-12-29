use crate::db::{self, update_job_status, UnfinishedJob};
use basin_common::errors::Result;
use basin_evm::EVMClient;
use basin_worker::{domain::Cid, web3storage::Web3StorageClient};
use sqlx::PgPool;

pub async fn process_jobs<E: EVMClient + 'static + std::marker::Sync>(
    pool: PgPool,
    w3s_client: Web3StorageClient,
    evm_client: E,
) -> Result<()> {
    for job in db::unfinished_jobs(&pool).await? {
        process_job(&pool, &w3s_client, &evm_client, job).await?;
    }

    Ok(())
}

pub async fn process_job<E: EVMClient + 'static + std::marker::Sync>(
    pool: &PgPool,
    w3s_client: &Web3StorageClient,
    evm_client: &E,
    job: UnfinishedJob,
) -> Result<()> {
    let status = w3s_client.status_of_cid(job.cid.as_str()).await.unwrap();

    if status.deals.is_empty() {
        return Ok(());
    }

    let active_deals = status
        .deals
        .into_iter()
        .filter_map(|deal| {
            if deal.status == "Active" {
                Some(deal)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if active_deals.is_empty() {
        return Ok(());
    }

    evm_client
        .add_cid(job.vault, job.cid.clone(), job.timestamp.unwrap_or(0))
        .await?;

    let earliest_deal = active_deals
        .into_iter()
        .min_by_key(|deal| deal.activation.unwrap().timestamp_micros())
        .unwrap();

    update_job_status(
        pool,
        Cid::from(job.cid).unwrap(),
        earliest_deal.activation.unwrap(),
    )
    .await?;

    Ok(())
}
