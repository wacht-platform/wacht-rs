use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{AiWorkflow, CreateAiWorkflowRequest, UpdateAiWorkflowRequest},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowListResponse {
    pub data: Vec<AiWorkflow>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListWorkflowsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

/// List all AI workflows
pub async fn fetch_workflows(options: Option<ListWorkflowsOptions>) -> Result<WorkflowListResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-workflows", config.base_url);
    
    let mut request = client.get(&url);
    
    if let Some(opts) = options {
        request = request.query(&opts);
    }
    
    let response = request.send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to list workflows: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Create a new AI workflow
pub async fn create_workflow(request: CreateAiWorkflowRequest) -> Result<AiWorkflow> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-workflows", config.base_url);
    
    let response = client.post(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to create workflow: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Get a specific AI workflow by ID
pub async fn fetch_workflow(workflow_id: &str) -> Result<AiWorkflow> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-workflows/{}", config.base_url, workflow_id);
    
    let response = client.get(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to get workflow {}: {}", workflow_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Update an AI workflow
pub async fn update_workflow(workflow_id: &str, request: UpdateAiWorkflowRequest) -> Result<AiWorkflow> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-workflows/{}", config.base_url, workflow_id);
    
    let response = client.patch(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to update workflow {}: {}", workflow_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Delete an AI workflow
pub async fn delete_workflow(workflow_id: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-workflows/{}", config.base_url, workflow_id);
    
    let response = client.delete(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to delete workflow {}: {}", workflow_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}