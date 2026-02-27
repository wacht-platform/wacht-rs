//! Organization Roles Module
//!
//! Handles role management within organizations using builder pattern.

use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{CreateRoleRequest, OrganizationRole, PaginatedResponse, UpdateRoleRequest},
};

pub type OrganizationRoleListResponse = PaginatedResponse<OrganizationRole>;

#[derive(Debug, Clone)]
pub struct OrganizationRolesApi {
    client: WachtClient,
}

impl OrganizationRolesApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn fetch_roles(&self, organization_id: &str) -> FetchRolesBuilder {
        FetchRolesBuilder::new(self.client.clone(), organization_id)
    }

    pub fn create_role(&self, organization_id: &str, request: CreateRoleRequest) -> CreateRoleBuilder {
        CreateRoleBuilder::new(self.client.clone(), organization_id, request)
    }

    pub fn update_role(
        &self,
        organization_id: &str,
        role_id: &str,
        request: UpdateRoleRequest,
    ) -> UpdateRoleBuilder {
        UpdateRoleBuilder::new(self.client.clone(), organization_id, role_id, request)
    }

    pub fn delete_role(&self, organization_id: &str, role_id: &str) -> DeleteRoleBuilder {
        DeleteRoleBuilder::new(self.client.clone(), organization_id, role_id)
    }
}

/// Builder for fetching organization roles
pub struct FetchRolesBuilder {
    client: WachtClient,
    organization_id: String,
}

impl FetchRolesBuilder {
    pub fn new(client: WachtClient, organization_id: &str) -> Self {
        Self {
            client,
            organization_id: organization_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<OrganizationRoleListResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/organizations/{}/roles",
            self.client.config().base_url,
            self.organization_id
        );

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch organization roles: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for creating organization role
pub struct CreateRoleBuilder {
    client: WachtClient,
    organization_id: String,
    request: CreateRoleRequest,
}

impl CreateRoleBuilder {
    pub fn new(client: WachtClient, organization_id: &str, request: CreateRoleRequest) -> Self {
        Self {
            client,
            organization_id: organization_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<OrganizationRole> {
        let client = self.client.http_client();
        let url = format!(
            "{}/organizations/{}/roles",
            self.client.config().base_url,
            self.organization_id
        );

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to create organization role: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for updating organization role
pub struct UpdateRoleBuilder {
    client: WachtClient,
    organization_id: String,
    role_id: String,
    request: UpdateRoleRequest,
}

impl UpdateRoleBuilder {
    pub fn new(
        client: WachtClient,
        organization_id: &str,
        role_id: &str,
        request: UpdateRoleRequest,
    ) -> Self {
        Self {
            client,
            organization_id: organization_id.to_string(),
            role_id: role_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<OrganizationRole> {
        let client = self.client.http_client();
        let url = format!(
            "{}/organizations/{}/roles/{}",
            self.client.config().base_url,
            self.organization_id,
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
                message: format!("Failed to update organization role: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for deleting organization role
pub struct DeleteRoleBuilder {
    client: WachtClient,
    organization_id: String,
    role_id: String,
}

impl DeleteRoleBuilder {
    pub fn new(client: WachtClient, organization_id: &str, role_id: &str) -> Self {
        Self {
            client,
            organization_id: organization_id.to_string(),
            role_id: role_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/organizations/{}/roles/{}",
            self.client.config().base_url,
            self.organization_id,
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
                message: format!("Failed to delete organization role: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}
