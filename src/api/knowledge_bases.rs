use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{
        AiKnowledgeBase, CreateAiKnowledgeBaseRequest, UpdateAiKnowledgeBaseRequest,
        KnowledgeBaseDocument, KnowledgeBaseSearchResult
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBaseListResponse {
    pub data: Vec<AiKnowledgeBase>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentListResponse {
    pub data: Vec<KnowledgeBaseDocument>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListKnowledgeBasesOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct KnowledgeBaseSearchOptions {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f32>,
}

/// List all knowledge bases
pub async fn fetch_knowledge_bases(options: Option<ListKnowledgeBasesOptions>) -> Result<KnowledgeBaseListResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-knowledge-bases", config.base_url);
    
    let mut request = client.get(&url);
    
    if let Some(opts) = options {
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
            message: format!("Failed to list knowledge bases: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Create a new knowledge base
pub async fn create_knowledge_base(request: CreateAiKnowledgeBaseRequest) -> Result<AiKnowledgeBase> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-knowledge-bases", config.base_url);
    
    let response = client.post(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to create knowledge base: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Get a specific knowledge base by ID
pub async fn fetch_knowledge_base(knowledge_base_id: &str) -> Result<AiKnowledgeBase> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-knowledge-bases/{}", config.base_url, knowledge_base_id);
    
    let response = client.get(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to get knowledge base {}: {}", knowledge_base_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Update a knowledge base
pub async fn update_knowledge_base(knowledge_base_id: &str, request: UpdateAiKnowledgeBaseRequest) -> Result<AiKnowledgeBase> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-knowledge-bases/{}", config.base_url, knowledge_base_id);
    
    let response = client.patch(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to update knowledge base {}: {}", knowledge_base_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Delete a knowledge base
pub async fn delete_knowledge_base(knowledge_base_id: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-knowledge-bases/{}", config.base_url, knowledge_base_id);
    
    let response = client.delete(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to delete knowledge base {}: {}", knowledge_base_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Get documents in a knowledge base
pub async fn fetch_documents(knowledge_base_id: &str) -> Result<DocumentListResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-knowledge-bases/{}/documents", config.base_url, knowledge_base_id);
    
    let response = client.get(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to list documents for knowledge base {}: {}", knowledge_base_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Upload a document to a knowledge base
pub async fn upload_document(knowledge_base_id: &str, file_content: Vec<u8>, file_name: String) -> Result<KnowledgeBaseDocument> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-knowledge-bases/{}/documents", config.base_url, knowledge_base_id);
    
    let part = reqwest::multipart::Part::bytes(file_content)
        .file_name(file_name);
    
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
            message: format!("Failed to upload document to knowledge base {}: {}", knowledge_base_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Delete a document from a knowledge base
pub async fn delete_document(knowledge_base_id: &str, document_id: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-knowledge-bases/{}/documents/{}", config.base_url, knowledge_base_id, document_id);
    
    let response = client.delete(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to delete document {} from knowledge base {}: {}", document_id, knowledge_base_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Search across all knowledge bases
pub async fn search_global(options: KnowledgeBaseSearchOptions) -> Result<KnowledgeBaseSearchResult> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-knowledge-bases/search", config.base_url);
    
    let response = client.get(&url)
        .query(&options)
        .send()
        .await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to search knowledge bases: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Search within a specific knowledge base
pub async fn search_knowledge_base(knowledge_base_id: &str, options: KnowledgeBaseSearchOptions) -> Result<KnowledgeBaseSearchResult> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai-knowledge-bases/{}/search", config.base_url, knowledge_base_id);
    
    let response = client.get(&url)
        .query(&options)
        .send()
        .await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to search knowledge base {}: {}", knowledge_base_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}