use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{
        Organization, CreateOrganizationRequest, UpdateOrganizationRequest,
        OrganizationMember, AddOrganizationMemberRequest, UpdateOrganizationMemberRequest,
        OrganizationRole, CreateRoleRequest, UpdateRoleRequest
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationListResponse {
    pub data: Vec<Organization>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationRoleListResponse {
    pub data: Vec<OrganizationRole>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListOrganizationsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

/// Fetch organizations
pub async fn fetch_organizations(options: Option<ListOrganizationsOptions>) -> Result<OrganizationListResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/organizations", config.base_url);
    
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
            message: format!("Failed to fetch organizations: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Create organization
pub async fn create_organization(request: CreateOrganizationRequest) -> Result<Organization> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/organizations", config.base_url);
    
    let response = client.post(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to create organization: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Fetch organization
pub async fn fetch_organization(organization_id: &str) -> Result<Organization> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/organizations/{}", config.base_url, organization_id);
    
    let response = client.get(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to fetch organization {}: {}", organization_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Update organization
pub async fn update_organization(organization_id: &str, request: UpdateOrganizationRequest) -> Result<Organization> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/organizations/{}", config.base_url, organization_id);
    
    let response = client.patch(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to update organization {}: {}", organization_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Delete organization
pub async fn delete_organization(organization_id: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/organizations/{}", config.base_url, organization_id);
    
    let response = client.delete(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to delete organization {}: {}", organization_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Add organization member
pub async fn add_organization_member(organization_id: &str, request: AddOrganizationMemberRequest) -> Result<OrganizationMember> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/organizations/{}/members", config.base_url, organization_id);
    
    let response = client.post(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to add member to organization {}: {}", organization_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Update organization member
pub async fn update_organization_member(organization_id: &str, membership_id: &str, request: UpdateOrganizationMemberRequest) -> Result<OrganizationMember> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/organizations/{}/members/{}", config.base_url, organization_id, membership_id);
    
    let response = client.patch(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to update member {} in organization {}: {}", membership_id, organization_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Remove organization member
pub async fn remove_organization_member(organization_id: &str, membership_id: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/organizations/{}/members/{}", config.base_url, organization_id, membership_id);
    
    let response = client.delete(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to remove member {} from organization {}: {}", membership_id, organization_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Fetch organization roles
pub async fn fetch_organization_roles() -> Result<OrganizationRoleListResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/organization-roles", config.base_url);
    
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

/// Create organization role
pub async fn create_organization_role(organization_id: &str, request: CreateRoleRequest) -> Result<OrganizationRole> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/organizations/{}/roles", config.base_url, organization_id);
    
    let response = client.post(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to create role for organization {}: {}", organization_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Update organization role
pub async fn update_organization_role(organization_id: &str, role_id: &str, request: UpdateRoleRequest) -> Result<OrganizationRole> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/organizations/{}/roles/{}", config.base_url, organization_id, role_id);
    
    let response = client.patch(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to update role {} for organization {}: {}", role_id, organization_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Delete organization role
pub async fn delete_organization_role(organization_id: &str, role_id: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/organizations/{}/roles/{}", config.base_url, organization_id, role_id);
    
    let response = client.delete(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to delete role {} for organization {}: {}", role_id, organization_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}