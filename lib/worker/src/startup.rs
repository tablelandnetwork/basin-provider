use crate::gcs::GcsClient;

use basin_evm::EVMClient;
use sqlx::postgres::PgPool;
use std::net::SocketAddr;

use warp::Future;

pub fn start_http_server<E: EVMClient + 'static + std::marker::Sync>(
    addr: SocketAddr,
    db_pool: PgPool,
    evm_client: E,
    gcs_client: GcsClient,
) -> (SocketAddr, impl Future<Output = ()>) {
    warp::serve(api::routes(db_pool, evm_client, gcs_client)).bind_ephemeral(addr)
}

mod api {
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
    pub fn routes<E: EVMClient + 'static + std::marker::Sync>(
        db: PgPool,
        evm_client: E,
        gcs_client: GcsClient,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        health()
            .or(vaults_list(evm_client.clone()))
            .or(vaults_create(evm_client, db.clone()))
            .or(vaults_events_create(db.clone(), gcs_client.clone()))
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
    pub fn vaults_events_create(
        db: PgPool,
        gcs_client: GcsClient,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("vaults" / String / "events")
            .and(warp::post())
            .and(with_gcs_client(gcs_client))
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

    fn with_evm_client<E: EVMClient + 'static + std::marker::Sync>(
        evm_client: E,
    ) -> impl Filter<Extract = (E,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || evm_client.clone())
    }
}
