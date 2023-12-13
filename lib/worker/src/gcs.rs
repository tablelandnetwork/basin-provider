use base64::{engine::general_purpose, Engine};
use google_cloud_auth::{credentials::CredentialsFile, error::Error};
use google_cloud_storage::client::{Client, ClientConfig};

/// Wrapper for a Google Cloud Storage client.
#[derive(Clone)]
pub struct GcsClient {
    pub inner: Client,
    pub bucket: String,
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

        Ok(Self {
            inner: Client::new(gcs_config),
            bucket,
        })
    }
}
