use serde::{Deserialize, Serialize};
use crate::models::WorkflowDefinition;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateAiWorkflowRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "trigger_type")]
    pub trigger_type: Option<TriggerType>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "workflow_definition")]
    pub workflow_definition: Option<WorkflowDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

impl UpdateAiWorkflowRequest {
    pub fn new() -> UpdateAiWorkflowRequest {
        UpdateAiWorkflowRequest {
            name: None,
            description: None,
            trigger_type: None,
            workflow_definition: None,
            is_active: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TriggerType {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "webhook")]
    Webhook,
    #[serde(rename = "schedule")]
    Schedule,
    #[serde(rename = "event")]
    Event,
}

impl Default for TriggerType {
    fn default() -> TriggerType {
        Self::Manual
    }
}

