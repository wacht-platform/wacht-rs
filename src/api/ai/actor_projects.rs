use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{
        ActorProject, AgentThread, AnswerSubmission, ApprovalSubmission, BinaryFileResponse,
        CreateActorProjectRequest, CreateAgentThreadRequest,
        CreateProjectTaskBoardItemCommentRequest, CreateProjectTaskBoardItemRequest, CursorPage,
        DelegateProjectTaskRequest, DelegateProjectTaskResponse, PaginatedResponse,
        ProjectTaskBoard, ProjectTaskBoardItem, ProjectTaskBoardItemAssignment,
        ProjectTaskBoardItemComment, TaskWorkspaceFileContent, TaskWorkspaceListing,
        UpdateActorProjectRequest, UpdateProjectTaskBoardItemRequest,
    },
};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct ActorProjectsApi {
    client: WachtClient,
}
impl ActorProjectsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }
    pub fn list_actor_projects(&self, actor_id: impl Into<String>) -> ListActorProjectsBuilder {
        ListActorProjectsBuilder::new(self.client.clone(), actor_id)
    }
    pub fn search_actor_projects(&self, actor_id: impl Into<String>) -> SearchActorProjectsBuilder {
        SearchActorProjectsBuilder::new(self.client.clone(), actor_id)
    }
    pub fn create_actor_project(
        &self,
        actor_id: impl Into<String>,
        request: CreateActorProjectRequest,
    ) -> CreateActorProjectBuilder {
        CreateActorProjectBuilder::new(self.client.clone(), actor_id, request)
    }
    pub fn fetch_actor_project(&self, project_id: impl Into<String>) -> FetchActorProjectBuilder {
        FetchActorProjectBuilder::new(self.client.clone(), project_id)
    }
    pub fn update_actor_project(
        &self,
        project_id: impl Into<String>,
        request: UpdateActorProjectRequest,
    ) -> UpdateActorProjectBuilder {
        UpdateActorProjectBuilder::new(self.client.clone(), project_id, request)
    }
    pub fn archive_actor_project(
        &self,
        project_id: impl Into<String>,
    ) -> ArchiveActorProjectBuilder {
        ArchiveActorProjectBuilder::new(self.client.clone(), project_id)
    }
    pub fn unarchive_actor_project(
        &self,
        project_id: impl Into<String>,
    ) -> UnarchiveActorProjectBuilder {
        UnarchiveActorProjectBuilder::new(self.client.clone(), project_id)
    }
    pub fn fetch_board(&self, project_id: impl Into<String>) -> FetchActorProjectBoardBuilder {
        FetchActorProjectBoardBuilder::new(self.client.clone(), project_id)
    }
    pub fn fetch_board_items(
        &self,
        project_id: impl Into<String>,
    ) -> FetchActorProjectBoardItemsBuilder {
        FetchActorProjectBoardItemsBuilder::new(self.client.clone(), project_id)
    }
    pub fn create_board_item(
        &self,
        project_id: impl Into<String>,
        request: CreateProjectTaskBoardItemRequest,
    ) -> CreateActorProjectBoardItemBuilder {
        CreateActorProjectBoardItemBuilder::new(self.client.clone(), project_id, request)
    }
    pub fn fetch_board_item(
        &self,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
    ) -> FetchActorProjectBoardItemBuilder {
        FetchActorProjectBoardItemBuilder::new(self.client.clone(), project_id, item_id)
    }
    pub fn fetch_board_item_assignments(
        &self,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
    ) -> FetchActorProjectBoardItemAssignmentsBuilder {
        FetchActorProjectBoardItemAssignmentsBuilder::new(self.client.clone(), project_id, item_id)
    }
    pub fn fetch_board_item_filesystem(
        &self,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
    ) -> FetchActorProjectBoardItemFilesystemBuilder {
        FetchActorProjectBoardItemFilesystemBuilder::new(self.client.clone(), project_id, item_id)
    }
    pub fn fetch_board_item_filesystem_file(
        &self,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
        path: impl Into<String>,
    ) -> FetchActorProjectBoardItemFilesystemFileBuilder {
        FetchActorProjectBoardItemFilesystemFileBuilder::new(
            self.client.clone(),
            project_id,
            item_id,
            path,
        )
    }

    /// Download a board item's filesystem entry as raw bytes. Distinct from
    /// `fetch_board_item_filesystem_file`, which returns JSON metadata.
    pub fn download_board_item_filesystem_file(
        &self,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
        path: impl Into<String>,
    ) -> DownloadActorProjectBoardItemFilesystemFileBuilder {
        DownloadActorProjectBoardItemFilesystemFileBuilder::new(
            self.client.clone(),
            project_id,
            item_id,
            path,
        )
    }

    /// Delegate a task from one thread to a target lane thread within the
    /// same project. Returns the assigned agent and the new task key.
    pub fn delegate_task(
        &self,
        project_id: impl Into<String>,
        request: DelegateProjectTaskRequest,
    ) -> DelegateProjectTaskBuilder {
        DelegateProjectTaskBuilder::new(self.client.clone(), project_id, request)
    }
    pub fn update_board_item(
        &self,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
        request: UpdateProjectTaskBoardItemRequest,
    ) -> UpdateActorProjectBoardItemBuilder {
        UpdateActorProjectBoardItemBuilder::new(self.client.clone(), project_id, item_id, request)
    }
    pub fn archive_board_item(
        &self,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
    ) -> ArchiveActorProjectBoardItemBuilder {
        ArchiveActorProjectBoardItemBuilder::new(self.client.clone(), project_id, item_id)
    }
    pub fn unarchive_board_item(
        &self,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
    ) -> UnarchiveActorProjectBoardItemBuilder {
        UnarchiveActorProjectBoardItemBuilder::new(self.client.clone(), project_id, item_id)
    }
    pub fn cancel_board_item(
        &self,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
    ) -> CancelActorProjectBoardItemBuilder {
        CancelActorProjectBoardItemBuilder::new(self.client.clone(), project_id, item_id)
    }
    pub fn answer_board_item_question(
        &self,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
        request: AnswerSubmission,
    ) -> AnswerActorProjectBoardItemQuestionBuilder {
        AnswerActorProjectBoardItemQuestionBuilder::new(
            self.client.clone(),
            project_id,
            item_id,
            request,
        )
    }
    pub fn approve_board_item_tool(
        &self,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
        request: ApprovalSubmission,
    ) -> ApproveActorProjectBoardItemToolBuilder {
        ApproveActorProjectBoardItemToolBuilder::new(
            self.client.clone(),
            project_id,
            item_id,
            request,
        )
    }
    pub fn fetch_board_item_comments(
        &self,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
    ) -> FetchActorProjectBoardItemCommentsBuilder {
        FetchActorProjectBoardItemCommentsBuilder::new(self.client.clone(), project_id, item_id)
    }
    pub fn create_board_item_comment(
        &self,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
        actor_id: impl Into<String>,
        request: CreateProjectTaskBoardItemCommentRequest,
    ) -> CreateActorProjectBoardItemCommentBuilder {
        CreateActorProjectBoardItemCommentBuilder::new(
            self.client.clone(),
            project_id,
            item_id,
            actor_id,
            request,
        )
    }
    pub fn fetch_threads(&self, project_id: impl Into<String>) -> FetchActorProjectThreadsBuilder {
        FetchActorProjectThreadsBuilder::new(self.client.clone(), project_id)
    }
    pub fn create_thread(
        &self,
        project_id: impl Into<String>,
        request: CreateAgentThreadRequest,
    ) -> CreateActorProjectThreadBuilder {
        CreateActorProjectThreadBuilder::new(self.client.clone(), project_id, request)
    }
    pub fn create_board_item_with_attachments(
        &self,
        project_id: impl Into<String>,
        request: CreateProjectTaskBoardItemRequest,
        files: Vec<WachtFileUpload>,
    ) -> CreateActorProjectBoardItemWithAttachmentsBuilder {
        CreateActorProjectBoardItemWithAttachmentsBuilder::new(
            self.client.clone(),
            project_id,
            request,
            files,
        )
    }
    pub fn update_board_item_with_attachments(
        &self,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
        request: UpdateProjectTaskBoardItemRequest,
        files: Vec<WachtFileUpload>,
    ) -> UpdateActorProjectBoardItemWithAttachmentsBuilder {
        UpdateActorProjectBoardItemWithAttachmentsBuilder::new(
            self.client.clone(),
            project_id,
            item_id,
            request,
            files,
        )
    }
    pub fn create_board_item_comment_with_attachments(
        &self,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
        actor_id: impl Into<String>,
        body: impl Into<String>,
        files: Vec<WachtFileUpload>,
    ) -> CreateActorProjectBoardItemCommentWithAttachmentsBuilder {
        CreateActorProjectBoardItemCommentWithAttachmentsBuilder::new(
            self.client.clone(),
            project_id,
            item_id,
            actor_id,
            body,
            files,
        )
    }
}

#[derive(Debug, Clone)]
pub struct WachtFileUpload {
    pub filename: String,
    pub content_type: Option<String>,
    pub bytes: Vec<u8>,
}

fn build_attachment_part(file: WachtFileUpload) -> Result<reqwest::multipart::Part> {
    let mime = file
        .content_type
        .unwrap_or_else(|| "application/octet-stream".to_string());
    reqwest::multipart::Part::bytes(file.bytes)
        .file_name(file.filename)
        .mime_str(&mime)
        .map_err(|e| Error::InvalidRequest(format!("invalid mime type: {e}")))
}

fn api_error(status: reqwest::StatusCode, prefix: &str, body: String) -> Error {
    Error::api_from_text(status, prefix, &body)
}

#[derive(Debug, Default, Serialize)]
struct ActorIdQuery {
    actor_id: String,
}
#[derive(Debug, Default, Serialize)]
struct ActorProjectsListQuery {
    actor_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_archived: Option<bool>,
}
#[derive(Debug, Default, Serialize)]
struct SearchActorProjectsQuery {
    actor_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cursor: Option<String>,
}
#[derive(Debug, Default, Serialize)]
struct LimitCursorQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cursor: Option<String>,
}
#[derive(Debug, Default, Serialize)]
struct PathQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
}
#[derive(Debug, Default, Serialize)]
struct IncludeArchivedQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_archived: Option<bool>,
}

pub struct ListActorProjectsBuilder {
    client: WachtClient,
    query: ActorProjectsListQuery,
}
impl ListActorProjectsBuilder {
    pub fn new(client: WachtClient, actor_id: impl Into<String>) -> Self {
        Self {
            client,
            query: ActorProjectsListQuery {
                actor_id: actor_id.into(),
                include_archived: None,
            },
        }
    }
    pub fn include_archived(mut self, include_archived: bool) -> Self {
        self.query.include_archived = Some(include_archived);
        self
    }
    pub async fn send(self) -> Result<PaginatedResponse<ActorProject>> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-projects",
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
                "Failed to list actor projects",
                response.text().await?,
            ))
        }
    }
}

pub struct SearchActorProjectsBuilder {
    client: WachtClient,
    query: SearchActorProjectsQuery,
}
impl SearchActorProjectsBuilder {
    pub fn new(client: WachtClient, actor_id: impl Into<String>) -> Self {
        Self {
            client,
            query: SearchActorProjectsQuery {
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
    pub async fn send(self) -> Result<CursorPage<ActorProject>> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-projects/search",
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
                "Failed to search actor projects",
                response.text().await?,
            ))
        }
    }
}

pub struct CreateActorProjectBuilder {
    client: WachtClient,
    actor_id: String,
    request: CreateActorProjectRequest,
}
impl CreateActorProjectBuilder {
    pub fn new(
        client: WachtClient,
        actor_id: impl Into<String>,
        request: CreateActorProjectRequest,
    ) -> Self {
        Self {
            client,
            actor_id: actor_id.into(),
            request,
        }
    }
    pub async fn send(self) -> Result<ActorProject> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-projects",
                self.client.config().base_url
            ))
            .query(&ActorIdQuery {
                actor_id: self.actor_id,
            })
            .json(&self.request)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to create actor project",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchActorProjectBuilder {
    client: WachtClient,
    project_id: String,
}
impl FetchActorProjectBuilder {
    pub fn new(client: WachtClient, project_id: impl Into<String>) -> Self {
        Self {
            client,
            project_id: project_id.into(),
        }
    }
    pub async fn send(self) -> Result<ActorProject> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-projects/{}",
                self.client.config().base_url,
                self.project_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to fetch actor project",
                response.text().await?,
            ))
        }
    }
}

pub struct UpdateActorProjectBuilder {
    client: WachtClient,
    project_id: String,
    request: UpdateActorProjectRequest,
}
impl UpdateActorProjectBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        request: UpdateActorProjectRequest,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            request,
        }
    }
    pub async fn send(self) -> Result<ActorProject> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-projects/{}/update",
                self.client.config().base_url,
                self.project_id
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
                "Failed to update actor project",
                response.text().await?,
            ))
        }
    }
}

pub struct ArchiveActorProjectBuilder {
    client: WachtClient,
    project_id: String,
}
impl ArchiveActorProjectBuilder {
    pub fn new(client: WachtClient, project_id: impl Into<String>) -> Self {
        Self {
            client,
            project_id: project_id.into(),
        }
    }
    pub async fn send(self) -> Result<ActorProject> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-projects/{}/archive",
                self.client.config().base_url,
                self.project_id
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
                "Failed to archive actor project",
                response.text().await?,
            ))
        }
    }
}

pub struct UnarchiveActorProjectBuilder {
    client: WachtClient,
    project_id: String,
}
impl UnarchiveActorProjectBuilder {
    pub fn new(client: WachtClient, project_id: impl Into<String>) -> Self {
        Self {
            client,
            project_id: project_id.into(),
        }
    }
    pub async fn send(self) -> Result<ActorProject> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-projects/{}/unarchive",
                self.client.config().base_url,
                self.project_id
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
                "Failed to unarchive actor project",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchActorProjectBoardBuilder {
    client: WachtClient,
    project_id: String,
}
impl FetchActorProjectBoardBuilder {
    pub fn new(client: WachtClient, project_id: impl Into<String>) -> Self {
        Self {
            client,
            project_id: project_id.into(),
        }
    }
    pub async fn send(self) -> Result<ProjectTaskBoard> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-projects/{}/board",
                self.client.config().base_url,
                self.project_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to fetch project board",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchActorProjectBoardItemsBuilder {
    client: WachtClient,
    project_id: String,
}
impl FetchActorProjectBoardItemsBuilder {
    pub fn new(client: WachtClient, project_id: impl Into<String>) -> Self {
        Self {
            client,
            project_id: project_id.into(),
        }
    }
    pub async fn send(self) -> Result<PaginatedResponse<ProjectTaskBoardItem>> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-projects/{}/board/items",
                self.client.config().base_url,
                self.project_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to list board items",
                response.text().await?,
            ))
        }
    }
}

pub struct CreateActorProjectBoardItemBuilder {
    client: WachtClient,
    project_id: String,
    request: CreateProjectTaskBoardItemRequest,
}
impl CreateActorProjectBoardItemBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        request: CreateProjectTaskBoardItemRequest,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            request,
        }
    }
    pub async fn send(self) -> Result<ProjectTaskBoardItem> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-projects/{}/board/items",
                self.client.config().base_url,
                self.project_id
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
                "Failed to create board item",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchActorProjectBoardItemBuilder {
    client: WachtClient,
    project_id: String,
    item_id: String,
}
impl FetchActorProjectBoardItemBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            item_id: item_id.into(),
        }
    }
    pub async fn send(self) -> Result<ProjectTaskBoardItem> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-projects/{}/board/items/{}",
                self.client.config().base_url,
                self.project_id,
                self.item_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to fetch board item",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchActorProjectBoardItemAssignmentsBuilder {
    client: WachtClient,
    project_id: String,
    item_id: String,
    query: LimitCursorQuery,
}
impl FetchActorProjectBoardItemAssignmentsBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            item_id: item_id.into(),
            query: LimitCursorQuery::default(),
        }
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.query.limit = Some(limit);
        self
    }
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.query.cursor = Some(cursor.into());
        self
    }
    pub async fn send(self) -> Result<PaginatedResponse<ProjectTaskBoardItemAssignment>> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-projects/{}/board/items/{}/assignments",
                self.client.config().base_url,
                self.project_id,
                self.item_id
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
                "Failed to fetch board item assignments",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchActorProjectBoardItemFilesystemBuilder {
    client: WachtClient,
    project_id: String,
    item_id: String,
    query: PathQuery,
}
impl FetchActorProjectBoardItemFilesystemBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            item_id: item_id.into(),
            query: PathQuery::default(),
        }
    }
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.query.path = Some(path.into());
        self
    }
    pub async fn send(self) -> Result<TaskWorkspaceListing> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-projects/{}/board/items/{}/filesystem",
                self.client.config().base_url,
                self.project_id,
                self.item_id
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
                "Failed to fetch board item filesystem",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchActorProjectBoardItemFilesystemFileBuilder {
    client: WachtClient,
    project_id: String,
    item_id: String,
    path: String,
}
impl FetchActorProjectBoardItemFilesystemFileBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            item_id: item_id.into(),
            path: path.into(),
        }
    }
    pub async fn send(self) -> Result<TaskWorkspaceFileContent> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-projects/{}/board/items/{}/filesystem/file",
                self.client.config().base_url,
                self.project_id,
                self.item_id
            ))
            .query(&PathQuery {
                path: Some(self.path),
            })
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to fetch board item file",
                response.text().await?,
            ))
        }
    }
}

pub struct UpdateActorProjectBoardItemBuilder {
    client: WachtClient,
    project_id: String,
    item_id: String,
    request: UpdateProjectTaskBoardItemRequest,
}
impl UpdateActorProjectBoardItemBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
        request: UpdateProjectTaskBoardItemRequest,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            item_id: item_id.into(),
            request,
        }
    }
    pub async fn send(self) -> Result<ProjectTaskBoardItem> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-projects/{}/board/items/{}/update",
                self.client.config().base_url,
                self.project_id,
                self.item_id
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
                "Failed to update board item",
                response.text().await?,
            ))
        }
    }
}

pub struct ArchiveActorProjectBoardItemBuilder {
    client: WachtClient,
    project_id: String,
    item_id: String,
}
impl ArchiveActorProjectBoardItemBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            item_id: item_id.into(),
        }
    }
    pub async fn send(self) -> Result<ProjectTaskBoardItem> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-projects/{}/board/items/{}/archive",
                self.client.config().base_url,
                self.project_id,
                self.item_id
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
                "Failed to archive board item",
                response.text().await?,
            ))
        }
    }
}

pub struct UnarchiveActorProjectBoardItemBuilder {
    client: WachtClient,
    project_id: String,
    item_id: String,
}
impl UnarchiveActorProjectBoardItemBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            item_id: item_id.into(),
        }
    }
    pub async fn send(self) -> Result<ProjectTaskBoardItem> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-projects/{}/board/items/{}/unarchive",
                self.client.config().base_url,
                self.project_id,
                self.item_id
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
                "Failed to unarchive board item",
                response.text().await?,
            ))
        }
    }
}

pub struct CancelActorProjectBoardItemBuilder {
    client: WachtClient,
    project_id: String,
    item_id: String,
}
impl CancelActorProjectBoardItemBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            item_id: item_id.into(),
        }
    }
    pub async fn send(self) -> Result<ProjectTaskBoardItem> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-projects/{}/board/items/{}/cancel",
                self.client.config().base_url,
                self.project_id,
                self.item_id
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
                "Failed to cancel board item",
                response.text().await?,
            ))
        }
    }
}

pub struct AnswerActorProjectBoardItemQuestionBuilder {
    client: WachtClient,
    project_id: String,
    item_id: String,
    request: AnswerSubmission,
}
impl AnswerActorProjectBoardItemQuestionBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
        request: AnswerSubmission,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            item_id: item_id.into(),
            request,
        }
    }
    pub async fn send(self) -> Result<ProjectTaskBoardItem> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-projects/{}/board/items/{}/answer",
                self.client.config().base_url,
                self.project_id,
                self.item_id
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
                "Failed to answer board item question",
                response.text().await?,
            ))
        }
    }
}

pub struct ApproveActorProjectBoardItemToolBuilder {
    client: WachtClient,
    project_id: String,
    item_id: String,
    request: ApprovalSubmission,
}
impl ApproveActorProjectBoardItemToolBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
        request: ApprovalSubmission,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            item_id: item_id.into(),
            request,
        }
    }
    pub async fn send(self) -> Result<ProjectTaskBoardItem> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-projects/{}/board/items/{}/approval",
                self.client.config().base_url,
                self.project_id,
                self.item_id
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
                "Failed to approve board item tool",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchActorProjectBoardItemCommentsBuilder {
    client: WachtClient,
    project_id: String,
    item_id: String,
}
impl FetchActorProjectBoardItemCommentsBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            item_id: item_id.into(),
        }
    }
    pub async fn send(self) -> Result<PaginatedResponse<ProjectTaskBoardItemComment>> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-projects/{}/board/items/{}/comments",
                self.client.config().base_url,
                self.project_id,
                self.item_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to fetch board item comments",
                response.text().await?,
            ))
        }
    }
}

pub struct CreateActorProjectBoardItemCommentBuilder {
    client: WachtClient,
    project_id: String,
    item_id: String,
    actor_id: String,
    request: CreateProjectTaskBoardItemCommentRequest,
}
impl CreateActorProjectBoardItemCommentBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
        actor_id: impl Into<String>,
        request: CreateProjectTaskBoardItemCommentRequest,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            item_id: item_id.into(),
            actor_id: actor_id.into(),
            request,
        }
    }
    pub async fn send(self) -> Result<ProjectTaskBoardItemComment> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-projects/{}/board/items/{}/comments",
                self.client.config().base_url,
                self.project_id,
                self.item_id
            ))
            .query(&ActorIdQuery {
                actor_id: self.actor_id,
            })
            .json(&self.request)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to create board item comment",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchActorProjectThreadsBuilder {
    client: WachtClient,
    project_id: String,
    include_archived: Option<bool>,
}
impl FetchActorProjectThreadsBuilder {
    pub fn new(client: WachtClient, project_id: impl Into<String>) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            include_archived: None,
        }
    }
    pub fn include_archived(mut self, include_archived: bool) -> Self {
        self.include_archived = Some(include_archived);
        self
    }
    pub async fn send(self) -> Result<PaginatedResponse<AgentThread>> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-projects/{}/threads",
                self.client.config().base_url,
                self.project_id
            ))
            .query(&IncludeArchivedQuery {
                include_archived: self.include_archived,
            })
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to list project threads",
                response.text().await?,
            ))
        }
    }
}

pub struct CreateActorProjectThreadBuilder {
    client: WachtClient,
    project_id: String,
    request: CreateAgentThreadRequest,
}
impl CreateActorProjectThreadBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        request: CreateAgentThreadRequest,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            request,
        }
    }
    pub async fn send(self) -> Result<AgentThread> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-projects/{}/threads",
                self.client.config().base_url,
                self.project_id
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
                "Failed to create project thread",
                response.text().await?,
            ))
        }
    }
}

pub struct DownloadActorProjectBoardItemFilesystemFileBuilder {
    client: WachtClient,
    project_id: String,
    item_id: String,
    path: String,
}
impl DownloadActorProjectBoardItemFilesystemFileBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            item_id: item_id.into(),
            path: path.into(),
        }
    }

    pub async fn send(self) -> Result<BinaryFileResponse> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-projects/{}/board/items/{}/filesystem/download",
                self.client.config().base_url,
                self.project_id,
                self.item_id
            ))
            .query(&PathQuery {
                path: Some(self.path),
            })
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            return Err(api_error(
                status,
                "Failed to download board item file",
                response.text().await?,
            ));
        }

        let mime_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned);
        let file_name = response
            .headers()
            .get(reqwest::header::CONTENT_DISPOSITION)
            .and_then(|v| v.to_str().ok())
            .and_then(parse_filename_from_content_disposition);
        let data = response.bytes().await?.to_vec();
        Ok(BinaryFileResponse {
            data,
            mime_type,
            file_name,
        })
    }
}

pub struct DelegateProjectTaskBuilder {
    client: WachtClient,
    project_id: String,
    request: DelegateProjectTaskRequest,
}
impl DelegateProjectTaskBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        request: DelegateProjectTaskRequest,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            request,
        }
    }

    pub async fn send(self) -> Result<DelegateProjectTaskResponse> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-projects/{}/board/items/delegate",
                self.client.config().base_url,
                self.project_id
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
                "Failed to delegate project task",
                response.text().await?,
            ))
        }
    }
}

/// Pull the `filename` parameter out of a `Content-Disposition` header.
/// Returns `None` when there's no `filename=...` segment or the value can't
/// be parsed cleanly; downloads then fall back to a server-derived name on
/// the caller side.
fn parse_filename_from_content_disposition(header: &str) -> Option<String> {
    for part in header.split(';') {
        let trimmed = part.trim();
        if let Some(value) = trimmed.strip_prefix("filename=") {
            let unquoted = value.trim().trim_matches('"').to_string();
            if !unquoted.is_empty() {
                return Some(unquoted);
            }
        }
    }
    None
}

pub struct CreateActorProjectBoardItemWithAttachmentsBuilder {
    client: WachtClient,
    project_id: String,
    request: CreateProjectTaskBoardItemRequest,
    files: Vec<WachtFileUpload>,
}
impl CreateActorProjectBoardItemWithAttachmentsBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        request: CreateProjectTaskBoardItemRequest,
        files: Vec<WachtFileUpload>,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            request,
            files,
        }
    }
    pub async fn send(self) -> Result<ProjectTaskBoardItem> {
        let mut form = reqwest::multipart::Form::new().text("title", self.request.title);
        if let Some(v) = self.request.description {
            form = form.text("description", v);
        }
        if let Some(v) = self.request.status {
            form = form.text("status", v);
        }
        if let Some(v) = self.request.schedule_kind {
            form = form.text("schedule_kind", v);
        }
        if let Some(v) = self.request.next_run_at {
            let text = match v {
                serde_json::Value::String(s) => s,
                other => serde_json::to_string(&other).map_err(Error::Json)?,
            };
            form = form.text("next_run_at", text);
        }
        if let Some(v) = self.request.interval_seconds {
            form = form.text("interval_seconds", v.to_string());
        }
        if let Some(v) = self.request.mounts {
            form = form.text(
                "mounts",
                serde_json::to_string(&v).map_err(Error::Json)?,
            );
        }
        for file in self.files {
            form = form.part("attachments", build_attachment_part(file)?);
        }
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-projects/{}/board/items",
                self.client.config().base_url,
                self.project_id
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
                "Failed to create board item with attachments",
                response.text().await?,
            ))
        }
    }
}

pub struct UpdateActorProjectBoardItemWithAttachmentsBuilder {
    client: WachtClient,
    project_id: String,
    item_id: String,
    request: UpdateProjectTaskBoardItemRequest,
    files: Vec<WachtFileUpload>,
}
impl UpdateActorProjectBoardItemWithAttachmentsBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
        request: UpdateProjectTaskBoardItemRequest,
        files: Vec<WachtFileUpload>,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            item_id: item_id.into(),
            request,
            files,
        }
    }
    pub async fn send(self) -> Result<ProjectTaskBoardItem> {
        let mut form = reqwest::multipart::Form::new();
        if let Some(v) = self.request.title {
            form = form.text("title", v);
        }
        if let Some(v) = self.request.description {
            form = form.text("description", v);
        }
        if let Some(v) = self.request.status {
            form = form.text("status", v);
        }
        if let Some(v) = self.request.schedule_kind {
            form = form.text("schedule_kind", v);
        }
        if let Some(v) = self.request.next_run_at {
            let text = match v {
                serde_json::Value::String(s) => s,
                other => serde_json::to_string(&other).map_err(Error::Json)?,
            };
            form = form.text("next_run_at", text);
        }
        if let Some(v) = self.request.interval_seconds {
            form = form.text("interval_seconds", v.to_string());
        }
        if let Some(v) = self.request.clear_schedule {
            form = form.text("clear_schedule", v.to_string());
        }
        if let Some(v) = self.request.mounts {
            form = form.text(
                "mounts",
                serde_json::to_string(&v).map_err(Error::Json)?,
            );
        }
        for file in self.files {
            form = form.part("attachments", build_attachment_part(file)?);
        }
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-projects/{}/board/items/{}/update",
                self.client.config().base_url,
                self.project_id,
                self.item_id
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
                "Failed to update board item with attachments",
                response.text().await?,
            ))
        }
    }
}

pub struct CreateActorProjectBoardItemCommentWithAttachmentsBuilder {
    client: WachtClient,
    project_id: String,
    item_id: String,
    actor_id: String,
    body: String,
    files: Vec<WachtFileUpload>,
}
impl CreateActorProjectBoardItemCommentWithAttachmentsBuilder {
    pub fn new(
        client: WachtClient,
        project_id: impl Into<String>,
        item_id: impl Into<String>,
        actor_id: impl Into<String>,
        body: impl Into<String>,
        files: Vec<WachtFileUpload>,
    ) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            item_id: item_id.into(),
            actor_id: actor_id.into(),
            body: body.into(),
            files,
        }
    }
    pub async fn send(self) -> Result<ProjectTaskBoardItemComment> {
        let mut form = reqwest::multipart::Form::new().text("body", self.body);
        for file in self.files {
            form = form.part("attachments", build_attachment_part(file)?);
        }
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-projects/{}/board/items/{}/comments",
                self.client.config().base_url,
                self.project_id,
                self.item_id
            ))
            .query(&ActorIdQuery {
                actor_id: self.actor_id,
            })
            .multipart(form)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to create board item comment with attachments",
                response.text().await?,
            ))
        }
    }
}
