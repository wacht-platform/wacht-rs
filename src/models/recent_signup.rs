use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Recent signup from analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentSignup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    pub date: DateTime<Utc>,
}
