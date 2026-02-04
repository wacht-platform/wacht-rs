use serde::{Deserialize, Serialize};
use crate::models::AiToolConfiguration;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAiToolRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub tool_type: String,
    pub configuration: AiToolConfiguration,
}

impl CreateAiToolRequest {
    pub fn new(name: String, tool_type: String, configuration: AiToolConfiguration) -> CreateAiToolRequest {
        CreateAiToolRequest {
            name,
            description: None,
            tool_type,
            configuration,
        }
    }
}

