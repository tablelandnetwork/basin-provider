use basin_exporter::start;
use clap::{arg, Parser};
use sqlx::postgres::PgPool;
use std::{net::SocketAddr, time::Duration};
use stderrlog::Timestamp;

/// Command line args
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// CockroachDB export sink
    #[arg(long, env)]
    export_sink: String,

    /// CockroachDB export sink
    #[arg(long, env, default_value = "specified")]
    export_auth: String,

    /// CockroachDB export sink
    #[arg(long, env)]
    export_credentials: String,

    /// CockroachDB export interval
    #[arg(long, env, value_parser = parse_duration, default_value = "24h")]
    export_interval: Duration,

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

fn parse_duration(arg: &str) -> Result<Duration, humantime::DurationError> {
    arg.parse::<humantime::Duration>().map(Into::into)
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
        args.export_sink,
        args.export_auth,
        args.export_credentials,
        args.export_interval,
    )
    .await?;

    Ok(tokio::signal::ctrl_c().await?)
}
