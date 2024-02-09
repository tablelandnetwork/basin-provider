use crate::crypto;
use crate::db;
use crate::domain::Cid;
use crate::domain::Vault;
use crate::gcs::GcsClient;
use crate::web3storage::Web3Storage;
use hex::ToHex;
use std::str::FromStr;

use basin_evm::EVMClient;
use chrono::DateTime;

use ethers::types::Address;
use futures::StreamExt;
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use google_cloud_storage::http::objects::{
    upload::{UploadObjectRequest, UploadType},
    Object,
};
use google_cloud_storage::http::resumable_upload_client::ChunkSize;
use google_cloud_storage::http::resumable_upload_client::ResumableUploadClient;
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;
use std::borrow::BorrowMut;
use std::convert::Infallible;
use tiny_keccak::{Hasher, Keccak};
use warp::http::StatusCode;
use warp::reply::{json, with_status};
use warp::Stream;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub async fn find_event_by_id(
    path: String,
    pool: PgPool,
    gcs_client: GcsClient,
) -> Result<Box<dyn warp::Reply>, Infallible> {
    let cid: Cid = match path.try_into() {
        Ok(v) => v,
        Err(err) => {
            return Ok(Box::new(with_status(
                json(&ErrorResponse {
                    error: err.to_string(),
                }),
                StatusCode::BAD_REQUEST,
            )));
        }
    };

    let cache_path = match db::find_job_cache_path_by_cid(&pool, cid).await {
        Ok(v) => v,
        Err(err) => {
            log::error!("{}", err);
            return Ok(Box::new(with_status(
                json(&ErrorResponse {
                    error: "error fetching the event".to_string(),
                }),
                StatusCode::BAD_REQUEST,
            )));
        }
    };

    let cache_path = match cache_path {
        Some(path) => path,
        None => {
            let empty: Vec<u8> = Vec::new();
            return Ok(Box::new(with_status(json(&empty), StatusCode::NOT_FOUND)));
        }
    };

    let stream = match gcs_client
        .inner
        .download_streamed_object(
            &GetObjectRequest {
                bucket: gcs_client.bucket.to_string(),
                object: cache_path.clone().as_ref().to_string(),
                ..Default::default()
            },
            &Range::default(),
        )
        .await
    {
        Ok(s) => s,
        Err(err) => {
            log::error!("{}", err);
            return Ok(Box::new(with_status(
                json(&ErrorResponse {
                    error: "failed to download cid".to_string(),
                }),
                StatusCode::BAD_REQUEST,
            )));
        }
    };

    let body = hyper::Body::wrap_stream(stream);

    let response = warp::reply::with_header(
        with_status(warp::reply::Response::new(body), StatusCode::OK),
        "content-disposition",
        format!(
            "attachment; filename=\"{}\"",
            cache_path.filename().unwrap()
        ),
    );

    Ok(Box::new(response))
}

impl TryFrom<String> for Cid {
    type Error = String;

    fn try_from(value: String) -> Result<Cid, String> {
        Cid::from(value.to_string())
    }
}

pub async fn find_vaults_by_account<E: EVMClient + 'static + std::marker::Sync>(
    evm_client: E,
    params: FindVaultsByAccountParams,
) -> Result<impl warp::Reply, Infallible> {
    let account = match Address::from_str(params.account.as_str()) {
        Ok(v) => v,
        Err(_) => {
            return Ok(with_status(
                json(&ErrorResponse {
                    error: "account is invalid".to_string(),
                }),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    let vaults = match evm_client.list_pub(account).await {
        Ok(vaults) => vaults,
        Err(err) => {
            return Ok(with_status(
                json(&ErrorResponse {
                    error: err.to_string(),
                }),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    Ok(with_status(json(&vaults), StatusCode::OK))
}

#[derive(Debug, Deserialize)]
pub struct FindVaultsByAccountParams {
    account: String,
}

pub async fn find_events_by_vault_id(
    path: String,
    pool: PgPool,
    params: FindEventsByPubIdQueryParams,
) -> Result<impl warp::Reply, Infallible> {
    let vault: Vault = match path.try_into() {
        Ok(p) => p,
        Err(err) => {
            return Ok(with_status(
                json(&ErrorResponse {
                    error: err.to_string(),
                }),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    let events = match db::pub_cids(
        &pool,
        &vault,
        params.limit(),
        params.offset(),
        params.before(),
        params.after(),
    )
    .await
    {
        Ok(events) => events,
        Err(_) => {
            return Ok(with_status(
                json(&ErrorResponse {
                    error: "failed to fetch events".to_string(),
                }),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    Ok(with_status(json(&events), StatusCode::OK))
}

impl TryFrom<String> for Vault {
    type Error = String;

    fn try_from(value: String) -> Result<Vault, String> {
        Vault::from(value.to_string())
    }
}

#[derive(Debug, Deserialize)]
pub struct FindEventsByPubIdQueryParams {
    limit: Option<i32>,
    offset: Option<i32>,
    before: Option<String>,
    after: Option<String>,
}

impl FindEventsByPubIdQueryParams {
    fn limit(&self) -> i32 {
        self.limit.unwrap_or(10)
    }

    fn offset(&self) -> i32 {
        self.offset.unwrap_or(0)
    }

    fn before(&self) -> i64 {
        if let Some(before) = &self.before {
            let before = match before.parse::<i64>() {
                Ok(timestamp) => return timestamp,
                Err(_) => before,
            };

            if let Ok(before) = DateTime::parse_from_str(before.as_str(), "%Y-%m-%d") {
                return before.timestamp();
            }

            if let Ok(before) = DateTime::parse_from_rfc3339(before.as_str()) {
                return before.timestamp();
            }

            // if we could not parse the datetime, use the default
            return 0;
        }
        0
    }

    fn after(&self) -> i64 {
        if let Some(after) = &self.after {
            let after = match after.parse::<i64>() {
                Ok(timestamp) => return timestamp,
                Err(_) => after,
            };

            if let Ok(after) = DateTime::parse_from_str(after.as_str(), "%Y-%m-%d") {
                return after.timestamp();
            }

            if let Ok(after) = DateTime::parse_from_rfc3339(after.as_str()) {
                return after.timestamp();
            }

            // if we could not parse the datetime, use the default
            return 0;
        }
        0
    }
}

pub async fn create_vault<E: EVMClient + 'static + std::marker::Sync>(
    path: String,
    evm_client: E,
    pool: PgPool,
    input: CreateVaultInput,
) -> Result<impl warp::Reply, Infallible> {
    let vault: Vault = match path.try_into() {
        Ok(p) => p,
        Err(err) => {
            return Ok(with_status(
                json(&ErrorResponse {
                    error: err.to_string(),
                }),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    let account: String = match &input.account {
        Some(v) => v.to_string(),
        None => {
            return Ok(with_status(
                json(&ErrorResponse {
                    error: "account is empty".to_string(),
                }),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    let address: Address = match Address::from_str(&account) {
        Ok(v) => v,
        Err(err) => {
            log::error!("{}", err);
            return Ok(with_status(
                json(&ErrorResponse {
                    error: "not a valid account".to_string(),
                }),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    match evm_client
        .add_pub(address, vault.to_string().as_str())
        .await
    {
        Ok(_) => {}
        Err(err) => {
            log::error!("{}", err);
            return Ok(with_status(
                json(&ErrorResponse {
                    error: "failed to create vault".to_string(),
                }),
                StatusCode::BAD_REQUEST,
            ));
        }
    }

    let created = match db::namespace_create(
        &pool,
        &vault.namespace(),
        &vault.relation(),
        input.cache,
        address,
    )
    .await
    {
        Ok(v) => v,
        Err(err) => {
            log::error!("{}", err);
            return Ok(with_status(
                json(&ErrorResponse {
                    error: "failed to create vault".to_string(),
                }),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    Ok(with_status(
        json(&CreateVaultResponse { created }),
        StatusCode::CREATED,
    ))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVaultInput {
    pub cache: Option<i64>,
    pub account: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateVaultResponse {
    created: bool,
}

#[allow(clippy::too_many_arguments)]
pub async fn write_event<W: Web3Storage>(
    path: String,
    size: u64,
    filename: String,
    gcs_client: GcsClient,
    w3s_client: W,
    pool: PgPool,
    params: WriteEventParams,
    mut stream: impl Stream<Item = Result<impl warp::Buf, warp::Error>> + Unpin + Send + Sync,
) -> Result<impl warp::Reply, Infallible> {
    let vault: Vault = match path.try_into() {
        Ok(p) => p,
        Err(err) => {
            return Ok(with_status(
                json(&ErrorResponse {
                    error: err.to_string(),
                }),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    log::info!("size = {}", size);

    match db::namespace_exists(&pool, &vault.namespace()).await {
        Ok(exists) => {
            if !exists {
                return Ok(with_status(
                    json(&ErrorResponse {
                        error: "namespace not found".to_string(),
                    }),
                    StatusCode::NOT_FOUND,
                ));
            }
        }
        Err(err) => {
            log::error!("{}", err);
            return Ok(with_status(
                json(&ErrorResponse {
                    error: "error checking the vault".to_string(),
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    }

    let cache_duration =
        match db::get_cache_config(&pool, &vault.namespace(), &vault.relation()).await {
            Ok(v) => v,
            Err(err) => {
                log::error!("{}", err);
                return Ok(with_status(
                    json(&ErrorResponse {
                        error: "error fetching the cache config".to_string(),
                    }),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        };

    let filename = format!(
        "{}/{}/{}-{}",
        vault.namespace(),
        vault.relation(),
        chrono::Utc::now().timestamp_micros(),
        filename
    );
    let upload_type = UploadType::Multipart(Box::new(Object {
        name: filename.to_string(),
        content_type: Some("application/octet-stream".into()),
        size: size as i64,
        metadata: None,
        ..Default::default()
    }));

    let uploader = gcs_client
        .inner
        .prepare_resumable_upload(
            &UploadObjectRequest {
                bucket: gcs_client.bucket.clone(),
                ..Default::default()
            },
            &upload_type,
        )
        .await;

    let gcs_uploader = match uploader {
        Ok(v) => v,
        Err(e) => {
            log::error!("{}", e);
            return Ok(with_status(
                json(&ErrorResponse {
                    error: "failed to upload event".to_string(),
                }),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    let hash_output = match upload_stream(gcs_uploader, stream.borrow_mut(), size).await {
        Ok(hasher) => hasher,
        Err(err) => {
            log::error!("{}", err);
            return Ok(with_status(
                json(&ErrorResponse {
                    error: "failed to upload event".to_string(),
                }),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    let signature = match &params.signature {
        Some(sig) => {
            let res = hex::decode(sig);
            if res.is_err() {
                return Ok(with_status(
                    json(&ErrorResponse {
                        error: "signature could not be decoded".to_string(),
                    }),
                    StatusCode::BAD_REQUEST,
                ));
            }

            res.unwrap()
        }
        None => {
            return Ok(with_status(
                json(&ErrorResponse {
                    error: "missing signature".to_string(),
                }),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    log::info!("content hash, hash={}", hash_output.encode_hex::<String>());

    let owner = match crypto::recover(&hash_output, &signature[..64], signature[64] as i32) {
        Ok(owner) => owner,
        Err(err) => {
            log::error!("{}", err);
            return Ok(with_status(
                json(&ErrorResponse {
                    error: "invalid signature".to_string(),
                }),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    log::info!(
        "checking ownership, vault={} owner={}",
        &vault.namespace(),
        owner.encode_hex::<String>()
    );

    match db::is_namespace_owner(&pool, &vault.namespace(), owner).await {
        Ok(is_owner) => {
            if !is_owner {
                return Ok(with_status(
                    json(&ErrorResponse {
                        error: "unauthorized".to_string(),
                    }),
                    StatusCode::UNAUTHORIZED,
                ));
            }
        }
        Err(err) => {
            log::error!("{}", err);
            return Ok(with_status(
                json(&ErrorResponse {
                    error: "failed to check authorization".to_string(),
                }),
                StatusCode::INTERNAL_SERVER_ERROR,
            ));
        }
    }

    let cid_bytes = match upload_w3s(gcs_client.clone(), w3s_client.clone(), &filename).await {
        Ok(cid) => cid,
        Err(err) => {
            log::error!("{}", err);
            return Ok(with_status(
                json(&ErrorResponse {
                    error: "failed to upload to w3s".to_string(),
                }),
                StatusCode::BAD_REQUEST,
            ));
        }
    };

    // Create the job in the database
    let job = db::create_job(
        &pool,
        &vault,
        cid_bytes,
        params.timestamp,
        filename,
        cache_duration,
        signature.to_vec(),
        hash_output.to_vec(),
    )
    .await;
    if let Err(err) = job {
        log::error!("{}", err);
        return Ok(with_status(
            json(&ErrorResponse {
                error: "failed to create job".to_string(),
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    let empty: Vec<u8> = Vec::new();
    Ok(with_status(json(&empty), StatusCode::CREATED))
}

#[derive(Deserialize)]
pub struct WriteEventParams {
    timestamp: Option<i64>,
    signature: Option<String>,
}

async fn upload_stream(
    uploader: ResumableUploadClient,
    stream: &mut (impl Stream<Item = Result<impl warp::Buf, warp::Error>> + Unpin + Send + Sync),
    size: u64,
) -> basin_common::errors::Result<[u8; 32]> {
    let mut hasher = Keccak::v256();

    const CHUNK_SIZE: usize = 8 * 1024 * 1024; // 8MiB. Must be multiple of 256KiB.
    let mut received: u64 = 0;
    let mut collected: Vec<u8> = Vec::new();
    loop {
        let first_byte = received;

        // first we try to collect as much bytes as possible until CHUNK_SIZE
        while let Some(buf) = stream.next().await {
            let mut buf = buf.unwrap();
            while buf.remaining() > 0 {
                let chunk = buf.chunk();
                let chunk_len = chunk.len();
                collected.extend_from_slice(chunk);
                buf.advance(chunk_len);
            }

            if collected.len() >= CHUNK_SIZE {
                break;
            }
        }

        // if we couldn't collect any more bytes to upload to GCS, then break out of the loop
        if collected.is_empty() {
            break;
        }

        let payload = collected
            .clone()
            .into_iter()
            .take(std::cmp::min(CHUNK_SIZE, collected.len()))
            .collect::<Vec<_>>();
        received += payload.len() as u64;
        let last_byte = received - 1;
        let chunk_size = ChunkSize::new(first_byte, last_byte, Some(size));

        uploader
            .upload_multiple_chunk(payload.clone(), &chunk_size)
            .await
            .map_err(|e| basin_common::errors::Error::Upload(e.to_string()))?;

        hasher.update(&payload);
        collected.drain(0..std::cmp::min(CHUNK_SIZE, collected.len()));
    }

    log::info!("received = {}", received);

    let mut output = [0u8; 32];
    hasher.finalize(&mut output);

    Ok(output)
}

async fn upload_w3s<W: Web3Storage>(
    gcs_client: GcsClient,
    w3s_client: W,
    filename: &str,
) -> basin_common::errors::Result<Vec<u8>> {
    let download_stream = gcs_client
        .inner
        .download_streamed_object(
            &GetObjectRequest {
                bucket: gcs_client.bucket.clone(),
                object: filename.to_string(),
                ..Default::default()
            },
            &Range::default(),
        )
        .await
        .map_err(|e| basin_common::errors::Error::Upload(e.to_string()))?;

    let res = w3s_client
        .upload(download_stream, filename)
        .await
        .map_err(|e| basin_common::errors::Error::Upload(e.to_string()))?;

    let cid = Cid::from(res.root.clone())
        .map_err(|e| basin_common::errors::Error::Upload(e.to_string()))?;
    log::info!("uploaded file: {:?}", res.root);

    Ok(cid.as_bytes())
}
