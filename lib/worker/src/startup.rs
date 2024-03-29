use crate::gcs::GcsClient;
use crate::web3storage::Web3Storage;

use basin_evm::EVMClient;
use sqlx::postgres::PgPool;
use std::net::SocketAddr;

use warp::Future;

pub fn start_http_server<E: EVMClient + 'static + Sync, W: Web3Storage + 'static + Sync>(
    addr: SocketAddr,
    db_pool: PgPool,
    evm_client: E,
    gcs_client: GcsClient,
    w3s_client: W,
) -> (SocketAddr, impl Future<Output = ()>) {
    warp::serve(api::routes(db_pool, evm_client, gcs_client, w3s_client)).bind_ephemeral(addr)
}

mod api {
    use crate::web3storage::Web3Storage;
    use crate::{
        gcs::GcsClient,
        routes::{
            self, CreateVaultInput, FindEventsByPubIdQueryParams, FindVaultsByAccountParams,
            WriteEventParams,
        },
    };
    use basin_evm::EVMClient;
    use sqlx::PgPool;
    use warp::Filter;

    // all routes
    pub fn routes<E: EVMClient + 'static + std::marker::Sync, W: Web3Storage>(
        db: PgPool,
        evm_client: E,
        gcs_client: GcsClient,
        w3s_client: W,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        health()
            .or(vaults_list(evm_client.clone()))
            .or(vaults_list_v2(evm_client.clone(), db.clone()))
            .or(vaults_create(evm_client, db.clone()))
            .or(vaults_events_create(
                db.clone(),
                gcs_client.clone(),
                w3s_client.clone(),
            ))
            .or(vaults_events_list(db.clone()))
            .or(events_get(db, gcs_client))
    }

    // GET /health
    pub fn health() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("health")
            .and(warp::get())
            .and_then(routes::health_check)
    }

    // GET /vaults
    pub fn vaults_list<E: EVMClient + 'static + std::marker::Sync>(
        evm_client: E,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("vaults")
            .and(warp::get())
            .and(with_evm_client(evm_client))
            .and(warp::query::<FindVaultsByAccountParams>())
            .and_then(routes::find_vaults_by_account)
    }

    // GET /v2/vaults
    pub fn vaults_list_v2<E: EVMClient + 'static + std::marker::Sync>(
        evm_client: E,
        db: PgPool,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("v2" / "vaults")
            .and(warp::get())
            .and(with_evm_client(evm_client))
            .and(with_db(db))
            .and(warp::query::<FindVaultsByAccountParams>())
            .and_then(routes::find_vaults_by_account_v2)
    }

    // POST /vaults/:id
    pub fn vaults_create<E: EVMClient + 'static + std::marker::Sync>(
        evm_client: E,
        db: PgPool,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("vaults" / String)
            .and(warp::post())
            .and(with_evm_client(evm_client))
            .and(with_db(db))
            .and(warp::filters::body::form::<CreateVaultInput>())
            .and_then(routes::create_vault)
    }

    // POST /vaults/:id/events
    pub fn vaults_events_create<W: Web3Storage>(
        db: PgPool,
        gcs_client: GcsClient,
        w3s_client: W,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("vaults" / String / "events")
            .and(warp::post())
            .and(warp::header::<u64>("content-length"))
            .and(warp::header::<String>("filename"))
            .and(with_gcs_client(gcs_client))
            .and(with_w3s_client(w3s_client))
            .and(with_db(db))
            .and(warp::query::<WriteEventParams>())
            .and(warp::filters::body::stream())
            .and_then(routes::write_event)
    }

    // GET /vaults/:id/events
    pub fn vaults_events_list(
        db: PgPool,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("vaults" / String / "events")
            .and(warp::get())
            .and(with_db(db))
            .and(warp::query::<FindEventsByPubIdQueryParams>())
            .and_then(routes::find_events_by_vault_id)
    }

    // GET /events/:id
    pub fn events_get(
        db: PgPool,
        gcs_client: GcsClient,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("events" / String)
            .and(warp::get())
            .and(with_db(db))
            .and(with_gcs_client(gcs_client))
            .and_then(routes::find_event_by_id)
    }

    fn with_db(
        db: PgPool,
    ) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }

    fn with_gcs_client(
        gcs_client: GcsClient,
    ) -> impl Filter<Extract = (GcsClient,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || gcs_client.clone())
    }

    fn with_w3s_client<W: Web3Storage>(
        w3s_client: W,
    ) -> impl Filter<Extract = (W,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || w3s_client.clone())
    }

    fn with_evm_client<E: EVMClient + 'static + std::marker::Sync>(
        evm_client: E,
    ) -> impl Filter<Extract = (E,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || evm_client.clone())
    }
}
