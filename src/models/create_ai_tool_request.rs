use crate::models::AiToolConfiguration;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAiToolRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub tool_type: String,
    #[serde(default)]
    pub requires_user_approval: bool,
    pub configuration: AiToolConfiguration,
}

impl CreateAiToolRequest {
    pub fn new(
        name: impl Into<String>,
        tool_type: impl Into<String>,
        configuration: AiToolConfiguration,
    ) -> Self {
        Self {
            name: name.into(),
            description: None,
            tool_type: tool_type.into(),
            requires_user_approval: false,
            configuration,
        }
    }
}
