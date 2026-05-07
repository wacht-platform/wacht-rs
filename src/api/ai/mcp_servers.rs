use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{
        CreateMcpServerRequest, McpServer, McpServerCreateResponse, McpServerDiscoveryResponse,
        PaginatedResponse, UpdateMcpServerRequest,
    },
};
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListMcpServersOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct McpServersApi {
    client: WachtClient,
}
impl McpServersApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }
    pub fn fetch_mcp_servers(&self) -> FetchMcpServersBuilder {
        FetchMcpServersBuilder::new(self.client.clone())
    }
    pub fn fetch_mcp_server(&self, mcp_server_id: impl Into<String>) -> FetchMcpServerBuilder {
        FetchMcpServerBuilder::new(self.client.clone(), mcp_server_id)
    }
    pub fn create_mcp_server(&self, request: CreateMcpServerRequest) -> CreateMcpServerBuilder {
        CreateMcpServerBuilder::new(self.client.clone(), request)
    }
    pub fn update_mcp_server(
        &self,
        mcp_server_id: impl Into<String>,
        request: UpdateMcpServerRequest,
    ) -> UpdateMcpServerBuilder {
        UpdateMcpServerBuilder::new(self.client.clone(), mcp_server_id, request)
    }
    pub fn delete_mcp_server(&self, mcp_server_id: impl Into<String>) -> DeleteMcpServerBuilder {
        DeleteMcpServerBuilder::new(self.client.clone(), mcp_server_id)
    }
    pub fn discover_mcp_server(&self, endpoint: impl Into<String>) -> DiscoverMcpServerBuilder {
        DiscoverMcpServerBuilder::new(self.client.clone(), endpoint)
    }
}

fn api_error(status: reqwest::StatusCode, prefix: &str, body: String) -> Error {
    Error::api_from_text(status, prefix, &body)
}

pub struct FetchMcpServersBuilder {
    client: WachtClient,
    options: ListMcpServersOptions,
}
impl FetchMcpServersBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
            options: ListMcpServersOptions::default(),
        }
    }
    pub fn limit(mut self, limit: i32) -> Self {
        self.options.limit = Some(limit);
        self
    }
    pub fn offset(mut self, offset: i32) -> Self {
        self.options.offset = Some(offset);
        self
    }
    pub async fn send(self) -> Result<PaginatedResponse<McpServer>> {
        let response = self
            .client
            .http_client()
            .get(format!("{}/ai/mcp-servers", self.client.config().base_url))
            .query(&self.options)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to list MCP servers",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchMcpServerBuilder {
    client: WachtClient,
    mcp_server_id: String,
}
impl FetchMcpServerBuilder {
    pub fn new(client: WachtClient, mcp_server_id: impl Into<String>) -> Self {
        Self {
            client,
            mcp_server_id: mcp_server_id.into(),
        }
    }
    pub async fn send(self) -> Result<McpServer> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/mcp-servers/{}",
                self.client.config().base_url,
                self.mcp_server_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to fetch MCP server",
                response.text().await?,
            ))
        }
    }
}

pub struct CreateMcpServerBuilder {
    client: WachtClient,
    request: CreateMcpServerRequest,
}
impl CreateMcpServerBuilder {
    pub fn new(client: WachtClient, request: CreateMcpServerRequest) -> Self {
        Self { client, request }
    }
    pub async fn send(self) -> Result<McpServerCreateResponse> {
        let response = self
            .client
            .http_client()
            .post(format!("{}/ai/mcp-servers", self.client.config().base_url))
            .json(&self.request)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to create MCP server",
                response.text().await?,
            ))
        }
    }
}

pub struct UpdateMcpServerBuilder {
    client: WachtClient,
    mcp_server_id: String,
    request: UpdateMcpServerRequest,
}
impl UpdateMcpServerBuilder {
    pub fn new(
        client: WachtClient,
        mcp_server_id: impl Into<String>,
        request: UpdateMcpServerRequest,
    ) -> Self {
        Self {
            client,
            mcp_server_id: mcp_server_id.into(),
            request,
        }
    }
    pub async fn send(self) -> Result<McpServer> {
        let response = self
            .client
            .http_client()
            .patch(format!(
                "{}/ai/mcp-servers/{}",
                self.client.config().base_url,
                self.mcp_server_id
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
                "Failed to update MCP server",
                response.text().await?,
            ))
        }
    }
}

pub struct DeleteMcpServerBuilder {
    client: WachtClient,
    mcp_server_id: String,
}
impl DeleteMcpServerBuilder {
    pub fn new(client: WachtClient, mcp_server_id: impl Into<String>) -> Self {
        Self {
            client,
            mcp_server_id: mcp_server_id.into(),
        }
    }
    pub async fn send(self) -> Result<()> {
        let response = self
            .client
            .http_client()
            .delete(format!(
                "{}/ai/mcp-servers/{}",
                self.client.config().base_url,
                self.mcp_server_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(api_error(
                status,
                "Failed to delete MCP server",
                response.text().await?,
            ))
        }
    }
}

#[derive(Serialize)]
struct DiscoverMcpServerRequest {
    endpoint: String,
}

pub struct DiscoverMcpServerBuilder {
    client: WachtClient,
    endpoint: String,
}
impl DiscoverMcpServerBuilder {
    pub fn new(client: WachtClient, endpoint: impl Into<String>) -> Self {
        Self {
            client,
            endpoint: endpoint.into(),
        }
    }
    pub async fn send(self) -> Result<McpServerDiscoveryResponse> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/mcp-servers/discover",
                self.client.config().base_url
            ))
            .json(&DiscoverMcpServerRequest {
                endpoint: self.endpoint,
            })
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to discover MCP server auth",
                response.text().await?,
            ))
        }
    }
}
