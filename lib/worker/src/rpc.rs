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
        _: publications::CreateResults,
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
        let table_stmt = pry!(crate::sql::schema_to_table_create(name.clone(), schema));

        info!("publication {name} create for {owner}");
        debug!("table statement: {table_stmt}");

        let p = self.pg_pool.clone();
        let e = self.evm_client.clone();
        Promise::from_future(async move {
            e.add_pub(owner, name.as_str()).await?;
            crate::db::namespace_create(&p, ns, owner).await?;
            crate::db::pub_table_create(&p, &table_stmt).await?;
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
        if sig.is_empty() {
            return Promise::err(Error::failed("signature is required".into()));
        }
        let tx = pry!(args.get_tx());
        let owner = pry!(crate::utils::recover_addr(tx, sig));
        let name = format!("{ns}.{rel}");
        let insert_stmt = pry!(crate::sql::tx_to_table_inserts(name.clone(), tx));

        info!("publication {name} push for {owner}");
        debug!("insert statements: {:?}", insert_stmt);

        let p = self.pg_pool.clone();
        Promise::from_future(async move {
            if crate::db::is_namespace_owner(&p, ns, owner).await? {
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
        let filename = format!(
            "{ns}/{rel}/{}.parquet",
            chrono::Utc::now().timestamp_micros()
        );

        println!("publication upload {filename} started");

        let c = self.gcs_client.clone();
        Promise::from_future(async move {
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
                .set_callback(capnp_rpc::new_client(UploadCallback::new(
                    filename, uploader,
                )));
            Ok(())
        })
    }
}

/// RPC service wrapper for publication uploads.
struct UploadCallback {
    filename: String,
    gcs_client: ResumableUploadClient,
    size: u64,
}

impl UploadCallback {
    fn new(filename: String, gcs_client: ResumableUploadClient) -> Self {
        Self {
            filename,
            gcs_client,
            size: 0,
        }
    }
}

impl publications::callback::Server for UploadCallback {
    // fixme: build hash
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
        let first_byte = self.size;
        self.size += chunk_len;
        let last_byte = self.size - 1;
        let total_size = if chunk_len < 8 * 1024 * 1024 {
            // fixme: If this is the last chunk, we can specify the total size but this won't work if last chunk = chunk size :/
            Some(self.size)
        } else {
            None
        };
        let chunk_size = ChunkSize::new(first_byte, last_byte, total_size);

        println!(
            "publication upload {} progress: fb={}; lb={}; total={:?}",
            self.filename, first_byte, last_byte, total_size
        );

        let c = self.gcs_client.clone();
        let s = self.size;
        Promise::from_future(async move {
            let result = c
                .upload_multiple_chunk(chunk, &chunk_size)
                .await
                .map_err(|e| Error::failed(e.to_string()))?;

            println!("total uploaded: {}; upload status: {:?}", s, result);

            Ok(())
        })
    }

    // fixme: check ns owner
    // fixme: check signature
    fn done(
        &mut self,
        params: publications::callback::DoneParams,
        _: publications::callback::DoneResults,
    ) -> Promise<(), Error> {
        let args = pry!(params.get());
        let sig = pry!(args.get_sig());
        if sig.is_empty() {
            return Promise::err(Error::failed("signature is required".into()));
        }

        println!("publication upload {} finished", self.filename);

        Promise::ok(())
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
