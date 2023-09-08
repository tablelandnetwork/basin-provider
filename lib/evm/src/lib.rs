mod client;
#[allow(clippy::all)]
mod contract;
mod errors;
pub mod testing;

use async_trait::async_trait;
use contract::BasinStorage as Contract;
pub use errors::{Error, Result};
use ethers::types::Address;

// fixme: add methods for listing pubs, deals, etc.
#[async_trait]
pub trait EVMClient: Clone + Send {
    /// Returns the attached contract address.
    fn address(&self) -> Address;

    /// Add a new publication for owner.
    async fn add_pub(&self, owner: Address, name: &str) -> Result<()>;
}
