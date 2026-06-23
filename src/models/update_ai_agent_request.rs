use crate::models::{AgentHooksConfig, AgentLimits, AgentModelOverride, AgentToolApprovalRule};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateAiAgentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_agents: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strong_model: Option<AgentModelOverride>,
    /// Clear the strong-model override (reset to deployment default).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub clear_strong_model: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_model: Option<AgentModelOverride>,
    /// Clear the weak-model override (reset to deployment default).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub clear_weak_model: bool,
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
