use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{PaginatedResponse, AiExecutionContext, CreateAiExecutionContextRequest, ExecuteAgentRequest, ExecuteAgentResponse},
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

/// Builder for listing execution contexts
#[derive(Debug, Clone)]
pub struct ListExecutionContextsBuilder {
    options: ListExecutionContextsOptions,
}

impl Default for ListExecutionContextsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ListExecutionContextsBuilder {
    pub fn new() -> Self {
        Self {
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
        fetch_execution_contexts_with_options(Some(self.options)).await
    }
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

/// Builder for updating execution contexts
#[derive(Debug, Clone)]
pub struct UpdateExecutionContextBuilder {
    id: String,
    request: UpdateExecutionContextRequest,
}

impl UpdateExecutionContextBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
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
        update_execution_context_with_id_and_request(&self.id, self.request).await
    }
}

/// Builder for creating execution contexts
#[derive(Debug, Clone)]
pub struct CreateExecutionContextBuilder {
    request: CreateAiExecutionContextRequest,
}

impl CreateExecutionContextBuilder {
    pub fn new(request: CreateAiExecutionContextRequest) -> Self {
        Self { request }
    }

    pub async fn send(self) -> Result<AiExecutionContext> {
        create_execution_context_with_request(self.request).await
    }
}

/// Builder for executing agents
#[derive(Debug, Clone)]
pub struct ExecuteAgentBuilder {
    context_id: String,
    request: ExecuteAgentRequest,
}

impl ExecuteAgentBuilder {
    pub fn new(context_id: impl Into<String>, request: ExecuteAgentRequest) -> Self {
        Self {
            context_id: context_id.into(),
            request,
        }
    }

    pub async fn send(self) -> Result<ExecuteAgentResponse> {
        execute_agent_with_context_id_and_request(&self.context_id, self.request).await
    }
}

/// Fetch all execution contexts for the current workspace
pub async fn fetch_execution_contexts(
    options: Option<ListExecutionContextsOptions>,
) -> Result<ExecutionContextListResponse> {
    fetch_execution_contexts_with_options(options).await
}

/// Create a new execution context for AI operations
pub async fn create_execution_context(
    request: CreateAiExecutionContextRequest,
) -> Result<AiExecutionContext> {
    create_execution_context_with_request(request).await
}

/// Update an existing execution context
pub async fn update_execution_context(
    id: &str,
    request: UpdateExecutionContextRequest,
) -> Result<AiExecutionContext> {
    update_execution_context_with_id_and_request(id, request).await
}

/// Execute agent in execution context
pub async fn execute_agent(
    context_id: &str,
    request: ExecuteAgentRequest,
) -> Result<ExecuteAgentResponse> {
    execute_agent_with_context_id_and_request(context_id, request).await
}

/// Internal function for fetching execution contexts with options
async fn fetch_execution_contexts_with_options(
    options: Option<ListExecutionContextsOptions>,
) -> Result<ExecutionContextListResponse> {
    let config = get_config();
    let client = get_client();
    let mut url = format!("{}/ai/execution-contexts", config.base_url);

    if let Some(opts) = options {
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

/// Internal function for creating execution contexts
async fn create_execution_context_with_request(
    request: CreateAiExecutionContextRequest,
) -> Result<AiExecutionContext> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai/execution-contexts", config.base_url);

    let response = client.post(&url).json(&request).send().await?;
    let status = response.status();

    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;

        println!("error {error_body}");

        Err(Error::Api {
            status,
            message: format!("Failed to create execution context: {error_body}"),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Internal function for updating execution contexts
async fn update_execution_context_with_id_and_request(
    id: &str,
    request: UpdateExecutionContextRequest,
) -> Result<AiExecutionContext> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai/execution-contexts/{}", config.base_url, id);

    let response = client.patch(&url).json(&request).send().await?;
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

/// Internal function for executing agents
async fn execute_agent_with_context_id_and_request(
    context_id: &str,
    request: ExecuteAgentRequest,
) -> Result<ExecuteAgentResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai/execution-contexts/{}/execute", config.base_url, context_id);

    let response = client.post(&url).json(&request).send().await?;
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

/// Convenience function to list execution contexts using builder pattern
pub fn list_execution_contexts() -> ListExecutionContextsBuilder {
    ListExecutionContextsBuilder::new()
}

/// Convenience function to update an execution context using builder pattern
pub fn update_execution_context_builder(id: impl Into<String>) -> UpdateExecutionContextBuilder {
    UpdateExecutionContextBuilder::new(id)
}

/// Convenience function to create an execution context using builder pattern
pub fn create_execution_context_with_request_builder(request: CreateAiExecutionContextRequest) -> CreateExecutionContextBuilder {
    CreateExecutionContextBuilder::new(request)
}

/// Convenience function to execute an agent using builder pattern
pub fn execute_agent_builder(context_id: impl Into<String>, request: ExecuteAgentRequest) -> ExecuteAgentBuilder {
    ExecuteAgentBuilder::new(context_id, request)
}
