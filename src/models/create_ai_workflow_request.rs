use serde::{Deserialize, Serialize};
use crate::models::WorkflowDefinition;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAiWorkflowRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "trigger_type")]
    pub trigger_type: TriggerType,
    #[serde(rename = "workflow_definition")]
    pub workflow_definition: WorkflowDefinition,
}

impl CreateAiWorkflowRequest {
    pub fn new(name: String, trigger_type: TriggerType, workflow_definition: WorkflowDefinition) -> CreateAiWorkflowRequest {
        CreateAiWorkflowRequest {
            name,
            description: None,
            trigger_type,
            workflow_definition,
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

