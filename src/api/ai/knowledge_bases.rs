use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{
        AiKnowledgeBase, CreateAiKnowledgeBaseRequest, KnowledgeBaseDocument, PaginatedResponse,
        UpdateAiKnowledgeBaseRequest,
    },
};
use serde::{Deserialize, Serialize};

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

    pub fn create_knowledge_base(&self, request: CreateAiKnowledgeBaseRequest) -> CreateKnowledgeBaseBuilder {
        CreateKnowledgeBaseBuilder::new(self.client.clone(), request)
    }

    pub fn fetch_knowledge_base(&self, knowledge_base_id: &str) -> FetchKnowledgeBaseBuilder {
        FetchKnowledgeBaseBuilder::new(self.client.clone(), knowledge_base_id)
    }

    pub fn update_knowledge_base(
        &self,
        knowledge_base_id: &str,
        request: UpdateAiKnowledgeBaseRequest,
    ) -> UpdateKnowledgeBaseBuilder {
        UpdateKnowledgeBaseBuilder::new(self.client.clone(), knowledge_base_id, request)
    }

    pub fn delete_knowledge_base(&self, knowledge_base_id: &str) -> DeleteKnowledgeBaseBuilder {
        DeleteKnowledgeBaseBuilder::new(self.client.clone(), knowledge_base_id)
    }

    pub fn fetch_documents(&self, knowledge_base_id: &str) -> FetchDocumentsBuilder {
        FetchDocumentsBuilder::new(self.client.clone(), knowledge_base_id)
    }

    pub fn upload_document(
        &self,
        knowledge_base_id: &str,
        file_content: Vec<u8>,
        file_name: String,
    ) -> UploadDocumentBuilder {
        UploadDocumentBuilder::new(self.client.clone(), knowledge_base_id, file_content, file_name)
    }

    pub fn delete_document(&self, knowledge_base_id: &str, document_id: &str) -> DeleteDocumentBuilder {
        DeleteDocumentBuilder::new(self.client.clone(), knowledge_base_id, document_id)
    }
}

#[derive(Debug, Clone)]
pub struct FetchKnowledgeBasesBuilder {
    client: WachtClient,
    options: Option<ListKnowledgeBasesOptions>,
}

impl FetchKnowledgeBasesBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
            options: None,
        }
    }

    pub fn options(mut self, options: ListKnowledgeBasesOptions) -> Self {
        self.options = Some(options);
        self
    }

    pub async fn send(self) -> Result<PaginatedResponse<AiKnowledgeBase>> {
        let client = self.client.http_client();
        let url = format!("{}/ai/knowledge-bases", self.client.config().base_url);

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
                message: format!("Failed to list knowledge bases: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateKnowledgeBaseBuilder {
    client: WachtClient,
    request: CreateAiKnowledgeBaseRequest,
}

impl CreateKnowledgeBaseBuilder {
    pub fn new(client: WachtClient, request: CreateAiKnowledgeBaseRequest) -> Self {
        Self { client, request }
    }

    pub async fn send(self) -> Result<AiKnowledgeBase> {
        let client = self.client.http_client();
        let url = format!("{}/ai/knowledge-bases", self.client.config().base_url);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to create knowledge base: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct FetchKnowledgeBaseBuilder {
    client: WachtClient,
    knowledge_base_id: String,
}

impl FetchKnowledgeBaseBuilder {
    pub fn new(client: WachtClient, knowledge_base_id: &str) -> Self {
        Self {
            client,
            knowledge_base_id: knowledge_base_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<AiKnowledgeBase> {
        let client = self.client.http_client();
        let url = format!(
            "{}/ai/knowledge-bases/{}",
            self.client.config().base_url,
            self.knowledge_base_id
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
                    "Failed to get knowledge base {}: {error_body}",
                    self.knowledge_base_id
                ),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct UpdateKnowledgeBaseBuilder {
    client: WachtClient,
    knowledge_base_id: String,
    request: UpdateAiKnowledgeBaseRequest,
}

impl UpdateKnowledgeBaseBuilder {
    pub fn new(
        client: WachtClient,
        knowledge_base_id: &str,
        request: UpdateAiKnowledgeBaseRequest,
    ) -> Self {
        Self {
            client,
            knowledge_base_id: knowledge_base_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<AiKnowledgeBase> {
        let client = self.client.http_client();
        let url = format!(
            "{}/ai/knowledge-bases/{}",
            self.client.config().base_url,
            self.knowledge_base_id
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
                    "Failed to update knowledge base {}: {error_body}",
                    self.knowledge_base_id
                ),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeleteKnowledgeBaseBuilder {
    client: WachtClient,
    knowledge_base_id: String,
}

impl DeleteKnowledgeBaseBuilder {
    pub fn new(client: WachtClient, knowledge_base_id: &str) -> Self {
        Self {
            client,
            knowledge_base_id: knowledge_base_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/ai/knowledge-bases/{}",
            self.client.config().base_url,
            self.knowledge_base_id
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
                    "Failed to delete knowledge base {}: {error_body}",
                    self.knowledge_base_id
                ),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct FetchDocumentsBuilder {
    client: WachtClient,
    knowledge_base_id: String,
}

impl FetchDocumentsBuilder {
    pub fn new(client: WachtClient, knowledge_base_id: &str) -> Self {
        Self {
            client,
            knowledge_base_id: knowledge_base_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<PaginatedResponse<KnowledgeBaseDocument>> {
        let client = self.client.http_client();
        let url = format!(
            "{}/ai/knowledge-bases/{}/documents",
            self.client.config().base_url,
            self.knowledge_base_id
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
                    "Failed to list documents for knowledge base {}: {error_body}",
                    self.knowledge_base_id
                ),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct UploadDocumentBuilder {
    client: WachtClient,
    knowledge_base_id: String,
    file_content: Vec<u8>,
    file_name: String,
}

impl UploadDocumentBuilder {
    pub fn new(
        client: WachtClient,
        knowledge_base_id: &str,
        file_content: Vec<u8>,
        file_name: String,
    ) -> Self {
        Self {
            client,
            knowledge_base_id: knowledge_base_id.to_string(),
            file_content,
            file_name,
        }
    }

    pub async fn send(self) -> Result<KnowledgeBaseDocument> {
        let client = self.client.http_client();
        let url = format!(
            "{}/ai/knowledge-bases/{}/documents",
            self.client.config().base_url,
            self.knowledge_base_id
        );

        let part = reqwest::multipart::Part::bytes(self.file_content).file_name(self.file_name);
        let form = reqwest::multipart::Form::new().part("file", part);

        let response = client.post(&url).multipart(form).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!(
                    "Failed to upload document to knowledge base {}: {error_body}",
                    self.knowledge_base_id
                ),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeleteDocumentBuilder {
    client: WachtClient,
    knowledge_base_id: String,
    document_id: String,
}

impl DeleteDocumentBuilder {
    pub fn new(client: WachtClient, knowledge_base_id: &str, document_id: &str) -> Self {
        Self {
            client,
            knowledge_base_id: knowledge_base_id.to_string(),
            document_id: document_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/ai/knowledge-bases/{}/documents/{}",
            self.client.config().base_url,
            self.knowledge_base_id,
            self.document_id
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
                    "Failed to delete document {} from knowledge base {}: {error_body}",
                    self.document_id, self.knowledge_base_id
                ),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

pub mod builders {
    pub use super::{
        CreateKnowledgeBaseBuilder, DeleteDocumentBuilder, DeleteKnowledgeBaseBuilder,
        FetchDocumentsBuilder, FetchKnowledgeBaseBuilder, FetchKnowledgeBasesBuilder,
        UpdateKnowledgeBaseBuilder, UploadDocumentBuilder,
    };
}
