use crate::models::{CallToAction, NotificationSeverity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request to create a notification
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateNotificationRequest {
    /// Single user ID (deprecated in favor of user_ids)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Multiple user IDs for bulk notification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// Organization ID for org-wide notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// Workspace ID for workspace-wide notifications
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    pub title: String,
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctas: Option<Vec<CallToAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Expiration time in hours
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_hours: Option<i64>,
}

impl CreateNotificationRequest {
    pub fn new(title: String, body: String) -> Self {
        Self {
            user_id: None,
            user_ids: None,
            organization_id: None,
            workspace_id: None,
            title,
            body,
            action_url: None,
            action_label: None,
            ctas: None,
            severity: None,
            metadata: None,
            expires_hours: None,
        }
    }

    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    pub fn with_organization_id(mut self, organization_id: String) -> Self {
        self.organization_id = Some(organization_id);
        self
    }

    pub fn with_workspace_id(mut self, workspace_id: String) -> Self {
        self.workspace_id = Some(workspace_id);
        self
    }

    pub fn with_action_url(mut self, action_url: String) -> Self {
        self.action_url = Some(action_url);
        self
    }

    pub fn with_action_label(mut self, action_label: String) -> Self {
        self.action_label = Some(action_label);
        self
    }

    pub fn with_ctas(mut self, ctas: Vec<CallToAction>) -> Self {
        self.ctas = Some(ctas);
        self
    }

    pub fn with_severity(mut self, severity: NotificationSeverity) -> Self {
        self.severity = Some(match severity {
            NotificationSeverity::Info => "info".to_string(),
            NotificationSeverity::Success => "success".to_string(),
            NotificationSeverity::Warning => "warning".to_string(),
            NotificationSeverity::Error => "error".to_string(),
        });
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn with_expires_hours(mut self, hours: i64) -> Self {
        self.expires_hours = Some(hours);
        self
    }
}
