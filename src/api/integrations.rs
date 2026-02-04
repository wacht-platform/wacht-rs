use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{
        AgentIntegration, CreateAgentIntegrationRequest, UpdateAgentIntegrationRequest, ListOptions,
    },
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentIntegrationListResponse {
    pub data: Vec<AgentIntegration>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

pub struct ListAgentIntegrationsBuilder {
    agent_id: Arc<String>,
    options: Option<ListOptions>,
}

impl ListAgentIntegrationsBuilder {
    /// Create a new builder for listing agent integrations
    pub fn new(agent_id: impl Into<String>) -> Self {
        Self {
            agent_id: Arc::new(agent_id.into()),
            options: None,
        }
    }

    /// Set list options (limit, offset, etc.)
    pub fn options(mut self, options: ListOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// Build and execute the request
    pub async fn send(self) -> Result<AgentIntegrationListResponse> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/agents/{}/integrations", config.base_url, &*self.agent_id);

        let mut request = client.get(&url);

        if let Some(opts) = self.options {
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
                message: format!("Failed to list integrations for agent {}: {}", &*self.agent_id, error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

pub struct CreateAgentIntegrationBuilder {
    agent_id: Arc<String>,
    request: CreateAgentIntegrationRequest,
}

impl CreateAgentIntegrationBuilder {
    /// Create a new builder for creating agent integrations
    pub fn new(agent_id: impl Into<String>) -> Self {
        Self {
            agent_id: Arc::new(agent_id.into()),
            request: CreateAgentIntegrationRequest::default(),
        }
    }

    /// Set the integration creation request
    pub fn request(mut self, request: CreateAgentIntegrationRequest) -> Self {
        self.request = request;
        self
    }

    /// Build and execute the request
    pub async fn send(self) -> Result<AgentIntegration> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/agents/{}/integrations", config.base_url, &*self.agent_id);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to create integration for agent {}: {}", &*self.agent_id, error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

pub struct GetAgentIntegrationBuilder {
    agent_id: Arc<String>,
    integration_id: Arc<String>,
}

impl GetAgentIntegrationBuilder {
    /// Create a new builder for getting agent integrations
    pub fn new(agent_id: impl Into<String>, integration_id: impl Into<String>) -> Self {
        Self {
            agent_id: Arc::new(agent_id.into()),
            integration_id: Arc::new(integration_id.into()),
        }
    }

    /// Build and execute the request
    pub async fn send(self) -> Result<AgentIntegration> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/agents/{}/integrations/{}", config.base_url, &*self.agent_id, &*self.integration_id);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to get integration {} for agent {}: {}", &*self.integration_id, &*self.agent_id, error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

pub struct UpdateAgentIntegrationBuilder {
    agent_id: Arc<String>,
    integration_id: Arc<String>,
    request: UpdateAgentIntegrationRequest,
}

impl UpdateAgentIntegrationBuilder {
    /// Create a new builder for updating agent integrations
    pub fn new(agent_id: impl Into<String>, integration_id: impl Into<String>) -> Self {
        Self {
            agent_id: Arc::new(agent_id.into()),
            integration_id: Arc::new(integration_id.into()),
            request: UpdateAgentIntegrationRequest::default(),
        }
    }

    /// Set the integration update request
    pub fn request(mut self, request: UpdateAgentIntegrationRequest) -> Self {
        self.request = request;
        self
    }

    /// Build and execute the request
    pub async fn send(self) -> Result<AgentIntegration> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/agents/{}/integrations/{}", config.base_url, &*self.agent_id, &*self.integration_id);

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update integration {} for agent {}: {}", &*self.integration_id, &*self.agent_id, error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

pub struct DeleteAgentIntegrationBuilder {
    agent_id: Arc<String>,
    integration_id: Arc<String>,
}

impl DeleteAgentIntegrationBuilder {
    /// Create a new builder for deleting agent integrations
    pub fn new(agent_id: impl Into<String>, integration_id: impl Into<String>) -> Self {
        Self {
            agent_id: Arc::new(agent_id.into()),
            integration_id: Arc::new(integration_id.into()),
        }
    }

    /// Build and execute the request
    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/agents/{}/integrations/{}", config.base_url, &*self.agent_id, &*self.integration_id);

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete integration {} for agent {}: {}", &*self.integration_id, &*self.agent_id, error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Convenience functions for backwards compatibility
pub async fn fetch_agent_integrations(
    agent_id: &str,
    options: Option<ListOptions>,
) -> Result<AgentIntegrationListResponse> {
    ListAgentIntegrationsBuilder::new(agent_id)
        .options(options.unwrap_or_default())
        .send()
        .await
}

pub async fn create_agent_integration(
    agent_id: &str,
    request: CreateAgentIntegrationRequest,
) -> Result<AgentIntegration> {
    CreateAgentIntegrationBuilder::new(agent_id)
        .request(request)
        .send()
        .await
}

pub async fn fetch_agent_integration(
    agent_id: &str,
    integration_id: &str,
) -> Result<AgentIntegration> {
    GetAgentIntegrationBuilder::new(agent_id, integration_id)
        .send()
        .await
}

pub async fn update_agent_integration(
    agent_id: &str,
    integration_id: &str,
    request: UpdateAgentIntegrationRequest,
) -> Result<AgentIntegration> {
    UpdateAgentIntegrationBuilder::new(agent_id, integration_id)
        .request(request)
        .send()
        .await
}

pub async fn delete_agent_integration(
    agent_id: &str,
    integration_id: &str,
) -> Result<()> {
    DeleteAgentIntegrationBuilder::new(agent_id, integration_id)
        .send()
        .await
}
