use crate::helpers::spawn_app;
use actix_web::http;
use serde_json::json;

#[tokio::test]
async fn list_vaults() {
    let app = spawn_app().await;

    // setup
    app.create_vault("api.test".to_string()).await;
    app.create_vault("api.test2".to_string()).await;

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
async fn list_records() {
    let app = spawn_app().await;

    // setup
    app.create_vault("api.test".to_string()).await;
    app.write_record(
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
        .get_records_from_vaults("api.test")
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
async fn list_records_limit_and_offset() {
    let app = spawn_app().await;

    // setup
    app.create_vault("api.test".to_string()).await;
    app.write_record(
        "api",
        "test",
        "bafybeibtg4guil6a3xr5aw26ta37ks2yboyxb6g6sp72fxhkiyfawqrpi4",
        1700844695,
        None,
        None,
    )
    .await;
    app.write_record(
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
        .get_records_from_vaults_limit_and_offset("api.test", 1, 0)
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
async fn list_records_before_and_after() {
    let app = spawn_app().await;

    // setup
    app.create_vault("api.test".to_string()).await;
    app.write_record(
        "api",
        "test",
        "bafybeibtg4guil6a3xr5aw26ta37ks2yboyxb6g6sp72fxhkiyfawqrpi4",
        1700844694,
        None,
        None,
    )
    .await;
    app.write_record(
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
        .get_records_from_vaults_before_and_after("api.test", 0, 1700844695)
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
async fn download_record_not_found() {
    let app = spawn_app().await;

    // setup
    app.create_vault("api.test".to_string()).await;
    app.write_record(
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
        .get_record("bafybeibtg4guil6a3xr5aw26ta37ks2yboyxb6g6sp72fxhkiyfawqrpi4")
        .await
        .status();

    assert_eq!(http::StatusCode::NOT_FOUND, status);
}

#[tokio::test]
async fn download_record() {
    let app = spawn_app().await;

    // setup
    app.create_vault("api.test".to_string()).await;
    app.write_record(
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
        .get_record("bafybeibtg4guil6a3xr5aw26ta37ks2yboyxb6g6sp72fxhkiyfawqrpi4")
        .await
        .text()
        .await
        .unwrap();

    assert_eq!("Hello\n", data);
}
