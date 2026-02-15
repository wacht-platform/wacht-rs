use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Request to update an agent integration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateAgentIntegrationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Integration-specific configuration (flexible JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Value>,
}

impl Default for UpdateAgentIntegrationRequest {
    fn default() -> Self {
        Self {
            name: None,
            config: None,
        }
    }
}
