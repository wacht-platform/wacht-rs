use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{
        AiTool, AiToolWithDetails, CreateAiToolRequest, PaginatedResponse,
        UpdateAgentToolApprovalActionRequest, UpdateAiToolRequest,
    },
};
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListToolsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ToolsApi {
    client: WachtClient,
}
impl ToolsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }
    pub fn list_tools(&self) -> ListToolsBuilder {
        ListToolsBuilder::new(self.client.clone())
    }
    pub fn fetch_tool(&self, tool_id: impl Into<String>) -> FetchToolBuilder {
        FetchToolBuilder::new(self.client.clone(), tool_id)
    }
    pub fn create_tool(&self, request: CreateAiToolRequest) -> CreateToolBuilder {
        CreateToolBuilder::new(self.client.clone(), request)
    }
    pub fn update_tool(
        &self,
        tool_id: impl Into<String>,
        request: UpdateAiToolRequest,
    ) -> UpdateToolBuilder {
        UpdateToolBuilder::new(self.client.clone(), tool_id, request)
    }
    pub fn delete_tool(&self, tool_id: impl Into<String>) -> DeleteToolBuilder {
        DeleteToolBuilder::new(self.client.clone(), tool_id)
    }
    pub fn list_agent_tools(&self, agent_id: impl Into<String>) -> FetchAgentToolsBuilder {
        FetchAgentToolsBuilder::new(self.client.clone(), agent_id)
    }
    pub fn attach_tool(
        &self,
        agent_id: impl Into<String>,
        tool_id: impl Into<String>,
    ) -> AttachToolBuilder {
        AttachToolBuilder::new(self.client.clone(), agent_id, tool_id)
    }
    pub fn detach_tool(
        &self,
        agent_id: impl Into<String>,
        tool_id: impl Into<String>,
    ) -> DetachToolBuilder {
        DetachToolBuilder::new(self.client.clone(), agent_id, tool_id)
    }
    pub fn set_agent_tool_approval_action(
        &self,
        agent_id: impl Into<String>,
        tool_id: impl Into<String>,
        request: UpdateAgentToolApprovalActionRequest,
    ) -> SetAgentToolApprovalActionBuilder {
        SetAgentToolApprovalActionBuilder::new(self.client.clone(), agent_id, tool_id, request)
    }
}

fn api_error(status: reqwest::StatusCode, prefix: &str, body: String) -> Error {
    Error::api_from_text(status, prefix, &body)
}

pub struct ListToolsBuilder {
    client: WachtClient,
    options: ListToolsOptions,
}
impl ListToolsBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
            options: ListToolsOptions::default(),
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
    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.options.search = Some(search.into());
        self
    }
    pub async fn send(self) -> Result<PaginatedResponse<AiToolWithDetails>> {
        let response = self
            .client
            .http_client()
            .get(format!("{}/ai/tools", self.client.config().base_url))
            .query(&self.options)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to list tools",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchToolBuilder {
    client: WachtClient,
    tool_id: String,
}
impl FetchToolBuilder {
    pub fn new(client: WachtClient, tool_id: impl Into<String>) -> Self {
        Self {
            client,
            tool_id: tool_id.into(),
        }
    }
    pub async fn send(self) -> Result<AiToolWithDetails> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/tools/{}",
                self.client.config().base_url,
                self.tool_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to fetch tool",
                response.text().await?,
            ))
        }
    }
}

pub struct CreateToolBuilder {
    client: WachtClient,
    request: CreateAiToolRequest,
}
impl CreateToolBuilder {
    pub fn new(client: WachtClient, request: CreateAiToolRequest) -> Self {
        Self { client, request }
    }
    pub async fn send(self) -> Result<AiTool> {
        let response = self
            .client
            .http_client()
            .post(format!("{}/ai/tools", self.client.config().base_url))
            .json(&self.request)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to create tool",
                response.text().await?,
            ))
        }
    }
}

pub struct UpdateToolBuilder {
    client: WachtClient,
    tool_id: String,
    request: UpdateAiToolRequest,
}
impl UpdateToolBuilder {
    pub fn new(
        client: WachtClient,
        tool_id: impl Into<String>,
        request: UpdateAiToolRequest,
    ) -> Self {
        Self {
            client,
            tool_id: tool_id.into(),
            request,
        }
    }
    pub async fn send(self) -> Result<AiTool> {
        let response = self
            .client
            .http_client()
            .patch(format!(
                "{}/ai/tools/{}",
                self.client.config().base_url,
                self.tool_id
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
                "Failed to update tool",
                response.text().await?,
            ))
        }
    }
}

pub struct DeleteToolBuilder {
    client: WachtClient,
    tool_id: String,
}
impl DeleteToolBuilder {
    pub fn new(client: WachtClient, tool_id: impl Into<String>) -> Self {
        Self {
            client,
            tool_id: tool_id.into(),
        }
    }
    pub async fn send(self) -> Result<()> {
        let response = self
            .client
            .http_client()
            .delete(format!(
                "{}/ai/tools/{}",
                self.client.config().base_url,
                self.tool_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(api_error(
                status,
                "Failed to delete tool",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchAgentToolsBuilder {
    client: WachtClient,
    agent_id: String,
}
impl FetchAgentToolsBuilder {
    pub fn new(client: WachtClient, agent_id: impl Into<String>) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
        }
    }
    pub async fn send(self) -> Result<PaginatedResponse<AiTool>> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/agents/{}/tools",
                self.client.config().base_url,
                self.agent_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to list agent tools",
                response.text().await?,
            ))
        }
    }
}

pub struct AttachToolBuilder {
    client: WachtClient,
    agent_id: String,
    tool_id: String,
}
impl AttachToolBuilder {
    pub fn new(
        client: WachtClient,
        agent_id: impl Into<String>,
        tool_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
            tool_id: tool_id.into(),
        }
    }
    pub async fn send(self) -> Result<()> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/agents/{}/tools/{}",
                self.client.config().base_url,
                self.agent_id,
                self.tool_id
            ))
            .json(&serde_json::json!({}))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(api_error(
                status,
                "Failed to attach tool",
                response.text().await?,
            ))
        }
    }
}

pub struct SetAgentToolApprovalActionBuilder {
    client: WachtClient,
    agent_id: String,
    tool_id: String,
    request: UpdateAgentToolApprovalActionRequest,
}
impl SetAgentToolApprovalActionBuilder {
    pub fn new(
        client: WachtClient,
        agent_id: impl Into<String>,
        tool_id: impl Into<String>,
        request: UpdateAgentToolApprovalActionRequest,
    ) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
            tool_id: tool_id.into(),
            request,
        }
    }
    pub async fn send(self) -> Result<()> {
        let response = self
            .client
            .http_client()
            .patch(format!(
                "{}/ai/agents/{}/tools/{}",
                self.client.config().base_url,
                self.agent_id,
                self.tool_id
            ))
            .json(&self.request)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(api_error(
                status,
                "Failed to set tool approval action",
                response.text().await?,
            ))
        }
    }
}

pub struct DetachToolBuilder {
    client: WachtClient,
    agent_id: String,
    tool_id: String,
}
impl DetachToolBuilder {
    pub fn new(
        client: WachtClient,
        agent_id: impl Into<String>,
        tool_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
            tool_id: tool_id.into(),
        }
    }
    pub async fn send(self) -> Result<()> {
        let response = self
            .client
            .http_client()
            .delete(format!(
                "{}/ai/agents/{}/tools/{}",
                self.client.config().base_url,
                self.agent_id,
                self.tool_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(api_error(
                status,
                "Failed to detach tool",
                response.text().await?,
            ))
        }
    }
}
