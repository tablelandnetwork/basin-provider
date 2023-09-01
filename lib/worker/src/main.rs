mod crypto;
mod db;
mod handlers;
mod helpers;

use basin_protocol::publications;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use clap::Parser;
use futures::{AsyncReadExt, FutureExt};
use log::info;
use serde::Serialize;
use sqlx::postgres::PgPool;
use std::{convert::Infallible, net::SocketAddr};
use stderrlog::Timestamp;
use warp::{http::StatusCode, Filter, Rejection, Reply};

/// Command line args
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    database_url: String,

    #[arg(long)]
    bind_address: SocketAddr,

    #[arg(long)]
    bind_health_address: SocketAddr,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbosity: u8,

    #[arg(short, long)]
    quiet: bool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    stderrlog::new()
        .module(module_path!())
        .quiet(args.quiet)
        .verbosity(args.verbosity as usize)
        .timestamp(Timestamp::Millisecond)
        .init()?;

    let warp_server = warp::serve(
        warp::path("health")
            .map(warp::reply)
            .recover(handle_warp_rejection),
    )
    .run(args.bind_health_address);
    tokio::spawn(async {
        info!("Basin HTTP API started");
        warp_server.await
    });

    listen(
        args.bind_address,
        PgPool::connect(&args.database_url).await?,
    )
    .await
}

/// Listens for RPC messages from Basin clients
pub async fn listen(addr: SocketAddr, pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    tokio::task::LocalSet::new()
        .run_until(async move {
            let listener = tokio::net::TcpListener::bind(&addr).await?;
            let publications_client: publications::Client =
                capnp_rpc::new_client(handlers::Publications { pool });

            info!("Basin RPC service started");
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

/// Error message used by HTTP API
#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

// HTTP API rejection handler
async fn handle_warp_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };
    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message,
    });
    Ok(warp::reply::with_status(json, code))
}
