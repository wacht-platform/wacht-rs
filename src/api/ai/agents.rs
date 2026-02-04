use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{AiAgent, CreateAiAgentRequest, PaginatedResponse, UpdateAiAgentRequest},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAgentsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

/// Builder for listing AI agents
pub struct ListAgentsBuilder {
    options: Option<ListAgentsOptions>,
}

impl Default for ListAgentsBuilder {
    fn default() -> Self {
        Self { options: None }
    }
}

impl ListAgentsBuilder {
    /// Create a new builder for listing agents
    pub fn new() -> Self {
        Self::default()
    }

    /// Set limit for the number of agents to return
    pub fn limit(mut self, limit: i32) -> Self {
        if let Some(ref mut opts) = self.options {
            opts.limit = Some(limit);
        } else {
            self.options = Some(ListAgentsOptions {
                limit: Some(limit),
                ..Default::default()
            });
        }
        self
    }

    /// Set offset for pagination
    pub fn offset(mut self, offset: i32) -> Self {
        if let Some(ref mut opts) = self.options {
            opts.offset = Some(offset);
        } else {
            self.options = Some(ListAgentsOptions {
                offset: Some(offset),
                ..Default::default()
            });
        }
        self
    }

    /// Filter by active status
    pub fn is_active(mut self, is_active: bool) -> Self {
        if let Some(ref mut opts) = self.options {
            opts.is_active = Some(is_active);
        } else {
            self.options = Some(ListAgentsOptions {
                is_active: Some(is_active),
                ..Default::default()
            });
        }
        self
    }

    /// Search agents by name or description
    pub fn search(mut self, search: impl Into<String>) -> Self {
        if let Some(ref mut opts) = self.options {
            opts.search = Some(search.into());
        } else {
            self.options = Some(ListAgentsOptions {
                search: Some(search.into()),
                ..Default::default()
            });
        }
        self
    }

    /// Execute the request and return paginated response
    pub async fn send(self) -> Result<PaginatedResponse<AiAgent>> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/agents", config.base_url);

        let mut request = client.get(&url);

        // Add query parameters
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
                message: format!("Failed to list agents: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for getting a specific AI agent by ID
pub struct FetchAgentBuilder {
    agent_id: String,
}

impl FetchAgentBuilder {
    /// Create a new builder for fetching an agent by ID
    pub fn new(agent_id: impl Into<String>) -> Self {
        Self {
            agent_id: agent_id.into(),
        }
    }

    /// Execute the request and return the agent
    pub async fn send(self) -> Result<AiAgent> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/agents/{}", config.base_url, self.agent_id);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to get agent {}: {error_body}", self.agent_id),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for creating a new AI agent
pub struct CreateAgentBuilder {
    request: CreateAiAgentRequest,
}

impl CreateAgentBuilder {
    /// Create a new builder for creating an agent
    pub fn new(request: CreateAiAgentRequest) -> Self {
        Self { request }
    }

    /// Execute the request and return the created agent
    pub async fn send(self) -> Result<AiAgent> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/agents", config.base_url);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to create agent: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for updating an existing AI agent
pub struct UpdateAgentBuilder {
    agent_id: String,
    request: UpdateAiAgentRequest,
}

impl UpdateAgentBuilder {
    /// Create a new builder for updating an agent
    pub fn new(agent_id: impl Into<String>, request: UpdateAiAgentRequest) -> Self {
        Self {
            agent_id: agent_id.into(),
            request,
        }
    }

    /// Execute the request and return the updated agent
    pub async fn send(self) -> Result<AiAgent> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/agents/{}", config.base_url, self.agent_id);

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update agent {}: {error_body}", self.agent_id),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for deleting an AI agent
pub struct DeleteAgentBuilder {
    agent_id: String,
}

impl DeleteAgentBuilder {
    /// Create a new builder for deleting an agent
    pub fn new(agent_id: impl Into<String>) -> Self {
        Self {
            agent_id: agent_id.into(),
        }
    }

    /// Execute the request and return success
    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/agents/{}", config.base_url, self.agent_id);

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete agent {}: {error_body}", self.agent_id),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for getting AI agent details
pub struct FetchAgentDetailsBuilder {
    agent_id: String,
}

impl FetchAgentDetailsBuilder {
    /// Create a new builder for fetching agent details
    pub fn new(agent_id: impl Into<String>) -> Self {
        Self {
            agent_id: agent_id.into(),
        }
    }

    /// Execute the request and return the agent details
    pub async fn send(self) -> Result<AiAgent> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/agents/{}/details", config.base_url, self.agent_id);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to get agent details {}: {error_body}", self.agent_id),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Convenience functions for backward compatibility
pub fn list_agents() -> ListAgentsBuilder {
    ListAgentsBuilder::new()
}

pub fn fetch_agent(agent_id: impl Into<String>) -> FetchAgentBuilder {
    FetchAgentBuilder::new(agent_id)
}

pub fn create_agent(request: CreateAiAgentRequest) -> CreateAgentBuilder {
    CreateAgentBuilder::new(request)
}

pub fn update_agent(agent_id: impl Into<String>, request: UpdateAiAgentRequest) -> UpdateAgentBuilder {
    UpdateAgentBuilder::new(agent_id, request)
}

pub fn delete_agent(agent_id: impl Into<String>) -> DeleteAgentBuilder {
    DeleteAgentBuilder::new(agent_id)
}

pub fn fetch_agent_details(agent_id: impl Into<String>) -> FetchAgentDetailsBuilder {
    FetchAgentDetailsBuilder::new(agent_id)
}
