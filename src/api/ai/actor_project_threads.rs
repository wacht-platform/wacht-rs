use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{
        AgentThread, CursorPage, ExecuteAgentRequest, ExecuteAgentResponse, PaginatedResponse,
        ProjectTaskBoardItemAssignment, TaskWorkspaceListing, ThreadMessagesResponse,
        ThreadTaskGraph, UpdateAgentThreadRequest,
    },
};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct ActorProjectThreadsApi {
    client: WachtClient,
}
impl ActorProjectThreadsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }
    pub fn search_threads(&self, actor_id: impl Into<String>) -> SearchActorProjectThreadsBuilder {
        SearchActorProjectThreadsBuilder::new(self.client.clone(), actor_id)
    }
    pub fn fetch_thread(&self, thread_id: impl Into<String>) -> FetchActorProjectThreadBuilder {
        FetchActorProjectThreadBuilder::new(self.client.clone(), thread_id)
    }
    pub fn update_thread(
        &self,
        thread_id: impl Into<String>,
        request: UpdateAgentThreadRequest,
    ) -> UpdateActorProjectThreadBuilder {
        UpdateActorProjectThreadBuilder::new(self.client.clone(), thread_id, request)
    }
    pub fn archive_thread(&self, thread_id: impl Into<String>) -> ArchiveActorProjectThreadBuilder {
        ArchiveActorProjectThreadBuilder::new(self.client.clone(), thread_id)
    }
    pub fn unarchive_thread(
        &self,
        thread_id: impl Into<String>,
    ) -> UnarchiveActorProjectThreadBuilder {
        UnarchiveActorProjectThreadBuilder::new(self.client.clone(), thread_id)
    }
    pub fn fetch_assignments(
        &self,
        thread_id: impl Into<String>,
    ) -> FetchActorProjectThreadAssignmentsBuilder {
        FetchActorProjectThreadAssignmentsBuilder::new(self.client.clone(), thread_id)
    }
    pub fn fetch_task_graphs(
        &self,
        thread_id: impl Into<String>,
    ) -> FetchActorProjectThreadTaskGraphsBuilder {
        FetchActorProjectThreadTaskGraphsBuilder::new(self.client.clone(), thread_id)
    }
    pub fn fetch_messages(
        &self,
        thread_id: impl Into<String>,
    ) -> FetchActorProjectThreadMessagesBuilder {
        FetchActorProjectThreadMessagesBuilder::new(self.client.clone(), thread_id)
    }
    pub fn fetch_filesystem(
        &self,
        thread_id: impl Into<String>,
    ) -> FetchActorProjectThreadFilesystemBuilder {
        FetchActorProjectThreadFilesystemBuilder::new(self.client.clone(), thread_id)
    }
    pub fn fetch_filesystem_file(
        &self,
        thread_id: impl Into<String>,
        path: impl Into<String>,
    ) -> FetchActorProjectThreadFilesystemFileBuilder {
        FetchActorProjectThreadFilesystemFileBuilder::new(self.client.clone(), thread_id, path)
    }
    pub fn run_thread(
        &self,
        thread_id: impl Into<String>,
        request: ExecuteAgentRequest,
    ) -> RunActorProjectThreadBuilder {
        RunActorProjectThreadBuilder::new(self.client.clone(), thread_id, request)
    }
}

fn api_error(status: reqwest::StatusCode, prefix: &str, body: String) -> Error {
    Error::api_from_text(status, prefix, &body)
}

#[derive(Debug, Default, Serialize)]
struct SearchThreadsQuery {
    actor_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cursor: Option<String>,
}
#[derive(Debug, Default, Serialize)]
struct ThreadMessagesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after_id: Option<String>,
}
#[derive(Debug, Default, Serialize)]
struct PathQuery {
    path: String,
}
#[derive(Debug, Default, Serialize)]
struct OptionalPathQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
}

pub struct SearchActorProjectThreadsBuilder {
    client: WachtClient,
    query: SearchThreadsQuery,
}
impl SearchActorProjectThreadsBuilder {
    pub fn new(client: WachtClient, actor_id: impl Into<String>) -> Self {
        Self {
            client,
            query: SearchThreadsQuery {
                actor_id: actor_id.into(),
                q: None,
                limit: None,
                cursor: None,
            },
        }
    }
    pub fn query(mut self, q: impl Into<String>) -> Self {
        self.query.q = Some(q.into());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.query.limit = Some(limit);
        self
    }
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.query.cursor = Some(cursor.into());
        self
    }
    pub async fn send(self) -> Result<CursorPage<AgentThread>> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-project-threads/search",
                self.client.config().base_url
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
                "Failed to search actor project threads",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchActorProjectThreadBuilder {
    client: WachtClient,
    thread_id: String,
}
impl FetchActorProjectThreadBuilder {
    pub fn new(client: WachtClient, thread_id: impl Into<String>) -> Self {
        Self {
            client,
            thread_id: thread_id.into(),
        }
    }
    pub async fn send(self) -> Result<AgentThread> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-project-threads/{}",
                self.client.config().base_url,
                self.thread_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to fetch actor project thread",
                response.text().await?,
            ))
        }
    }
}

pub struct UpdateActorProjectThreadBuilder {
    client: WachtClient,
    thread_id: String,
    request: UpdateAgentThreadRequest,
}
impl UpdateActorProjectThreadBuilder {
    pub fn new(
        client: WachtClient,
        thread_id: impl Into<String>,
        request: UpdateAgentThreadRequest,
    ) -> Self {
        Self {
            client,
            thread_id: thread_id.into(),
            request,
        }
    }
    pub async fn send(self) -> Result<AgentThread> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-project-threads/{}/update",
                self.client.config().base_url,
                self.thread_id
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
                "Failed to update thread",
                response.text().await?,
            ))
        }
    }
}

pub struct ArchiveActorProjectThreadBuilder {
    client: WachtClient,
    thread_id: String,
}
impl ArchiveActorProjectThreadBuilder {
    pub fn new(client: WachtClient, thread_id: impl Into<String>) -> Self {
        Self {
            client,
            thread_id: thread_id.into(),
        }
    }
    pub async fn send(self) -> Result<AgentThread> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-project-threads/{}/archive",
                self.client.config().base_url,
                self.thread_id
            ))
            .json(&serde_json::json!({}))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to archive thread",
                response.text().await?,
            ))
        }
    }
}

pub struct UnarchiveActorProjectThreadBuilder {
    client: WachtClient,
    thread_id: String,
}
impl UnarchiveActorProjectThreadBuilder {
    pub fn new(client: WachtClient, thread_id: impl Into<String>) -> Self {
        Self {
            client,
            thread_id: thread_id.into(),
        }
    }
    pub async fn send(self) -> Result<AgentThread> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-project-threads/{}/unarchive",
                self.client.config().base_url,
                self.thread_id
            ))
            .json(&serde_json::json!({}))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to unarchive thread",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchActorProjectThreadAssignmentsBuilder {
    client: WachtClient,
    thread_id: String,
}
impl FetchActorProjectThreadAssignmentsBuilder {
    pub fn new(client: WachtClient, thread_id: impl Into<String>) -> Self {
        Self {
            client,
            thread_id: thread_id.into(),
        }
    }
    pub async fn send(self) -> Result<PaginatedResponse<ProjectTaskBoardItemAssignment>> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-project-threads/{}/assignments",
                self.client.config().base_url,
                self.thread_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to fetch thread assignments",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchActorProjectThreadTaskGraphsBuilder {
    client: WachtClient,
    thread_id: String,
}
impl FetchActorProjectThreadTaskGraphsBuilder {
    pub fn new(client: WachtClient, thread_id: impl Into<String>) -> Self {
        Self {
            client,
            thread_id: thread_id.into(),
        }
    }
    pub async fn send(self) -> Result<Option<ThreadTaskGraph>> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-project-threads/{}/task-graphs",
                self.client.config().base_url,
                self.thread_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to fetch thread task graphs",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchActorProjectThreadMessagesBuilder {
    client: WachtClient,
    thread_id: String,
    query: ThreadMessagesQuery,
}
impl FetchActorProjectThreadMessagesBuilder {
    pub fn new(client: WachtClient, thread_id: impl Into<String>) -> Self {
        Self {
            client,
            thread_id: thread_id.into(),
            query: ThreadMessagesQuery::default(),
        }
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.query.limit = Some(limit);
        self
    }
    pub fn before_id(mut self, before_id: impl Into<String>) -> Self {
        self.query.before_id = Some(before_id.into());
        self
    }
    pub fn after_id(mut self, after_id: impl Into<String>) -> Self {
        self.query.after_id = Some(after_id.into());
        self
    }
    pub async fn send(self) -> Result<ThreadMessagesResponse> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-project-threads/{}/messages",
                self.client.config().base_url,
                self.thread_id
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
                "Failed to fetch thread messages",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchActorProjectThreadFilesystemBuilder {
    client: WachtClient,
    thread_id: String,
    path: Option<String>,
}
impl FetchActorProjectThreadFilesystemBuilder {
    pub fn new(client: WachtClient, thread_id: impl Into<String>) -> Self {
        Self {
            client,
            thread_id: thread_id.into(),
            path: None,
        }
    }
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
    pub async fn send(self) -> Result<TaskWorkspaceListing> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-project-threads/{}/filesystem",
                self.client.config().base_url,
                self.thread_id
            ))
            .query(&OptionalPathQuery { path: self.path })
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to fetch thread filesystem",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchActorProjectThreadFilesystemFileBuilder {
    client: WachtClient,
    thread_id: String,
    path: String,
}
impl FetchActorProjectThreadFilesystemFileBuilder {
    pub fn new(client: WachtClient, thread_id: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            client,
            thread_id: thread_id.into(),
            path: path.into(),
        }
    }
    pub async fn send_bytes(self) -> Result<(Vec<u8>, String)> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-project-threads/{}/filesystem/file",
                self.client.config().base_url,
                self.thread_id
            ))
            .query(&PathQuery { path: self.path })
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            let mime = response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("application/octet-stream")
                .to_string();
            Ok((response.bytes().await?.to_vec(), mime))
        } else {
            Err(api_error(
                status,
                "Failed to fetch thread filesystem file",
                response.text().await?,
            ))
        }
    }
}

pub struct RunActorProjectThreadBuilder {
    client: WachtClient,
    thread_id: String,
    request: ExecuteAgentRequest,
}
impl RunActorProjectThreadBuilder {
    pub fn new(
        client: WachtClient,
        thread_id: impl Into<String>,
        request: ExecuteAgentRequest,
    ) -> Self {
        Self {
            client,
            thread_id: thread_id.into(),
            request,
        }
    }
    pub async fn send(self) -> Result<ExecuteAgentResponse> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-project-threads/{}/run",
                self.client.config().base_url,
                self.thread_id
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
                "Failed to run thread",
                response.text().await?,
            ))
        }
    }
}
