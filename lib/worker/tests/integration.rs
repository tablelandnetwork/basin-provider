use basin_evm::testing::MockClient;
use basin_protocol::publications;
use basin_worker::{exporter, rpc, utils};
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

async fn spawn_worker() -> SocketAddr {
    spawn_local(async {
        let bind_address: SocketAddr = std::env::var("BIND_ADDRESS").unwrap().parse().unwrap();
        let database_url = std::env::var("DATABASE_URL").unwrap();
        let pg_pool = PgPool::connect(&database_url).await.unwrap();
        rpc::listen(bind_address, MockClient::new().await.unwrap(), pg_pool)
            .await
            .unwrap()
    });
    sleep(Duration::from_millis(5_000)).await;
    std::env::var("BIND_ADDRESS").unwrap().parse().unwrap()
}

async fn spawn_exporter() {
    spawn_local(async {
        let database_url = std::env::var("DATABASE_URL").unwrap();
        let pg_pool = PgPool::connect(&database_url).await.unwrap();
        let sink = std::env::var("EXPORT_SINK").unwrap();
        let schedule = std::env::var("EXPORT_SCHEDULE").unwrap();
        exporter::start_exporter(pg_pool, sink, parse_duration(&schedule).unwrap())
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

#[tokio::test(flavor = "current_thread")]
async fn create_publication_works() {
    let local = LocalSet::new();
    local
        .run_until(async {
            let worker_address = spawn_worker().await;
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
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn push_publication_works() {
    let local = LocalSet::new();
    local
        .run_until(async {
            let worker_address = spawn_worker().await;
            spawn_exporter().await;
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

            // tokio::time::sleep(Duration::from_secs(30)).await;
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
