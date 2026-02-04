use serde::{Deserialize, Serialize};
use crate::models::{IntegrationConfig, TeamsConfig};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAgentIntegrationRequest {
    pub integration_type: String,
    pub name: String,
    pub config: IntegrationConfig,
}

impl Default for CreateAgentIntegrationRequest {
    fn default() -> Self {
        Self {
            integration_type: "teams".to_string(),
            name: "".to_string(),
            config: IntegrationConfig::Teams(TeamsConfig::Bot {
                app_id: None,
                webhook_url: None,
            }),
        }
    }
}
