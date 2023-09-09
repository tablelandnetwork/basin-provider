use crate::crypto::{keccak256, recover};
use crate::db::{is_namespace_owner, namespace_create, pub_table_create, pub_table_insert};
use crate::helpers::{schema_to_table_create_sql, tx_to_table_inserts_sql};
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
    pub(crate) pg_pool: PgPool,
    pub(crate) evm_client: E,
}

impl<E: EVMClient + 'static> Publications<E> {
    pub fn new(pg_pool: PgPool, evm_client: E) -> Self {
        Self {
            pg_pool,
            evm_client,
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
        let table_stmt = pry!(schema_to_table_create_sql(ns.clone(), rel.clone(), schema));
        let name = format!("{}.{}", ns, rel);

        debug!(
            "publication create {} for {}: {}",
            name,
            owner.to_string(),
            table_stmt
        );

        let p = self.pg_pool.clone();
        let e = self.evm_client.clone();
        Promise::from_future(async move {
            e.add_pub(owner, name.as_str()).await?;
            namespace_create(&p, ns, owner).await?;
            pub_table_create(&p, &table_stmt).await?;
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
        let insert_stmt = pry!(tx_to_table_inserts_sql(ns.clone(), rel.clone(), tx));

        debug!(
            "publication push {}.{} for {}: {:?}",
            ns.clone(),
            rel,
            owner.to_string(),
            insert_stmt
        );

        let p = self.pg_pool.clone();
        Promise::from_future(async move {
            let is_owner = is_namespace_owner(&p, ns, owner).await?;
            if is_owner {
                pub_table_insert(&p, insert_stmt).await?;
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
    let hash = keccak256(payload.as_slice());
    let addr = recover(hash.as_slice(), &sig[..64], sig[64] as i32)?;
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
            "canonical tx size must be less than {}",
            size
        )));
    }
    Ok(output.to_vec())
}

/// Listens for RPC messages from Basin clients
pub async fn listen<E: EVMClient>(
    addr: SocketAddr,
    pg_pool: PgPool,
    evm_client: E,
) -> Result<(), Box<dyn std::error::Error>> {
    tokio::task::LocalSet::new()
        .run_until(async move {
            let pubs_handler = Publications::new(pg_pool, evm_client);
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
