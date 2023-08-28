use crate::crypto::Address;
use basin_protocol::{tableschema, tx};
use sqlx::postgres::PgPool;

// 1. create ns if not exists
// 2. create schema for ns
// 3. add table in ns for pub
// 4. setup changelog

// Adds a new namespace for owner.
pub async fn add_namespace<'a>(
    pool: &PgPool,
    ns: String,
    rel: String,
    // schema: tableschema::Reader<'a>,
    owner: Address,
) -> anyhow::Result<()> {
    // Insert a new namespace for owner
    sqlx::query!(
        "
INSERT INTO namespaces ( name, owner )
VALUES ( $1, $2 )
ON CONFLICT (name) DO NOTHING
RETURNING id
        ",
        ns.clone(),
        owner.as_bytes()
    )
    .fetch_one(pool)
    .await?;

    // Create schema for the namespace
    let schema_stmt = format!("CREATE SCHEMA IF NOT EXISTS {}", ns);
    sqlx::query(schema_stmt.as_str()).execute(pool).await?;

    Ok(())
}

// pub async fn append_row(pool: &PgPool) -> anyhow::Result<()> {
//
// }

// list by owner?
// list tables under schema
// pub async fn list_publications(pool: &PgPool, address: String) -> anyhow::Result<()> {
//     let recs = sqlx::query!(
//         r#"
// SELECT id, address
// FROM publications
// WHERE address=$1
// ORDER BY id
//         "#,
//         address
//     )
//     .fetch_all(pool)
//     .await?;
//
//     for rec in recs {
//         println!("- {}: {}", rec.id, &rec.address);
//     }
//
//     Ok(())
// }
