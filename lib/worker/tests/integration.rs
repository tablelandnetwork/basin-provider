use basin_evm::testing::MockClient;
use basin_protocol::publications;
use basin_worker::{db, rpc, utils};
use capnp::capability::Request;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use ethers::{
    core::rand,
    signers::{LocalWallet, Signer},
    utils::keccak256,
};
use futures::AsyncReadExt;
use rand::{thread_rng, Rng};
use secp256k1::{Message, Secp256k1, SecretKey};
use sqlx::PgPool;
use std::{net::SocketAddr, time::Duration};
use tokio::{
    task::{spawn_local, LocalSet},
    time::sleep,
};

async fn spawn_worker(pool: PgPool) -> SocketAddr {
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    let bind_addr = listener.local_addr().unwrap();
    spawn_local(async move {
        rpc::listen(MockClient::new().await.unwrap(), pool, listener)
            .await
            .unwrap()
    });
    sleep(Duration::from_millis(5_000)).await;
    bind_addr
}

async fn spawn_exporter(pool: PgPool) {
    spawn_local(async {
        let interval = std::env::var("EXPORT_INTERVAL").unwrap();
        basin_exporter::start(
            pool,
            std::env::var("EXPORT_SINK").unwrap(),
            "specified".to_string(),
            std::env::var("EXPORT_CREDENTIALS").unwrap(),
            parse_duration(&interval).unwrap(),
        )
        .await
        .unwrap()
    });
}

fn parse_duration(arg: &str) -> Result<Duration, humantime::DurationError> {
    arg.parse::<humantime::Duration>().map(Into::into)
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
async fn create_publication_works() {
    let local = LocalSet::new();
    local
        .run_until(async {
            let db_name = rand_str(8);
            let (pool, db_url) = get_pg_pool(db_name).await;
            db::setup(pool.clone(), &db_url).await.unwrap();

            let worker_address = spawn_worker(pool.clone()).await;
            let client = get_client(worker_address).await;

            let wallet = LocalWallet::new(&mut thread_rng());

            let mut request = client.create_request();
            request.get().set_ns(&rand_str(12));
            request.get().set_rel(&rand_str(12));
            request.get().set_owner(wallet.address().as_bytes());

            let mut cols = request.get().init_schema().init_columns(1);
            {
                let mut c = cols.reborrow().get(0);
                c.set_name("id");
                c.set_type("SERIAL");
                c.set_is_nullable(false);
                c.set_is_part_of_primary_key(true);
            }

            request.send().promise.await.unwrap();

            db::drop(pool.clone(), &db_url).await.unwrap();
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn push_publication_works() {
    let local = LocalSet::new();
    local
        .run_until(async {
            let db_name = rand_str(8);
            let (pool, db_url) = get_pg_pool(db_name).await;
            db::setup(pool.clone(), &db_url).await.unwrap();

            let worker_address = spawn_worker(pool.clone()).await;
            spawn_exporter(pool.clone()).await;
            let client = get_client(worker_address).await;

            let wallet = LocalWallet::new(&mut thread_rng());
            let ns = rand_str(12);
            let rel = rand_str(12);

            let mut request = client.create_request();
            request.get().set_ns(&ns);
            request.get().set_rel(&rel);
            request.get().set_owner(wallet.address().as_bytes());
            let mut cols = request.get().init_schema().init_columns(3);
            {
                let mut c = cols.reborrow().get(0);
                c.set_name("id");
                c.set_type("SERIAL");
                c.set_is_nullable(false);
                c.set_is_part_of_primary_key(true);
            }
            {
                let mut c = cols.reborrow().get(1);
                c.set_name("msg");
                c.set_type("TEXT");
                c.set_is_nullable(true);
                c.set_is_part_of_primary_key(false);
            }
            {
                let mut c = cols.reborrow().get(2);
                c.set_name("val");
                c.set_type("REAL");
                c.set_is_nullable(true);
                c.set_is_part_of_primary_key(false);
            }
            request.send().promise.await.unwrap();

            let mut request = client.push_request();
            request.get().set_ns(&ns);
            request.get().set_rel(&rel);
            rand_records(&mut request, wallet, 10);

            request.send().promise.await.unwrap();

            db::drop(pool.clone(), &db_url).await.unwrap();
        })
        .await;
}

fn rand_records(
    req: &mut Request<publications::push_params::Owned, publications::push_results::Owned>,
    wallet: LocalWallet,
    num: u32,
) {
    let secp = Secp256k1::new();
    let pk = SecretKey::from_slice(&wallet.signer().to_bytes().to_vec()).unwrap();

    let mut recs = req.get().init_tx().init_records(num);
    for i in 0..num {
        let mut r = recs.reborrow().get(i);
        r.set_action("I");
        let mut cols = r.init_columns(3);
        {
            let mut c = cols.reborrow().get(0);
            c.set_name("id");
            let id = i + 1;
            c.set_value(serde_json::to_string(&id).unwrap().as_bytes());
        }
        {
            let mut c = cols.reborrow().get(1);
            c.set_name("msg");
            let m = rand_str(16);
            c.set_value(serde_json::to_string(&m).unwrap().as_bytes());
        }
        {
            let mut c = cols.reborrow().get(2);
            c.set_name("val");
            let mut rng = rand::thread_rng();
            let v = rng.gen::<f64>();
            c.set_value(serde_json::to_string(&v).unwrap().as_bytes());
        }
    }

    let tx = utils::canonicalize_tx(req.get().get_tx().unwrap().reborrow_as_reader()).unwrap();
    let hash = keccak256(&tx);
    let msg = Message::from_slice(&hash).unwrap();
    let (rid, sig) = secp.sign_ecdsa_recoverable(&msg, &pk).serialize_compact();
    let mut sigb = Vec::with_capacity(65);
    sigb.extend_from_slice(&sig);
    sigb.push(rid.to_i32() as u8);
    req.get().set_sig(&sigb);
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
