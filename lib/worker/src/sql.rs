use basin_protocol::{tableschema, tx};

/// Returns a SQL CREATE TABLE statement from a `tableschema::Reader`.
pub fn schema_to_table_create(
    pub_name: String,
    schema: tableschema::Reader,
) -> capnp::Result<String> {
    let columns = schema.get_columns()?;
    let mut cols = String::new();
    let mut pks = String::new();

    for (i, column) in columns.iter().enumerate() {
        let cname = column.get_name()?.to_string()?;
        let ctype = column.get_type()?.to_string()?;
        let mut col = format!("{cname} {ctype}");

        if !column.get_is_nullable() {
            col = format!("{col} NOT NULL");
        }
        let is_pk = column.get_is_part_of_primary_key();

        if i == 0 {
            cols = col;
            if is_pk {
                pks = cname;
            }
        } else {
            cols = format!("{cols},{col}");
            if is_pk {
                pks = format!("{pks},{cname}");
            }
        }
    }

    if !pks.is_empty() {
        cols = format!("{cols},PRIMARY KEY ({pks})");
    }

    if cols.is_empty() {
        return Err(capnp::Error::failed(
            "schema must have at least one column".into(),
        ));
    }
    cols = format!("__basin_created TIMESTAMP DEFAULT now(),{cols}");

    Ok(format!(
        "CREATE TABLE IF NOT EXISTS {pub_name} ({cols},INDEX (__basin_created))",
    ))
}

/// Returns a SQL transaction statement that inserts records in a `tx::Reader`.
/// Note: Instead of a SQL transaction, we could use a bulk insert. However, can
/// we be sure that the columns will always match across records in a `tx::Reader`?
pub fn tx_to_table_inserts(pub_name: String, txn: tx::Reader) -> capnp::Result<Vec<String>> {
    let records = txn.get_records()?;

    let mut inserts: Vec<String> = Vec::new();
    for record in records {
        let action = record.get_action()?.to_string()?;
        let action = action.as_str();
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
            let cname = column.get_name()?.to_string()?;
            let value: serde_json::Value = serde_json::from_slice(column.get_value()?)
                .map_err(|e| capnp::Error::failed(e.to_string()))?;
            if i == 0 {
                cols = cname;
                vals = value.to_string();
            } else {
                cols = format!("{cols},{cname}");
                vals = format!("{vals},{value}");
            }
        }
        if !(cols.is_empty() || vals.is_empty()) {
            inserts
                .push(format!("INSERT INTO {pub_name} ({cols}) VALUES({vals})").replace('\"', "'"));
        }
    }

    if !inserts.is_empty() {
        Ok(inserts)
    } else {
        Err(capnp::Error::failed(
            "transaction must have at least one record".into(),
        ))
    }
}
