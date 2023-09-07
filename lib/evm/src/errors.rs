use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error for database operations.
#[derive(Error, Debug)]
pub enum Error {
    #[error("EVM error: {0}")]
    EVM(String),
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::EVM(err)
    }
}

impl From<Error> for capnp::Error {
    fn from(err: Error) -> capnp::Error {
        capnp::Error::failed(err.to_string())
    }
}
