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
        let cname = column.get_name()?;
        let ctype = column.get_type()?;
        let mut col = format!("{cname} {ctype}");

        if !column.get_is_nullable() {
            col = format!("{col} NOT NULL");
        }
        let is_pk = column.get_is_part_of_primary_key();

        if i == 0 {
            cols = col;
            if is_pk {
                pks = cname.into();
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

    Ok(format!("CREATE TABLE IF NOT EXISTS {pub_name} ({cols})",))
}

/// Returns a SQL transaction statement that inserts records in a `tx::Reader`.
/// Note: Instead of a SQL transaction, we could use a bulk insert. However, can
/// we be sure that the columns will always match across records in a `tx::Reader`?
pub fn tx_to_table_inserts(pub_name: String, txn: tx::Reader) -> capnp::Result<Vec<String>> {
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
            let cname = column.get_name()?;
            let value: serde_json::Value = serde_json::from_slice(column.get_value()?)
                .map_err(|e| capnp::Error::failed(e.to_string()))?;
            if i == 0 {
                cols = cname.into();
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

    if inserts.len() > 0 {
        Ok(inserts)
    } else {
        Err(capnp::Error::failed(
            "transaction must have at least one record".into(),
        ))
    }
}

/// Returns a SQL scheduled changefeed create statement.
pub fn scheduled_changefeed_create(
    pub_name: String,
    cf_sink: String,
    cf_schedule: String,
) -> capnp::Result<String> {
    let schedule = format!("{}_schedule", pub_name.replace(".", "_"));
    Ok(format!(
        "CREATE SCHEDULE IF NOT EXISTS {schedule} FOR CHANGEFEED {pub_name} INTO '{cf_sink}' WITH full_table_name, format=parquet RECURRING '{cf_schedule}' WITH SCHEDULE OPTIONS first_run=now, on_execution_failure=reschedule, on_previous_running=wait",
    ))
}
