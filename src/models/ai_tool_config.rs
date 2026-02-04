use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AiToolConfiguration {
    Api(ApiToolConfiguration),
    PlatformEvent(PlatformEventToolConfiguration),
    PlatformFunction(PlatformFunctionToolConfiguration),
    Internal(InternalToolConfiguration),
    UseExternalService(UseExternalServiceToolConfiguration),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiToolType {
    Api,
    PlatformEvent,
    PlatformFunction,
    Internal,
    UseExternalService,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiToolConfiguration {
    pub endpoint: String,
    pub method: HttpMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<AuthorizationConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body_schema: Option<Vec<SchemaField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_params_schema: Option<Vec<SchemaField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlatformEventToolConfiguration {
    pub event_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data: Option<serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlatformFunctionToolConfiguration {
    pub function_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<Vec<SchemaField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<Vec<SchemaField>>,
    pub is_overridable: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InternalToolType {
    ReadFile,
    WriteFile,
    ListDirectory,
    SearchFiles,
    ExecuteCommand,
    SaveMemory,
    ExecutePython,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InternalToolConfiguration {
    pub tool_type: InternalToolType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<Vec<SchemaField>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UseExternalServiceToolType {
    TeamsListUsers,
    TeamsSearchUsers,
    TeamsSendDm,
    TeamsSendContextMessage,
    TeamsListMessages,
    TeamsGetMeetingRecording,
    TeamsTranscribeMeeting,
    TeamsSaveAttachment,
    TeamsDescribeImage,
    TeamsTranscribeAudio,
    TeamsListContexts,
    TriggerContext,
    #[serde(rename = "clickup_create_task")]
    ClickUpCreateTask,
    #[serde(rename = "clickup_create_list")]
    ClickUpCreateList,
    #[serde(rename = "clickup_update_task")]
    ClickUpUpdateTask,
    #[serde(rename = "clickup_add_comment")]
    ClickUpAddComment,
    #[serde(rename = "clickup_get_task")]
    ClickUpGetTask,
    #[serde(rename = "clickup_get_space_lists")]
    ClickUpGetSpaceLists,
    #[serde(rename = "clickup_get_spaces")]
    ClickUpGetSpaces,
    #[serde(rename = "clickup_get_teams")]
    ClickUpGetTeams,
    #[serde(rename = "clickup_get_current_user")]
    ClickUpGetCurrentUser,
    #[serde(rename = "clickup_get_tasks")]
    ClickUpGetTasks,
    #[serde(rename = "clickup_search_tasks")]
    ClickUpSearchTasks,
    #[serde(rename = "clickup_task_add_attachment")]
    ClickUpTaskAddAttachment,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UseExternalServiceToolConfiguration {
    pub service_type: UseExternalServiceToolType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<Vec<SchemaField>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationConfiguration {
    pub authorize_as_user: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<Vec<SchemaField>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HttpMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "PATCH")]
    Patch,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SchemaField {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub field_type: String,
    #[serde(default)]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_type: Option<String>,
}

impl Default for ApiToolConfiguration {
    fn default() -> Self {
        Self {
            endpoint: String::new(),
            method: HttpMethod::Get,
            authorization: None,
            request_body_schema: None,
            url_params_schema: None,
            timeout_seconds: Some(30),
        }
    }
}

impl Default for PlatformFunctionToolConfiguration {
    fn default() -> Self {
        Self {
            function_name: String::new(),
            function_description: None,
            input_schema: None,
            output_schema: None,
            is_overridable: true,
        }
    }
}

impl Default for AiToolConfiguration {
    fn default() -> Self {
        Self::Api(ApiToolConfiguration::default())
    }
}
