use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    pub nodes: Vec<WorkflowNode>,
    pub edges: Vec<WorkflowEdge>,
    pub version: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowNode {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: WorkflowNodeType,
    pub position: NodePosition,
    pub data: WorkflowNodeData,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodePosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum WorkflowNodeType {
    Trigger(TriggerNodeConfig),
    ErrorHandler(ErrorHandlerNodeConfig),
    LLMCall(LLMCallNodeConfig),
    Switch(SwitchNodeConfig),
    ToolCall(ToolCallNodeConfig),
    UserInput(UserInputNodeConfig),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowNodeData {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub enabled: bool,
    pub config: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_handle: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerNodeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub condition: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorHandlerNodeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub enable_retry: bool,
    pub max_retries: u32,
    pub retry_delay_seconds: u32,
    pub log_errors: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_error_message: Option<String>,
    pub contained_nodes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LLMCallNodeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub prompt_template: String,
    pub response_format: ResponseFormat,
    pub json_schema: Vec<SchemaField>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseFormat {
    Text,
    Json,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchNodeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub switch_condition: String,
    pub cases: Vec<SwitchCase>,
    pub default_case: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchCase {
    pub case_condition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolCallNodeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "tool_id")]
    pub tool_id: String,
    pub input_parameters: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInputNodeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub prompt: String,
    #[serde(rename = "input_type")]
    pub input_type: UserInputType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserInputType {
    Text,
    Number,
    Select,
    MultiSelect,
    Boolean,
    Date,
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

impl Default for WorkflowDefinition {
    fn default() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
            version: "1.0.0".to_string(),
        }
    }
}

impl Default for NodePosition {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
