mod errors;
#[allow(clippy::all)]
mod evm;
pub mod testing;

use async_trait::async_trait;
pub use errors::{Error, Result};
use ethers::types::Address;

#[async_trait]
pub trait EVMClient: Clone + Send {
    /// Creates a new BasinStorage client.
    async fn new() -> Result<Self>;

    /// Add a new publication for owner.
    async fn add_pub(&self, owner: Address, name: &str) -> Result<()>;
}
