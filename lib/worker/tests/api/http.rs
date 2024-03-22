use crate::helpers::spawn_app;
use basin_evm::EVMClient;
use ethers::{
    core::rand::{distributions::Uniform, thread_rng, Rng},
    signers::Signer,
};

use reqwest::StatusCode;
use serde_json::json;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let response = app.health_status().await;

    assert_eq!(200, response.status());
}

#[tokio::test]
async fn list_vaults() {
    let app = spawn_app().await;

    // setup
    app.create_vault("api.test").await;
    app.create_vault("api.test2").await;

    // make request
    let response = app
        .get_vaults()
        .await
        .text()
        .await
        .unwrap()
        .parse::<serde_json::Value>()
        .unwrap();
    assert_eq!(json!(["api.test", "api.test2"]), response);
}

#[tokio::test]
async fn list_vaults_v2() {
    let app = spawn_app().await;

    // setup
    app.create_vault("api.test").await;
    app.create_vault_with_cache("api.test2", 100).await;

    // make request
    let response = app
        .get_vaults_v2()
        .await
        .text()
        .await
        .unwrap()
        .parse::<serde_json::Value>()
        .unwrap();
    assert_eq!(
        json!([
        {
            "vault": "api.test",
            "cache_duration": null,
        },
        {
            "vault": "api.test2",
            "cache_duration": 100,
        }]),
        response
    );
}

#[tokio::test]
async fn list_events() {
    let app = spawn_app().await;

    // setup
    app.create_vault("api.test").await;
    app.write_event_to_db(
        "api",
        "test",
        "bafybeibtg4guil6a3xr5aw26ta37ks2yboyxb6g6sp72fxhkiyfawqrpi4",
        1700844695,
        None,
        None,
    )
    .await;

    // make request
    let response = app
        .get_events_from_vaults("api.test")
        .await
        .text()
        .await
        .unwrap()
        .parse::<serde_json::Value>()
        .unwrap();

    assert_eq!(
        json!([{"cid":"bafybeibtg4guil6a3xr5aw26ta37ks2yboyxb6g6sp72fxhkiyfawqrpi4","timestamp":1700844695,"cache_expiry":null}]),
        response
    );
}

#[tokio::test]
async fn list_events_limit_and_offset() {
    let app = spawn_app().await;

    // setup
    app.create_vault("api.test").await;
    app.write_event_to_db(
        "api",
        "test",
        "bafybeibtg4guil6a3xr5aw26ta37ks2yboyxb6g6sp72fxhkiyfawqrpi4",
        1700844695,
        None,
        None,
    )
    .await;
    app.write_event_to_db(
        "api",
        "test",
        "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi",
        1700844696,
        None,
        None,
    )
    .await;

    // make request
    let response = app
        .get_events_from_vaults_limit_and_offset("api.test", 1, 0)
        .await
        .text()
        .await
        .unwrap()
        .parse::<serde_json::Value>()
        .unwrap();

    assert_eq!(
        json!([{"cid":"bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi","timestamp":1700844696,"cache_expiry":null}]),
        response
    );
}

#[tokio::test]
async fn list_events_before_and_after() {
    let app = spawn_app().await;

    // setup
    app.create_vault("api.test").await;
    app.write_event_to_db(
        "api",
        "test",
        "bafybeibtg4guil6a3xr5aw26ta37ks2yboyxb6g6sp72fxhkiyfawqrpi4",
        1700844694,
        None,
        None,
    )
    .await;
    app.write_event_to_db(
        "api",
        "test",
        "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi",
        1700844696,
        None,
        None,
    )
    .await;

    // make request
    let response = app
        .get_events_from_vaults_before_and_after("api.test", 0, 1700844695)
        .await
        .text()
        .await
        .unwrap()
        .parse::<serde_json::Value>()
        .unwrap();

    assert_eq!(
        json!([{"cid":"bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi","timestamp":1700844696,"cache_expiry":null}]),
        response
    );
}

#[tokio::test]
async fn download_event_not_found() {
    let app = spawn_app().await;

    // setup
    app.create_vault("api.test").await;
    app.write_event_to_db(
        "api",
        "test",
        "bafybeibtg4guil6a3xr5aw26ta37ks2yboyxb6g6sp72fxhkiyfawqrpi4",
        1700844695,
        None,
        None,
    )
    .await;

    // make request
    let status = app
        .get_event("bafybeibtg4guil6a3xr5aw26ta37ks2yboyxb6g6sp72fxhkiyfawqrpi4")
        .await
        .status();

    assert_eq!(StatusCode::NOT_FOUND, status);
}

#[tokio::test]
async fn download_event() {
    let app = spawn_app().await;

    // setup
    app.create_vault("api.test").await;
    app.write_data_to_gcs(
        "api/test/1700685959292362.parquet".to_string(),
        "Hello\n".to_string(),
    )
    .await;
    app.write_event_to_db(
        "api",
        "test",
        "bafybeibtg4guil6a3xr5aw26ta37ks2yboyxb6g6sp72fxhkiyfawqrpi4",
        1700844695,
        Some("api/test/1700685959292362.parquet".to_string()),
        None,
    )
    .await;

    // make request
    let data = app
        .get_event("bafybeibtg4guil6a3xr5aw26ta37ks2yboyxb6g6sp72fxhkiyfawqrpi4")
        .await
        .text()
        .await
        .unwrap();

    assert_eq!("Hello\n", data);
}

#[tokio::test]
async fn create_vault() {
    let app = spawn_app().await;

    app.create_vault("api.test").await;
    let vaults = app
        .evm_client
        .list_pub(app.account.address())
        .await
        .unwrap();

    assert_eq!("api.test", vaults[0]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn write_event() {
    let app = spawn_app().await;

    app.create_vault_with_cache("api.test", 10).await;

    // creating random event
    let range = Uniform::from(0..20);
    let event_content = thread_rng().sample_iter(&range).take(100).collect();

    let response = app
        .upload_event("api.test", 1701372646, event_content)
        .await;

    assert_eq!(response.status(), StatusCode::CREATED);
}
