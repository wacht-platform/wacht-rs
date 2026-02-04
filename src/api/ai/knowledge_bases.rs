use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{
        AiKnowledgeBase, CreateAiKnowledgeBaseRequest,
        KnowledgeBaseDocument, PaginatedResponse, UpdateAiKnowledgeBaseRequest,
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
pub struct FetchKnowledgeBasesBuilder {
    options: Option<ListKnowledgeBasesOptions>,
}

impl FetchKnowledgeBasesBuilder {
    pub fn new() -> Self {
        Self { options: None }
    }

    pub fn options(mut self, options: ListKnowledgeBasesOptions) -> Self {
        self.options = Some(options);
        self
    }

    pub async fn send(self) -> Result<PaginatedResponse<AiKnowledgeBase>> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/knowledge-bases", config.base_url);

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
    request: CreateAiKnowledgeBaseRequest,
}

impl CreateKnowledgeBaseBuilder {
    pub fn new(request: CreateAiKnowledgeBaseRequest) -> Self {
        Self { request }
    }

    pub async fn send(self) -> Result<AiKnowledgeBase> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/knowledge-bases", config.base_url);

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
    knowledge_base_id: String,
}

impl FetchKnowledgeBaseBuilder {
    pub fn new(knowledge_base_id: &str) -> Self {
        Self {
            knowledge_base_id: knowledge_base_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<AiKnowledgeBase> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/knowledge-bases/{}", config.base_url, self.knowledge_base_id);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to get knowledge base {}: {error_body}", self.knowledge_base_id),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct UpdateKnowledgeBaseBuilder {
    knowledge_base_id: String,
    request: UpdateAiKnowledgeBaseRequest,
}

impl UpdateKnowledgeBaseBuilder {
    pub fn new(knowledge_base_id: &str, request: UpdateAiKnowledgeBaseRequest) -> Self {
        Self {
            knowledge_base_id: knowledge_base_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<AiKnowledgeBase> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/knowledge-bases/{}", config.base_url, self.knowledge_base_id);

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update knowledge base {}: {error_body}", self.knowledge_base_id),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeleteKnowledgeBaseBuilder {
    knowledge_base_id: String,
}

impl DeleteKnowledgeBaseBuilder {
    pub fn new(knowledge_base_id: &str) -> Self {
        Self {
            knowledge_base_id: knowledge_base_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/knowledge-bases/{}", config.base_url, self.knowledge_base_id);

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete knowledge base {}: {error_body}", self.knowledge_base_id),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct FetchDocumentsBuilder {
    knowledge_base_id: String,
}

impl FetchDocumentsBuilder {
    pub fn new(knowledge_base_id: &str) -> Self {
        Self {
            knowledge_base_id: knowledge_base_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<PaginatedResponse<KnowledgeBaseDocument>> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/knowledge-bases/{}/documents", config.base_url, self.knowledge_base_id);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to list documents for knowledge base {}: {error_body}", self.knowledge_base_id),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct UploadDocumentBuilder {
    knowledge_base_id: String,
    file_content: Vec<u8>,
    file_name: String,
}

impl UploadDocumentBuilder {
    pub fn new(knowledge_base_id: &str, file_content: Vec<u8>, file_name: String) -> Self {
        Self {
            knowledge_base_id: knowledge_base_id.to_string(),
            file_content,
            file_name,
        }
    }

    pub async fn send(self) -> Result<KnowledgeBaseDocument> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/knowledge-bases/{}/documents", config.base_url, self.knowledge_base_id);

        let part = reqwest::multipart::Part::bytes(self.file_content)
            .file_name(self.file_name);

        let form = reqwest::multipart::Form::new()
            .part("file", part);

        let response = client.post(&url)
            .multipart(form)
            .send()
            .await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to upload document to knowledge base {}: {error_body}", self.knowledge_base_id),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeleteDocumentBuilder {
    knowledge_base_id: String,
    document_id: String,
}

impl DeleteDocumentBuilder {
    pub fn new(knowledge_base_id: &str, document_id: &str) -> Self {
        Self {
            knowledge_base_id: knowledge_base_id.to_string(),
            document_id: document_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/knowledge-bases/{}/documents/{}", config.base_url, self.knowledge_base_id, self.document_id);

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete document {} from knowledge base {}: {error_body}", self.document_id, self.knowledge_base_id),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Convenience function for list knowledge bases (deprecated - use builder pattern)
pub async fn fetch_knowledge_bases(options: Option<ListKnowledgeBasesOptions>) -> Result<PaginatedResponse<AiKnowledgeBase>> {
    FetchKnowledgeBasesBuilder::new()
        .options(options.unwrap_or_default())
        .send()
        .await
}

/// Convenience function for create knowledge base (deprecated - use builder pattern)
pub async fn create_knowledge_base(request: CreateAiKnowledgeBaseRequest) -> Result<AiKnowledgeBase> {
    CreateKnowledgeBaseBuilder::new(request).send().await
}

/// Convenience function for fetch knowledge base (deprecated - use builder pattern)
pub async fn fetch_knowledge_base(knowledge_base_id: &str) -> Result<AiKnowledgeBase> {
    FetchKnowledgeBaseBuilder::new(knowledge_base_id).send().await
}

/// Convenience function for update knowledge base (deprecated - use builder pattern)
pub async fn update_knowledge_base(knowledge_base_id: &str, request: UpdateAiKnowledgeBaseRequest) -> Result<AiKnowledgeBase> {
    UpdateKnowledgeBaseBuilder::new(knowledge_base_id, request).send().await
}

/// Convenience function for delete knowledge base (deprecated - use builder pattern)
pub async fn delete_knowledge_base(knowledge_base_id: &str) -> Result<()> {
    DeleteKnowledgeBaseBuilder::new(knowledge_base_id).send().await
}

/// Convenience function for fetch documents (deprecated - use builder pattern)
pub async fn fetch_documents(knowledge_base_id: &str) -> Result<PaginatedResponse<KnowledgeBaseDocument>> {
    FetchDocumentsBuilder::new(knowledge_base_id).send().await
}

/// Convenience function for upload document (deprecated - use builder pattern)
pub async fn upload_document(knowledge_base_id: &str, file_content: Vec<u8>, file_name: String) -> Result<KnowledgeBaseDocument> {
    UploadDocumentBuilder::new(knowledge_base_id, file_content, file_name).send().await
}

/// Convenience function for delete document (deprecated - use builder pattern)
pub async fn delete_document(knowledge_base_id: &str, document_id: &str) -> Result<()> {
    DeleteDocumentBuilder::new(knowledge_base_id, document_id).send().await
}

pub mod builders {
    pub use super::{
        FetchKnowledgeBasesBuilder,
        CreateKnowledgeBaseBuilder,
        FetchKnowledgeBaseBuilder,
        UpdateKnowledgeBaseBuilder,
        DeleteKnowledgeBaseBuilder,
        FetchDocumentsBuilder,
        UploadDocumentBuilder,
        DeleteDocumentBuilder,
    };
}