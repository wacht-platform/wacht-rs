use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{
        Workspace, CreateWorkspaceRequest, UpdateWorkspaceRequest,
        WorkspaceRole, CreateRoleRequest, UpdateRoleRequest
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceListResponse {
    pub data: Vec<Workspace>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceRoleListResponse {
    pub data: Vec<WorkspaceRole>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListWorkspacesOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
}

/// Fetch workspaces
pub async fn fetch_workspaces(options: Option<ListWorkspacesOptions>) -> Result<WorkspaceListResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/workspaces", config.base_url);
    
    let mut request = client.get(&url);
    
    if let Some(opts) = options {
        request = request.query(&opts);
    }
    
    let response = request.send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to fetch workspaces: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Fetch workspace
pub async fn fetch_workspace(workspace_id: &str) -> Result<Workspace> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/workspaces/{}", config.base_url, workspace_id);
    
    let response = client.get(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to fetch workspace {}: {}", workspace_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Update workspace
pub async fn update_workspace(workspace_id: &str, request: UpdateWorkspaceRequest) -> Result<Workspace> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/workspaces/{}", config.base_url, workspace_id);
    
    let response = client.patch(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to update workspace {}: {}", workspace_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Delete workspace
pub async fn delete_workspace(workspace_id: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/workspaces/{}", config.base_url, workspace_id);
    
    let response = client.delete(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to delete workspace {}: {}", workspace_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Create workspace in organization
pub async fn create_workspace_in_organization(organization_id: &str, request: CreateWorkspaceRequest) -> Result<Workspace> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/organizations/{}/workspaces", config.base_url, organization_id);
    
    let response = client.post(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to create workspace in organization {}: {}", organization_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Fetch workspace roles
pub async fn fetch_workspace_roles() -> Result<WorkspaceRoleListResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/workspace-roles", config.base_url);
    
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

/// Create workspace role
pub async fn create_workspace_role(workspace_id: &str, request: CreateRoleRequest) -> Result<WorkspaceRole> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/workspaces/{}/roles", config.base_url, workspace_id);
    
    let response = client.post(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to create role for workspace {}: {}", workspace_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Update workspace role
pub async fn update_workspace_role(workspace_id: &str, role_id: &str, request: UpdateRoleRequest) -> Result<WorkspaceRole> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/workspaces/{}/roles/{}", config.base_url, workspace_id, role_id);
    
    let response = client.patch(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to update role {} for workspace {}: {}", role_id, workspace_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Delete workspace role
pub async fn delete_workspace_role(workspace_id: &str, role_id: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/workspaces/{}/roles/{}", config.base_url, workspace_id, role_id);
    
    let response = client.delete(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to delete role {} for workspace {}: {}", role_id, workspace_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}