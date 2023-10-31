use basin_common::errors::Result;
use ethers::types::Address;
use multibase::Base;
use sqlx::postgres::{PgPool, PgQueryResult};
use sqlx::Row;

/// Creates a namespace for owner.
/// Returns whether or not the namespace was created.
pub async fn namespace_create(pool: &PgPool, ns: &str, owner: Address) -> Result<bool> {
    // Insert a new namespace for owner
    let insert = sqlx::query!(
        "INSERT INTO namespaces (name, owner) VALUES ($1, $2) ON CONFLICT (name) DO NOTHING",
        ns,
        owner.as_bytes()
    )
    .execute(pool)
    .await?;

    // Create schema for the namespace
    sqlx::query(&format!("CREATE SCHEMA IF NOT EXISTS {ns}"))
        .execute(pool)
        .await?;

    Ok(insert.rows_affected() != 0)
}

/// Returns whether or not the namespace exists.
pub async fn namespace_exists(pool: &PgPool, ns: &str) -> Result<bool> {
    let res = sqlx::query("SELECT id FROM namespaces WHERE name=$1")
        .bind(ns)
        .fetch_one(pool)
        .await?;
    Ok(!res.is_empty())
}

/// Returns whether or not the namespace is owned by `owner`.
pub async fn is_namespace_owner(pool: &PgPool, ns: &str, owner: Address) -> Result<bool> {
    let res = sqlx::query("SELECT id FROM namespaces WHERE name=$1 AND owner=$2")
        .bind(ns)
        .bind(owner.as_bytes())
        .fetch_one(pool)
        .await?;
    Ok(!res.is_empty())
}

/// Creates a data table and scheduled changefeed for pub.
pub async fn pub_table_create(pool: &PgPool, table_stmt: &str) -> Result<()> {
    sqlx::query(table_stmt).execute(pool).await?;
    Ok(())
}

/// Inserts data into a pub table.
pub async fn pub_table_insert(pool: &PgPool, stmts: Vec<String>) -> Result<()> {
    let mut txn = pool.begin().await?;
    for s in stmts {
        txn_execute(&mut txn, &s).await?;
    }
    Ok(txn.commit().await?)
}

// Lists cids of a given publication
pub async fn pub_cids(
    pool: &PgPool,
    ns: String,
    rel: String,
    limit: i32,
    offset: i32,
) -> Result<Vec<String>> {
    let res = sqlx::query(
        "SELECT cid FROM jobs 
                                                JOIN namespaces ON namespaces.id = jobs.ns_id 
                                                WHERE name=$1 AND relation = $2
                                                ORDER BY jobs.id DESC
                                                LIMIT $3
                                                OFFSET $4",
    )
    .bind(ns)
    .bind(rel)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let cids = res
        .iter()
        .map(|row| multibase::encode::<Vec<u8>>(Base::Base32Lower, row.get("cid")))
        .collect();

    Ok(cids)
}

// Insert a job (used for testing)
pub async fn pub_jobs_insert(
    pool: &PgPool,
    ns: &str,
    rel: &str,
    cid: Vec<u8>,
    activated: chrono::NaiveDateTime,
) -> Result<()> {
    sqlx::query!(
        "INSERT INTO jobs (ns_id, cid, relation, activated) SELECT id, $2, $3, $4 FROM namespaces WHERE name = $1",
        ns,
        cid,
        rel,
        activated,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Runs sqlx query within a database transaction.
async fn txn_execute(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    stmt: &str,
) -> sqlx::Result<PgQueryResult> {
    sqlx::query(stmt).execute(&mut **txn).await
}
