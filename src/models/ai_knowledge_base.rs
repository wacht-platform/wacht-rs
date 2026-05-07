use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AiKnowledgeBase {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub deployment_id: String,
    pub configuration: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AiKnowledgeBaseWithDetails {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub deployment_id: String,
    pub configuration: Value,
    pub documents_count: i64,
    pub total_size: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeBaseListResponse {
    pub data: Vec<AiKnowledgeBaseWithDetails>,
    pub has_more: bool,
}
