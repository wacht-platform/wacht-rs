//! Organization Roles Module
//!
//! Handles role management within organizations using builder pattern.

use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{CreateRoleRequest, OrganizationRole, PaginatedResponse, UpdateRoleRequest},
};

pub type OrganizationRoleListResponse = PaginatedResponse<OrganizationRole>;

/// Builder for fetching organization roles
pub struct FetchRolesBuilder {
    organization_id: String,
}

impl FetchRolesBuilder {
    pub fn new(organization_id: &str) -> Self {
        Self {
            organization_id: organization_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<OrganizationRoleListResponse> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/organizations/{}/roles",
            config.base_url, self.organization_id
        );

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch organization roles: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Fetch organization roles using builder pattern
pub fn fetch_roles(organization_id: &str) -> FetchRolesBuilder {
    FetchRolesBuilder::new(organization_id)
}

/// Builder for creating organization role
pub struct CreateRoleBuilder {
    organization_id: String,
    request: CreateRoleRequest,
}

impl CreateRoleBuilder {
    pub fn new(organization_id: &str, request: CreateRoleRequest) -> Self {
        Self {
            organization_id: organization_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<OrganizationRole> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/organizations/{}/roles",
            config.base_url, self.organization_id
        );

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to create organization role: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Create organization role using builder pattern
pub fn create_role(organization_id: &str, request: CreateRoleRequest) -> CreateRoleBuilder {
    CreateRoleBuilder::new(organization_id, request)
}

/// Builder for updating organization role
pub struct UpdateRoleBuilder {
    organization_id: String,
    role_id: String,
    request: UpdateRoleRequest,
}

impl UpdateRoleBuilder {
    pub fn new(organization_id: &str, role_id: &str, request: UpdateRoleRequest) -> Self {
        Self {
            organization_id: organization_id.to_string(),
            role_id: role_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<OrganizationRole> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/organizations/{}/roles/{}",
            config.base_url, self.organization_id, self.role_id
        );

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update organization role: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Update organization role using builder pattern
pub fn update_role(
    organization_id: &str,
    role_id: &str,
    request: UpdateRoleRequest,
) -> UpdateRoleBuilder {
    UpdateRoleBuilder::new(organization_id, role_id, request)
}

/// Builder for deleting organization role
pub struct DeleteRoleBuilder {
    organization_id: String,
    role_id: String,
}

impl DeleteRoleBuilder {
    pub fn new(organization_id: &str, role_id: &str) -> Self {
        Self {
            organization_id: organization_id.to_string(),
            role_id: role_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/organizations/{}/roles/{}",
            config.base_url, self.organization_id, self.role_id
        );

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete organization role: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Delete organization role using builder pattern
pub fn delete_role(organization_id: &str, role_id: &str) -> DeleteRoleBuilder {
    DeleteRoleBuilder::new(organization_id, role_id)
}
