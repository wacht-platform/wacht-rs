use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ExecuteAgentRequestType {
    #[serde(rename = "new_message")]
    NewMessage {
        message: String,
        files: Option<Vec<FileData>>,
    },

    #[serde(rename = "user_input_response")]
    UserInputResponse { message: String },

    #[serde(rename = "platform_function_result")]
    PlatformFunctionResult {
        execution_id: String,
        result: serde_json::Value,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileData {
    pub name: String,
    pub content: String,
    pub content_type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecuteAgentRequest {
    pub agent_name: String,
    #[serde(flatten)]
    pub execution_type: ExecuteAgentRequestType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecuteAgentResponse {
    pub status: String,
}
