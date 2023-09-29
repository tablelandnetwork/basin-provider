use crate::{Contract, EVMClient};
use async_trait::async_trait;
use basin_common::errors::{Error, Result};
use ethers::{
    core::utils::Anvil,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
    utils::{keccak256, AnvilInstance},
};
use std::{sync::Arc, time::Duration};

#[derive(Clone)]
pub struct MockClient {
    _anvil: Arc<AnvilInstance>, // unused, but ref'd here to keep the task from dropping
    contract: Contract<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

impl MockClient {
    pub async fn new() -> Result<Self> {
        let anvil = match std::env::var("FOUNDRY_BIN") {
            Ok(p) => Anvil::at(format!("{p}/anvil")).spawn(),
            Err(_) => Anvil::new().spawn(), // try to load from PATH
        };
        let provider = Provider::<Http>::try_from(anvil.endpoint())
            .unwrap()
            .interval(Duration::from_millis(10u64));
        let wallet: LocalWallet = anvil.keys()[0].clone().into();
        let wallet_address = wallet.address();
        let client = Arc::new(SignerMiddleware::new(
            provider,
            wallet.with_chain_id(anvil.chain_id()),
        ));

        let contract = Contract::deploy(client, ()).unwrap().send().await.unwrap();

        contract
            .grant_role(keccak256("PUB_ADMIN_ROLE".as_bytes()), wallet_address)
            .send()
            .await
            .unwrap();

        Ok(Self {
            _anvil: Arc::new(anvil),
            contract,
        })
    }
}

#[async_trait]
impl EVMClient for MockClient {
    fn address(&self) -> Address {
        self.contract.address()
    }

    async fn add_pub(&self, owner: Address, name: &str) -> Result<()> {
        self.contract
            .create_pub(owner, name.into())
            .send()
            .await
            .unwrap();
        Ok(())
    }

    async fn list_pub(&self, owner: Address) -> Result<Vec<String>, Error> {
        self.contract
            .pubs_of_owner(owner)
            .call()
            .await
            .map_err(|e| Error::Evm(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn new_works() {
        let client = MockClient::new().await.unwrap();
        assert!(!client.address().is_zero());
    }

    #[tokio::test]
    async fn add_and_list_pub_works() {
        let client = MockClient::new().await.unwrap();
        let owner = Address::random();
        let name = "foo.bar";
        client.add_pub(owner, name).await.unwrap();

        let owner_pubs: Vec<String> = client.list_pub(owner).await.unwrap();
        assert_eq!(owner_pubs[0], name);
    }
}
