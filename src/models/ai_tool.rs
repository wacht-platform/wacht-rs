use crate::models::{AiToolConfiguration, AiToolType, ApprovalAction};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AiTool {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub tool_type: AiToolType,
    pub deployment_id: String,
    pub configuration: AiToolConfiguration,
    #[serde(default)]
    pub approval_action: ApprovalAction,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AiToolWithDetails {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub tool_type: AiToolType,
    pub deployment_id: String,
    pub configuration: AiToolConfiguration,
    #[serde(default)]
    pub approval_action: ApprovalAction,
}
