use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{DeploymentAiSettings, UpdateDeploymentAiSettingsRequest},
};

/// Get AI settings
pub async fn fetch_ai_settings() -> Result<DeploymentAiSettings> {
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

/// Update AI settings
pub async fn update_ai_settings(request: UpdateDeploymentAiSettingsRequest) -> Result<DeploymentAiSettings> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai/settings", config.base_url);

    let response = client.put(&url).json(&request).send().await?;
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
