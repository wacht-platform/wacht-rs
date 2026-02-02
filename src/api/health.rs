use crate::{
    client::{get_client, get_config},
    error::Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// Check the health status of the API
pub async fn check_health() -> Result<HealthStatus> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/health", config.base_url);
    
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        Ok(response.json().await?)
    } else {
        Err(crate::error::Error::Api {
            status: response.status(),
            message: "Health check failed".to_string(),
            details: None,
        })
    }
}

/// Check if the API is alive (simple ping)
pub async fn ping() -> Result<bool> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/health/ping", config.base_url);
    
    let response = client.get(&url).send().await?;
    Ok(response.status().is_success())
}