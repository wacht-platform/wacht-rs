use serde::{Deserialize, Serialize};
use crate::models::WorkflowDefinition;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AiWorkflow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<WorkflowConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "workflow_definition")]
    pub workflow_definition: Option<WorkflowDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl AiWorkflow {
    pub fn new() -> AiWorkflow {
        AiWorkflow {
            id: None,
            name: None,
            description: None,
            configuration: None,
            workflow_definition: None,
            is_active: None,
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_delay_seconds: Option<u32>,
    pub enable_logging: bool,
    pub enable_metrics: bool,
}

impl Default for WorkflowConfiguration {
    fn default() -> Self {
        Self {
            timeout_seconds: Some(300),
            max_retries: Some(3),
            retry_delay_seconds: Some(5),
            enable_logging: true,
            enable_metrics: true,
        }
    }
}


