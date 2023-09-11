use crate::db::Result;
use ethers::types::Address;
use sqlx::postgres::{PgPool, PgQueryResult};
use sqlx::Row;

/// Creates a namespace for owner.
pub async fn namespace_create(pool: &PgPool, ns: String, owner: Address) -> Result<()> {
    // Insert a new namespace for owner
    sqlx::query!(
        "INSERT INTO namespaces (name, owner) VALUES ($1, $2) ON CONFLICT (name) DO NOTHING",
        ns.clone(),
        owner.as_bytes()
    )
    .execute(pool)
    .await?;

    // Create schema for the namespace
    let schema_stmt = format!("CREATE SCHEMA IF NOT EXISTS {ns}");
    Ok(sqlx::query(&schema_stmt).execute(pool).await.map(|_| ())?)
}

/// Returns whether or not the namespace is owned by `owner`.
pub async fn is_namespace_owner(pool: &PgPool, ns: String, owner: Address) -> Result<bool> {
    let res = sqlx::query("SELECT id FROM namespaces WHERE name=$1 AND owner=$2")
        .bind(ns)
        .bind(owner.as_bytes())
        .fetch_one(pool)
        .await?;
    Ok(!res.is_empty())
}

/// Creates a data table and scheduled changefeed for pub.
pub async fn pub_table_create(pool: &PgPool, table_stmt: &str, cf_stmt: &str) -> Result<()> {
    sqlx::query(table_stmt).execute(pool).await.map(|_| ())?;
    sqlx::query(cf_stmt).execute(pool).await.map(|_| ())?;
    Ok(())
}

/// Inserts data into a pub table.
pub async fn pub_table_insert(pool: &PgPool, stmts: Vec<String>) -> Result<()> {
    let mut txn = pool.begin().await?;
    for s in stmts {
        txn_query(&mut txn, &s).await?;
    }
    Ok(txn.commit().await.map(|_| ())?)
}

/// Runs sqlx query within a database transaction.
async fn txn_query(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    stmt: &str,
) -> sqlx::Result<PgQueryResult> {
    sqlx::query(stmt).execute(&mut **txn).await
}
