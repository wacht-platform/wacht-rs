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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AiAgent {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub deployment_id: String,
    pub configuration: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_agents: Option<Vec<String>>,
    #[serde(default)]
    pub require_approval_mcp: bool,
    #[serde(default)]
    pub require_approval_virtual: bool,
    #[serde(default)]
    pub tool_approval_rules: Vec<AgentToolApprovalRule>,
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
    pub configuration: Value,
    pub tools_count: i64,
    pub knowledge_bases_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_agents: Option<Vec<String>>,
    #[serde(default)]
    pub require_approval_mcp: bool,
    #[serde(default)]
    pub require_approval_virtual: bool,
    #[serde(default)]
    pub tool_approval_rules: Vec<AgentToolApprovalRule>,
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
    pub configuration: Value,
    pub tools_count: i64,
    pub knowledge_bases_count: i64,
    pub tools: Vec<Value>,
    pub knowledge_bases: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_agents: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateAgentToolApprovalActionRequest {
    pub approval_action: ApprovalAction,
}
