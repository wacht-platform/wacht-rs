use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Default, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalAction {
    #[default]
    Allow,
    Deny,
    Review,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentToolApprovalRule {
    pub pattern: String,
    pub action: ApprovalAction,
}

/// Per-agent override for one of the model roles (strong / weak). Any field
/// left `None` falls back to the deployment-level AI settings.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AgentModelOverride {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
}

/// Per-agent runtime limits. Each field falls back to the engine default when
/// `None`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AgentLimits {
    /// Prompt-token count at which conversation history compacts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_window_tokens: Option<u32>,
    /// Cumulative token cap per execution run; exceeding it preempts the run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_token_budget: Option<u64>,
}

/// One tool invocation in a lifecycle hook list. The runtime calls
/// `tool_name` with `args` at the appropriate hook point.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentHookStep {
    pub tool_name: String,
    #[serde(default)]
    pub args: Value,
}

/// Lifecycle hooks an agent runs at well-defined points during an
/// execution. Each list runs in order; empty / omitted lists are no-ops.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AgentHooksConfig {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub execution_start: Vec<AgentHookStep>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub before_llm: Vec<AgentHookStep>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub after_llm: Vec<AgentHookStep>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub before_tool: Vec<AgentHookStep>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub after_tool: Vec<AgentHookStep>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub on_budget_exhausted: Vec<AgentHookStep>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub execution_end: Vec<AgentHookStep>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AiAgent {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub deployment_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_agents: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strong_model: Option<AgentModelOverride>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weak_model: Option<AgentModelOverride>,
    #[serde(default)]
    pub limits: AgentLimits,
    /// Names of built-in (internal) tools disabled for this agent. Empty = all
    /// internal tools enabled.
    #[serde(default)]
    pub disabled_internal_tools: Vec<String>,
    #[serde(default)]
    pub require_approval_mcp: bool,
    #[serde(default)]
    pub require_approval_virtual: bool,
    #[serde(default)]
    pub tool_approval_rules: Vec<AgentToolApprovalRule>,
    #[serde(default)]
    pub hooks: AgentHooksConfig,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AiAgentWithDetails {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub deployment_id: String,
    pub tools_count: i64,
    pub knowledge_bases_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_agents: Option<Vec<String>>,
    /// Designated reviewer agent (None = the agent reviews its own work).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer_agent_id: Option<String>,
    /// Designated conversation agent (None = the agent handles its own chats).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_agent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strong_model: Option<AgentModelOverride>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weak_model: Option<AgentModelOverride>,
    #[serde(default)]
    pub limits: AgentLimits,
    /// Names of built-in (internal) tools disabled for this agent. Empty = all
    /// internal tools enabled.
    #[serde(default)]
    pub disabled_internal_tools: Vec<String>,
    #[serde(default)]
    pub require_approval_mcp: bool,
    #[serde(default)]
    pub require_approval_virtual: bool,
    #[serde(default)]
    pub tool_approval_rules: Vec<AgentToolApprovalRule>,
    #[serde(default)]
    pub hooks: AgentHooksConfig,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentDetailsResponse {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub deployment_id: String,
    pub tools_count: i64,
    pub knowledge_bases_count: i64,
    pub tools: Vec<Value>,
    pub knowledge_bases: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_agents: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer_agent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation_agent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strong_model: Option<AgentModelOverride>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weak_model: Option<AgentModelOverride>,
    #[serde(default)]
    pub limits: AgentLimits,
    #[serde(default)]
    pub disabled_internal_tools: Vec<String>,
    #[serde(default)]
    pub require_approval_mcp: bool,
    #[serde(default)]
    pub require_approval_virtual: bool,
    #[serde(default)]
    pub tool_approval_rules: Vec<AgentToolApprovalRule>,
    #[serde(default)]
    pub hooks: AgentHooksConfig,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateAgentToolApprovalActionRequest {
    pub approval_action: ApprovalAction,
}

/// Request to set (or reset) a role agent for an agent. `agent_id` of `None`
/// resets the role back to the agent itself (the default).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SetAgentRoleAgentRequest {
    /// Which role to set: "reviewer" or "conversation".
    pub role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}
