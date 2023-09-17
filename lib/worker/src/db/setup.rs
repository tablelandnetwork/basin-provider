use crate::db::{Error, Result};
use sqlx::PgPool;
use url::Url;

// Creates database and runs migrations.
pub async fn setup(pool: PgPool, database_url: &str) -> Result<()> {
    let db_name = database_name(database_url)?;
    sqlx::query(&format!("CREATE DATABASE IF NOT EXISTS {}", db_name))
        .execute(&pool)
        .await?;
    sqlx::migrate!().set_locking(false).run(&pool).await?;
    Ok(())
}

// Drops database.
pub async fn drop(pool: PgPool, database_url: &str) -> Result<()> {
    let db_name = database_name(database_url)?;
    sqlx::query(&format!("DROP DATABASE {}", db_name))
        .execute(&pool)
        .await?;
    Ok(())
}

/// Returns the database name from a Postgres connection string.
fn database_name(conn_str: &str) -> Result<String> {
    let url = Url::parse(conn_str).expect("Invalid connection string");
    match url.path_segments().and_then(|segments| segments.last()) {
        Some(n) => Ok(n.to_owned()),
        None => Err(Error::Url(
            "could not parse database name from database URL".to_string(),
        )),
    }
}
