use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentOrganizationRole {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub permissions: Vec<String>,
    pub deployment_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentWorkspaceRole {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub permissions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    pub deployment_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}
