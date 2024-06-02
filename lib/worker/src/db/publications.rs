use crate::domain::CachePath;
use crate::domain::Cid;
use crate::domain::Event;
use crate::domain::Vault;
use basin_common::errors::Result;
use ethers::types::Address;
use sqlx::{
    postgres::{PgPool, PgRow},
    query_builder::QueryBuilder,
    Execute, Postgres, Row,
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
        "WITH i AS (
            INSERT INTO namespaces (name, owner) VALUES ($1, $2) ON CONFLICT (name) DO NOTHING RETURNING id
        )
        SELECT id, true as created FROM i
        UNION ALL
        SELECT id, false as created FROM namespaces WHERE name = $1 LIMIT 1"
        ,
        ns,
        owner.as_bytes()
    )
    .fetch_one(pool)
    .await?;

    // Create schema for the namespace
    sqlx::query::<Postgres>(&format!("CREATE SCHEMA IF NOT EXISTS {ns}"))
        .execute(pool)
        .await?;

    sqlx::query!(
        "INSERT INTO cache_config (ns_id, relation, duration) VALUES ($1, $2, $3)",
        record.id,
        rel,
        cache_duration,
    )
    .execute(pool)
    .await?;

    Ok(record.created.unwrap())
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

/// Returns cache config from a vault
pub async fn get_cache_config(pool: &PgPool, vault: &Vault) -> Result<Option<i64>> {
    let (duration, ) : (Option<i64>, ) = sqlx::query_as("SELECT duration FROM cache_config JOIN namespaces ON ns_id = namespaces.id WHERE name = $1 AND relation = $2")
        .bind(vault.namespace())
        .bind(vault.relation())
        .fetch_one(pool)
        .await
        .unwrap_or((None, ));
    Ok(duration)
}

pub async fn find_cache_config_by_vaults(
    pool: &PgPool,
    vaults: &Vec<Vault>,
) -> Result<Vec<(String, Option<i64>)>> {
    let where_clause = (1..2 * vaults.len() + 1)
        .step_by(2)
        .map(|i| format!("(name = ${} AND relation = ${})", i, i + 1))
        .collect::<Vec<String>>()
        .join(" OR ");

    let sql = format!("SELECT name || '.' || relation as vault, duration FROM namespaces JOIN cache_config ON ns_id = namespaces.id WHERE {}", where_clause);
    let mut query = sqlx::query(sql.as_str());
    for vault in vaults {
        query = query.bind(vault.namespace()).bind(vault.relation());
    }

    query
        .map(|row: PgRow| (row.get("vault"), row.get("duration")))
        .fetch_all(pool)
        .await
        .map_err(basin_common::errors::Error::from)
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
    vault: &Vault,
    limit: i32,
    offset: i32,
    before: i64,
    after: i64,
) -> Result<Vec<Event>> {
    let sql: String = pub_cids_build_query(
        &vault.namespace(),
        &vault.relation(),
        &limit,
        &offset,
        &before,
        &after,
    );

    let mut query = sqlx::query(&sql)
        .bind(vault.namespace())
        .bind(vault.relation());

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
            Event::new(
                row.get("cid"),
                row.try_get("timestamp").unwrap_or(0),
                row.try_get("expires_at").ok(),
            )
        })
        .collect();

    Ok(rows)
}

#[allow(clippy::too_many_arguments)]
pub async fn create_job(
    pool: &PgPool,
    vault: &Vault,
    cid: Vec<u8>,
    timestamp: Option<i64>,
    cache_path: String,
    cache_duration: Option<i64>,
    signature: Vec<u8>,
    hash: Vec<u8>,
) -> Result<()> {
    let expires_at = cache_duration.map(|duration| {
        chrono::Utc::now()
            .checked_add_signed(chrono::Duration::minutes(duration))
            .unwrap()
            .naive_utc()
    });

    sqlx::query!(
        "INSERT INTO jobs (
            ns_id, relation, cid, timestamp, cache_path,
            expires_at, signature, hash
        ) 
        SELECT id, $2, $3, $4, $5, $6, $7, $8
        FROM namespaces
        WHERE name = $1",
        vault.namespace(),
        vault.relation(),
        cid,
        timestamp.unwrap_or(chrono::Utc::now().timestamp()),
        cache_path,
        expires_at,
        signature,
        hash
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_job_cache_path_by_cid(pool: &PgPool, cid: Cid) -> Result<Option<CachePath>> {
    sqlx::query(
        format!(
            "SELECT cache_path FROM jobs WHERE cid = '\\x{}'",
            hex::encode(cid.as_ref())
        )
        .as_str(),
    )
    .map(|row: PgRow| match row.try_get("cache_path") {
        Ok(v) => Some(CachePath::from(v)),
        Err(err) => {
            log::error!("try get cache_path, err = {:?}", err);
            None
        }
    })
    .fetch_one(pool)
    .await
    .map_err(basin_common::errors::Error::from)
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
        query.push(" AND timestamp > ");
        query.push_bind(after);
    }

    if *before > 0 {
        query.push(" AND timestamp < ");
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
        assert_eq!("SELECT cid, timestamp, expires_at FROM jobs JOIN namespaces ON namespaces.id = jobs.ns_id WHERE name = $1 AND relation = $2 AND timestamp > $3 AND timestamp < $4 ORDER BY jobs.id DESC LIMIT $5 OFFSET $6", sql);
    }
}
