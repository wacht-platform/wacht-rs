use serde::{Deserialize, Serialize};

/// Integration type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IntegrationType {
    Teams,
    Slack,
    WhatsApp,
    Discord,
    ClickUp,
}

/// Agent integration model
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentIntegration {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub deployment_id: String,
    pub agent_id: String,
    pub integration_type: IntegrationType,
    pub name: String,
    /// Integration-specific configuration (flexible JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}
