use thiserror::Error;

/// Common result type for database operations.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error for database operations.
#[derive(Error, Debug)]
pub enum Error {
    #[error("SQL error: {0}")]
    Sql(sqlx::Error),
    #[error("Migrate error: {0}")]
    Migrate(sqlx::migrate::MigrateError),
    #[error("URL error: {0}")]
    Url(String),
    #[error("EVM error: {0}")]
    Evm(String),
    #[error("Upload error: {0}")]
    Upload(String),
    #[error("W3S JSON error: {0}")]
    Web3StorageJson(String),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::Sql(err)
    }
}

impl From<sqlx::migrate::MigrateError> for Error {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        Self::Migrate(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Self::Url(err.to_string())
    }
}

impl From<Error> for capnp::Error {
    fn from(err: Error) -> capnp::Error {
        capnp::Error::failed(err.to_string())
    }
}
