use crate::{contract::DealInfo, Contract, EVMClient};
use async_trait::async_trait;
use basin_common::errors::{Error, Result};
use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Chain, U256},
};
use std::{sync::Arc, time::Duration};

#[derive(Clone)]
pub struct BasinClient {
    contract: Contract<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

impl BasinClient {
    pub async fn new(
        wallet: LocalWallet,
        contract_address: Address,
        provider_url: &str,
        chain: Chain,
    ) -> Result<Self> {
        let provider = Provider::<Http>::try_from(provider_url)
            .map_err(|e| Error::Evm(e.to_string()))?
            .interval(Duration::from_millis(10u64));
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
            .map_err(|e| Error::Evm(e.to_string()))?;
        Ok(())
    }

    async fn list_pub(&self, owner: Address) -> Result<Vec<String>, Error> {
        self.contract
            .pubs_of_owner(owner)
            .call()
            .await
            .map_err(|e| Error::Evm(e.to_string()))
    }

    async fn deals(
        &self,
        publication: &str,
        offset: u64,
        limit: u32,
    ) -> Result<Vec<DealInfo>, Error> {
        self.contract
            .paginated_deals(
                publication.to_string(),
                U256::from(offset),
                U256::from(limit),
            )
            .call()
            .await
            .map_err(|e| Error::Evm(e.to_string()))
    }

    async fn latest_deals(&self, publication: &str, n: u32) -> Result<Vec<DealInfo>, Error> {
        self.contract
            .latest_n_deals(publication.to_string(), U256::from(n))
            .call()
            .await
            .map_err(|e| Error::Evm(e.to_string()))
    }
}
