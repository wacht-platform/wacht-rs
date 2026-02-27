//! Workspace Roles Module
//!
//! Handles role management within workspaces using builder pattern.

use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{CreateRoleRequest, PaginatedResponse, UpdateRoleRequest, WorkspaceRole},
};

pub type WorkspaceRoleListResponse = PaginatedResponse<WorkspaceRole>;

#[derive(Debug, Clone)]
pub struct WorkspaceRolesApi {
    client: WachtClient,
}

impl WorkspaceRolesApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn fetch_roles(&self, workspace_id: &str) -> FetchRolesBuilder {
        FetchRolesBuilder::new(self.client.clone(), workspace_id)
    }

    pub fn create_role(&self, workspace_id: &str, request: CreateRoleRequest) -> CreateRoleBuilder {
        CreateRoleBuilder::new(self.client.clone(), workspace_id, request)
    }

    pub fn update_role(
        &self,
        workspace_id: &str,
        role_id: &str,
        request: UpdateRoleRequest,
    ) -> UpdateRoleBuilder {
        UpdateRoleBuilder::new(self.client.clone(), workspace_id, role_id, request)
    }

    pub fn delete_role(&self, workspace_id: &str, role_id: &str) -> DeleteRoleBuilder {
        DeleteRoleBuilder::new(self.client.clone(), workspace_id, role_id)
    }
}

/// Builder for fetching workspace roles
pub struct FetchRolesBuilder {
    client: WachtClient,
    workspace_id: String,
}

impl FetchRolesBuilder {
    pub fn new(client: WachtClient, workspace_id: &str) -> Self {
        Self {
            client,
            workspace_id: workspace_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<WorkspaceRoleListResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/workspaces/{}/roles",
            self.client.config().base_url,
            self.workspace_id
        );

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch workspace roles: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for creating workspace role
pub struct CreateRoleBuilder {
    client: WachtClient,
    workspace_id: String,
    request: CreateRoleRequest,
}

impl CreateRoleBuilder {
    pub fn new(client: WachtClient, workspace_id: &str, request: CreateRoleRequest) -> Self {
        Self {
            client,
            workspace_id: workspace_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<WorkspaceRole> {
        let client = self.client.http_client();
        let url = format!(
            "{}/workspaces/{}/roles",
            self.client.config().base_url,
            self.workspace_id
        );

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to create workspace role: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for updating workspace role
pub struct UpdateRoleBuilder {
    client: WachtClient,
    workspace_id: String,
    role_id: String,
    request: UpdateRoleRequest,
}

impl UpdateRoleBuilder {
    pub fn new(client: WachtClient, workspace_id: &str, role_id: &str, request: UpdateRoleRequest) -> Self {
        Self {
            client,
            workspace_id: workspace_id.to_string(),
            role_id: role_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<WorkspaceRole> {
        let client = self.client.http_client();
        let url = format!(
            "{}/workspaces/{}/roles/{}",
            self.client.config().base_url,
            self.workspace_id,
            self.role_id
        );

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update workspace role: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for deleting workspace role
pub struct DeleteRoleBuilder {
    client: WachtClient,
    workspace_id: String,
    role_id: String,
}

impl DeleteRoleBuilder {
    pub fn new(client: WachtClient, workspace_id: &str, role_id: &str) -> Self {
        Self {
            client,
            workspace_id: workspace_id.to_string(),
            role_id: role_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/workspaces/{}/roles/{}",
            self.client.config().base_url,
            self.workspace_id,
            self.role_id
        );

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete workspace role: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}
