use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Admin-safe view of a user's passkey. The actual credential bytes are
/// never returned — only descriptive metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPasskey {
    pub id: String,
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,
    pub user_id: String,
    pub name: String,
    pub sign_count: i64,
    #[serde(default)]
    pub transports: Option<Vec<String>>,
    #[serde(default)]
    pub last_used_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub backed_up: Option<bool>,
    #[serde(default)]
    pub device_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RenamePasskeyRequest {
    pub name: String,
}
