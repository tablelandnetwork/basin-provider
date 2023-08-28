use crate::crypto::{keccak256, recover, Address};
use crate::db::add_namespace;
use basin_protocol::{publications, tableschema, tx};
use capnp::{capability::Promise, data, message, private::units::BYTES_PER_WORD};
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt};
use sqlx::postgres::PgPool;
use std::net::ToSocketAddrs;
use std::str;

struct Publications {
    pool: PgPool,
}

impl publications::Server for Publications {
    // fixme: error handling
    // fixme: validate owner byte length is address (42)
    fn create(
        &mut self,
        params: publications::CreateParams,
        _: publications::CreateResults,
    ) -> Promise<(), capnp::Error> {
        let args = pry!(params.get());
        let ns = args.get_ns().unwrap().to_string();
        let rel = args.get_rel().unwrap().to_string();
        let schema = args.get_schema().unwrap();
        let owner = args.get_owner().unwrap();
        let owner_addr = Address::from_slice(owner.as_ref());

        let p = self.pool.clone();
        Promise::from_future(async move {
            add_namespace(&p, ns, rel, owner_addr).await.unwrap();
            Ok(())
        })
    }

    // fixme: error handling
    // fixme: validate signature length
    fn push(
        &mut self,
        params: publications::PushParams,
        mut results: publications::PushResults,
    ) -> Promise<(), capnp::Error> {
        let args = pry!(params.get());
        let ns = args.get_ns().unwrap();
        let rel = args.get_rel().unwrap();
        let tx = args.get_tx().unwrap();
        let sig = args.get_sig().unwrap();
        let addr = recover_addr(tx, sig);

        let p = self.pool.clone();
        Promise::from_future(async move { Ok(()) })
    }
}

pub async fn listen(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} start ADDRESS:PORT", args[0]);
        return Ok(());
    }

    let addr = args[2]
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("could not parse address");

    tokio::task::LocalSet::new()
        .run_until(async move {
            let listener = tokio::net::TcpListener::bind(&addr).await?;
            let publications_client: publications::Client =
                capnp_rpc::new_client(Publications { pool });

            println!("Server running");
            loop {
                let (stream, _) = listener.accept().await?;
                stream.set_nodelay(true)?;
                let (reader, writer) =
                    tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();

                let network = twoparty::VatNetwork::new(
                    reader,
                    writer,
                    rpc_twoparty_capnp::Side::Server,
                    Default::default(),
                );

                let rpc_system =
                    RpcSystem::new(Box::new(network), Some(publications_client.clone().client));

                tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));
            }
        })
        .await
}

/// Returns the canonical bytes of tx::Reader
fn canonicalize_tx(reader: tx::Reader) -> Vec<u8> {
    let size = reader.total_size().unwrap().word_count + 1;
    let mut message =
        message::Builder::new(message::HeapAllocator::new().first_segment_words(size as u32));
    message.set_root_canonical(reader).unwrap();
    let output_segments = message.get_segments_for_output();
    debug_assert_eq!(1, output_segments.len());
    let output = output_segments[0];
    debug_assert!((output.len() / BYTES_PER_WORD) as u64 <= size);
    output.to_vec()
}

/// Recovers address from tx:Reader
fn recover_addr(tx: tx::Reader, sig: data::Reader) -> Address {
    let payload = canonicalize_tx(tx);
    let hash = keccak256(payload.as_slice());
    recover(hash.as_slice(), &sig[..64], sig[64] as i32).unwrap()
}

// fixme: error handling
fn pub_to_sql(name: &str, schema: &str, owner: &str, columns: &str) {
    let schema_stmt = format!("CREATE SCHEMA IF NOT EXISTS {}", schema);
    let table_stmt = format!("CREATE TABLE {}.{}({})", schema, name, columns);
    println!("{schema_stmt}");
    println!("{table_stmt}");
}

// fixme: error handling
fn tx_to_sql(txn: tx::Reader) {
    let records = txn.get_records().unwrap();

    for record in records {
        let action = record.get_action().unwrap();
        match action {
            "I" => {}
            _ => {
                // fixme: properly handle cases (return error for other types?)
                continue;
            }
        }

        let table = record.get_table().unwrap();
        let columns = record.get_columns().unwrap();

        let mut cols = String::new();
        let mut vals = String::new();
        // let _ = sqlx::query("DELETE FROM table");
        for (i, column) in columns.iter().enumerate() {
            let name = column.get_name().unwrap();
            // let ctype = column.get_type().unwrap();
            let value: serde_json::Value =
                serde_json::from_slice(column.get_value().unwrap()).unwrap();

            if i == 0 {
                cols = name.into();
                vals = value.to_string();
            } else {
                cols = format!("{},{}", cols, name);
                vals = format!("{},{}", vals, value.to_string());
            }
        }

        // let primary_key = record.get_primary_key().unwrap();

        let stmt1 = format!("INSERT INTO {}({}) VALUES({})", table, cols, vals).replace("\"", "'");
        println!("{stmt1}");
    }
}
