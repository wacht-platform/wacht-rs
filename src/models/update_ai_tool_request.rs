use serde::{Deserialize, Serialize};
use crate::models::AiToolConfiguration;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateAiToolRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<AiToolConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl UpdateAiToolRequest {
    pub fn new() -> UpdateAiToolRequest {
        UpdateAiToolRequest {
            name: None,
            description: None,
            config: None,
            is_active: None,
        }
    }
}

