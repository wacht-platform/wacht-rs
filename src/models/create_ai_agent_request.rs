use crate::models::{AgentHooksConfig, AgentLimits, AgentModelOverride, AgentToolApprovalRule};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAiAgentRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_agents: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strong_model: Option<AgentModelOverride>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_model: Option<AgentModelOverride>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<AgentLimits>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_approval_mcp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_approval_virtual: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_approval_rules: Option<Vec<AgentToolApprovalRule>>,
    /// Built-in tool names to disable for this agent (empty = all enabled).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_internal_tools: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<AgentHooksConfig>,
}

impl CreateAiAgentRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            tool_ids: None,
            knowledge_base_ids: None,
            sub_agents: None,
            strong_model: None,
            weak_model: None,
            limits: None,
            require_approval_mcp: None,
            require_approval_virtual: None,
            tool_approval_rules: None,
            disabled_internal_tools: None,
            hooks: None,
        }
    }
}
