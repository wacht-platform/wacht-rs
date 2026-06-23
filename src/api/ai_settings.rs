use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{
        CreateDeploymentAiProviderProfileRequest, DeploymentAiProviderProfileResponse,
        DeploymentAiSettings, PaginatedResponse, UpdateDeploymentAiProviderProfileRequest,
        UpdateDeploymentAiSettingsRequest,
    },
};
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAiProviderProfilesOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
}

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

    pub fn list_provider_profiles(&self) -> ListAiProviderProfilesBuilder {
        ListAiProviderProfilesBuilder::new(self.client.clone())
    }

    pub fn create_provider_profile(
        &self,
        request: CreateDeploymentAiProviderProfileRequest,
    ) -> CreateAiProviderProfileBuilder {
        CreateAiProviderProfileBuilder::new(self.client.clone(), request)
    }

    pub fn fetch_provider_profile(
        &self,
        profile_id: impl Into<String>,
    ) -> FetchAiProviderProfileBuilder {
        FetchAiProviderProfileBuilder::new(self.client.clone(), profile_id)
    }

    pub fn update_provider_profile(
        &self,
        profile_id: impl Into<String>,
        request: UpdateDeploymentAiProviderProfileRequest,
    ) -> UpdateAiProviderProfileBuilder {
        UpdateAiProviderProfileBuilder::new(self.client.clone(), profile_id, request)
    }

    pub fn delete_provider_profile(
        &self,
        profile_id: impl Into<String>,
    ) -> DeleteAiProviderProfileBuilder {
        DeleteAiProviderProfileBuilder::new(self.client.clone(), profile_id)
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

pub struct ListAiProviderProfilesBuilder {
    client: WachtClient,
    options: ListAiProviderProfilesOptions,
}

impl ListAiProviderProfilesBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
            options: ListAiProviderProfilesOptions::default(),
        }
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.options.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.options.offset = Some(offset);
        self
    }

    pub async fn send(self) -> Result<PaginatedResponse<DeploymentAiProviderProfileResponse>> {
        let client = self.client.http_client();
        let url = format!(
            "{}/ai/settings/provider-profiles",
            self.client.config().base_url
        );

        let response = client.get(&url).query(&self.options).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to list AI provider profiles",
                &error_body,
            ))
        }
    }
}

pub struct CreateAiProviderProfileBuilder {
    client: WachtClient,
    request: CreateDeploymentAiProviderProfileRequest,
}

impl CreateAiProviderProfileBuilder {
    pub fn new(client: WachtClient, request: CreateDeploymentAiProviderProfileRequest) -> Self {
        Self { client, request }
    }

    pub async fn send(self) -> Result<DeploymentAiProviderProfileResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/ai/settings/provider-profiles",
            self.client.config().base_url
        );

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to create AI provider profile",
                &error_body,
            ))
        }
    }
}

pub struct FetchAiProviderProfileBuilder {
    client: WachtClient,
    profile_id: String,
}

impl FetchAiProviderProfileBuilder {
    pub fn new(client: WachtClient, profile_id: impl Into<String>) -> Self {
        Self {
            client,
            profile_id: profile_id.into(),
        }
    }

    pub async fn send(self) -> Result<DeploymentAiProviderProfileResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/ai/settings/provider-profiles/{}",
            self.client.config().base_url,
            self.profile_id
        );

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to fetch AI provider profile",
                &error_body,
            ))
        }
    }
}

pub struct UpdateAiProviderProfileBuilder {
    client: WachtClient,
    profile_id: String,
    request: UpdateDeploymentAiProviderProfileRequest,
}

impl UpdateAiProviderProfileBuilder {
    pub fn new(
        client: WachtClient,
        profile_id: impl Into<String>,
        request: UpdateDeploymentAiProviderProfileRequest,
    ) -> Self {
        Self {
            client,
            profile_id: profile_id.into(),
            request,
        }
    }

    pub async fn send(self) -> Result<DeploymentAiProviderProfileResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/ai/settings/provider-profiles/{}",
            self.client.config().base_url,
            self.profile_id
        );

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to update AI provider profile",
                &error_body,
            ))
        }
    }
}

pub struct DeleteAiProviderProfileBuilder {
    client: WachtClient,
    profile_id: String,
}

impl DeleteAiProviderProfileBuilder {
    pub fn new(client: WachtClient, profile_id: impl Into<String>) -> Self {
        Self {
            client,
            profile_id: profile_id.into(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/ai/settings/provider-profiles/{}",
            self.client.config().base_url,
            self.profile_id
        );

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to delete AI provider profile",
                &error_body,
            ))
        }
    }
}
