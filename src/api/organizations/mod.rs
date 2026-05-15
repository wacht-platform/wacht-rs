//! Organization Management Module
//!
//! This module contains all organization-related functionality including core operations,
//! member management, and role management.

pub mod invitations;
pub mod members;
pub mod roles;

use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{
        CreateOrganizationRequest, ListOptions, Organization, PaginatedResponse,
        UpdateOrganizationRequest,
    },
};

pub type OrganizationListResponse = PaginatedResponse<Organization>;

#[derive(Debug, Clone)]
pub struct OrganizationsApi {
    client: WachtClient,
}

impl OrganizationsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn fetch_organizations(&self) -> FetchOrganizationsBuilder {
        FetchOrganizationsBuilder::new(self.client.clone())
    }

    pub fn create_organization(
        &self,
        request: CreateOrganizationRequest,
    ) -> CreateOrganizationBuilder {
        CreateOrganizationBuilder::new(self.client.clone(), request)
    }

    pub fn fetch_organization(&self, organization_id: &str) -> FetchOrganizationBuilder {
        FetchOrganizationBuilder::new(self.client.clone(), organization_id)
    }

    pub fn update_organization(
        &self,
        organization_id: &str,
        request: UpdateOrganizationRequest,
    ) -> UpdateOrganizationBuilder {
        UpdateOrganizationBuilder::new(self.client.clone(), organization_id, request)
    }

    pub fn delete_organization(&self, organization_id: &str) -> DeleteOrganizationBuilder {
        DeleteOrganizationBuilder::new(self.client.clone(), organization_id)
    }

    pub fn create_organization_workspace(
        &self,
        organization_id: &str,
        request: crate::models::CreateWorkspaceRequest,
    ) -> CreateOrganizationWorkspaceBuilder {
        CreateOrganizationWorkspaceBuilder::new(self.client.clone(), organization_id, request)
    }

    pub fn members(&self) -> members::OrganizationMembersApi {
        members::OrganizationMembersApi::new(self.client.clone())
    }

    pub fn roles(&self) -> roles::OrganizationRolesApi {
        roles::OrganizationRolesApi::new(self.client.clone())
    }

    pub fn invitations(&self) -> invitations::OrganizationInvitationsApi {
        invitations::OrganizationInvitationsApi::new(self.client.clone())
    }
}

/// Builder for fetching organizations
pub struct FetchOrganizationsBuilder {
    client: WachtClient,
    options: ListOptions,
}

impl FetchOrganizationsBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
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
        let client = self.client.http_client();
        let url = format!("{}/organizations", self.client.config().base_url);

        let mut request = client.get(&url);
        request = request.query(&self.options);

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to fetch organizations",
                &error_body,
            ))
        }
    }
}

/// Builder for creating an organization
pub struct CreateOrganizationBuilder {
    client: WachtClient,
    request: CreateOrganizationRequest,
}

impl CreateOrganizationBuilder {
    pub fn new(client: WachtClient, request: CreateOrganizationRequest) -> Self {
        Self { client, request }
    }

    pub async fn send(self) -> Result<Organization> {
        let client = self.client.http_client();
        let url = format!("{}/organizations", self.client.config().base_url);

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
                .map_err(|e| {
                    Error::InvalidRequest(format!("Failed to create multipart payload: {e}"))
                })?;
            form = form.part("organization_image", part);
        }

        let response = client.post(&url).multipart(form).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to create organization",
                &error_body,
            ))
        }
    }
}

/// Builder for fetching an organization
pub struct FetchOrganizationBuilder {
    client: WachtClient,
    organization_id: String,
}

impl FetchOrganizationBuilder {
    pub fn new(client: WachtClient, organization_id: &str) -> Self {
        Self {
            client,
            organization_id: organization_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<Organization> {
        let client = self.client.http_client();
        let url = format!(
            "{}/organizations/{}",
            self.client.config().base_url,
            self.organization_id
        );

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to fetch organization",
                &error_body,
            ))
        }
    }
}

/// Builder for updating an organization
pub struct UpdateOrganizationBuilder {
    client: WachtClient,
    organization_id: String,
    request: UpdateOrganizationRequest,
}

impl UpdateOrganizationBuilder {
    pub fn new(
        client: WachtClient,
        organization_id: &str,
        request: UpdateOrganizationRequest,
    ) -> Self {
        Self {
            client,
            organization_id: organization_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<Organization> {
        let client = self.client.http_client();
        let url = format!(
            "{}/organizations/{}",
            self.client.config().base_url,
            self.organization_id
        );

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
        if let Some(remove_image) = self.request.remove_image {
            form = form.text("remove_image", remove_image.to_string());
        }
        if let Some(image_bytes) = &self.request.organization_image {
            let part = reqwest::multipart::Part::bytes(image_bytes.clone())
                .file_name("organization_image.jpg")
                .mime_str("image/jpeg")
                .map_err(|e| {
                    Error::InvalidRequest(format!("Failed to create multipart payload: {e}"))
                })?;
            form = form.part("organization_image", part);
        }

        let response = client.patch(&url).multipart(form).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to update organization",
                &error_body,
            ))
        }
    }
}

/// Builder for deleting an organization
pub struct DeleteOrganizationBuilder {
    client: WachtClient,
    organization_id: String,
}

impl DeleteOrganizationBuilder {
    pub fn new(client: WachtClient, organization_id: &str) -> Self {
        Self {
            client,
            organization_id: organization_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/organizations/{}",
            self.client.config().base_url,
            self.organization_id
        );

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to delete organization",
                &error_body,
            ))
        }
    }
}

/// Builder for creating a workspace under an organization
pub struct CreateOrganizationWorkspaceBuilder {
    client: WachtClient,
    organization_id: String,
    request: crate::models::CreateWorkspaceRequest,
}

impl CreateOrganizationWorkspaceBuilder {
    pub fn new(
        client: WachtClient,
        organization_id: &str,
        request: crate::models::CreateWorkspaceRequest,
    ) -> Self {
        Self {
            client,
            organization_id: organization_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<crate::models::Workspace> {
        let client = self.client.http_client();
        let url = format!(
            "{}/organizations/{}/workspaces",
            self.client.config().base_url,
            self.organization_id
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
                .map_err(|e| {
                    Error::InvalidRequest(format!("Failed to create multipart payload: {e}"))
                })?;
            form = form.part("workspace_image", part);
        }

        let response = client.post(&url).multipart(form).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to create workspace under organization",
                &error_body,
            ))
        }
    }
}
