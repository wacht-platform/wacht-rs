use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAiKnowledgeBaseRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Value>,
}

impl CreateAiKnowledgeBaseRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            configuration: None,
        }
    }
}
