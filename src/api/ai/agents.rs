use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{
        AgentDetailsResponse, AgentSkillsSummary, AiAgent, AiAgentWithDetails, CreateAiAgentRequest,
        PaginatedResponse, SkillFileResponse, SkillScope, SkillTreeResponse, UpdateAiAgentRequest,
    },
};
use reqwest::multipart::{Form, Part};
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAgentsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
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

    pub fn fetch_agent_details(&self, agent_id: impl Into<String>) -> FetchAgentDetailsBuilder {
        FetchAgentDetailsBuilder::new(self.client.clone(), agent_id)
    }
    /// Summary of system + agent skills available to the agent. System skills
    /// are built-in; agent skills are uploaded per-agent via the import
    /// bundle endpoint.
    pub fn list_skills_summary(
        &self,
        agent_id: impl Into<String>,
    ) -> ListAgentSkillsSummaryBuilder {
        ListAgentSkillsSummaryBuilder::new(self.client.clone(), agent_id)
    }

    pub fn list_skill_tree(
        &self,
        agent_id: impl Into<String>,
        scope: SkillScope,
    ) -> ListAgentSkillTreeBuilder {
        ListAgentSkillTreeBuilder::new(self.client.clone(), agent_id, scope)
    }
    pub fn read_skill_file(
        &self,
        agent_id: impl Into<String>,
        scope: SkillScope,
        path: impl Into<String>,
    ) -> ReadAgentSkillFileBuilder {
        ReadAgentSkillFileBuilder::new(self.client.clone(), agent_id, scope, path)
    }
    pub fn import_skill_bundle(
        &self,
        agent_id: impl Into<String>,
        file_name: impl Into<String>,
        file_content: Vec<u8>,
    ) -> ImportAgentSkillBundleBuilder {
        ImportAgentSkillBundleBuilder::new(self.client.clone(), agent_id, file_name, file_content)
    }
    pub fn delete_skill(
        &self,
        agent_id: impl Into<String>,
        skill_slug: impl Into<String>,
    ) -> DeleteAgentSkillBuilder {
        DeleteAgentSkillBuilder::new(self.client.clone(), agent_id, skill_slug)
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

    pub fn list_sub_agents(&self, agent_id: impl Into<String>) -> ListAgentSubAgentsBuilder {
        ListAgentSubAgentsBuilder::new(self.client.clone(), agent_id)
    }

    pub fn attach_sub_agent(
        &self,
        agent_id: impl Into<String>,
        sub_agent_id: impl Into<String>,
    ) -> AttachSubAgentBuilder {
        AttachSubAgentBuilder::new(self.client.clone(), agent_id, sub_agent_id)
    }

    pub fn detach_sub_agent(
        &self,
        agent_id: impl Into<String>,
        sub_agent_id: impl Into<String>,
    ) -> DetachSubAgentBuilder {
        DetachSubAgentBuilder::new(self.client.clone(), agent_id, sub_agent_id)
    }
}

fn api_error(status: reqwest::StatusCode, prefix: &str, body: String) -> Error {
    Error::api_from_text(status, prefix, &body)
}

#[derive(Debug, Clone, Serialize)]
struct SkillTreeQuery {
    scope: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
}

pub struct ListAgentsBuilder {
    client: WachtClient,
    options: ListAgentsOptions,
}

impl ListAgentsBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
            options: ListAgentsOptions::default(),
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
    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.options.search = Some(search.into());
        self
    }

    pub async fn send(self) -> Result<PaginatedResponse<AiAgentWithDetails>> {
        let response = self
            .client
            .http_client()
            .get(format!("{}/ai/agents", self.client.config().base_url))
            .query(&self.options)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to list agents",
                response.text().await?,
            ))
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
    pub async fn send(self) -> Result<AiAgentWithDetails> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/agents/{}",
                self.client.config().base_url,
                self.agent_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to fetch agent",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchAgentDetailsBuilder {
    client: WachtClient,
    agent_id: String,
}

pub struct ListAgentSkillTreeBuilder {
    client: WachtClient,
    agent_id: String,
    query: SkillTreeQuery,
}
impl ListAgentSkillTreeBuilder {
    pub fn new(client: WachtClient, agent_id: impl Into<String>, scope: SkillScope) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
            query: SkillTreeQuery {
                scope: scope.as_str().to_string(),
                path: None,
            },
        }
    }
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.query.path = Some(path.into());
        self
    }
    pub async fn send(self) -> Result<SkillTreeResponse> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/agents/{}/skills/tree",
                self.client.config().base_url,
                self.agent_id
            ))
            .query(&self.query)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to list agent skill tree",
                response.text().await?,
            ))
        }
    }
}

pub struct ReadAgentSkillFileBuilder {
    client: WachtClient,
    agent_id: String,
    query: SkillTreeQuery,
}
impl ReadAgentSkillFileBuilder {
    pub fn new(
        client: WachtClient,
        agent_id: impl Into<String>,
        scope: SkillScope,
        path: impl Into<String>,
    ) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
            query: SkillTreeQuery {
                scope: scope.as_str().to_string(),
                path: Some(path.into()),
            },
        }
    }
    pub async fn send(self) -> Result<SkillFileResponse> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/agents/{}/skills/file",
                self.client.config().base_url,
                self.agent_id
            ))
            .query(&self.query)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to read agent skill file",
                response.text().await?,
            ))
        }
    }
}

pub struct ImportAgentSkillBundleBuilder {
    client: WachtClient,
    agent_id: String,
    file_name: String,
    file_content: Vec<u8>,
    replace_existing: bool,
}
impl ImportAgentSkillBundleBuilder {
    pub fn new(
        client: WachtClient,
        agent_id: impl Into<String>,
        file_name: impl Into<String>,
        file_content: Vec<u8>,
    ) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
            file_name: file_name.into(),
            file_content,
            replace_existing: false,
        }
    }
    pub fn replace_existing(mut self, replace_existing: bool) -> Self {
        self.replace_existing = replace_existing;
        self
    }
    pub async fn send(self) -> Result<SkillTreeResponse> {
        let part = Part::bytes(self.file_content).file_name(self.file_name);
        let form = Form::new()
            .part("file", part)
            .text("replace_existing", self.replace_existing.to_string());
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/agents/{}/skills",
                self.client.config().base_url,
                self.agent_id
            ))
            .multipart(form)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to import agent skill bundle",
                response.text().await?,
            ))
        }
    }
}

pub struct DeleteAgentSkillBuilder {
    client: WachtClient,
    agent_id: String,
    skill_slug: String,
}
impl DeleteAgentSkillBuilder {
    pub fn new(
        client: WachtClient,
        agent_id: impl Into<String>,
        skill_slug: impl Into<String>,
    ) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
            skill_slug: skill_slug.into(),
        }
    }
    pub async fn send(self) -> Result<()> {
        let response = self
            .client
            .http_client()
            .delete(format!(
                "{}/ai/agents/{}/skills/{}",
                self.client.config().base_url,
                self.agent_id,
                self.skill_slug
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(api_error(
                status,
                "Failed to delete agent skill",
                response.text().await?,
            ))
        }
    }
}
impl FetchAgentDetailsBuilder {
    pub fn new(client: WachtClient, agent_id: impl Into<String>) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
        }
    }
    pub async fn send(self) -> Result<AgentDetailsResponse> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/agents/{}/details",
                self.client.config().base_url,
                self.agent_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to fetch agent details",
                response.text().await?,
            ))
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
        let response = self
            .client
            .http_client()
            .post(format!("{}/ai/agents", self.client.config().base_url))
            .json(&self.request)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to create agent",
                response.text().await?,
            ))
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
        let response = self
            .client
            .http_client()
            .patch(format!(
                "{}/ai/agents/{}",
                self.client.config().base_url,
                self.agent_id
            ))
            .json(&self.request)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to update agent",
                response.text().await?,
            ))
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
        let response = self
            .client
            .http_client()
            .delete(format!(
                "{}/ai/agents/{}",
                self.client.config().base_url,
                self.agent_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(api_error(
                status,
                "Failed to delete agent",
                response.text().await?,
            ))
        }
    }
}

pub struct ListAgentSubAgentsBuilder {
    client: WachtClient,
    agent_id: String,
}
impl ListAgentSubAgentsBuilder {
    pub fn new(client: WachtClient, agent_id: impl Into<String>) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
        }
    }
    pub async fn send(self) -> Result<PaginatedResponse<AiAgentWithDetails>> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/agents/{}/sub-agents",
                self.client.config().base_url,
                self.agent_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to list sub-agents",
                response.text().await?,
            ))
        }
    }
}

pub struct AttachSubAgentBuilder {
    client: WachtClient,
    agent_id: String,
    sub_agent_id: String,
}
impl AttachSubAgentBuilder {
    pub fn new(
        client: WachtClient,
        agent_id: impl Into<String>,
        sub_agent_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
            sub_agent_id: sub_agent_id.into(),
        }
    }
    pub async fn send(self) -> Result<()> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/agents/{}/sub-agents/{}",
                self.client.config().base_url,
                self.agent_id,
                self.sub_agent_id
            ))
            .json(&serde_json::json!({}))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(api_error(
                status,
                "Failed to attach sub-agent",
                response.text().await?,
            ))
        }
    }
}

pub struct DetachSubAgentBuilder {
    client: WachtClient,
    agent_id: String,
    sub_agent_id: String,
}
impl DetachSubAgentBuilder {
    pub fn new(
        client: WachtClient,
        agent_id: impl Into<String>,
        sub_agent_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
            sub_agent_id: sub_agent_id.into(),
        }
    }
    pub async fn send(self) -> Result<()> {
        let response = self
            .client
            .http_client()
            .delete(format!(
                "{}/ai/agents/{}/sub-agents/{}",
                self.client.config().base_url,
                self.agent_id,
                self.sub_agent_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(api_error(
                status,
                "Failed to detach sub-agent",
                response.text().await?,
            ))
        }
    }
}

pub struct ListAgentSkillsSummaryBuilder {
    client: WachtClient,
    agent_id: String,
}
impl ListAgentSkillsSummaryBuilder {
    pub fn new(client: WachtClient, agent_id: impl Into<String>) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
        }
    }
    pub async fn send(self) -> Result<AgentSkillsSummary> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/agents/{}/skills/list",
                self.client.config().base_url,
                self.agent_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to list agent skills summary",
                response.text().await?,
            ))
        }
    }
}
