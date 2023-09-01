use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error for database operations.
#[derive(Error, Debug)]
pub enum Error {
    #[error("SQL error: {0}")]
    SQL(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::SQL(err)
    }
}

impl From<Error> for capnp::Error {
    fn from(err: Error) -> capnp::Error {
        capnp::Error::failed(err.to_string())
    }
}
