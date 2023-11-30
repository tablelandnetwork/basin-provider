use std::str::FromStr;

use crate::crypto;
use crate::db;
use crate::domain::Cid;
use crate::domain::Vault;
use crate::gcs::GcsClient;

use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, Result};
use basin_evm::EVMClient;
use chrono::DateTime;

use ethers::types::Address;
use futures::TryStreamExt;
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use google_cloud_storage::http::objects::{
    upload::{UploadObjectRequest, UploadType},
    Object,
};
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;
use std::collections::HashMap;
use tiny_keccak::{Hasher, Keccak};

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub async fn find_record_by_id(
    path: web::Path<String>,
    pool: web::Data<PgPool>,
    gcs_client: web::Data<GcsClient>,
) -> HttpResponse {
    let cid: Cid = match path.try_into() {
        Ok(v) => v,
        Err(err) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: err.to_string(),
            })
        }
    };

    let cache_path = match db::find_job_cache_path_by_cid(&pool, cid).await {
        Ok(v) => v,
        Err(err) => {
            log::error!("{}", err);
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "error fetching the record".to_string(),
            });
        }
    };

    if cache_path.is_none() {
        return HttpResponse::NotFound().finish();
    }

    let stream = match gcs_client
        .inner
        .download_streamed_object(
            &GetObjectRequest {
                bucket: gcs_client.bucket.to_string(),
                object: cache_path.unwrap().as_ref().to_string(),
                ..Default::default()
            },
            &Range::default(),
        )
        .await
    {
        Ok(s) => s,
        Err(err) => {
            log::error!("{}", err);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "failed to download cid".to_string(),
            });
        }
    };

    HttpResponse::Ok()
        .content_type(ContentType::octet_stream())
        .streaming(stream)
}

impl TryFrom<web::Path<std::string::String>> for Cid {
    type Error = String;

    fn try_from(value: web::Path<std::string::String>) -> Result<Cid, String> {
        Cid::from(value.to_string())
    }
}

pub async fn find_vaults_by_account<E: EVMClient + 'static + std::marker::Sync>(
    params: web::Query<FindVaultsByAccountParams>,
    evm_client: web::Data<E>,
) -> HttpResponse {
    let account = match Address::from_str(params.account.as_str()) {
        Ok(v) => v,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "account is invalid".to_string(),
            });
        }
    };

    let vaults = match evm_client.list_pub(account).await {
        Ok(vaults) => vaults,
        Err(err) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: err.to_string(),
            });
        }
    };

    HttpResponse::Ok().json(vaults)
}

#[derive(Debug, Deserialize)]
pub struct FindVaultsByAccountParams {
    account: String,
}

pub async fn find_records_by_vault_id(
    path: web::Path<String>,
    params: web::Query<FindRecordsByPubIdQueryParams>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let vault: Vault = match path.try_into() {
        Ok(p) => p,
        Err(err) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: err.to_string(),
            })
        }
    };

    let records = match db::pub_cids(
        &pool,
        &vault,
        params.limit(),
        params.offset(),
        params.before(),
        params.after(),
    )
    .await
    {
        Ok(records) => records,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "failed to fetch deals".to_string(),
            })
        }
    };

    HttpResponse::Ok().json(records)
}

impl TryFrom<web::Path<std::string::String>> for Vault {
    type Error = String;

    fn try_from(value: web::Path<std::string::String>) -> Result<Vault, String> {
        Vault::from(value.to_string())
    }
}

#[derive(Debug, Deserialize)]
pub struct FindRecordsByPubIdQueryParams {
    limit: Option<i32>,
    offset: Option<i32>,
    before: Option<String>,
    after: Option<String>,
}

impl FindRecordsByPubIdQueryParams {
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
    path: web::Path<String>,
    input: web::Form<CreateVaultInput>,
    evm_client: web::Data<E>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let vault: Vault = match path.try_into() {
        Ok(p) => p,
        Err(err) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: err.to_string(),
            })
        }
    };

    let account: String = match &input.account {
        Some(v) => v.to_string(),
        None => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "account is empty".to_string(),
            })
        }
    };

    let address: Address = match Address::from_str(&account) {
        Ok(v) => v,
        Err(err) => {
            log::error!("{}", err);
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "not a valid account".to_string(),
            });
        }
    };

    match evm_client
        .add_pub(address, vault.to_string().as_str())
        .await
    {
        Ok(_) => {}
        Err(err) => {
            log::error!("{}", err);
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "failed to create vault".to_string(),
            });
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
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "failed to create vault".to_string(),
            });
        }
    };

    HttpResponse::Ok().json(CreateVaultResponse { exists: created })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVaultInput {
    pub cache: Option<i64>,
    pub account: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateVaultResponse {
    exists: bool,
}

pub async fn write_record(
    path: web::Path<String>,
    params: web::Query<WriteRecordParams>,
    mut payload: web::Payload,
    gcs_client: web::Data<GcsClient>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let vault: Vault = match path.try_into() {
        Ok(p) => p,
        Err(err) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: err.to_string(),
            })
        }
    };

    match db::namespace_exists(&pool, &vault.namespace()).await {
        Ok(exists) => {
            if !exists {
                return HttpResponse::NotFound().json(ErrorResponse {
                    error: "namespace not found".to_string(),
                });
            }
        }
        Err(err) => {
            println!("{}", err);
            log::error!("{}", err);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "error checking the vault".to_string(),
            });
        }
    }

    let cache_duration =
        match db::get_cache_config(&pool, &vault.namespace(), &vault.relation()).await {
            Ok(v) => v,
            Err(err) => {
                log::error!("{}", err);
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "error fetching the cache config".to_string(),
                });
            }
        };

    let mut metadata: HashMap<String, String> = HashMap::from([(
        "timestamp".into(),
        format!(
            "{}",
            params
                .timestamp
                .unwrap_or(chrono::offset::Utc::now().timestamp())
        ),
    )]);

    if cache_duration.is_some() {
        metadata.insert(
            "cache_duration".into(),
            format!("{}", cache_duration.unwrap()),
        );
    }

    let filename = format!(
        "{}/{}/{}.parquet",
        vault.namespace(),
        vault.relation(),
        chrono::Utc::now().timestamp_micros()
    );
    let upload_type = UploadType::Multipart(Box::new(Object {
        name: filename.to_string(),
        content_type: Some("application/octet-stream".into()),
        metadata: Some(metadata),
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
        .await
        .unwrap();

    let mut hasher = Keccak::v256();

    while let Ok(Some(bytes)) = payload.try_next().await {
        let data = bytes.to_vec();
        let length = data.len();

        uploader
            .upload_single_chunk(data.clone(), length)
            .await
            .unwrap();
        hasher.update(&data);
    }

    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    let signature = match &params.signature {
        Some(sig) => {
            let res = hex::decode(sig);
            if res.is_err() {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: "signature could not be decoded".to_string(),
                });
            }

            res.unwrap()
        }
        None => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "missing signature".to_string(),
            })
        }
    };

    let owner = match crypto::recover(&output, &signature[..64], signature[64] as i32) {
        Ok(owner) => owner,
        Err(err) => {
            log::error!("{}", err);
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "invalid signature".to_string(),
            });
        }
    };

    match db::is_namespace_owner(&pool, &vault.namespace(), owner).await {
        Ok(is_owner) => {
            if !is_owner {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: "unauthorized".to_string(),
                });
            }
        }
        Err(err) => {
            log::error!("{}", err);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "failed to check authorization".to_string(),
            });
        }
    }

    HttpResponse::Created().finish()
}

#[derive(Deserialize)]
pub struct WriteRecordParams {
    timestamp: Option<i64>,
    signature: Option<String>,
}
