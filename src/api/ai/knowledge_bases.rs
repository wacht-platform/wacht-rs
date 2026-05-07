use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{
        AiKnowledgeBase, AiKnowledgeBaseWithDetails, CreateAiKnowledgeBaseRequest,
        KnowledgeBaseDocument, KnowledgeBaseListResponse, PaginatedResponse,
        UpdateAiKnowledgeBaseRequest,
    },
};
use reqwest::multipart::{Form, Part};
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListKnowledgeBasesOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

#[derive(Debug, Clone)]
pub struct KnowledgeBasesApi {
    client: WachtClient,
}
impl KnowledgeBasesApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }
    pub fn fetch_knowledge_bases(&self) -> FetchKnowledgeBasesBuilder {
        FetchKnowledgeBasesBuilder::new(self.client.clone())
    }
    pub fn create_knowledge_base(
        &self,
        request: CreateAiKnowledgeBaseRequest,
    ) -> CreateKnowledgeBaseBuilder {
        CreateKnowledgeBaseBuilder::new(self.client.clone(), request)
    }
    pub fn fetch_knowledge_base(
        &self,
        knowledge_base_id: impl Into<String>,
    ) -> FetchKnowledgeBaseBuilder {
        FetchKnowledgeBaseBuilder::new(self.client.clone(), knowledge_base_id)
    }
    pub fn update_knowledge_base(
        &self,
        knowledge_base_id: impl Into<String>,
        request: UpdateAiKnowledgeBaseRequest,
    ) -> UpdateKnowledgeBaseBuilder {
        UpdateKnowledgeBaseBuilder::new(self.client.clone(), knowledge_base_id, request)
    }
    pub fn delete_knowledge_base(
        &self,
        knowledge_base_id: impl Into<String>,
    ) -> DeleteKnowledgeBaseBuilder {
        DeleteKnowledgeBaseBuilder::new(self.client.clone(), knowledge_base_id)
    }
    pub fn fetch_agent_knowledge_bases(
        &self,
        agent_id: impl Into<String>,
    ) -> FetchAgentKnowledgeBasesBuilder {
        FetchAgentKnowledgeBasesBuilder::new(self.client.clone(), agent_id)
    }
    pub fn attach_agent_knowledge_base(
        &self,
        agent_id: impl Into<String>,
        kb_id: impl Into<String>,
    ) -> AttachKnowledgeBaseBuilder {
        AttachKnowledgeBaseBuilder::new(self.client.clone(), agent_id, kb_id)
    }
    pub fn detach_agent_knowledge_base(
        &self,
        agent_id: impl Into<String>,
        kb_id: impl Into<String>,
    ) -> DetachKnowledgeBaseBuilder {
        DetachKnowledgeBaseBuilder::new(self.client.clone(), agent_id, kb_id)
    }
    pub fn fetch_documents(&self, kb_id: impl Into<String>) -> FetchDocumentsBuilder {
        FetchDocumentsBuilder::new(self.client.clone(), kb_id)
    }
    pub fn upload_document(
        &self,
        kb_id: impl Into<String>,
        file_content: Vec<u8>,
        file_name: impl Into<String>,
    ) -> UploadDocumentBuilder {
        UploadDocumentBuilder::new(self.client.clone(), kb_id, file_content, file_name)
    }
    pub fn delete_document(
        &self,
        kb_id: impl Into<String>,
        document_id: impl Into<String>,
    ) -> DeleteDocumentBuilder {
        DeleteDocumentBuilder::new(self.client.clone(), kb_id, document_id)
    }
}

fn api_error(status: reqwest::StatusCode, prefix: &str, body: String) -> Error {
    Error::api_from_text(status, prefix, &body)
}

pub struct FetchKnowledgeBasesBuilder {
    client: WachtClient,
    options: ListKnowledgeBasesOptions,
}
impl FetchKnowledgeBasesBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
            options: ListKnowledgeBasesOptions::default(),
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
    pub async fn send(self) -> Result<KnowledgeBaseListResponse> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/knowledge-bases",
                self.client.config().base_url
            ))
            .query(&self.options)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to list knowledge bases",
                response.text().await?,
            ))
        }
    }
}

pub struct CreateKnowledgeBaseBuilder {
    client: WachtClient,
    request: CreateAiKnowledgeBaseRequest,
}
impl CreateKnowledgeBaseBuilder {
    pub fn new(client: WachtClient, request: CreateAiKnowledgeBaseRequest) -> Self {
        Self { client, request }
    }
    pub async fn send(self) -> Result<AiKnowledgeBase> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/knowledge-bases",
                self.client.config().base_url
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
                "Failed to create knowledge base",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchKnowledgeBaseBuilder {
    client: WachtClient,
    knowledge_base_id: String,
}
impl FetchKnowledgeBaseBuilder {
    pub fn new(client: WachtClient, knowledge_base_id: impl Into<String>) -> Self {
        Self {
            client,
            knowledge_base_id: knowledge_base_id.into(),
        }
    }
    pub async fn send(self) -> Result<AiKnowledgeBaseWithDetails> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/knowledge-bases/{}",
                self.client.config().base_url,
                self.knowledge_base_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to fetch knowledge base",
                response.text().await?,
            ))
        }
    }
}

pub struct UpdateKnowledgeBaseBuilder {
    client: WachtClient,
    knowledge_base_id: String,
    request: UpdateAiKnowledgeBaseRequest,
}
impl UpdateKnowledgeBaseBuilder {
    pub fn new(
        client: WachtClient,
        knowledge_base_id: impl Into<String>,
        request: UpdateAiKnowledgeBaseRequest,
    ) -> Self {
        Self {
            client,
            knowledge_base_id: knowledge_base_id.into(),
            request,
        }
    }
    pub async fn send(self) -> Result<AiKnowledgeBase> {
        let response = self
            .client
            .http_client()
            .patch(format!(
                "{}/ai/knowledge-bases/{}",
                self.client.config().base_url,
                self.knowledge_base_id
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
                "Failed to update knowledge base",
                response.text().await?,
            ))
        }
    }
}

pub struct DeleteKnowledgeBaseBuilder {
    client: WachtClient,
    knowledge_base_id: String,
}
impl DeleteKnowledgeBaseBuilder {
    pub fn new(client: WachtClient, knowledge_base_id: impl Into<String>) -> Self {
        Self {
            client,
            knowledge_base_id: knowledge_base_id.into(),
        }
    }
    pub async fn send(self) -> Result<()> {
        let response = self
            .client
            .http_client()
            .delete(format!(
                "{}/ai/knowledge-bases/{}",
                self.client.config().base_url,
                self.knowledge_base_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(api_error(
                status,
                "Failed to delete knowledge base",
                response.text().await?,
            ))
        }
    }
}

pub struct FetchAgentKnowledgeBasesBuilder {
    client: WachtClient,
    agent_id: String,
}
impl FetchAgentKnowledgeBasesBuilder {
    pub fn new(client: WachtClient, agent_id: impl Into<String>) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
        }
    }
    pub async fn send(self) -> Result<PaginatedResponse<AiKnowledgeBaseWithDetails>> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/agents/{}/knowledge-bases",
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
                "Failed to list agent knowledge bases",
                response.text().await?,
            ))
        }
    }
}

pub struct AttachKnowledgeBaseBuilder {
    client: WachtClient,
    agent_id: String,
    kb_id: String,
}
impl AttachKnowledgeBaseBuilder {
    pub fn new(client: WachtClient, agent_id: impl Into<String>, kb_id: impl Into<String>) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
            kb_id: kb_id.into(),
        }
    }
    pub async fn send(self) -> Result<()> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/agents/{}/knowledge-bases/{}",
                self.client.config().base_url,
                self.agent_id,
                self.kb_id
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
                "Failed to attach knowledge base",
                response.text().await?,
            ))
        }
    }
}

pub struct DetachKnowledgeBaseBuilder {
    client: WachtClient,
    agent_id: String,
    kb_id: String,
}
impl DetachKnowledgeBaseBuilder {
    pub fn new(client: WachtClient, agent_id: impl Into<String>, kb_id: impl Into<String>) -> Self {
        Self {
            client,
            agent_id: agent_id.into(),
            kb_id: kb_id.into(),
        }
    }
    pub async fn send(self) -> Result<()> {
        let response = self
            .client
            .http_client()
            .delete(format!(
                "{}/ai/agents/{}/knowledge-bases/{}",
                self.client.config().base_url,
                self.agent_id,
                self.kb_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(api_error(
                status,
                "Failed to detach knowledge base",
                response.text().await?,
            ))
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct DocumentListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

pub struct FetchDocumentsBuilder {
    client: WachtClient,
    kb_id: String,
    options: DocumentListOptions,
}
impl FetchDocumentsBuilder {
    pub fn new(client: WachtClient, kb_id: impl Into<String>) -> Self {
        Self {
            client,
            kb_id: kb_id.into(),
            options: DocumentListOptions::default(),
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
    pub async fn send(self) -> Result<PaginatedResponse<KnowledgeBaseDocument>> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/knowledge-bases/{}/documents",
                self.client.config().base_url,
                self.kb_id
            ))
            .query(&self.options)
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to fetch knowledge base documents",
                response.text().await?,
            ))
        }
    }
}

pub struct UploadDocumentBuilder {
    client: WachtClient,
    kb_id: String,
    file_content: Vec<u8>,
    file_name: String,
    title: Option<String>,
    description: Option<String>,
}
impl UploadDocumentBuilder {
    pub fn new(
        client: WachtClient,
        kb_id: impl Into<String>,
        file_content: Vec<u8>,
        file_name: impl Into<String>,
    ) -> Self {
        Self {
            client,
            kb_id: kb_id.into(),
            file_content,
            file_name: file_name.into(),
            title: None,
            description: None,
        }
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    pub async fn send(self) -> Result<KnowledgeBaseDocument> {
        let part = Part::bytes(self.file_content).file_name(self.file_name);
        let mut form = Form::new().part("file", part);
        if let Some(title) = self.title {
            form = form.text("title", title);
        }
        if let Some(description) = self.description {
            form = form.text("description", description);
        }
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/knowledge-bases/{}/documents",
                self.client.config().base_url,
                self.kb_id
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
                "Failed to upload knowledge base document",
                response.text().await?,
            ))
        }
    }
}

pub struct DeleteDocumentBuilder {
    client: WachtClient,
    kb_id: String,
    document_id: String,
}
impl DeleteDocumentBuilder {
    pub fn new(
        client: WachtClient,
        kb_id: impl Into<String>,
        document_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            kb_id: kb_id.into(),
            document_id: document_id.into(),
        }
    }
    pub async fn send(self) -> Result<()> {
        let response = self
            .client
            .http_client()
            .delete(format!(
                "{}/ai/knowledge-bases/{}/documents/{}",
                self.client.config().base_url,
                self.kb_id,
                self.document_id
            ))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(api_error(
                status,
                "Failed to delete knowledge base document",
                response.text().await?,
            ))
        }
    }
}
