use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AiKnowledgeBaseDocument {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub file_name: String,
    pub file_size: i64,
    pub file_type: String,
    pub storage_object_key: String,
    pub knowledge_base_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_metadata: Option<Value>,
}
