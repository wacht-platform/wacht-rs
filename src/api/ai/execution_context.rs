use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{
        AiExecutionContext, CreateAiExecutionContextRequest, ExecuteAgentRequest,
        ExecuteAgentResponse, PaginatedResponse,
    },
};
use serde::{Deserialize, Serialize};

pub type ExecutionContextListResponse = PaginatedResponse<AiExecutionContext>;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListExecutionContextsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_group: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateExecutionContextRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ExecutionContextsApi {
    client: WachtClient,
}

impl ExecutionContextsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn list_execution_contexts(&self) -> ListExecutionContextsBuilder {
        ListExecutionContextsBuilder::new(self.client.clone())
    }

    pub fn update_execution_context_builder(
        &self,
        id: impl Into<String>,
    ) -> UpdateExecutionContextBuilder {
        UpdateExecutionContextBuilder::new(self.client.clone(), id)
    }

    pub fn create_execution_context_with_request_builder(
        &self,
        request: CreateAiExecutionContextRequest,
    ) -> CreateExecutionContextBuilder {
        CreateExecutionContextBuilder::new(self.client.clone(), request)
    }

    pub fn execute_agent_builder(
        &self,
        context_id: impl Into<String>,
        request: ExecuteAgentRequest,
    ) -> ExecuteAgentBuilder {
        ExecuteAgentBuilder::new(self.client.clone(), context_id, request)
    }
}

#[derive(Debug, Clone)]
pub struct ListExecutionContextsBuilder {
    client: WachtClient,
    options: ListExecutionContextsOptions,
}

impl ListExecutionContextsBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
            options: ListExecutionContextsOptions::default(),
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

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.options.status = Some(status.into());
        self
    }

    pub fn context_group(mut self, context_group: impl Into<String>) -> Self {
        self.options.context_group = Some(context_group.into());
        self
    }

    pub async fn send(self) -> Result<ExecutionContextListResponse> {
        let client = self.client.http_client();
        let mut url = format!("{}/ai/execution-contexts", self.client.config().base_url);

        let opts = self.options;
        let mut params = vec![];
        if let Some(limit) = opts.limit {
            params.push(format!("limit={limit}"));
        }
        if let Some(offset) = opts.offset {
            params.push(format!("offset={offset}"));
        }
        if let Some(status) = opts.status {
            params.push(format!("status={status}"));
        }
        if let Some(context_group) = opts.context_group {
            params.push(format!("context_group={context_group}"));
        }
        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch execution contexts: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct UpdateExecutionContextBuilder {
    client: WachtClient,
    id: String,
    request: UpdateExecutionContextRequest,
}

impl UpdateExecutionContextBuilder {
    pub fn new(client: WachtClient, id: impl Into<String>) -> Self {
        Self {
            client,
            id: id.into(),
            request: UpdateExecutionContextRequest::default(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.request.title = Some(title.into());
        self
    }

    pub fn with_system_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.request.system_instructions = Some(instructions.into());
        self
    }

    pub fn with_context_group(mut self, context_group: impl Into<String>) -> Self {
        self.request.context_group = Some(context_group.into());
        self
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.request.status = Some(status.into());
        self
    }

    pub async fn send(self) -> Result<AiExecutionContext> {
        let client = self.client.http_client();
        let url = format!("{}/ai/execution-contexts/{}", self.client.config().base_url, self.id);

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update execution context: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateExecutionContextBuilder {
    client: WachtClient,
    request: CreateAiExecutionContextRequest,
}

impl CreateExecutionContextBuilder {
    pub fn new(client: WachtClient, request: CreateAiExecutionContextRequest) -> Self {
        Self { client, request }
    }

    pub async fn send(self) -> Result<AiExecutionContext> {
        let client = self.client.http_client();
        let url = format!("{}/ai/execution-contexts", self.client.config().base_url);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to create execution context: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExecuteAgentBuilder {
    client: WachtClient,
    context_id: String,
    request: ExecuteAgentRequest,
}

impl ExecuteAgentBuilder {
    pub fn new(
        client: WachtClient,
        context_id: impl Into<String>,
        request: ExecuteAgentRequest,
    ) -> Self {
        Self {
            client,
            context_id: context_id.into(),
            request,
        }
    }

    pub async fn send(self) -> Result<ExecuteAgentResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/ai/execution-contexts/{}/execute",
            self.client.config().base_url,
            self.context_id
        );

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to execute agent: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}
