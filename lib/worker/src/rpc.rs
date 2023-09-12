use basin_evm::EVMClient;
use basin_protocol::{publications, tx};
use capnp::{capability::Promise, data, message, private::units::BYTES_PER_WORD};
use capnp_rpc::pry;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use ethers::types::Address;
use futures::{AsyncReadExt, FutureExt};
use log::debug;
use log::info;
use sqlx::postgres::PgPool;
use std::net::SocketAddr;

/// RPC service wrapper for publications.
pub struct Publications<E: EVMClient + 'static> {
    evm_client: E,
    pg_pool: PgPool,
    cf_sink: String,
    cf_schedule: String,
}

impl<E: EVMClient + 'static> Publications<E> {
    pub fn new(evm_client: E, pg_pool: PgPool, cf_sink: String, cf_schedule: String) -> Self {
        Self {
            evm_client,
            pg_pool,
            cf_sink,
            cf_schedule,
        }
    }
}

impl<E: EVMClient + 'static> publications::Server for Publications<E> {
    /// Receives new namespace requests.
    fn create(
        &mut self,
        params: publications::CreateParams,
        _: publications::CreateResults,
    ) -> Promise<(), capnp::Error> {
        let args = pry!(params.get());
        let ns = pry!(args.get_ns()).to_string();
        let rel = pry!(args.get_rel()).to_string();
        let schema = pry!(args.get_schema());
        let owner = Address::from_slice(pry!(args.get_owner()));
        let name = format!("{ns}.{rel}");
        let table_stmt = pry!(crate::sql::schema_to_table_create(name.clone(), schema));
        let cf_sink = self.cf_sink.clone();
        let cf_schedule = self.cf_schedule.clone();
        let cf_stmt = pry!(crate::sql::scheduled_changefeed_create(
            name.clone(),
            cf_sink,
            cf_schedule
        ));

        debug!(
            "publication create {name} for {}: {table_stmt}",
            owner.to_string(),
        );

        let p = self.pg_pool.clone();
        let e = self.evm_client.clone();
        Promise::from_future(async move {
            e.add_pub(owner, name.as_str()).await?;
            crate::db::namespace_create(&p, ns, owner).await?;
            crate::db::pub_table_create(&p, &table_stmt, &cf_stmt).await?;
            Ok(())
        })
    }

    /// Receives publication data.
    fn push(
        &mut self,
        params: publications::PushParams,
        _: publications::PushResults,
    ) -> Promise<(), capnp::Error> {
        let args = pry!(params.get());
        let ns = pry!(args.get_ns()).to_string();
        let rel = pry!(args.get_rel()).to_string();
        let tx = pry!(args.get_tx());
        let sig = pry!(args.get_sig());
        let owner = pry!(recover_addr(tx, sig));
        let name = format!("{ns}.{rel}");
        let insert_stmt = pry!(crate::sql::tx_to_table_inserts(name.clone(), tx));

        debug!(
            "publication push {name} for {}: {:?}",
            owner.to_string(),
            insert_stmt
        );

        let p = self.pg_pool.clone();
        Promise::from_future(async move {
            let is_owner = crate::db::is_namespace_owner(&p, ns, owner).await?;
            if is_owner {
                crate::db::pub_table_insert(&p, insert_stmt).await?;
                Ok(())
            } else {
                Err(capnp::Error::failed("Unauthorized".into()))
            }
        })
    }
}

/// Recovers address from tx:Reader
fn recover_addr(tx: tx::Reader, sig: data::Reader) -> capnp::Result<Address> {
    let payload = canonicalize_tx(tx)?;
    let hash = crate::crypto::keccak256(payload.as_slice());
    let addr = crate::crypto::recover(hash.as_slice(), &sig[..64], sig[64] as i32)?;
    Ok(addr)
}

/// Returns the canonical bytes of tx::Reader
fn canonicalize_tx(reader: tx::Reader) -> capnp::Result<Vec<u8>> {
    let size = reader.total_size()?.word_count + 1;
    let mut message =
        message::Builder::new(message::HeapAllocator::new().first_segment_words(size as u32));
    message.set_root_canonical(reader)?;
    let output_segments = message.get_segments_for_output();
    if output_segments.len() != 1 {
        return Err(capnp::Error::failed(format!(
            "canonical tx has {} segments; expected 1",
            output_segments.len()
        )));
    }
    let output = output_segments[0];
    if (output.len() / BYTES_PER_WORD) as u64 > size {
        return Err(capnp::Error::failed(format!(
            "canonical tx size must be less than {size}"
        )));
    }
    Ok(output.to_vec())
}

/// Listens for RPC messages from Basin clients
pub async fn listen<E: EVMClient>(
    addr: SocketAddr,
    evm_client: E,
    pg_pool: PgPool,
    cf_sink: String,
    cf_schedule: String,
) -> Result<(), Box<dyn std::error::Error>> {
    tokio::task::LocalSet::new()
        .run_until(async move {
            let pubs_handler = Publications::new(evm_client, pg_pool, cf_sink, cf_schedule);
            let pubs_client: publications::Client = capnp_rpc::new_client(pubs_handler);

            let listener = tokio::net::TcpListener::bind(&addr).await?;
            info!("Basin RPC API started");
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
                    RpcSystem::new(Box::new(network), Some(pubs_client.clone().client));

                tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));
            }
        })
        .await
}
