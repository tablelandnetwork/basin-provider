use std::str::FromStr;

use basin_common::db;
use basin_evm::testing::MockClient;
use basin_evm::EVMClient;
use basin_worker::web3storage::{Web3StorageClient, DEFAULT_BASE_URL};
use ethers::{
    core::rand::{thread_rng, Rng},
    types::Address,
};
use sqlx::{types::chrono::NaiveDateTime, PgPool, Row};
use status_checker::status_checker::process_jobs;

pub struct TestApp {
    pub db_pool: PgPool,
    pub evm_client: MockClient,
    pub w3s_client: Web3StorageClient,
}

impl TestApp {
    async fn add_unfinished_job(&self, address: &str, cid: Vec<u8>) {
        let address = Address::from_str(address).unwrap();
        let address = address.as_bytes();
        let _ = sqlx::query!(
            "INSERT INTO namespaces (id, name, owner) values($1, $2, $3)",
            925450265432784897,
            "n",
            address,
        )
        .execute(&self.db_pool)
        .await
        .unwrap();

        let _ = sqlx::query!(
            "INSERT INTO jobs (ns_id, cid, relation, timestamp) values($1, $2, $3, $4)",
            925450265432784897,
            cid,
            "test",
            0
        )
        .execute(&self.db_pool)
        .await
        .unwrap();
    }

    async fn add_vault_to_contract(&self, address: &str) {
        let address = Address::from_str(address).unwrap();
        self.evm_client.add_pub(address, "n.test").await.unwrap();
    }
}

#[tokio::test]
async fn process_job_works() {
    let app = spawn_app().await;

    let account = "0x00FEEc1fC91074f5F38a8FC5129dbc4FD204eca6";

    let cid_hex = "0170122048fc65647160540fa525a0fdf02e77e87eed08c18bffa214bfa8a2e4eda26a03";
    let cid: Vec<u8> = hex::decode(cid_hex).unwrap();

    // setup
    app.add_unfinished_job(account, cid.clone()).await;
    app.add_vault_to_contract(account).await;

    process_jobs(
        app.db_pool.clone(),
        app.w3s_client.clone(),
        app.evm_client.clone(),
    )
    .await
    .unwrap();

    // assert
    let record =
        sqlx::query(format!("SELECT activated FROM jobs WHERE cid = '\\x{}'", cid_hex).as_str())
            .fetch_one(&app.db_pool)
            .await
            .unwrap();

    let activated: Option<NaiveDateTime> = record.get("activated");

    assert_eq!(
        NaiveDateTime::from_str("2023-12-17T17:45:00").ok(),
        activated
    );
}

async fn spawn_app() -> TestApp {
    let evm_client = MockClient::new().await.unwrap();
    let db_name = rand_str(8);

    let (db_pool, db_url) = get_pg_pool(db_name).await;
    db::setup(db_pool.clone(), &db_url).await.unwrap();

    let w3s_client = Web3StorageClient::new(DEFAULT_BASE_URL.to_string());

    TestApp {
        db_pool,
        evm_client,
        w3s_client,
    }
}

async fn get_pg_pool(db_name: String) -> (PgPool, String) {
    let host = std::env::var("DATABASE_HOST").unwrap();
    let url = format!("postgres://root@{}/{}?sslmode=disable", host, db_name);
    (PgPool::connect(&url).await.unwrap(), url)
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
