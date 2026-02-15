//! Organization Management Module
//!
//! This module contains all organization-related functionality including core operations,
//! member management, and role management.

pub mod members;
pub mod roles;

// Core organization functions
use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{
        CreateOrganizationRequest, ListOptions, Organization, PaginatedResponse,
        UpdateOrganizationRequest,
    },
};

pub type OrganizationListResponse = PaginatedResponse<Organization>;

/// Builder for fetching organizations
pub struct FetchOrganizationsBuilder {
    options: ListOptions,
}

impl FetchOrganizationsBuilder {
    pub fn new() -> Self {
        Self {
            options: ListOptions::default(),
        }
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.options.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.options.offset = Some(offset);
        self
    }

    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.options.search = Some(search.into());
        self
    }

    pub fn sort_key(mut self, sort_key: impl Into<String>) -> Self {
        self.options.sort_key = Some(sort_key.into());
        self
    }

    pub fn sort_order(mut self, sort_order: impl Into<String>) -> Self {
        self.options.sort_order = Some(sort_order.into());
        self
    }

    pub async fn send(self) -> Result<OrganizationListResponse> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/organizations", config.base_url);

        let mut request = client.get(&url);
        request = request.query(&self.options);

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
}

/// List all organizations using builder pattern
pub fn fetch_organizations() -> FetchOrganizationsBuilder {
    FetchOrganizationsBuilder::new()
}

/// Builder for creating an organization
pub struct CreateOrganizationBuilder {
    request: CreateOrganizationRequest,
}

impl CreateOrganizationBuilder {
    pub fn new(request: CreateOrganizationRequest) -> Self {
        Self { request }
    }

    pub async fn send(self) -> Result<Organization> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/organizations", config.base_url);

        let mut form = reqwest::multipart::Form::new();
        form = form.text("name", self.request.name.clone());
        if let Some(description) = &self.request.description {
            form = form.text("description", description.clone());
        }
        if let Some(public_metadata) = &self.request.public_metadata {
            form = form.text(
                "public_metadata",
                serde_json::to_string(public_metadata).unwrap_or_default(),
            );
        }
        if let Some(private_metadata) = &self.request.private_metadata {
            form = form.text(
                "private_metadata",
                serde_json::to_string(private_metadata).unwrap_or_default(),
            );
        }
        if let Some(image_bytes) = &self.request.organization_image {
            let part = reqwest::multipart::Part::bytes(image_bytes.clone())
                .file_name("organization_image.jpg")
                .mime_str("image/jpeg")
                .map_err(|e| Error::Api {
                    status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("Failed to create multipart: {}", e),
                    details: None,
                })?;
            form = form.part("organization_image", part);
        }

        let response = client.post(&url).multipart(form).send().await?;
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
}

/// Create a new organization using builder pattern
pub fn create_organization(request: CreateOrganizationRequest) -> CreateOrganizationBuilder {
    CreateOrganizationBuilder::new(request)
}

/// Builder for fetching an organization
pub struct FetchOrganizationBuilder {
    organization_id: String,
}

impl FetchOrganizationBuilder {
    pub fn new(organization_id: &str) -> Self {
        Self {
            organization_id: organization_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<Organization> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/organizations/{}", config.base_url, self.organization_id);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch organization: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Fetch organization details using builder pattern
pub fn fetch_organization(organization_id: &str) -> FetchOrganizationBuilder {
    FetchOrganizationBuilder::new(organization_id)
}

/// Builder for updating an organization
pub struct UpdateOrganizationBuilder {
    organization_id: String,
    request: UpdateOrganizationRequest,
}

impl UpdateOrganizationBuilder {
    pub fn new(organization_id: &str, request: UpdateOrganizationRequest) -> Self {
        Self {
            organization_id: organization_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<Organization> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/organizations/{}", config.base_url, self.organization_id);

        let mut form = reqwest::multipart::Form::new();
        if let Some(name) = &self.request.name {
            form = form.text("name", name.clone());
        }
        if let Some(description) = &self.request.description {
            form = form.text("description", description.clone());
        }
        if let Some(public_metadata) = &self.request.public_metadata {
            form = form.text(
                "public_metadata",
                serde_json::to_string(public_metadata).unwrap_or_default(),
            );
        }
        if let Some(private_metadata) = &self.request.private_metadata {
            form = form.text(
                "private_metadata",
                serde_json::to_string(private_metadata).unwrap_or_default(),
            );
        }
        if let Some(image_bytes) = &self.request.organization_image {
            let part = reqwest::multipart::Part::bytes(image_bytes.clone())
                .file_name("organization_image.jpg")
                .mime_str("image/jpeg")
                .map_err(|e| Error::Api {
                    status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("Failed to create multipart: {}", e),
                    details: None,
                })?;
            form = form.part("organization_image", part);
        }

        let response = client.patch(&url).multipart(form).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update organization: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Update an organization using builder pattern
pub fn update_organization(
    organization_id: &str,
    request: UpdateOrganizationRequest,
) -> UpdateOrganizationBuilder {
    UpdateOrganizationBuilder::new(organization_id, request)
}

/// Builder for deleting an organization
pub struct DeleteOrganizationBuilder {
    organization_id: String,
}

impl DeleteOrganizationBuilder {
    pub fn new(organization_id: &str) -> Self {
        Self {
            organization_id: organization_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/organizations/{}", config.base_url, self.organization_id);

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete organization: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Delete an organization using builder pattern
pub fn delete_organization(organization_id: &str) -> DeleteOrganizationBuilder {
    DeleteOrganizationBuilder::new(organization_id)
}

/// Builder for creating a workspace under an organization
pub struct CreateOrganizationWorkspaceBuilder {
    organization_id: String,
    request: crate::models::CreateWorkspaceRequest,
}

impl CreateOrganizationWorkspaceBuilder {
    pub fn new(organization_id: &str, request: crate::models::CreateWorkspaceRequest) -> Self {
        Self {
            organization_id: organization_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<crate::models::Workspace> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/organizations/{}/workspaces",
            config.base_url, self.organization_id
        );

        let mut form = reqwest::multipart::Form::new();
        form = form.text("name", self.request.name.clone());
        if let Some(description) = &self.request.description {
            form = form.text("description", description.clone());
        }
        if let Some(public_metadata) = &self.request.public_metadata {
            form = form.text(
                "public_metadata",
                serde_json::to_string(public_metadata).unwrap_or_default(),
            );
        }
        if let Some(private_metadata) = &self.request.private_metadata {
            form = form.text(
                "private_metadata",
                serde_json::to_string(private_metadata).unwrap_or_default(),
            );
        }
        if let Some(image_bytes) = &self.request.workspace_image {
            let part = reqwest::multipart::Part::bytes(image_bytes.clone())
                .file_name("workspace_image.jpg")
                .mime_str("image/jpeg")
                .map_err(|e| Error::Api {
                    status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("Failed to create multipart: {}", e),
                    details: None,
                })?;
            form = form.part("workspace_image", part);
        }

        let response = client.post(&url).multipart(form).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!(
                    "Failed to create workspace under organization: {}",
                    error_body
                ),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Create a workspace under an organization using builder pattern
pub fn create_organization_workspace(
    organization_id: &str,
    request: crate::models::CreateWorkspaceRequest,
) -> CreateOrganizationWorkspaceBuilder {
    CreateOrganizationWorkspaceBuilder::new(organization_id, request)
}
