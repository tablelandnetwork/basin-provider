mod client;
#[allow(clippy::all)]
mod contract;
pub mod testing;

use async_trait::async_trait;
use basin_common::errors::Result;
pub use client::BasinClient;
use contract::BasinStorage as Contract;
use contract::DealInfo;
use ethers::types::{Address, U256};

// fixme: add methods for listing pubs, deals, etc.
#[async_trait]
pub trait EVMClient: Clone + Send {
    /// Returns the attached contract address.
    fn address(&self) -> Address;

    /// Add a new publication for owner.
    async fn add_pub(&self, owner: Address, name: &str) -> Result<()>;

    // List publications from a specific owner.
    async fn list_pub(&self, owner: Address) -> Result<Vec<String>>;

    // List deals of a given publication.
    async fn deals(&self, publication: &str, offset: U256, limit: u32) -> Result<Vec<DealInfo>>;

    // List latest N deals of a given publication.
    async fn latest_deals(&self, publication: &str, n: u32) -> Result<Vec<DealInfo>>;
}
