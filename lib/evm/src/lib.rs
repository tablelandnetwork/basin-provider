mod client;
#[allow(clippy::all)]
mod contract;
pub mod testing;

use async_trait::async_trait;
use basin_common::errors::Result;
pub use client::BasinClient;
use contract::BasinStorage as Contract;
use ethers::types::Address;

// fixme: add methods for listing pubs, deals, etc.
#[async_trait]
pub trait EVMClient: Clone + Send {
    /// Returns the attached contract address.
    fn address(&self) -> Address;

    /// Add a new publication for owner.
    async fn add_pub(&self, owner: Address, name: &str) -> Result<()>;

    // List publications from a specific owner.
    async fn list_pub(&self, owner: Address) -> Result<Vec<String>>;
}
