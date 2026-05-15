use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Admin view of a single active sign-in for a user. Mirrors the `signins`
/// table row with the columns admins typically need (location, browser,
/// device, expiry).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSignin {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub session_id: String,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub active_organization_membership_id: Option<String>,
    #[serde(default)]
    pub active_workspace_membership_id: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
    pub ip_address: String,
    pub browser: String,
    pub device: String,
    pub city: String,
    pub region: String,
    pub region_code: String,
    pub country: String,
    pub country_code: String,
}

/// Returned by `POST /users/{id}/sessions/revoke-all` — count of sign-ins
/// that were actively revoked by this call (excludes already-expired rows).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeAllSigninsResponse {
    pub revoked: i64,
}
