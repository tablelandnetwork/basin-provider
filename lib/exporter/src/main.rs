use basin_exporter::start;
use clap::{arg, Parser};
use sqlx::postgres::PgPool;
use std::net::SocketAddr;
use stderrlog::Timestamp;

#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

/// Command line args
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Parquet export GCS bucket
    #[arg(long, env)]
    export_bucket: String,

    /// Parquet export sink credentials
    #[arg(long, env)]
    export_credentials: String,

    /// Parquet export crontab schedule
    #[arg(long, env, default_value = "0 0 0 * * *")]
    export_schedule: String,

    /// Postgres-style database URL
    #[arg(long, env)]
    database_url: String,

    /// Host and port to bind the RPC API to
    #[arg(long, env, default_value = "127.0.0.1:3000")]
    bind_address: SocketAddr,

    /// Host and port to bind the Health API to
    #[arg(long, env, default_value = "127.0.0.1:3001")]
    bind_health_address: SocketAddr,

    /// Logging verbosity (repeat for more verbose logging)
    #[arg(short, long, env, action = clap::ArgAction::Count)]
    verbosity: u8,

    /// Silence logging
    #[arg(short, long, env)]
    quiet: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    stderrlog::new()
        .module(module_path!())
        .quiet(args.quiet)
        .verbosity(args.verbosity as usize)
        .timestamp(Timestamp::Millisecond)
        .init()?;

    let pg_pool = PgPool::connect(&args.database_url).await?;
    start(
        pg_pool.clone(),
        args.export_bucket,
        args.export_credentials,
        &args.export_schedule,
    )
    .await?;

    Ok(tokio::signal::ctrl_c().await?)
}
