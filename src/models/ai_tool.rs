use serde::{Deserialize, Serialize};
use crate::models::{AiToolConfiguration, AiToolType};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AiTool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub tool_type: Option<AiToolType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<AiToolConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl AiTool {
    pub fn new() -> AiTool {
        AiTool {
            id: None,
            name: None,
            description: None,
            tool_type: None,
            config: None,
            is_active: None,
            created_at: None,
            updated_at: None,
        }
    }
}


