use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
}

/// Builder for checking API health
pub struct CheckHealthBuilder;

impl CheckHealthBuilder {
    pub fn new() -> Self {
        Self
    }

    pub async fn send(self) -> Result<HealthStatus> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/health", config.base_url);

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

/// Check the health status of the API using builder pattern
pub fn check() -> CheckHealthBuilder {
    CheckHealthBuilder::new()
}
