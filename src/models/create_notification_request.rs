use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::models::{CallToAction, NotificationSeverity};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateNotificationRequest {
    #[serde(rename = "user_id")]
    pub user_id: String,
    pub title: String,
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "action_url")]
    pub action_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "action_label")]
    pub action_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctas: Option<Vec<CallToAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expires_hours")]
    pub expires_hours: Option<i64>,
}

impl CreateNotificationRequest {
    pub fn new(user_id: String, title: String, body: String) -> Self {
        Self {
            user_id,
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

    pub fn with_action(mut self, url: String, label: Option<String>) -> Self {
        self.action_url = Some(url);
        self.action_label = label;
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
