use serde::{Deserialize, Serialize};
use crate::models::IntegrationConfig;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateAgentIntegrationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<IntegrationConfig>,
}
