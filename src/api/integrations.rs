use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{
        AgentIntegration, CreateAgentIntegrationRequest, ListOptions, UpdateAgentIntegrationRequest,
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

#[derive(Debug, Clone)]
pub struct IntegrationsApi {
    client: WachtClient,
}

impl IntegrationsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn list_agent_integrations(&self, agent_id: impl Into<String>) -> ListAgentIntegrationsBuilder {
        ListAgentIntegrationsBuilder::new(self.client.clone(), agent_id)
    }

    pub fn create_agent_integration(
        &self,
        agent_id: impl Into<String>,
    ) -> CreateAgentIntegrationBuilder {
        CreateAgentIntegrationBuilder::new(self.client.clone(), agent_id)
    }

    pub fn get_agent_integration(
        &self,
        agent_id: impl Into<String>,
        integration_id: impl Into<String>,
    ) -> GetAgentIntegrationBuilder {
        GetAgentIntegrationBuilder::new(self.client.clone(), agent_id, integration_id)
    }

    pub fn update_agent_integration(
        &self,
        agent_id: impl Into<String>,
        integration_id: impl Into<String>,
    ) -> UpdateAgentIntegrationBuilder {
        UpdateAgentIntegrationBuilder::new(self.client.clone(), agent_id, integration_id)
    }

    pub fn delete_agent_integration(
        &self,
        agent_id: impl Into<String>,
        integration_id: impl Into<String>,
    ) -> DeleteAgentIntegrationBuilder {
        DeleteAgentIntegrationBuilder::new(self.client.clone(), agent_id, integration_id)
    }
}

pub struct ListAgentIntegrationsBuilder {
    client: WachtClient,
    agent_id: String,
    options: Option<ListOptions>,
}

impl ListAgentIntegrationsBuilder {
    pub fn new(client: WachtClient, agent_id: impl Into<String>) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
            options: None,
        }
    }

    pub fn options(mut self, options: ListOptions) -> Self {
        self.options = Some(options);
        self
    }

    pub async fn send(self) -> Result<AgentIntegrationListResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/ai/agents/{}/integrations",
            self.client.config().base_url,
            self.agent_id
        );

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
                message: format!(
                    "Failed to list integrations for agent {}: {error_body}",
                    self.agent_id
                ),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

pub struct CreateAgentIntegrationBuilder {
    client: WachtClient,
    agent_id: String,
    request: CreateAgentIntegrationRequest,
}

impl CreateAgentIntegrationBuilder {
    pub fn new(client: WachtClient, agent_id: impl Into<String>) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
            request: CreateAgentIntegrationRequest::default(),
        }
    }

    pub fn request(mut self, request: CreateAgentIntegrationRequest) -> Self {
        self.request = request;
        self
    }

    pub async fn send(self) -> Result<AgentIntegration> {
        let client = self.client.http_client();
        let url = format!(
            "{}/ai/agents/{}/integrations",
            self.client.config().base_url,
            self.agent_id
        );

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!(
                    "Failed to create integration for agent {}: {error_body}",
                    self.agent_id
                ),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

pub struct GetAgentIntegrationBuilder {
    client: WachtClient,
    agent_id: String,
    integration_id: String,
}

impl GetAgentIntegrationBuilder {
    pub fn new(
        client: WachtClient,
        agent_id: impl Into<String>,
        integration_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
            integration_id: integration_id.into(),
        }
    }

    pub async fn send(self) -> Result<AgentIntegration> {
        let client = self.client.http_client();
        let url = format!(
            "{}/ai/agents/{}/integrations/{}",
            self.client.config().base_url,
            self.agent_id,
            self.integration_id
        );

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!(
                    "Failed to get integration {} for agent {}: {error_body}",
                    self.integration_id, self.agent_id
                ),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

pub struct UpdateAgentIntegrationBuilder {
    client: WachtClient,
    agent_id: String,
    integration_id: String,
    request: UpdateAgentIntegrationRequest,
}

impl UpdateAgentIntegrationBuilder {
    pub fn new(
        client: WachtClient,
        agent_id: impl Into<String>,
        integration_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
            integration_id: integration_id.into(),
            request: UpdateAgentIntegrationRequest::default(),
        }
    }

    pub fn request(mut self, request: UpdateAgentIntegrationRequest) -> Self {
        self.request = request;
        self
    }

    pub async fn send(self) -> Result<AgentIntegration> {
        let client = self.client.http_client();
        let url = format!(
            "{}/ai/agents/{}/integrations/{}",
            self.client.config().base_url,
            self.agent_id,
            self.integration_id
        );

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!(
                    "Failed to update integration {} for agent {}: {error_body}",
                    self.integration_id, self.agent_id
                ),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

pub struct DeleteAgentIntegrationBuilder {
    client: WachtClient,
    agent_id: String,
    integration_id: String,
}

impl DeleteAgentIntegrationBuilder {
    pub fn new(
        client: WachtClient,
        agent_id: impl Into<String>,
        integration_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
            integration_id: integration_id.into(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/ai/agents/{}/integrations/{}",
            self.client.config().base_url,
            self.agent_id,
            self.integration_id
        );

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!(
                    "Failed to delete integration {} for agent {}: {error_body}",
                    self.integration_id, self.agent_id
                ),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}
