use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{
        AgentIntegration, CreateAgentIntegrationRequest, UpdateAgentIntegrationRequest,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentIntegrationListResponse {
    pub data: Vec<AgentIntegration>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// List agent integrations
pub async fn fetch_agent_integrations(
    agent_id: &str,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<AgentIntegrationListResponse> {
    let config = get_config();
    let client = get_client();
    let mut url = format!("{}/ai/agents/{}/integrations", config.base_url, agent_id);

    let mut params = Vec::new();
    if let Some(lim) = limit {
        params.push(format!("limit={lim}"));
    }
    if let Some(off) = offset {
        params.push(format!("offset={off}"));
    }

    if !params.is_empty() {
        url.push('?');
        url.push_str(&params.join("&"));
    }

    let response = client.get(&url).send().await?;
    let status = response.status();

    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to list integrations for agent {agent_id}: {error_body}"),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Create agent integration
pub async fn create_agent_integration(
    agent_id: &str,
    request: CreateAgentIntegrationRequest,
) -> Result<AgentIntegration> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai/agents/{}/integrations", config.base_url, agent_id);

    let response = client.post(&url).json(&request).send().await?;
    let status = response.status();

    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to create integration for agent {agent_id}: {error_body}"),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Get agent integration by ID
pub async fn fetch_agent_integration(
    agent_id: &str,
    integration_id: &str,
) -> Result<AgentIntegration> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai/agents/{}/integrations/{}", config.base_url, agent_id, integration_id);

    let response = client.get(&url).send().await?;
    let status = response.status();

    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to get integration {integration_id} for agent {agent_id}: {error_body}"),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Update agent integration
pub async fn update_agent_integration(
    agent_id: &str,
    integration_id: &str,
    request: UpdateAgentIntegrationRequest,
) -> Result<AgentIntegration> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai/agents/{}/integrations/{}", config.base_url, agent_id, integration_id);

    let response = client.patch(&url).json(&request).send().await?;
    let status = response.status();

    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to update integration {integration_id} for agent {agent_id}: {error_body}"),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Delete agent integration
pub async fn delete_agent_integration(
    agent_id: &str,
    integration_id: &str,
) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai/agents/{}/integrations/{}", config.base_url, agent_id, integration_id);

    let response = client.delete(&url).send().await?;
    let status = response.status();

    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to delete integration {integration_id} for agent {agent_id}: {error_body}"),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}
