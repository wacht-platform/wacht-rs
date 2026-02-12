use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Request to create an agent integration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAgentIntegrationRequest {
    pub integration_type: String,
    pub name: String,
    /// Integration-specific configuration (flexible JSON)
    pub config: Value,
}

impl Default for CreateAgentIntegrationRequest {
    fn default() -> Self {
        Self {
            integration_type: "teams".to_string(),
            name: "".to_string(),
            config: serde_json::json!({}),
        }
    }
}
