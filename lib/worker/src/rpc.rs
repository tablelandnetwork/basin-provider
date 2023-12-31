use crate::{crypto, db, domain::Vault, gcs::GcsClient, web3storage::Web3StorageClient};
use basin_evm::EVMClient;
use basin_protocol::publications;
use capnp::{capability::Promise, Error};
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};
use ethers::types::Address;
use futures::AsyncReadExt;
use google_cloud_storage::http::objects::{
    upload::{UploadObjectRequest, UploadType},
    Object,
};
use google_cloud_storage::http::resumable_upload_client::{ChunkSize, ResumableUploadClient};
use log::{debug, info};
use sqlx::postgres::PgPool;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use tiny_keccak::{Hasher, Keccak};
use tokio::net::TcpListener;

/// RPC service wrapper for publications.
pub struct Publications<E: EVMClient + 'static> {
    evm_client: E,
    pg_pool: PgPool,
    gcs_client: GcsClient,
    web3storage_client: Web3StorageClient,
}

impl<E: EVMClient + 'static> Publications<E> {
    pub fn new(
        evm_client: E,
        pg_pool: PgPool,
        gcs_client: GcsClient,
        web3storage_client: Web3StorageClient,
    ) -> Self {
        Self {
            evm_client,
            pg_pool,
            gcs_client,
            web3storage_client,
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

        let cache_duration = if args.get_cache_duration() > 0 {
            Some(args.get_cache_duration())
        } else {
            None
        };
        let name = format!("{ns}.{rel}");

        info!("publication {name} create for {owner}");

        let p = self.pg_pool.clone();
        let e = self.evm_client.clone();
        Promise::from_future(async move {
            e.add_pub(owner, name.as_str()).await?;
            let created = db::namespace_create(&p, &ns, &rel, cache_duration, owner).await?;
            results.get().set_exists(!created);
            Ok(())
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

        let timestamp = args.get_timestamp();
        if timestamp == 0 {
            return Promise::err(Error::failed("timestamp is required".into()));
        }

        let filename = format!(
            "{ns}/{rel}/{}.parquet",
            chrono::Utc::now().timestamp_micros()
        );

        info!("publication upload {filename} started");

        let p = self.pg_pool.clone();
        let c = self.gcs_client.clone();
        Promise::from_future(async move {
            if !db::namespace_exists(&p, &ns).await? {
                return Err(Error::failed("namespace not found".into()));
            }

            let cache_duration = db::get_cache_config(&p, &ns, &rel).await?;

            let mut metadata: HashMap<String, String> =
                HashMap::from([("timestamp".into(), format!("{}", timestamp))]);

            if cache_duration.is_some() {
                metadata.insert(
                    "cache_duration".into(),
                    format!("{}", cache_duration.unwrap()),
                );
            }

            let upload_type = UploadType::Multipart(Box::new(Object {
                name: filename.clone(),
                content_type: Some("application/octet-stream".into()),
                size: size as i64,
                metadata: Some(metadata),
                ..Default::default()
            }));

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

    fn list(
        &mut self,
        params: publications::ListParams,
        mut results: publications::ListResults,
    ) -> Promise<(), Error> {
        let args = pry!(params.get());
        let owner = Address::from_slice(pry!(args.get_owner()));
        if owner.is_zero() {
            return Promise::err(Error::failed("owner is required".into()));
        }

        let e = self.evm_client.clone();
        Promise::from_future(async move {
            let publications = e.list_pub(owner).await?;

            let mut publication_list = results.get().init_publications(publications.len() as u32);
            for (i, p) in publications.iter().enumerate() {
                publication_list.set(i as u32, p.as_str().into());
            }

            Ok(())
        })
    }

    fn deals(
        &mut self,
        params: publications::DealsParams,
        mut results: publications::DealsResults,
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
        let limit = args.get_limit() as i32;
        let offset = args.get_offset() as i32;

        let before = args.get_before();
        let after = args.get_after();

        let pg_pool = self.pg_pool.clone();
        let web3storage_client = self.web3storage_client.clone();
        Promise::from_future(async move {
            let vault = Vault::from(format!("{}.{}", ns, rel)).unwrap();
            let rows = db::pub_cids(&pg_pool, &vault, limit, offset, before, after).await?;

            let mut deals_list = results.get().init_deals(rows.len() as u32);

            let futures = rows
                .iter()
                .map(|deal_info| {
                    let client = web3storage_client.clone();
                    async move {
                        client
                            .status_of_cid(deal_info.cid.as_str())
                            .await
                            .map_err(|e| Error::failed(e.to_string()))
                    }
                })
                .collect::<Vec<_>>();

            let responses = futures::future::join_all(futures).await;

            for (i, (response, deal_info)) in std::iter::zip(responses, rows).enumerate() {
                let status = response
                    .as_ref()
                    .map_err(|e| Error::failed(e.to_string()))?;
                let mut builder = deals_list.reborrow().get(i as u32);

                builder.set_cid(status.cid.as_str().into());
                builder.set_timestamp(deal_info.timestamp);
                builder.set_size(status.dag_size);
                builder.set_archived(!status.deals.is_empty());

                if let Some(dt) = deal_info.cache_expiry {
                    builder
                        .set_expires_at(dt.format("%Y-%m-%d %H:%M:%S").to_string().as_str().into())
                } else {
                    builder.set_expires_at("".into())
                }
            }

            Ok(())
        })
    }

    fn latest_deals(
        &mut self,
        params: publications::LatestDealsParams,
        mut results: publications::LatestDealsResults,
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
        let n = args.get_n() as i32;
        let before = args.get_before();
        let after = args.get_after();

        let pg_pool = self.pg_pool.clone();
        let web3storage_client = self.web3storage_client.clone();
        Promise::from_future(async move {
            let vault = Vault::from(format!("{}.{}", ns, rel)).unwrap();
            let rows = db::pub_cids(&pg_pool, &vault, n, 0, before, after).await?;
            let mut deals_list = results.get().init_deals(rows.len() as u32);

            let futures = rows
                .iter()
                .map(|deal_info| {
                    let client = web3storage_client.clone();
                    async move {
                        client
                            .status_of_cid(deal_info.cid.as_str())
                            .await
                            .map_err(|e| Error::failed(e.to_string()))
                    }
                })
                .collect::<Vec<_>>();

            let responses = futures::future::join_all(futures).await;

            for (i, (response, deal_info)) in std::iter::zip(responses, rows).enumerate() {
                let status = response
                    .as_ref()
                    .map_err(|e| Error::failed(e.to_string()))?;
                let mut builder = deals_list.reborrow().get(i as u32);

                builder.set_cid(status.cid.as_str().into());
                builder.set_timestamp(deal_info.timestamp);
                builder.set_size(status.dag_size);
                builder.set_archived(!status.deals.is_empty());

                if let Some(dt) = deal_info.cache_expiry {
                    builder
                        .set_expires_at(dt.format("%Y-%m-%d %H:%M:%S").to_string().as_str().into())
                } else {
                    builder.set_expires_at("".into())
                }
            }

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
        let owner = pry!(crypto::recover(&output, &sig[..64], sig[64] as i32));

        debug!("publication upload {} finished for {}", self.fname, owner);

        let n = self.ns.clone();
        let p: Pool<Postgres> = self.pg_pool.clone();
        Promise::from_future(async move {
            if db::is_namespace_owner(&p, &n, owner).await? {
                Ok(())
            } else {
                // fixme: delete uploaded file?
                Err(Error::failed("unauthorized".into()))
            }
        })
    }
}

/// Listens for RPC messages from Basin clients
pub async fn listen<E: EVMClient + 'static>(
    evm_client: E,
    pg_pool: PgPool,
    gcs_client: GcsClient,
    web3storage_client: Web3StorageClient,
    tcp_listener: TcpListener,
) -> Result<(), Box<dyn std::error::Error>> {
    tokio::task::LocalSet::new()
        .run_until(async move {
            let pubs_handler =
                Publications::new(evm_client, pg_pool, gcs_client, web3storage_client);
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
