use crate::Contract;
use crate::EVMClient;
use crate::{errors::Result, Error};
use async_trait::async_trait;
use ethers::{
    middleware::SignerMiddleware,
    providers::{Provider, Ws},
    signers::{LocalWallet, Signer},
    types::{Address, Chain},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct BasinClient {
    contract: Contract<SignerMiddleware<Provider<Ws>, LocalWallet>>,
}

impl BasinClient {
    pub async fn new(
        wallet: LocalWallet,
        contract_address: Address,
        provider_url: &str,
        provider_reconnects: usize,
        chain: Chain,
    ) -> Result<Self> {
        let provider = Provider::<Ws>::connect_with_reconnects(provider_url, provider_reconnects)
            .await
            .map_err(|e| return Error::Evm(e.to_string()))?;
        let client = Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(chain)));

        Ok(Self {
            contract: Contract::new(contract_address, client),
        })
    }
}

#[async_trait]
impl EVMClient for BasinClient {
    fn address(&self) -> Address {
        self.contract.address()
    }

    async fn add_pub(&self, owner: Address, name: &str) -> Result<()> {
        self.contract
            .create_pub(owner, name.into())
            .send()
            .await
            .map_err(|e| return Error::Evm(e.to_string()))?;
        Ok(())
    }
}
