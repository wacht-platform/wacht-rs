use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Pending invitation to an organization. Soft-deleted rows (either accepted
/// by the user or discarded by an admin) are excluded by default; pass
/// `include_deleted = true` to list to see them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationInvitation {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub organization_id: String,
    pub email: String,
    #[serde(default)]
    pub initial_organization_role_id: Option<String>,
    #[serde(default)]
    pub initial_organization_role_name: Option<String>,
    #[serde(default)]
    pub inviter_id: Option<String>,
    #[serde(default)]
    pub workspace_id: Option<String>,
    #[serde(default)]
    pub workspace_name: Option<String>,
    #[serde(default)]
    pub initial_workspace_role_id: Option<String>,
    #[serde(default)]
    pub initial_workspace_role_name: Option<String>,
    pub expired: bool,
    #[serde(default)]
    pub expiry: Option<DateTime<Utc>>,
    /// Random token used to construct the accept-invitation URL. Treat as a
    /// secret; only surface to admin tooling for out-of-band sharing.
    pub token: String,
}

/// Slim shape returned by the create endpoint. The full row is exposed via
/// the list endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationInvitationSummary {
    pub id: String,
    pub token: String,
    pub email: String,
    pub organization_id: String,
    pub organization_name: String,
    #[serde(default)]
    pub workspace_id: Option<String>,
}

/// Request to create a new organization invitation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateOrganizationInvitationRequest {
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_role_id: Option<String>,
    /// Days before the invitation token expires. Defaults to 10 when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_days: Option<i64>,
}
