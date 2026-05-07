use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAiAgentRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_agents: Option<Vec<String>>,
}

impl CreateAiAgentRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            configuration: None,
            tool_ids: None,
            knowledge_base_ids: None,
            sub_agents: None,
        }
    }
}
