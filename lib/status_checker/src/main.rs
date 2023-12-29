use basin_evm::BasinClient;
use basin_worker::web3storage;
use clap::error::ErrorKind;
use clap::{arg, CommandFactory, Parser};
use ethers::{
    signers::LocalWallet,
    types::{Address, Chain},
};
use sqlx::postgres::PgPool;
use status_checker::status_checker::process_jobs;
use stderrlog::Timestamp;

/// Command line args
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Postgres-style database URL
    #[arg(long, env)]
    database_url: String,

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

    let w3s_client = web3storage::Web3StorageClient::new(web3storage::DEFAULT_BASE_URL.to_string());

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

    process_jobs(pg_pool, w3s_client, evm_client).await?;

    Ok(tokio::signal::ctrl_c().await?)
}
