use basin_common::db;
use basin_common::ecmh::Hasher;
use basin_common::errors::Result;
use basin_evm::testing::MockClient;
use basin_worker::gcs::GcsClient;
use basin_worker::routes::CreateVaultInput;
use chrono::NaiveDateTime;
use ethers::{
    core::{
        k256::ecdsa::SigningKey,
        rand::{thread_rng, Rng},
    },
    signers::{LocalWallet, Signer, Wallet},
};
use google_cloud_storage::http::{
    buckets::insert::{BucketCreationConfig, InsertBucketParam, InsertBucketRequest},
    objects::upload::{Media, UploadObjectRequest, UploadType},
};
use reqwest::Response;
use secp256k1::{Message, Secp256k1, SecretKey};
use sqlx::PgPool;
use std::net::SocketAddr;

pub async fn spawn_app() -> TestApp {
    let addr = "127.0.0.1:0".parse::<SocketAddr>().unwrap();
    let db_name = rand_str(8);

    let (db_pool, db_url) = get_pg_pool(db_name).await;
    db::setup(db_pool.clone(), &db_url).await.unwrap();

    let gcs_client = GcsClient::new(
        std::env::var("EXPORT_BUCKET").unwrap(),
        std::env::var("EXPORT_CREDENTIALS").unwrap(),
        std::env::var("EXPORT_ENDPOINT").ok(),
    )
    .await
    .unwrap();

    let _ = gcs_client
        .inner
        .insert_bucket(&InsertBucketRequest {
            name: std::env::var("EXPORT_BUCKET").unwrap(),
            bucket: BucketCreationConfig {
                ..Default::default()
            },
            param: InsertBucketParam {
                ..Default::default()
            },
        })
        .await
        .unwrap_err();

    let evm_client = MockClient::new().await.unwrap();

    let (addr, server) = basin_worker::startup::start_http_server(
        addr,
        db_pool.clone(),
        evm_client.clone(),
        gcs_client.clone(),
    );

    tokio::spawn(server);

    let address = format!("{}:{}", "http://127.0.0.1", addr.port());
    let account = LocalWallet::new(&mut thread_rng());
    let api_client = reqwest::Client::builder().build().unwrap();

    TestApp {
        address,
        db_pool,
        gcs_client,
        evm_client,
        api_client,
        account,
    }
}

async fn get_pg_pool(db_name: String) -> (PgPool, String) {
    let host = std::env::var("DATABASE_HOST").unwrap();
    let url = format!("postgres://root@{}/{}?sslmode=disable", host, db_name);
    (PgPool::connect(&url).await.unwrap(), url)
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
    pub api_client: reqwest::Client,
    pub evm_client: MockClient,
    pub gcs_client: GcsClient,
    pub account: Wallet<SigningKey>,
}

impl TestApp {
    pub async fn health_status(&self) -> Response {
        self.api_client
            .get(&format!("{}/health", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn create_vault(&self, name: &str) {
        self.api_client
            .post(&format!("{}/vaults/{}", &self.address, name))
            .form::<CreateVaultInput>(&CreateVaultInput {
                cache: None,
                account: Some(format!("{:#x}", self.account.address())),
            })
            .send()
            .await
            .expect("Failed to execute request.");
    }

    pub async fn create_vault_with_cache(&self, name: &str, cache: i64) {
        self.api_client
            .post(&format!("{}/vaults/{}", &self.address, name))
            .form::<CreateVaultInput>(&CreateVaultInput {
                cache: Some(cache),
                account: Some(format!("{:#x}", self.account.address())),
            })
            .send()
            .await
            .expect("Failed to execute request.");
    }

    pub async fn get_vaults(&self) -> Response {
        self.api_client
            .get(&format!(
                "{}/vaults?account={:#x}",
                &self.address,
                self.account.address()
            ))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_events_from_vaults(&self, vault: &str) -> Response {
        self.api_client
            .get(&format!("{}/vaults/{}/events", &self.address, vault))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_events_from_vaults_limit_and_offset(
        &self,
        vault: &str,
        limit: i32,
        offset: i32,
    ) -> Response {
        self.api_client
            .get(&format!(
                "{}/vaults/{}/events?limit={}&offset={}",
                &self.address, vault, limit, offset
            ))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_event(&self, cid: &str) -> Response {
        self.api_client
            .get(&format!("{}/events/{}", &self.address, cid))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_events_from_vaults_before_and_after(
        &self,
        vault: &str,
        before: i32,
        after: i32,
    ) -> Response {
        self.api_client
            .get(&format!(
                "{}/vaults/{}/events?after={}&before={}",
                &self.address, vault, after, before
            ))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn upload_event(
        &self,
        vault: &str,
        timestamp: i64,
        event_content: [u8; 256],
    ) -> Response {
        // calculating hash
        let mut hasher = Hasher::new();
        hasher.update(&event_content[..256]);
        let mut output = [0u8; 32];
        hasher.finalize(&mut output);

        // signing
        let secp = Secp256k1::new();
        let pk = SecretKey::from_slice(&self.account.signer().to_bytes()).unwrap();
        let msg = Message::from_slice(&output).unwrap();
        let (rid, sig) = secp.sign_ecdsa_recoverable(&msg, &pk).serialize_compact();
        let mut sigb = Vec::with_capacity(65);
        sigb.extend_from_slice(&sig);
        sigb.push(rid.to_i32() as u8);

        self.api_client
            .post(&format!(
                "{}/vaults/{}/events?timestamp={}&signature={}",
                &self.address,
                vault,
                timestamp,
                hex::encode(sigb)
            ))
            .body(event_content.to_vec())
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn write_event_to_db(
        &self,
        ns: &str,
        rel: &str,
        cid: &str,
        timestamp: i64,
        cache_path: Option<String>,
        expires_at: Option<NaiveDateTime>,
    ) {
        let (_, data) = multibase::decode(cid).unwrap();

        pub_jobs_insert(
            &self.db_pool,
            ns,
            rel,
            data,
            timestamp,
            cache_path,
            expires_at,
        )
        .await
        .unwrap();
    }

    pub async fn write_data_to_gcs(&self, filename: String, data: String) {
        let _ = self
            .gcs_client
            .inner
            .upload_object(
                &UploadObjectRequest {
                    bucket: self.gcs_client.bucket.clone(),
                    ..Default::default()
                },
                data,
                &UploadType::Simple(Media::new(filename)),
            )
            .await
            .unwrap();
    }
}

fn rand_str(l: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    let mut rng = thread_rng();
    let s: String = (0..l)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    s.to_lowercase()
}

async fn pub_jobs_insert(
    pool: &PgPool,
    ns: &str,
    rel: &str,
    cid: Vec<u8>,
    timestamp: i64,
    cache_path: Option<String>,
    expires_at: Option<NaiveDateTime>,
) -> Result<()> {
    sqlx::query!(
        "INSERT INTO jobs (ns_id, cid, relation, timestamp, cache_path, expires_at) SELECT id, $2, $3, $4, $5, $6 FROM namespaces WHERE name = $1",
        ns,
        cid,
        rel,
        timestamp,
        cache_path,
        expires_at
    )
    .execute(pool)
    .await?;

    Ok(())
}
