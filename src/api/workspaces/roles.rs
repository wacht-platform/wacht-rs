//! Workspace Roles Module
//!
//! Handles role management within workspaces using builder pattern.

use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{PaginatedResponse, WorkspaceRole, CreateRoleRequest, UpdateRoleRequest},
};

pub type WorkspaceRoleListResponse = PaginatedResponse<WorkspaceRole>;

/// Builder for fetching workspace roles
pub struct FetchRolesBuilder {
    workspace_id: String,
}

impl FetchRolesBuilder {
    pub fn new(workspace_id: &str) -> Self {
        Self {
            workspace_id: workspace_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<WorkspaceRoleListResponse> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/workspaces/{}/roles", config.base_url, self.workspace_id);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch workspace roles: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Fetch workspace roles using builder pattern
pub fn fetch_roles(workspace_id: &str) -> FetchRolesBuilder {
    FetchRolesBuilder::new(workspace_id)
}

/// Builder for creating workspace role
pub struct CreateRoleBuilder {
    workspace_id: String,
    request: CreateRoleRequest,
}

impl CreateRoleBuilder {
    pub fn new(workspace_id: &str, request: CreateRoleRequest) -> Self {
        Self {
            workspace_id: workspace_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<WorkspaceRole> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/workspaces/{}/roles", config.base_url, self.workspace_id);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to create workspace role: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Create workspace role using builder pattern
pub fn create_role(workspace_id: &str, request: CreateRoleRequest) -> CreateRoleBuilder {
    CreateRoleBuilder::new(workspace_id, request)
}

/// Builder for updating workspace role
pub struct UpdateRoleBuilder {
    workspace_id: String,
    role_id: String,
    request: UpdateRoleRequest,
}

impl UpdateRoleBuilder {
    pub fn new(workspace_id: &str, role_id: &str, request: UpdateRoleRequest) -> Self {
        Self {
            workspace_id: workspace_id.to_string(),
            role_id: role_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<WorkspaceRole> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/workspaces/{}/roles/{}",
            config.base_url, self.workspace_id, self.role_id
        );

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update workspace role: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Update workspace role using builder pattern
pub fn update_role(workspace_id: &str, role_id: &str, request: UpdateRoleRequest) -> UpdateRoleBuilder {
    UpdateRoleBuilder::new(workspace_id, role_id, request)
}

/// Builder for deleting workspace role
pub struct DeleteRoleBuilder {
    workspace_id: String,
    role_id: String,
}

impl DeleteRoleBuilder {
    pub fn new(workspace_id: &str, role_id: &str) -> Self {
        Self {
            workspace_id: workspace_id.to_string(),
            role_id: role_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/workspaces/{}/roles/{}",
            config.base_url, self.workspace_id, self.role_id
        );

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete workspace role: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Delete workspace role using builder pattern
pub fn delete_role(workspace_id: &str, role_id: &str) -> DeleteRoleBuilder {
    DeleteRoleBuilder::new(workspace_id, role_id)
}
