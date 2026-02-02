use serde::{Deserialize, Serialize};
use crate::models::IntegrationConfig;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAgentIntegrationRequest {
    pub integration_type: String,
    pub name: String,
    pub config: IntegrationConfig,
}
