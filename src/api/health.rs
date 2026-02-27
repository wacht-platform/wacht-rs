use crate::{
    client::WachtClient,
    error::{Error, Result},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct HealthApi {
    client: WachtClient,
}

impl HealthApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn check(&self) -> CheckHealthBuilder {
        CheckHealthBuilder::new(self.client.clone())
    }
}

/// Builder for checking API health
pub struct CheckHealthBuilder {
    client: WachtClient,
}

impl CheckHealthBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub async fn send(self) -> Result<HealthStatus> {
        let client = self.client.http_client();
        let url = format!("{}/health", self.client.config().base_url);

        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(Error::Api {
                status: response.status(),
                message: "Health check failed".to_string(),
                details: None,
            })
        }
    }
}
