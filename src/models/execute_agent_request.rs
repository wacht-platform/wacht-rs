use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewMessageRequest {
    pub message: String,
    pub files: Option<Vec<FileData>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInputResponseRequest {
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlatformFunctionResultRequest {
    pub execution_id: String,
    pub result: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CancelRequest {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecuteAgentRequestType {
    pub new_message: Option<NewMessageRequest>,
    pub user_input_response: Option<UserInputResponseRequest>,
    pub platform_function_result: Option<PlatformFunctionResultRequest>,
    pub cancel: Option<CancelRequest>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileData {
    pub name: String,
    pub content: String,
    pub content_type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecuteAgentRequest {
    pub agent_name: Option<String>,
    pub execution_type: ExecuteAgentRequestType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecuteAgentResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
}
