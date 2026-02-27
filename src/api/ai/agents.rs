use crate::{
    client::WachtClient,
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

#[derive(Debug, Clone)]
pub struct AgentsApi {
    client: WachtClient,
}

impl AgentsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn list_agents(&self) -> ListAgentsBuilder {
        ListAgentsBuilder::new(self.client.clone())
    }

    pub fn fetch_agent(&self, agent_id: impl Into<String>) -> FetchAgentBuilder {
        FetchAgentBuilder::new(self.client.clone(), agent_id)
    }

    pub fn create_agent(&self, request: CreateAiAgentRequest) -> CreateAgentBuilder {
        CreateAgentBuilder::new(self.client.clone(), request)
    }

    pub fn update_agent(
        &self,
        agent_id: impl Into<String>,
        request: UpdateAiAgentRequest,
    ) -> UpdateAgentBuilder {
        UpdateAgentBuilder::new(self.client.clone(), agent_id, request)
    }

    pub fn delete_agent(&self, agent_id: impl Into<String>) -> DeleteAgentBuilder {
        DeleteAgentBuilder::new(self.client.clone(), agent_id)
    }

    pub fn fetch_agent_details(&self, agent_id: impl Into<String>) -> FetchAgentDetailsBuilder {
        FetchAgentDetailsBuilder::new(self.client.clone(), agent_id)
    }
}

pub struct ListAgentsBuilder {
    client: WachtClient,
    options: Option<ListAgentsOptions>,
}

impl ListAgentsBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
            options: None,
        }
    }

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

    pub async fn send(self) -> Result<PaginatedResponse<AiAgent>> {
        let client = self.client.http_client();
        let url = format!("{}/ai/agents", self.client.config().base_url);

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
                message: format!("Failed to list agents: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

pub struct FetchAgentBuilder {
    client: WachtClient,
    agent_id: String,
}

impl FetchAgentBuilder {
    pub fn new(client: WachtClient, agent_id: impl Into<String>) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
        }
    }

    pub async fn send(self) -> Result<AiAgent> {
        let client = self.client.http_client();
        let url = format!("{}/ai/agents/{}", self.client.config().base_url, self.agent_id);

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

pub struct CreateAgentBuilder {
    client: WachtClient,
    request: CreateAiAgentRequest,
}

impl CreateAgentBuilder {
    pub fn new(client: WachtClient, request: CreateAiAgentRequest) -> Self {
        Self { client, request }
    }

    pub async fn send(self) -> Result<AiAgent> {
        let client = self.client.http_client();
        let url = format!("{}/ai/agents", self.client.config().base_url);

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

pub struct UpdateAgentBuilder {
    client: WachtClient,
    agent_id: String,
    request: UpdateAiAgentRequest,
}

impl UpdateAgentBuilder {
    pub fn new(
        client: WachtClient,
        agent_id: impl Into<String>,
        request: UpdateAiAgentRequest,
    ) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
            request,
        }
    }

    pub async fn send(self) -> Result<AiAgent> {
        let client = self.client.http_client();
        let url = format!("{}/ai/agents/{}", self.client.config().base_url, self.agent_id);

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

pub struct DeleteAgentBuilder {
    client: WachtClient,
    agent_id: String,
}

impl DeleteAgentBuilder {
    pub fn new(client: WachtClient, agent_id: impl Into<String>) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!("{}/ai/agents/{}", self.client.config().base_url, self.agent_id);

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

pub struct FetchAgentDetailsBuilder {
    client: WachtClient,
    agent_id: String,
}

impl FetchAgentDetailsBuilder {
    pub fn new(client: WachtClient, agent_id: impl Into<String>) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
        }
    }

    pub async fn send(self) -> Result<AiAgent> {
        let client = self.client.http_client();
        let url = format!(
            "{}/ai/agents/{}/details",
            self.client.config().base_url,
            self.agent_id
        );

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
