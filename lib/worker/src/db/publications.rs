use basin_common::errors::Result;
use ethers::types::Address;
use multibase::Base;
use sqlx::{
    postgres::{PgPool, PgQueryResult},
    query_builder::QueryBuilder,
    Execute, Postgres, Row,
};

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
    before: i64,
    after: i64,
) -> Result<Vec<(String, i64)>> {
    let sql = pub_cids_build_query(&ns, &rel, &limit, &offset, &before, &after);
    let mut query = sqlx::query(&sql).bind(ns).bind(rel);

    if after > 0 {
        query = query.bind(after);
    }

    if before > 0 {
        query = query.bind(before);
    }

    let res = query.bind(limit).bind(offset).fetch_all(pool).await?;

    let rows = res
        .iter()
        .map(|row| {
            (
                multibase::encode::<Vec<u8>>(Base::Base32Lower, row.get("cid")),
                row.get("timestamp"),
            )
        })
        .collect();

    Ok(rows)
}

/// Runs sqlx query within a database transaction.
async fn txn_execute(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    stmt: &str,
) -> sqlx::Result<PgQueryResult> {
    sqlx::query(stmt).execute(&mut **txn).await
}

fn pub_cids_build_query(
    ns: &str,
    rel: &str,
    limit: &i32,
    offset: &i32,
    before: &i64,
    after: &i64,
) -> String {
    let mut query: QueryBuilder<'_, Postgres> = QueryBuilder::new(
        "SELECT cid, timestamp FROM jobs JOIN namespaces ON namespaces.id = jobs.ns_id WHERE name = ",
    );

    query.push_bind(ns);
    query.push(" AND relation = ");
    query.push_bind(rel);

    if *after > 0 {
        query.push(" AND timestamp >= ");
        query.push_bind(after);
    }

    if *before > 0 {
        query.push(" AND timestamp <= ");
        query.push_bind(before);
    }

    query.push(" ORDER BY jobs.id DESC LIMIT ");
    query.push_bind(limit);
    query.push(" OFFSET ");
    query.push_bind(offset);

    return query.build().sql().into();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pub_cids_query_building() {
        let sql = pub_cids_build_query("ns", "rel", &1, &1, &0, &0);
        assert_eq!("SELECT cid, timestamp FROM jobs JOIN namespaces ON namespaces.id = jobs.ns_id WHERE name = $1 AND relation = $2 ORDER BY jobs.id DESC LIMIT $3 OFFSET $4", sql);

        let sql = pub_cids_build_query("ns", "rel", &1, &1, &1699395131, &1699395131);
        assert_eq!("SELECT cid, timestamp FROM jobs JOIN namespaces ON namespaces.id = jobs.ns_id WHERE name = $1 AND relation = $2 AND timestamp >= $3 AND timestamp <= $4 ORDER BY jobs.id DESC LIMIT $5 OFFSET $6", sql);
    }
}
