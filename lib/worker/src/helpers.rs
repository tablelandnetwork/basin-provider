use basin_protocol::{tableschema, tx};

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
