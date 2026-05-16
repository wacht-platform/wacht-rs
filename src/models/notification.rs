use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    #[serde(rename = "deployment_id")]
    pub deployment_id: String,
    // Null on broadcast / org-scoped / workspace-scoped notifications.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "user_id")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "organization_id")]
    pub organization_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workspace_id")]
    pub workspace_id: Option<String>,
    // Content
    pub title: String,
    pub body: String,
    // Action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctas: Option<Vec<CallToAction>>,
    // Severity
    pub severity: NotificationSeverity,
    // Status
    #[serde(rename = "is_read")]
    pub is_read: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "read_at")]
    pub read_at: Option<String>,
    #[serde(rename = "is_archived")]
    pub is_archived: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "archived_at")]
    pub archived_at: Option<String>,
    // Metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    // Timestamps
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expires_at")]
    pub expires_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum NotificationSeverity {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CallToAction {
    pub label: String,
    pub payload: String,
}
