use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{Organization, OrganizationRole, Workspace, WorkspaceRole};

/// Admin view of a user's organization membership — includes the resolved
/// organization + the user's roles within it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOrganizationMembership {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub organization_id: String,
    pub user_id: String,
    #[serde(default)]
    pub public_metadata: HashMap<String, serde_json::Value>,
    pub roles: Vec<OrganizationRole>,
    pub organization: Organization,
}

/// Admin view of a user's workspace membership.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWorkspaceMembership {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub workspace_id: String,
    pub organization_id: String,
    pub organization_membership_id: String,
    pub user_id: String,
    #[serde(default)]
    pub public_metadata: HashMap<String, serde_json::Value>,
    pub roles: Vec<WorkspaceRole>,
    pub workspace: Workspace,
}
