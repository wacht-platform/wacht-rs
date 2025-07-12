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
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub agent_id: Option<String>,
    pub workflow_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExecutionContextRequest {
    pub context: Option<serde_json::Value>,
    pub session_id: Option<String>,
}

/// Fetch all execution contexts for the current workspace
pub async fn fetch_execution_contexts(options: Option<ListExecutionContextsOptions>) -> Result<ExecutionContextListResponse> {
    let config = get_config();
    let client = get_client();
    let mut url = format!("{}/ai-execution-context", config.base_url);
    
    if let Some(opts) = options {
        let mut params = vec![];
        if let Some(page) = opts.page {
            params.push(format!("page={}", page));
        }
        if let Some(per_page) = opts.per_page {
            params.push(format!("per_page={}", per_page));
        }
        if let Some(agent_id) = opts.agent_id {
            params.push(format!("agent_id={}", agent_id));
        }
        if let Some(workflow_id) = opts.workflow_id {
            params.push(format!("workflow_id={}", workflow_id));
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
            message: format!("Failed to fetch execution contexts: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Fetch a specific execution context by ID
pub async fn fetch_execution_context(id: &str) -> Result<AiExecutionContext> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-execution-context/{}", config.base_url, id);
    
    let response = client.get(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to fetch execution context: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Create a new execution context for AI operations
pub async fn create_execution_context(request: CreateAiExecutionContextRequest) -> Result<AiExecutionContext> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-execution-context", config.base_url);
    
    let response = client.post(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to create execution context: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Update an existing execution context
pub async fn update_execution_context(id: &str, request: UpdateExecutionContextRequest) -> Result<AiExecutionContext> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-execution-context/{}", config.base_url, id);
    
    let response = client.put(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to update execution context: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Delete an execution context
pub async fn delete_execution_context(id: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-execution-context/{}", config.base_url, id);
    
    let response = client.delete(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to delete execution context: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}