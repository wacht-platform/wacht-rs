use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AiToolConfiguration {
    Api(ApiToolConfiguration),
    PlatformEvent(PlatformEventToolConfiguration),
    CodeRunner(CodeRunnerToolConfiguration),
    Internal(InternalToolConfiguration),
    UseExternalService(UseExternalServiceToolConfiguration),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiToolType {
    Api,
    PlatformEvent,
    CodeRunner,
    Internal,
    UseExternalService,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiToolConfiguration {
    pub endpoint: String,
    #[serde(default)]
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
    pub event_data: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeRunnerRuntime {
    Python,
}

impl Default for CodeRunnerRuntime {
    fn default() -> Self {
        Self::Python
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeRunnerToolConfiguration {
    #[serde(default)]
    pub runtime: CodeRunnerRuntime,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<Vec<SchemaField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<Vec<SchemaField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_variables: Option<Vec<CodeRunnerEnvVariable>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u32>,
    #[serde(default)]
    pub allow_network: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeRunnerEnvVariable {
    pub name: String,
    pub value: String,
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
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl Default for HttpMethod {
    fn default() -> Self {
        Self::Get
    }
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
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_schema: Option<Box<SchemaField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<SchemaField>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InternalToolType {
    ReadImage,
    ReadFile,
    WriteFile,
    EditFile,
    ExecuteCommand,
    Sleep,
    SnapshotExecutionState,
    WebSearch,
    UrlContent,
    SearchKnowledgebase,
    LoadMemory,
    SearchTools,
    LoadTools,
    CreateProjectTask,
    UpdateProjectTask,
    AssignProjectTask,
    AppendTaskJournal,
    ListThreads,
    CreateThread,
    UpdateThread,
    SaveMemory,
    TaskGraphAddNode,
    TaskGraphAddDependency,
    TaskGraphMarkInProgress,
    TaskGraphCompleteNode,
    TaskGraphFailNode,
    TaskGraphMarkCompleted,
    TaskGraphMarkFailed,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InternalToolConfiguration {
    pub tool_type: InternalToolType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<Vec<SchemaField>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UseExternalServiceToolConfiguration {
    pub service_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<Vec<SchemaField>>,
}

impl Default for ApiToolConfiguration {
    fn default() -> Self {
        Self {
            endpoint: String::new(),
            method: HttpMethod::Get,
            authorization: None,
            request_body_schema: None,
            url_params_schema: None,
            timeout_seconds: None,
        }
    }
}

impl Default for AiToolConfiguration {
    fn default() -> Self {
        Self::Api(ApiToolConfiguration::default())
    }
}
