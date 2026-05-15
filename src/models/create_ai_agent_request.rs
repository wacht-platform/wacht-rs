use crate::models::{AgentHooksConfig, AgentToolApprovalRule};
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_approval_mcp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_approval_virtual: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_approval_rules: Option<Vec<AgentToolApprovalRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<AgentHooksConfig>,
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
            require_approval_mcp: None,
            require_approval_virtual: None,
            tool_approval_rules: None,
            hooks: None,
        }
    }
}
