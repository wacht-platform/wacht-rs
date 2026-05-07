use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{DeploymentAiSettings, UpdateDeploymentAiSettingsRequest},
};

#[derive(Debug, Clone)]
pub struct AiSettingsApi {
    client: WachtClient,
}

impl AiSettingsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn fetch_ai_settings(&self) -> FetchAiSettingsBuilder {
        FetchAiSettingsBuilder::new(self.client.clone())
    }

    pub fn update_ai_settings(
        &self,
        request: UpdateDeploymentAiSettingsRequest,
    ) -> UpdateAiSettingsBuilder {
        UpdateAiSettingsBuilder::new(self.client.clone(), request)
    }
}

pub struct FetchAiSettingsBuilder {
    client: WachtClient,
}

impl FetchAiSettingsBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub async fn send(self) -> Result<DeploymentAiSettings> {
        let client = self.client.http_client();
        let url = format!("{}/ai/settings", self.client.config().base_url);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to fetch AI settings",
                &error_body,
            ))
        }
    }
}

pub struct UpdateAiSettingsBuilder {
    client: WachtClient,
    request: UpdateDeploymentAiSettingsRequest,
}

impl UpdateAiSettingsBuilder {
    pub fn new(client: WachtClient, request: UpdateDeploymentAiSettingsRequest) -> Self {
        Self { client, request }
    }

    pub async fn send(self) -> Result<DeploymentAiSettings> {
        let client = self.client.http_client();
        let url = format!("{}/ai/settings", self.client.config().base_url);

        let response = client.put(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to update AI settings",
                &error_body,
            ))
        }
    }
}
