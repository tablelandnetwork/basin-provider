use crate::gcs::GcsClient;
use basin_evm::EVMClient;
use basin_protocol::publications;
use capnp::{capability::Promise, Error};
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};
use ethers::types::Address;
use futures::AsyncReadExt;
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};
use google_cloud_storage::http::resumable_upload_client::{ChunkSize, ResumableUploadClient};
use log::{debug, info};
use sqlx::postgres::PgPool;
use tiny_keccak::{Hasher, Keccak};
use tokio::net::TcpListener;

/// RPC service wrapper for publications.
pub struct Publications<E: EVMClient + 'static> {
    evm_client: E,
    pg_pool: PgPool,
    gcs_client: GcsClient,
}

impl<E: EVMClient + 'static> Publications<E> {
    pub fn new(evm_client: E, pg_pool: PgPool, gcs_client: GcsClient) -> Self {
        Self {
            evm_client,
            pg_pool,
            gcs_client,
        }
    }
}

impl<E: EVMClient + 'static> publications::Server for Publications<E> {
    /// Receives new namespace requests.
    fn create(
        &mut self,
        params: publications::CreateParams,
        mut results: publications::CreateResults,
    ) -> Promise<(), Error> {
        let args = pry!(params.get());
        let ns = pry!(pry!(args.get_ns()).to_string());
        if ns.is_empty() {
            return Promise::err(Error::failed("namespace is required".into()));
        }
        let rel = pry!(pry!(args.get_rel()).to_string());
        if rel.is_empty() {
            return Promise::err(Error::failed("relation is required".into()));
        }
        let owner = Address::from_slice(pry!(args.get_owner()));
        if owner.is_zero() {
            return Promise::err(Error::failed("owner is required".into()));
        }
        let name = format!("{ns}.{rel}");
        let schema = pry!(args.get_schema());
        let mut table_stmt = String::new();
        if schema.has_columns() {
            table_stmt = pry!(crate::sql::schema_to_table_create(&name, schema));
        }

        info!("publication {name} create for {owner}");

        let p = self.pg_pool.clone();
        let e = self.evm_client.clone();
        Promise::from_future(async move {
            e.add_pub(owner, name.as_str()).await?;
            let created = crate::db::namespace_create(&p, &ns, owner).await?;
            if created && !table_stmt.is_empty() {
                debug!("table statement: {table_stmt}");
                crate::db::pub_table_create(&p, &table_stmt).await?;
            }
            results.get().set_exists(!created);
            Ok(())
        })
    }

    /// Receives publication data.
    fn push(
        &mut self,
        params: publications::PushParams,
        _: publications::PushResults,
    ) -> Promise<(), Error> {
        let args = pry!(params.get());
        let ns = pry!(pry!(args.get_ns()).to_string());
        if ns.is_empty() {
            return Promise::err(Error::failed("namespace is required".into()));
        }
        let rel = pry!(pry!(args.get_rel()).to_string());
        if rel.is_empty() {
            return Promise::err(Error::failed("relation is required".into()));
        }
        let sig = pry!(args.get_sig());
        let tx = pry!(args.get_tx());
        let owner = pry!(crate::utils::recover_addr_from_tx(tx, sig));
        let name = format!("{ns}.{rel}");
        let insert_stmt = pry!(crate::sql::tx_to_table_inserts(&name, tx));

        info!("publication {name} push for {owner}");
        debug!("insert statements: {:?}", insert_stmt);

        let p = self.pg_pool.clone();
        Promise::from_future(async move {
            if crate::db::is_namespace_owner(&p, &ns, owner).await? {
                crate::db::pub_table_insert(&p, insert_stmt).await?;
                Ok(())
            } else {
                Err(Error::failed("unauthorized".into()))
            }
        })
    }

    /// Receives raw files.
    fn upload(
        &mut self,
        params: publications::UploadParams,
        mut results: publications::UploadResults,
    ) -> Promise<(), Error> {
        let args = pry!(params.get());
        let ns = pry!(pry!(args.get_ns()).to_string());
        if ns.is_empty() {
            return Promise::err(Error::failed("namespace is required".into()));
        }
        let rel = pry!(pry!(args.get_rel()).to_string());
        if rel.is_empty() {
            return Promise::err(Error::failed("relation is required".into()));
        }
        let size = args.get_size();
        if size == 0 {
            return Promise::err(Error::failed("size is required".into()));
        }
        let filename = format!(
            "{ns}/{rel}/{}.parquet",
            chrono::Utc::now().timestamp_micros()
        );

        info!("publication upload {filename} started");

        let p = self.pg_pool.clone();
        let c = self.gcs_client.clone();
        Promise::from_future(async move {
            if crate::db::namespace_exists(&p, &ns).await? {
                return Err(Error::failed("namespace not found".into()));
            }

            let upload_type = UploadType::Simple(Media::new(filename.clone()));
            let uploader = c
                .inner
                .prepare_resumable_upload(
                    &UploadObjectRequest {
                        bucket: c.bucket,
                        ..Default::default()
                    },
                    &upload_type,
                )
                .await
                .map_err(|e| Error::failed(e.to_string()))?;

            results
                .get()
                .set_callback(capnp_rpc::new_client(UploadCallback {
                    ns,
                    pg_pool: p,
                    gcs_client: uploader,
                    fname: filename,
                    fsize: size,
                    received: 0,
                    hasher: Keccak::v256(),
                }));
            Ok(())
        })
    }
}

/// RPC service wrapper for publication uploads.
struct UploadCallback {
    ns: String,
    pg_pool: PgPool,
    gcs_client: ResumableUploadClient,
    fname: String,
    fsize: u64,
    received: u64,
    hasher: Keccak,
}

impl publications::callback::Server for UploadCallback {
    fn write(
        &mut self,
        params: publications::callback::WriteParams,
        _: publications::callback::WriteResults,
    ) -> Promise<(), Error> {
        let args = pry!(params.get());
        let chunk = pry!(args.get_chunk()).to_vec();
        if chunk.is_empty() {
            return Promise::err(Error::failed("received empty chunk".into()));
        }
        let chunk_len = chunk.len() as u64;
        let first_byte = self.received;
        self.received += chunk_len;
        let last_byte = self.received - 1;
        let total_size = Some(self.fsize);
        let chunk_size = ChunkSize::new(first_byte, last_byte, total_size);

        self.hasher.update(&chunk);

        debug!(
            "processing: fb={}; lb={}; total={:?}",
            first_byte, last_byte, total_size
        );

        let c = self.gcs_client.clone();
        let s = self.received;
        Promise::from_future(async move {
            let result = c
                .upload_multiple_chunk(chunk, &chunk_size)
                .await
                .map_err(|e| Error::failed(e.to_string()))?;

            debug!("uploaded: {}; status: {:?}", s, result);

            Ok(())
        })
    }

    fn done(
        &mut self,
        params: publications::callback::DoneParams,
        _: publications::callback::DoneResults,
    ) -> Promise<(), Error> {
        let args = pry!(params.get());
        let sig = pry!(args.get_sig());
        if sig.len() != 65 {
            return Promise::err(Error::failed("signature must be 65 bytes".into()));
        }
        let mut output = [0u8; 32];
        self.hasher.clone().finalize(&mut output);
        let owner = pry!(crate::crypto::recover(&output, &sig[..64], sig[64] as i32));

        debug!("publication upload {} finished for {}", self.fname, owner);

        let n = self.ns.clone();
        let p = self.pg_pool.clone();
        Promise::from_future(async move {
            if crate::db::is_namespace_owner(&p, &n, owner).await? {
                Ok(())
            } else {
                // fixme: delete uploaded file?
                Err(Error::failed("unauthorized".into()))
            }
        })
    }
}

/// Listens for RPC messages from Basin clients
pub async fn listen<E: EVMClient>(
    evm_client: E,
    pg_pool: PgPool,
    gcs_client: GcsClient,
    tcp_listener: TcpListener,
) -> Result<(), Box<dyn std::error::Error>> {
    tokio::task::LocalSet::new()
        .run_until(async move {
            let pubs_handler = Publications::new(evm_client, pg_pool, gcs_client);
            let pubs_client: publications::Client = capnp_rpc::new_client(pubs_handler);
            info!("RPC API started");
            loop {
                let (stream, _) = tcp_listener.accept().await?;
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
