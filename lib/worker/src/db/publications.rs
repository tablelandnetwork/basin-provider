use basin_common::errors::Result;
use ethers::types::Address;
use multibase::Base;
use sqlx::{
    postgres::PgPool, query_builder::QueryBuilder, types::chrono::NaiveDateTime, Execute, Postgres,
    Row,
};

/// Creates a namespace for owner.
/// Returns whether or not the namespace was created.
pub async fn namespace_create(
    pool: &PgPool,
    ns: &str,
    rel: &str,
    cache_duration: Option<i64>,
    owner: Address,
) -> Result<bool> {
    // Insert a new namespace for owner
    let record = sqlx::query!(
        "INSERT INTO namespaces (name, owner) VALUES ($1, $2) ON CONFLICT (name) DO NOTHING RETURNING id",
        ns,
        owner.as_bytes()
    )
    .fetch_optional(pool)
    .await?;

    // Create schema for the namespace
    sqlx::query::<Postgres>(&format!("CREATE SCHEMA IF NOT EXISTS {ns}"))
        .execute(pool)
        .await?;

    if record.is_some() {
        sqlx::query!(
            "INSERT INTO cache_config (ns_id, relation, duration) VALUES ($1, $2, $3)",
            record.unwrap().id,
            rel,
            cache_duration,
        )
        .execute(pool)
        .await?;

        return Ok(true);
    }

    Ok(false)
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

/// Returns cache config
pub async fn get_cache_config(pool: &PgPool, ns: &str, rel: &str) -> Result<Option<i64>> {
    let (duration, ) : (Option<i64>, ) = sqlx::query_as("SELECT duration FROM cache_config JOIN namespaces ON ns_id = namespaces.id WHERE name = $1 AND relation = $2")
        .bind(ns)
        .bind(rel)
        .fetch_one(pool)
        .await
        .unwrap_or((None, ));
    Ok(duration)
}

// Unsets cache_path and expires_at
pub async fn delete_expired_job(pool: &PgPool) -> Result<()> {
    sqlx::query!("UPDATE jobs SET expires_at = null, cache_path = null WHERE now() at time zone 'utc' >= expires_at;")
        .execute(pool)
        .await?;
    Ok(())
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
) -> Result<Vec<(String, i64, Option<NaiveDateTime>)>> {
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
                row.try_get("timestamp").unwrap_or(0),
                row.try_get("expires_at").ok(),
            )
        })
        .collect();

    Ok(rows)
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
        "SELECT cid, timestamp, expires_at FROM jobs JOIN namespaces ON namespaces.id = jobs.ns_id WHERE name = ",
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
        assert_eq!("SELECT cid, timestamp, expires_at FROM jobs JOIN namespaces ON namespaces.id = jobs.ns_id WHERE name = $1 AND relation = $2 ORDER BY jobs.id DESC LIMIT $3 OFFSET $4", sql);

        let sql = pub_cids_build_query("ns", "rel", &1, &1, &1699395131, &1699395131);
        assert_eq!("SELECT cid, timestamp, expires_at FROM jobs JOIN namespaces ON namespaces.id = jobs.ns_id WHERE name = $1 AND relation = $2 AND timestamp >= $3 AND timestamp <= $4 ORDER BY jobs.id DESC LIMIT $5 OFFSET $6", sql);
    }
}
