use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{
        ComposioAuthConfigListResponse, ComposioConfigResponse, ComposioToolkitDetailsResponse,
        ComposioToolkitListResponse, EnableComposioAppRequest, UpdateComposioConfigRequest,
    },
};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct ComposioApi {
    client: WachtClient,
}

impl ComposioApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn fetch_config(&self) -> FetchComposioConfigBuilder {
        FetchComposioConfigBuilder::new(self.client.clone())
    }

    pub fn update_config(
        &self,
        request: UpdateComposioConfigRequest,
    ) -> UpdateComposioConfigBuilder {
        UpdateComposioConfigBuilder::new(self.client.clone(), request)
    }

    pub fn list_toolkits(&self) -> ListComposioToolkitsBuilder {
        ListComposioToolkitsBuilder::new(self.client.clone())
    }

    pub fn enable_app(&self, request: EnableComposioAppRequest) -> EnableComposioAppBuilder {
        EnableComposioAppBuilder::new(self.client.clone(), request)
    }

    pub fn disable_app(&self, slug: impl Into<String>) -> DisableComposioAppBuilder {
        DisableComposioAppBuilder::new(self.client.clone(), slug)
    }

    pub fn list_toolkit_auth_configs(
        &self,
        slug: impl Into<String>,
    ) -> ListComposioToolkitAuthConfigsBuilder {
        ListComposioToolkitAuthConfigsBuilder::new(self.client.clone(), slug)
    }

    pub fn fetch_toolkit_auth_details(
        &self,
        slug: impl Into<String>,
    ) -> FetchComposioToolkitAuthDetailsBuilder {
        FetchComposioToolkitAuthDetailsBuilder::new(self.client.clone(), slug)
    }
}

fn api_error(status: reqwest::StatusCode, prefix: &str, body: String) -> Error {
    Error::api_from_text(status, prefix, &body)
}

#[derive(Debug, Default, Serialize)]
struct ListToolkitsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

pub struct FetchComposioConfigBuilder {
    client: WachtClient,
}

impl FetchComposioConfigBuilder {
    fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub async fn send(self) -> Result<ComposioConfigResponse> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/composio/config",
                self.client.config().base_url
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to fetch Composio config",
                response.text().await?,
            ))
        }
    }
}

pub struct UpdateComposioConfigBuilder {
    client: WachtClient,
    request: UpdateComposioConfigRequest,
}

impl UpdateComposioConfigBuilder {
    fn new(client: WachtClient, request: UpdateComposioConfigRequest) -> Self {
        Self { client, request }
    }

    pub async fn send(self) -> Result<ComposioConfigResponse> {
        let response = self
            .client
            .http_client()
            .patch(format!(
                "{}/ai/composio/config",
                self.client.config().base_url
            ))
            .json(&self.request)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to update Composio config",
                response.text().await?,
            ))
        }
    }
}

pub struct ListComposioToolkitsBuilder {
    client: WachtClient,
    query: ListToolkitsQuery,
}

impl ListComposioToolkitsBuilder {
    fn new(client: WachtClient) -> Self {
        Self {
            client,
            query: ListToolkitsQuery::default(),
        }
    }

    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.query.search = Some(search.into());
        self
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.query.category = Some(category.into());
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.query.cursor = Some(cursor.into());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.query.limit = Some(limit);
        self
    }

    pub async fn send(self) -> Result<ComposioToolkitListResponse> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/composio/toolkits",
                self.client.config().base_url
            ))
            .query(&self.query)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to list Composio toolkits",
                response.text().await?,
            ))
        }
    }
}

pub struct EnableComposioAppBuilder {
    client: WachtClient,
    request: EnableComposioAppRequest,
}

impl EnableComposioAppBuilder {
    fn new(client: WachtClient, request: EnableComposioAppRequest) -> Self {
        Self { client, request }
    }

    pub async fn send(self) -> Result<ComposioConfigResponse> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/composio/apps",
                self.client.config().base_url
            ))
            .json(&self.request)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to enable Composio app",
                response.text().await?,
            ))
        }
    }
}

pub struct DisableComposioAppBuilder {
    client: WachtClient,
    slug: String,
}

impl DisableComposioAppBuilder {
    fn new(client: WachtClient, slug: impl Into<String>) -> Self {
        Self {
            client,
            slug: slug.into(),
        }
    }

    pub async fn send(self) -> Result<ComposioConfigResponse> {
        let response = self
            .client
            .http_client()
            .delete(format!(
                "{}/ai/composio/apps/{}",
                self.client.config().base_url,
                self.slug
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to disable Composio app",
                response.text().await?,
            ))
        }
    }
}

pub struct ListComposioToolkitAuthConfigsBuilder {
    client: WachtClient,
    slug: String,
}

impl ListComposioToolkitAuthConfigsBuilder {
    fn new(client: WachtClient, slug: impl Into<String>) -> Self {
        Self {
            client,
            slug: slug.into(),
        }
    }

    pub async fn send(self) -> Result<ComposioAuthConfigListResponse> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/composio/toolkits/{}/auth-configs",
                self.client.config().base_url,
                self.slug
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to list Composio auth configs",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchComposioToolkitAuthDetailsBuilder {
    client: WachtClient,
    slug: String,
}

impl FetchComposioToolkitAuthDetailsBuilder {
    fn new(client: WachtClient, slug: impl Into<String>) -> Self {
        Self {
            client,
            slug: slug.into(),
        }
    }

    pub async fn send(self) -> Result<ComposioToolkitDetailsResponse> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/composio/toolkits/{}/auth-details",
                self.client.config().base_url,
                self.slug
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to fetch Composio auth details",
                response.text().await?,
            ))
        }
    }
}
