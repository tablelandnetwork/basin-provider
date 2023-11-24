use crate::gcs::GcsClient;
use crate::routes;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use basin_evm::EVMClient;
use sqlx::postgres::PgPool;
use std::net::TcpListener;

pub fn start_http_server<E: EVMClient + 'static + std::marker::Sync>(
    listener: TcpListener,
    db_pool: PgPool,
    evm_client: E,
    gcs_client: GcsClient,
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let evm_client = web::Data::new(evm_client);
    let gcs_client = web::Data::new(gcs_client);

    let server = HttpServer::new(move || {
        App::new()
            .route(
                "/vaults",
                web::get().to(routes::find_vaults_by_account::<E>),
            )
            .route(
                "/vaults/{pub_id}/records",
                web::get().to(routes::find_records_by_pub_id),
            )
            .route(
                "/records/{record_id}",
                web::get().to(routes::find_record_by_id),
            )
            .route("/health_check", web::get().to(routes::health_check))
            .app_data(db_pool.clone())
            .app_data(evm_client.clone())
            .app_data(gcs_client.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
