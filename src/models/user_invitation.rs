use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInvitation {
    pub id: String,
    pub deployment_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub token: String,
    pub expiry: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
