use crate::crypto::Address;
use crate::db::Result;
use basin_protocol::{tableschema, tx};
use sqlx::postgres::{PgPool, PgQueryResult};
use sqlx::Row;

/// Adds a new namespace for owner.
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
    let schema_stmt = format!("CREATE SCHEMA IF NOT EXISTS {}", ns);
    Ok(sqlx::query(&schema_stmt).execute(pool).await.map(|_| ())?)
}

/// Creates a table for a new pub.
pub async fn pub_table_create(pool: &PgPool, stmt: &str) -> Result<()> {
    Ok(sqlx::query(stmt).execute(pool).await.map(|_| ())?)
}

/// Creates a table for a new pub.
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

/// Returns whether or not the namespace is owned by `owner`.
pub async fn is_namespace_owner(pool: &PgPool, ns: String, owner: Address) -> Result<bool> {
    let res = sqlx::query("SELECT id FROM namespaces WHERE name=$1 AND owner=$2")
        .bind(ns)
        .bind(owner.as_bytes())
        .fetch_one(pool)
        .await?;
    Ok(!res.is_empty())
}

/// Returns a SQL CREATE TABLE statement from a `tableschema::Reader`.
pub fn schema_to_table_create_sql(
    ns: String,
    rel: String,
    schema: tableschema::Reader,
) -> capnp::Result<String> {
    let columns = schema.get_columns()?;
    let mut sql_cols = String::new();
    let mut sql_pks = String::new();

    for (i, column) in columns.iter().enumerate() {
        let name = column.get_name()?;
        let ctype = column.get_type()?;
        let mut sql_col = format!("{} {}", name, ctype);

        let is_nullable = column.get_is_nullable();
        if is_nullable {
            sql_col = format!("{} {}", sql_col, "NOT NULL");
        }
        let is_pk = column.get_is_part_of_primary_key();

        if i == 0 {
            sql_cols = sql_col;
            if is_pk {
                sql_pks = name.into();
            }
        } else {
            sql_cols = format!("{},{}", sql_cols, sql_col);
            if is_pk {
                sql_pks = format!("{},{}", sql_pks, name);
            }
        }
    }
    if !sql_pks.is_empty() {
        sql_cols = format!("{},PRIMARY KEY ({})", sql_cols, sql_pks);
    }
    Ok(format!(
        "CREATE TABLE IF NOT EXISTS {}.{} ({})",
        ns, rel, sql_cols
    ))
}

/// Returns a SQL transaction statement that inserts records in a `tx::Reader`.
/// Note: Instead of a SQL transaction, we could use a bulk insert. However, can
/// we be sure that the columns will always match across records in a `tx::Reader`?
pub fn tx_to_table_inserts_sql(
    ns: String,
    rel: String,
    txn: tx::Reader,
) -> capnp::Result<Vec<String>> {
    let records = txn.get_records()?;

    let mut inserts: Vec<String> = Vec::new();
    for record in records {
        let action = record.get_action()?;
        match action {
            "I" => {}
            _ => {
                // fixme: properly handle cases (return error for other types?)
                continue;
            }
        }

        let mut cols = String::new();
        let mut vals = String::new();
        let columns = record.get_columns()?;
        for (i, column) in columns.iter().enumerate() {
            let name = column.get_name()?;
            let value: serde_json::Value = serde_json::from_slice(column.get_value()?)
                .map_err(|e| capnp::Error::failed(e.to_string()))?;
            if i == 0 {
                cols = name.into();
                vals = value.to_string();
            } else {
                cols = format!("{},{}", cols, name);
                vals = format!("{},{}", vals, value);
            }
        }
        inserts.push(
            format!("INSERT INTO {}.{} ({}) VALUES({})", ns, rel, cols, vals).replace('\"', "'"),
        );
    }
    Ok(inserts)
}
