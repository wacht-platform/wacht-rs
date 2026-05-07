use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateAiKnowledgeBaseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Value>,
}
