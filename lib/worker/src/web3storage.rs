use bytes::Bytes;

use async_trait::async_trait;
use cid::Cid;
use futures::StreamExt;
use multihash_codetable::{Code, MultihashDigest};
use reqwest::{multipart, Body, Client};
use serde::Deserialize;

use thiserror::Error;

#[derive(Deserialize, Debug)]
pub struct UploadResponse {
    pub root: String,
    pub shard: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Reqwest error: {0:?}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Serde JSON parsing error. Response: {1}")]
    SerdeJSONError(#[source] serde_json::Error, String),
}

#[async_trait]
pub trait Web3StorageClient: Clone + Send {
    async fn upload(
        &self,
        stream: impl StreamExt<Item = Result<Bytes, google_cloud_storage::http::Error>>
            + Sync
            + Send
            + 'static,
        filename: &str,
    ) -> Result<UploadResponse, Error>;
}

#[derive(Clone)]
pub struct Web3StorageImpl {
    base_url: String,
}

impl Web3StorageImpl {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
}

#[async_trait]
impl Web3StorageClient for Web3StorageImpl {
    async fn upload(
        &self,
        stream: impl StreamExt<Item = Result<Bytes, google_cloud_storage::http::Error>>
            + Sync
            + Send
            + 'static,
        filename: &str,
    ) -> Result<UploadResponse, Error> {
        let file = multipart::Part::stream(Body::wrap_stream(stream))
            .file_name(filename.to_string())
            .mime_str("application/octet-stream")?;

        let form = multipart::Form::new().part("file", file);

        let result = Client::new()
            .post(format!("http://{}/api/v1/upload", self.base_url))
            .multipart(form)
            .send()
            .await?
            .text()
            .await?;

        let response: UploadResponse =
            serde_json::from_str(&result).map_err(|e| Error::SerdeJSONError(e, result))?;

        Ok(response)
    }
}

#[derive(Clone)]
pub struct Web3StorageMock {}

#[async_trait]
impl Web3StorageClient for Web3StorageMock {
    async fn upload(
        &self,
        _stream: impl StreamExt<Item = Result<Bytes, google_cloud_storage::http::Error>>
            + Sync
            + Send
            + 'static,
        _filename: &str,
    ) -> Result<UploadResponse, Error> {
        let h = Code::Sha2_256.digest(b"mocked cid");
        const RAW: u64 = 0x55;
        let cid = Cid::new_v1(RAW, h);

        Ok(UploadResponse {
            root: cid.to_string(),
            shard: cid.to_string(),
        })
    }
}
