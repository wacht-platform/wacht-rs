use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileData {
    pub filename: String,
    pub mime_type: String,
    pub data: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewMessageRequest {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<FileData>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolApprovalSelection {
    pub tool_name: String,
    pub mode: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApprovalResponseRequest {
    pub request_message_id: String,
    pub approvals: Vec<ToolApprovalSelection>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CancelRequest {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecuteAgentRequestType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_message: Option<NewMessageRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_response: Option<ApprovalResponseRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel: Option<CancelRequest>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecuteAgentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    pub execution_type: ExecuteAgentRequestType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecuteAgentResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
}
