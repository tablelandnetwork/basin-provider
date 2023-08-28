mod crypto;
mod db;
mod handlers;

use basin_protocol::publications;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use clap::Parser;
use futures::{AsyncReadExt, FutureExt};
use sqlx::postgres::PgPool;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    database_url: String,

    #[arg(short, long)]
    bind_address: SocketAddr,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let pool = PgPool::connect(&args.database_url).await?;
    listen(args.bind_address, pool).await
}

pub async fn listen(addr: SocketAddr, pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    tokio::task::LocalSet::new()
        .run_until(async move {
            let listener = tokio::net::TcpListener::bind(&addr).await?;
            let publications_client: publications::Client =
                capnp_rpc::new_client(handlers::Publications { pool });

            println!("Basin RPC service started");
            loop {
                let (stream, _) = listener.accept().await?;
                stream.set_nodelay(true)?;
                let (reader, writer) =
                    tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();

                let network = twoparty::VatNetwork::new(
                    reader,
                    writer,
                    rpc_twoparty_capnp::Side::Server,
                    Default::default(),
                );

                let rpc_system =
                    RpcSystem::new(Box::new(network), Some(publications_client.clone().client));

                tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));
            }
        })
        .await
}
