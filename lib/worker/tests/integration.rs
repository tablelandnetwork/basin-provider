use basin_common::db;
use basin_common::errors::Result;
use basin_evm::testing::MockClient;
use basin_protocol::publications;
use basin_worker::{
    gcs::GcsClient, rpc, web3storage::Web3StorageClient, web3storage::DEFAULT_BASE_URL,
};

use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use ethers::{
    core::rand::{thread_rng, Rng},
    signers::{LocalWallet, Signer},
};
use futures::AsyncReadExt;
use secp256k1::{Message, Secp256k1, SecretKey};
use sqlx::PgPool;
use std::{net::SocketAddr, time::Duration};
use tiny_keccak::{Hasher, Keccak};
use tokio::{
    fs::File,
    io::{AsyncReadExt as _, AsyncWriteExt},
    io::{BufReader, BufWriter},
    task::{spawn_local, LocalSet},
    time::sleep,
};

async fn spawn_worker(pool: PgPool) -> SocketAddr {
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let bind_addr = listener.local_addr().unwrap();
    let gcs_client = GcsClient::new(
        std::env::var("EXPORT_BUCKET").unwrap(),
        std::env::var("EXPORT_CREDENTIALS").unwrap(),
        Some(std::env::var("EXPORT_CREDENTIALS").unwrap()),
    )
    .await
    .unwrap();

    let web3store_client = Web3StorageClient::new(DEFAULT_BASE_URL.to_string());

    spawn_local(async move {
        rpc::listen(
            MockClient::new().await.unwrap(),
            pool,
            gcs_client,
            web3store_client,
            listener,
        )
        .await
        .unwrap()
    });
    sleep(Duration::from_millis(5_000)).await;
    bind_addr
}

async fn get_client(worker_address: SocketAddr) -> publications::Client {
    let stream = tokio::net::TcpStream::connect(&worker_address)
        .await
        .unwrap();
    stream.set_nodelay(true).unwrap();
    let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
    let rpc_network = Box::new(twoparty::VatNetwork::new(
        reader,
        writer,
        rpc_twoparty_capnp::Side::Client,
        Default::default(),
    ));
    let mut rpc_system = RpcSystem::new(rpc_network, None);
    let client: publications::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);
    spawn_local(rpc_system);
    client
}

async fn get_pg_pool(db_name: String) -> (PgPool, String) {
    let host = std::env::var("DATABASE_HOST").unwrap();
    let url = format!("postgres://root@{}/{}?sslmode=disable", host, db_name);
    (PgPool::connect(&url).await.unwrap(), url)
}

#[tokio::test(flavor = "current_thread")]
#[ignore]
async fn create_publication_and_list_works() {
    let local = LocalSet::new();
    local
        .run_until(async {
            let db_name = rand_str(8);
            let (pool, db_url) = get_pg_pool(db_name).await;
            db::setup(pool.clone(), &db_url).await.unwrap();

            let worker_address = spawn_worker(pool.clone()).await;
            let client = get_client(worker_address).await;

            let wallet = LocalWallet::new(&mut thread_rng());

            let ns = rand_str(12);
            let rel = rand_str(12);

            let mut request = client.create_request();
            request.get().set_ns(ns.as_str().into());
            request.get().set_rel(rel.as_str().into());
            request.get().set_owner(wallet.address().as_bytes());
            request.get().set_cache_duration(0);

            let mut cols = request.get().init_schema().init_columns(1);
            {
                let mut c = cols.reborrow().get(0);
                c.set_name("id".into());
                c.set_type("SERIAL".into());
                c.set_is_nullable(false);
                c.set_is_part_of_primary_key(true);
            }

            let exists = request
                .send()
                .promise
                .await
                .unwrap()
                .get()
                .unwrap()
                .get_exists();
            assert!(!exists);

            let mut list_request = client.list_request();
            list_request.get().set_owner(wallet.address().as_bytes());

            let response = list_request.send().promise.await.unwrap();
            let publications = response.get().unwrap().get_publications().unwrap();
            let p = publications.get(0).unwrap().to_string().unwrap();

            assert_eq!(format!("{}.{}", ns, rel), p);

            // insert a job and request the deal
            let (_, data) =
                multibase::decode("bafybeibw2zctx4ca3udcfcsizjmo57bomhb6vvzf63rvc25d6hzotncn2i")
                    .unwrap();
            pub_jobs_insert(
                &pool,
                ns.as_str(),
                rel.as_str(),
                data,
                chrono::NaiveDateTime::from_timestamp_millis(1699475142).unwrap(),
                1698763113,
            )
            .await
            .unwrap();

            // fetch timestamp in range
            {
                let mut deals_request = client.latest_deals_request();
                deals_request.get().set_ns(ns.as_str().into());
                deals_request.get().set_rel(rel.as_str().into());
                deals_request.get().set_n(1);
                deals_request.get().set_after(1698763112);
                deals_request.get().set_before(1698763114);

                let response = deals_request.send().promise.await.unwrap();
                let deals = response.get().unwrap().get_deals().unwrap();

                assert_eq!(1, deals.len());

                assert_eq!(
                    "bafybeibw2zctx4ca3udcfcsizjmo57bomhb6vvzf63rvc25d6hzotncn2i",
                    deals.get(0).get_cid().unwrap()
                );
                assert_eq!(1698763113, deals.get(0).get_timestamp());
                assert!(deals.get(0).get_archived());
                assert_eq!(380733, deals.get(0).get_size());
            }

            // fetch wrong timestamp
            {
                let mut deals_request = client.latest_deals_request();
                deals_request.get().set_ns(ns.as_str().into());
                deals_request.get().set_rel(rel.as_str().into());
                deals_request.get().set_n(1);
                deals_request.get().set_after(1698763114);

                let response = deals_request.send().promise.await.unwrap();
                let deals = response.get().unwrap().get_deals().unwrap();

                assert_eq!(0, deals.len());
            }

            db::drop(pool.clone(), &db_url).await.unwrap();
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
#[ignore]
async fn upload_publication_works() {
    let local = LocalSet::new();
    local
        .run_until(async {
            let db_name = rand_str(8);
            let (pool, db_url) = get_pg_pool(db_name).await;
            db::setup(pool.clone(), &db_url).await.unwrap();

            let worker_address = spawn_worker(pool.clone()).await;
            let client = get_client(worker_address).await;

            let wallet = LocalWallet::new(&mut thread_rng());
            let secp = Secp256k1::new();
            let pk = SecretKey::from_slice(&wallet.signer().to_bytes()).unwrap();

            let ns = rand_str(12);
            let rel = rand_str(12);

            let mut request = client.create_request();
            request.get().set_ns(ns.as_str().into());
            request.get().set_rel(rel.as_str().into());
            request.get().set_owner(wallet.address().as_bytes());
            request.get().set_cache_duration(0);
            request.send().promise.await.unwrap();

            let size = 16 * 1024 * 1024 + 256;
            let mut request = client.upload_request();
            request.get().set_ns(ns.as_str().into());
            request.get().set_rel(rel.as_str().into());
            request.get().set_size(size as u64);
            request.get().set_timestamp(1699472234);

            let callback = request.send().pipeline.get_callback();

            let mut reader = rand_file(size).await;
            let mut hasher = Keccak::v256();
            let mut buffer = vec![0u8; 8 * 1024 * 1024];
            loop {
                let n = reader.read(&mut buffer).await.unwrap();
                if n == 0 {
                    break;
                }
                let c = &buffer[..n];
                hasher.update(c);
                let mut write_request = callback.write_request();
                write_request.get().set_chunk(c);
                write_request.send().promise.await.unwrap();
            }

            let mut done_request = callback.done_request();
            let mut output = [0u8; 32];
            hasher.finalize(&mut output);
            let msg = Message::from_slice(&output).unwrap();
            let (rid, sig) = secp.sign_ecdsa_recoverable(&msg, &pk).serialize_compact();
            let mut sigb = Vec::with_capacity(65);
            sigb.extend_from_slice(&sig);
            sigb.push(rid.to_i32() as u8);
            done_request.get().set_sig(&sigb);
            done_request.send().promise.await.unwrap();

            db::drop(pool.clone(), &db_url).await.unwrap();
        })
        .await;
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

async fn rand_file(s: usize) -> BufReader<File> {
    let p = format!("/tmp/{}", rand_str(8));
    let f = File::create(&p).await.unwrap();
    let mut writer = BufWriter::new(f);

    let mut rng = thread_rng();
    let mut buffer = vec![0u8; 1024 * 1024];
    let mut remaining_size = s;

    while remaining_size > 0 {
        let to_write = std::cmp::min(remaining_size, buffer.len());
        let buffer = &mut buffer[..to_write];
        rng.fill(buffer);
        _ = writer.write(buffer).await.unwrap();

        remaining_size -= to_write;
    }
    writer.flush().await.unwrap();

    let f = File::open(&p).await.unwrap();
    BufReader::new(f)
}

async fn pub_jobs_insert(
    pool: &PgPool,
    ns: &str,
    rel: &str,
    cid: Vec<u8>,
    activated: chrono::NaiveDateTime,
    timestamp: i64,
) -> Result<()> {
    sqlx::query!(
        "INSERT INTO jobs (ns_id, cid, relation, activated, timestamp) SELECT id, $2, $3, $4, $5 FROM namespaces WHERE name = $1",
        ns,
        cid,
        rel,
        activated,
        timestamp
    )
    .execute(pool)
    .await?;

    Ok(())
}
