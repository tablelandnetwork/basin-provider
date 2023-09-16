use thiserror::Error;

/// Common result type for export operations.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error for export operations.
#[derive(Error, Debug)]
pub enum Error {
    #[error("SQL error: {0}")]
    Sql(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::Sql(err)
    }
}
