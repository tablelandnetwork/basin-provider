use basin_common::db;
use basin_common::errors::Result;
use basin_evm::testing::MockClient;
use basin_evm::EVMClient;
use basin_worker::gcs::GcsClient;
use chrono::NaiveDateTime;
use ethers::core::rand::{thread_rng, Rng};
use ethers::types::{Address, H160};
use reqwest::Response;
use sqlx::PgPool;
use std::net::TcpListener;
use std::str::FromStr;

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let db_name = rand_str(8);

    let (db_pool, db_url) = get_pg_pool(db_name).await;
    db::setup(db_pool.clone(), &db_url).await.unwrap();

    let gcs_client = GcsClient::new(
        std::env::var("EXPORT_BUCKET").unwrap(),
        std::env::var("EXPORT_CREDENTIALS").unwrap(),
    )
    .await
    .unwrap();

    let evm_client = MockClient::new().await.unwrap();

    let server = basin_worker::startup::start_http_server(
        listener,
        db_pool.clone(),
        evm_client.clone(),
        gcs_client.clone(),
    )
    .expect("Failed to bind address");

    tokio::spawn(server);

    let address = format!("{}:{}", "http://127.0.0.1", port);
    let account = Address::from_str("0x9e93F8a7A1035EF984979814ae79B53270cd306A").unwrap();
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
    pub account: H160,
}

impl TestApp {
    pub async fn create_vault(&self, name: String) {
        let _ = self.evm_client.add_pub(self.account, name.as_str()).await;

        let parts: Vec<&str> = name.split('.').collect();
        let _ = basin_worker::db::namespace_create(
            &self.db_pool,
            parts[0],
            parts[1],
            None,
            self.account,
        )
        .await;
    }

    pub async fn write_record(
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

    pub async fn get_vaults(&self) -> Response {
        self.api_client
            .get(&format!(
                "{}/vaults?account={:#x}",
                &self.address, self.account
            ))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_records_from_vaults(&self, vault: &str) -> Response {
        self.api_client
            .get(&format!("{}/vaults/{}/records", &self.address, vault))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_records_from_vaults_limit_and_offset(
        &self,
        vault: &str,
        limit: i32,
        offset: i32,
    ) -> Response {
        self.api_client
            .get(&format!(
                "{}/vaults/{}/records?limit={}&offset={}",
                &self.address, vault, limit, offset
            ))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_record(&self, cid: &str) -> Response {
        self.api_client
            .get(&format!("{}/records/{}", &self.address, cid))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_records_from_vaults_before_and_after(
        &self,
        vault: &str,
        before: i32,
        after: i32,
    ) -> Response {
        self.api_client
            .get(&format!(
                "{}/vaults/{}/records?after={}&before={}",
                &self.address, vault, after, before
            ))
            .send()
            .await
            .expect("Failed to execute request.")
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
