mod crypto;
mod db;
mod http;
mod rpc;
mod sql;

use basin_evm::{testing::MockClient, BasinClient};
use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, ValueEnum};
use ethers::signers::LocalWallet;
use ethers::types::{Address, Chain};
use log::info;
use sqlx::postgres::PgPool;
use std::net::SocketAddr;
use stderrlog::Timestamp;
use warp::Filter;

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
    #[arg(long, env, default_value = "ws://127.0.0.1:8545")]
    evm_provider_url: String,

    /// Number of times Basin will attempt to reconnect to the provider
    #[arg(long, env, default_value_t = 10)]
    evm_provider_reconnects: usize,

    /// EVM chain ID
    #[arg(long, env, default_value_t = 31337)]
    evm_chain_id: usize,

    /// Postgres-style database URL
    #[arg(long, env)]
    database_url: String,

    /// CockroachDB changefeed sink
    #[arg(long, env)]
    changefeed_sink: String,

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum EvmType {
    /// Use an in-memory ephemeral EVM w/ random wallet (a BasinStorage contract will be deployed)
    Mem,
    /// Requires wallet private key, contract address, provider URL, and chain ID
    Remote,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    stderrlog::new()
        .module(module_path!())
        .quiet(args.quiet)
        .verbosity(args.verbosity as usize)
        .timestamp(Timestamp::Millisecond)
        .init()?;

    let warp_server = warp::serve(
        warp::path("health")
            .map(warp::reply)
            .recover(http::handle_warp_rejection),
    )
    .run(args.bind_health_address);
    tokio::spawn(async {
        info!("Basin Health API started");
        warp_server.await
    });

    let pg_pool = PgPool::connect(&args.database_url).await?;
    match args.evm_type {
        EvmType::Mem => {
            rpc::listen(
                args.bind_address,
                MockClient::new().await?,
                pg_pool,
                args.changefeed_sink,
            )
            .await
        }
        EvmType::Remote => {
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
                args.evm_provider_reconnects,
                chain_id,
            )
            .await?;

            rpc::listen(args.bind_address, evm_client, pg_pool, args.changefeed_sink).await
        }
    }
}
