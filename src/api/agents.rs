use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{AiAgent, CreateAiAgentRequest, UpdateAiAgentRequest},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentListResponse {
    pub data: Vec<AiAgent>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAgentsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

/// List all AI agents with optional filtering
pub async fn fetch_agents(options: Option<ListAgentsOptions>) -> Result<AgentListResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-agents", config.base_url);
    
    let mut request = client.get(&url);
    
    // Add query parameters
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
            message: format!("Failed to list agents: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Get a specific AI agent by ID
pub async fn fetch_agent(agent_id: &str) -> Result<AiAgent> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-agents/{}", config.base_url, agent_id);
    
    let response = client.get(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to get agent {}: {}", agent_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Create a new AI agent
pub async fn create_agent(request: CreateAiAgentRequest) -> Result<AiAgent> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-agents", config.base_url);
    
    let response = client.post(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to create agent: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Update an existing AI agent
pub async fn update_agent(agent_id: &str, request: UpdateAiAgentRequest) -> Result<AiAgent> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-agents/{}", config.base_url, agent_id);
    
    let response = client.patch(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to update agent {}: {}", agent_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Delete an AI agent
pub async fn delete_agent(agent_id: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-agents/{}", config.base_url, agent_id);
    
    let response = client.delete(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to delete agent {}: {}", agent_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}