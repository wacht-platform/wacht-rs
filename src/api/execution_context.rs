use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{AiExecutionContext, CreateAiExecutionContextRequest},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContextListResponse {
    pub data: Vec<AiExecutionContext>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListExecutionContextsOptions {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub status: Option<String>,
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

impl UpdateExecutionContextRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_system_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.system_instructions = Some(instructions.into());
        self
    }

    pub fn with_context_group(mut self, group: impl Into<String>) -> Self {
        self.context_group = Some(group.into());
        self
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }
}

/// Fetch all execution contexts for the current workspace
pub async fn fetch_execution_contexts(
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

/// Fetch a specific execution context by ID
pub async fn fetch_execution_context(id: &str) -> Result<AiExecutionContext> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai/execution-contexts/{}", config.base_url, id);

    let response = client.get(&url).send().await?;
    let status = response.status();

    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to fetch execution context: {error_body}"),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Create a new execution context for AI operations
pub async fn create_execution_context(
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

/// Update an existing execution context
pub async fn update_execution_context(
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

/// Execute agent in execution context
pub async fn execute_agent(
    context_id: &str,
    request: serde_json::Value,
) -> Result<serde_json::Value> {
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
