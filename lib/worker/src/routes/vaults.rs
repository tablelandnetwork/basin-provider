use std::str::FromStr;

use crate::db;
use crate::domain::Cid;
use crate::domain::Vault;
use crate::gcs::GcsClient;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, Result};
use basin_evm::EVMClient;
use chrono::DateTime;
use ethers::types::Address;
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use serde::Deserialize;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
struct ErrorResponse<'a> {
    error: &'a str,
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
                error: err.as_str(),
            })
        }
    };

    let cache_path = match db::find_job_cache_path_by_cid(&pool, cid).await {
        Ok(v) => v,
        Err(err) => {
            log::error!("{}", err);
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "error fetching the record",
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
                error: "failed to download cid",
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
                error: "account is invalid",
            });
        }
    };

    let vaults = match evm_client.list_pub(account).await {
        Ok(vaults) => vaults,
        Err(err) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: err.to_string().as_str(),
            });
        }
    };

    HttpResponse::Ok().json(vaults)
}

#[derive(Debug, Deserialize)]
pub struct FindVaultsByAccountParams {
    account: String,
}

pub async fn find_records_by_pub_id(
    path: web::Path<String>,
    params: web::Query<FindRecordsByPubIdQueryParams>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let vault: Vault = match path.try_into() {
        Ok(p) => p,
        Err(err) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: err.as_str(),
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
                error: "failed to fetch deals",
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
