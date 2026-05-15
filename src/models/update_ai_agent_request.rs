use crate::models::{AgentHooksConfig, AgentToolApprovalRule};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateAiAgentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
