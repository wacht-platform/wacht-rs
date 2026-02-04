use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{DeploymentAiSettings, UpdateDeploymentAiSettingsRequest},
};

/// Builder for fetching AI settings
pub struct FetchAiSettingsBuilder {
    _private: (),
}

impl FetchAiSettingsBuilder {
    /// Create a new builder for fetching AI settings
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Execute the fetch operation
    pub async fn send(self) -> Result<DeploymentAiSettings> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/settings", config.base_url);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch AI settings: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for updating AI settings
pub struct UpdateAiSettingsBuilder {
    request: UpdateDeploymentAiSettingsRequest,
}

impl UpdateAiSettingsBuilder {
    /// Create a new builder for updating AI settings
    pub fn new(request: UpdateDeploymentAiSettingsRequest) -> Self {
        Self { request }
    }

    /// Execute the update operation
    pub async fn send(self) -> Result<DeploymentAiSettings> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/settings", config.base_url);

        let response = client.put(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update AI settings: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Convenience function to create a fetch AI settings builder
pub fn fetch_ai_settings() -> FetchAiSettingsBuilder {
    FetchAiSettingsBuilder::new()
}

/// Convenience function to create an update AI settings builder
pub fn update_ai_settings(request: UpdateDeploymentAiSettingsRequest) -> UpdateAiSettingsBuilder {
    UpdateAiSettingsBuilder::new(request)
}
