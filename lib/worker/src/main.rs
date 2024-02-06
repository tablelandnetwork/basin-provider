use basin_common::db;
use basin_evm::BasinClient;
use basin_worker::gcs::GcsClient;
use basin_worker::web3storage::Web3StorageImpl;
use clap::error::ErrorKind;
use clap::{arg, CommandFactory, Parser, ValueEnum};
use ethers::{
    signers::LocalWallet,
    types::{Address, Chain},
};
use log::info;
use sqlx::postgres::PgPool;
use std::net::SocketAddr;
use stderrlog::Timestamp;

use basin_worker::startup;
use std::time::Duration;
use tokio::{task, time};

#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

/// Command line args
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// EVM type (other EVM flags are ignored when this is 'mem')
    #[arg(long, value_enum, env, default_value_t = EvmType::Remote)]
    evm_type: EvmType,

    /// Wallet private key (ECDSA, secp256k1) to use with the EVM (must have PUB_ADMIN_ROLE)
    #[arg(long, env)]
    evm_wallet_pk: Option<String>,

    /// BasinStorage EVM contract address (ECDSA, secp256k1)
    #[arg(long, env)]
    evm_contract_address: Option<String>,

    /// EVM provider URL
    #[arg(long, env, default_value = "http://127.0.0.1:8545")]
    evm_provider_url: String,

    /// EVM chain ID
    #[arg(long, env, default_value_t = 31337)]
    evm_chain_id: usize,

    /// Parquet export GCS bucket
    #[arg(long, env)]
    export_bucket: String,

    /// Parquet export sink credentials
    #[arg(long, env)]
    export_credentials: String,

    /// GCS HTTP endpoint (used for testing)
    #[arg(long, env)]
    export_endpoint: Option<String>,

    /// Postgres-style database URL
    #[arg(long, env)]
    database_url: String,

    /// Web3Storage API token
    #[arg(long, env)]
    basin_w3s_endpoint: String,

    /// Host and port to bind the Health API to
    #[arg(long, env, default_value = "127.0.0.1:8080")]
    bind_address: SocketAddr,

    /// Logging verbosity (repeat for more verbose logging)
    #[arg(short, long, env, action = clap::ArgAction::Count)]
    verbosity: u8,

    /// Silence logging
    #[arg(short, long, env)]
    quiet: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum EvmType {
    /// Use an in-memory ephemeral EVM w/ random wallet (a BasinStorage contract will be deployed)
    Mem,
    /// Requires wallet private key, contract address, provider URL, and chain ID
    Remote,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    stderrlog::new()
        .module(module_path!())
        .quiet(args.quiet)
        .verbosity(args.verbosity as usize)
        .timestamp(Timestamp::Millisecond)
        .init()?;

    let pg_pool = PgPool::connect(&args.database_url).await?;
    db::setup(pg_pool.clone(), &args.database_url).await?;

    let pool = pg_pool.clone();
    let p = pool.clone();
    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(60));

        loop {
            interval.tick().await;
            let _ = basin_worker::db::delete_expired_job(&p).await;
        }
    });

    let gcs_client = GcsClient::new(
        args.export_bucket,
        args.export_credentials,
        args.export_endpoint,
    )
    .await?;
    let web3store_client = Web3StorageImpl::new(args.basin_w3s_endpoint);

    let mut cmd = Cli::command();

    let wallet_pk = match args.evm_wallet_pk {
                Some(s) => s.replace("0x", ""),
                None => cmd
                    .error(
                        ErrorKind::MissingRequiredArgument,
                        "the following required arguments were not provided: --evm-wallet-pk <EVM_WALLET_PK>",
                    )
                    .exit(),
            };
    let wallet_pk = match hex::decode(wallet_pk) {
        Ok(v) => v,
        Err(_) => cmd
            .error(
                ErrorKind::ValueValidation,
                "invalid hex value for --evm-wallet-pk <EVM_WALLET_PK>",
            )
            .exit(),
    };
    let wallet = match LocalWallet::from_bytes(wallet_pk.as_slice()) {
        Ok(w) => w,
        Err(_) => cmd
            .error(
                ErrorKind::ValueValidation,
                "invalid key value for --evm-wallet-pk <EVM_WALLET_PK>",
            )
            .exit(),
    };

    let contract_address = match args.evm_contract_address {
                Some(s) => s.replace("0x", ""),
                None => cmd
                    .error(
                        ErrorKind::MissingRequiredArgument,
                        "the following required arguments were not provided: --evm-contract-address <EVM_CONTRACT_ADDRESS>",
                    )
                    .exit(),
            };
    let contract_address = match hex::decode(contract_address) {
        Ok(v) => v,
        Err(_) => cmd
            .error(
                ErrorKind::ValueValidation,
                "invalid hex value for --evm-contract-address <EVM_CONTRACT_ADDRESS>",
            )
            .exit(),
    };

    let chain_id = match Chain::try_from(args.evm_chain_id) {
        Ok(c) => c,
        Err(_) => cmd
            .error(
                ErrorKind::ValueValidation,
                "invalid chain ID value for --evm-chain-id <EVM_CHAIN_ID>",
            )
            .exit(),
    };

    let evm_client = BasinClient::new(
        wallet,
        Address::from_slice(contract_address.as_slice()),
        args.evm_provider_url.as_str(),
        chain_id,
    )
    .await?;

    info!("HTTP API started");
    let (_, server) = startup::start_http_server(
        args.bind_address,
        pool,
        evm_client,
        gcs_client,
        web3store_client,
    );
    server.await;

    Ok(())
}
