use crate::errors::Result;
use crate::evm::BasinStorage as Contract;
use crate::EVMClient;
use async_trait::async_trait;
use ethers::{
    core::utils::Anvil,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
    utils::AnvilInstance,
};
use std::{sync::Arc, time::Duration};
use tiny_keccak::{Hasher, Keccak};

#[derive(Clone)]
pub struct MockClient {
    _anvil: Arc<AnvilInstance>, // unused, but ref'd here to keep the task from dropping
    storage: Contract<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

#[async_trait]
impl EVMClient for MockClient {
    async fn new() -> Result<Self> {
        let anvil = match std::env::var("FOUNDRY_BIN") {
            Ok(p) => Anvil::at(format!("{}/anvil", p)).spawn(),
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

        let storage = Contract::deploy(client, ()).unwrap().send().await.unwrap();

        storage
            .grant_role(keccak256("PUB_ADMIN_ROLE".as_bytes()), wallet_address)
            .send()
            .await
            .unwrap();

        Ok(Self {
            _anvil: Arc::new(anvil),
            storage,
        })
    }

    async fn add_pub(&self, owner: Address, name: &str) -> Result<()> {
        self.storage
            .create_pub(owner, name.into())
            .send()
            .await
            .unwrap();
        Ok(())
    }
}

fn keccak256(bytes: &[u8]) -> [u8; 32] {
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(bytes);
    hasher.finalize(&mut output);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn new_client_works() {
        MockClient::new().await.unwrap();
    }

    #[tokio::test]
    async fn add_pub_works() {
        let client = MockClient::new().await.unwrap();
        let owner = Address::random();
        let name = "foo.bar";
        client.add_pub(owner, name).await.unwrap();

        let owner_pubs: Vec<String> = client.storage.pubs_of_owner(owner).call().await.unwrap();
        assert_eq!(owner_pubs[0], name);
    }
}
