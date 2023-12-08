use base64::{engine::general_purpose, Engine};
use google_cloud_auth::{credentials::CredentialsFile, error::Error};
use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http;
use url::Url;

/// Wrapper for a Google Cloud Storage client.
#[derive(Clone)]
pub struct GcsClient {
    pub inner: Client,
    pub bucket: String,
    endpoint: String,
    access_token: String,
}

impl GcsClient {
    pub async fn new(
        bucket: String,
        creds: String,
        endpoint: Option<String>,
    ) -> Result<Self, Error> {
        let gcs_creds = general_purpose::STANDARD
            .decode(creds.clone())
            .map_err(|_| Error::NoCredentialsFileFound)?;
        let gcs_creds: CredentialsFile = serde_json::from_slice(gcs_creds.as_slice())?;
        let default_config = ClientConfig::default().with_credentials(gcs_creds).await?;

        let gcs_config = match endpoint.clone() {
            Some(v) => {
                let http_client = reqwest::Client::builder()
                    .danger_accept_invalid_certs(true)
                    .build()
                    .ok();
                ClientConfig {
                    http: http_client,
                    storage_endpoint: v,
                    ..default_config
                }
            }
            None => ClientConfig { ..default_config },
        };

        let endpoint = gcs_config.storage_endpoint.clone();

        Ok(Self {
            inner: Client::new(gcs_config),
            bucket,
            endpoint,
            access_token: creds,
        })
    }

    pub async fn patch_object(
        &self,
        filename: String,
        req: http::objects::patch::PatchObjectRequest,
    ) -> Result<(), http::Error> {
        let endpoint = Url::parse(&self.endpoint).unwrap();
        let hostname = endpoint.host_str().unwrap();
        let client = if hostname == "localhost" {
            reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .ok()
                .unwrap()
        } else {
            reqwest::Client::builder().build().ok().unwrap()
        };

        let url = format!(
            "{}/storage/v1/b/{}/o/{}",
            self.endpoint, self.bucket, filename
        );
        let access_token = self.access_token.clone();
        let builder = client
            .patch(url)
            .query(&req)
            .json(&req.metadata)
            .header("X-Goog-Api-Client", "rust")
            .header(reqwest::header::USER_AGENT, "google-cloud-storage")
            .header(reqwest::header::AUTHORIZATION, access_token);
        builder.send().await?;
        Ok(())
    }
}
