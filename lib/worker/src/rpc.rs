use basin_evm::EVMClient;
use basin_protocol::publications;
use capnp::capability::Promise;
use capnp_rpc::pry;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use ethers::types::Address;
use futures::AsyncReadExt;
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
        if ns.is_empty() {
            return Promise::err(capnp::Error::failed("namespace is required".into()));
        }
        let rel = pry!(args.get_rel()).to_string();
        if rel.is_empty() {
            return Promise::err(capnp::Error::failed("relation is required".into()));
        }
        let owner = Address::from_slice(pry!(args.get_owner()));
        if owner.is_zero() {
            return Promise::err(capnp::Error::failed("owner is required".into()));
        }
        println!("create address: {}", owner.to_string());
        let name = format!("{ns}.{rel}");
        let schema = pry!(args.get_schema());
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
        if ns.is_empty() {
            return Promise::err(capnp::Error::failed("namespace is required".into()));
        }
        let rel = pry!(args.get_rel()).to_string();
        if rel.is_empty() {
            return Promise::err(capnp::Error::failed("relation is required".into()));
        }
        let sig = pry!(args.get_sig());
        if sig.is_empty() {
            return Promise::err(capnp::Error::failed("signature is required".into()));
        }
        let tx = pry!(args.get_tx());
        let owner = pry!(crate::utils::recover_addr(tx, sig));
        println!("push address: {}", owner.to_string());
        let name = format!("{ns}.{rel}");
        let insert_stmt = pry!(crate::sql::tx_to_table_inserts(name.clone(), tx));
        println!("{:?}", insert_stmt);

        debug!(
            "publication push {name} for {}: {:?}",
            owner.to_string(),
            insert_stmt
        );

        let p = self.pg_pool.clone();
        Promise::from_future(async move {
            let is_owner = crate::db::is_namespace_owner(&p, ns, owner).await?;
            println!("is owner: {}", is_owner);
            if is_owner {
                crate::db::pub_table_insert(&p, insert_stmt).await?;
                Ok(())
            } else {
                Err(capnp::Error::failed("unauthorized".into()))
            }
        })
    }
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
                tokio::task::spawn_local(rpc_system);
            }
        })
        .await
}
