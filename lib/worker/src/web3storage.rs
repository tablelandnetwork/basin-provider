use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;
use w3s::writer::uploader;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Status {
    pub created: String,
    pub cid: String,
    #[serde(rename = "dagSize")]
    pub dag_size: u32,
    pins: Vec<Pin>,
    pub deals: Vec<Deal>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Pin {
    status: String,
    updated: String,
    #[serde(rename = "peerId")]
    peer_id: String,
    #[serde(rename = "peerName")]
    peer_name: String,
    region: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Deal {
    #[serde(rename = "dealId")]
    deal_id: Option<u32>,
    #[serde(rename = "storageProvider")]
    storage_provider: Option<String>,
    status: String,
    #[serde(rename = "pieceCid")]
    piece_cid: String,
    #[serde(rename = "dataCid")]
    data_cid: String,
    #[serde(rename = "dataModelSelector")]
    data_model_selector: String,
    activation: Option<String>,
    expiration: Option<String>,
    created: Option<String>,
    updated: Option<String>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Reqwest error: {0:?}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Serde JSON parsing error. Response: {1}")]
    SerdeJSONError(#[source] serde_json::Error, String),
}

pub const DEFAULT_BASE_URL: &str = "https://api.web3.storage";

#[derive(Clone)]
pub struct Web3StorageClient {
    base_url: String,
    token: String,
}

impl Web3StorageClient {
    pub fn new(base_url: String, token: String) -> Self {
        Self { base_url, token }
    }

    pub async fn status_of_cid(&self, cid: &str) -> Result<Status, Error> {
        let result = Client::new()
            .get(format!("{}/status/{}", self.base_url, cid))
            .header("accept", "application/json")
            .send()
            .await?
            .text()
            .await?;

        let status: Status =
            serde_json::from_str(&result).map_err(|e| Error::SerdeJSONError(e, result))?;

        Ok(status)
    }

    pub fn get_uploader(&self, filename: String) -> uploader::Uploader {
        uploader::Uploader::new(
            self.token.clone(),
            self.base_url.clone(),
            filename,
            uploader::UploadType::Car,
            2,
            None,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::web3storage::Web3StorageClient;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn status_of_cid() {
        let mock_server = MockServer::start().await;

        let web3storage_client = Web3StorageClient::new(mock_server.uri(), String::from(""));

        let status_response = r#"{"cid":"bafybeibw2zctx4ca3udcfcsizjmo57bomhb6vvzf63rvc25d6hzotncn2i","dagSize":380733,"created":"2023-10-27T20:08:24.015+00:00","pins":[{"status":"Pinned","updated":"2023-10-27T20:08:24.015+00:00","peerId":"bafzbeibhqavlasjc7dvbiopygwncnrtvjd2xmryk5laib7zyjor6kf3avm","peerName":"elastic-ipfs","region":null}],"deals":[{"dealId":60497440,"storageProvider":"f01392893","status":"Active","pieceCid":"baga6ea4seaqmjfxq45gotde77ay7sqljb7gt5gns3vojgwoj3fb3zmqvddkx2py","dataCid":"bafybeibbpkhnm5wdyw2y2zirndu2coa7mw67vg52hzsl3ogae4owajkd4q","dataModelSelector":"Links/4/Hash/Links/54/Hash/Links/0/Hash","activation":"2023-10-31T07:33:00+00:00","expiration":"2025-04-15T07:33:00+00:00","created":"2023-10-31T13:20:03.875131+00:00","updated":"2023-10-31T13:20:03.875131+00:00"}]}"#;
        let response = ResponseTemplate::new(200).set_body_string(status_response);

        Mock::given(method("GET"))
            .and(path(
                "/status/bafybeibw2zctx4ca3udcfcsizjmo57bomhb6vvzf63rvc25d6hzotncn2i",
            ))
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;

        let status = web3storage_client
            .status_of_cid("bafybeibw2zctx4ca3udcfcsizjmo57bomhb6vvzf63rvc25d6hzotncn2i")
            .await
            .unwrap();

        assert_eq!(
            "bafybeibw2zctx4ca3udcfcsizjmo57bomhb6vvzf63rvc25d6hzotncn2i",
            status.cid
        );
        assert_eq!(380733, status.dag_size);
        assert_eq!("2023-10-27T20:08:24.015+00:00", status.created);

        // pin
        assert_eq!("Pinned", status.pins[0].status);
        assert_eq!("2023-10-27T20:08:24.015+00:00", status.pins[0].updated);
        assert_eq!(
            "bafzbeibhqavlasjc7dvbiopygwncnrtvjd2xmryk5laib7zyjor6kf3avm",
            status.pins[0].peer_id
        );
        assert_eq!("elastic-ipfs", status.pins[0].peer_name);
        assert_eq!(None, status.pins[0].region);

        // deal
        assert_eq!(Some(60497440), status.deals[0].deal_id);
        assert_eq!(
            Some("f01392893".to_string()),
            status.deals[0].storage_provider
        );
        assert_eq!("Active", status.deals[0].status);
        assert_eq!(
            "baga6ea4seaqmjfxq45gotde77ay7sqljb7gt5gns3vojgwoj3fb3zmqvddkx2py",
            status.deals[0].piece_cid
        );
        assert_eq!(
            "bafybeibbpkhnm5wdyw2y2zirndu2coa7mw67vg52hzsl3ogae4owajkd4q",
            status.deals[0].data_cid
        );
        assert_eq!(
            "Links/4/Hash/Links/54/Hash/Links/0/Hash",
            status.deals[0].data_model_selector
        );
        assert_eq!(
            Some("2023-10-31T07:33:00+00:00".to_string()),
            status.deals[0].activation
        );
        assert_eq!(
            Some("2025-04-15T07:33:00+00:00".to_string()),
            status.deals[0].expiration
        );
        assert_eq!(
            Some("2023-10-31T13:20:03.875131+00:00".to_string()),
            status.deals[0].created
        );
        assert_eq!(
            Some("2023-10-31T13:20:03.875131+00:00".to_string()),
            status.deals[0].updated
        );
    }
}
