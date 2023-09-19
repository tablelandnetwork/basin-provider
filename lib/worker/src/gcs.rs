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
    pub async fn new(bucket: String, creds: String) -> Result<Self, Error> {
        let gcs_creds = general_purpose::STANDARD
            .decode(creds)
            .map_err(|_| Error::NoCredentialsFileFound)?;
        let gcs_creds: CredentialsFile = serde_json::from_slice(gcs_creds.as_slice())?;
        let gcs_config = ClientConfig::default().with_credentials(gcs_creds).await?;

        Ok(Self {
            inner: Client::new(gcs_config),
            bucket,
        })
    }
}
