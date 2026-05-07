//! Workspace Management Module
//!
//! This module contains all workspace-related functionality including core operations,
//! member management, and role management.

pub mod members;
pub mod roles;

use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{ListOptions, PaginatedResponse, UpdateWorkspaceRequest, Workspace},
};

pub type WorkspaceListResponse = PaginatedResponse<Workspace>;

#[derive(Debug, Clone)]
pub struct WorkspacesApi {
    client: WachtClient,
}

impl WorkspacesApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn fetch_workspaces(&self) -> FetchWorkspacesBuilder {
        FetchWorkspacesBuilder::new(self.client.clone())
    }

    pub fn fetch_workspace(&self, workspace_id: &str) -> FetchWorkspaceBuilder {
        FetchWorkspaceBuilder::new(self.client.clone(), workspace_id)
    }

    pub fn update_workspace(
        &self,
        workspace_id: &str,
        request: UpdateWorkspaceRequest,
    ) -> UpdateWorkspaceBuilder {
        UpdateWorkspaceBuilder::new(self.client.clone(), workspace_id, request)
    }

    pub fn delete_workspace(&self, workspace_id: &str) -> DeleteWorkspaceBuilder {
        DeleteWorkspaceBuilder::new(self.client.clone(), workspace_id)
    }

    pub fn members(&self) -> members::WorkspaceMembersApi {
        members::WorkspaceMembersApi::new(self.client.clone())
    }

    pub fn roles(&self) -> roles::WorkspaceRolesApi {
        roles::WorkspaceRolesApi::new(self.client.clone())
    }
}

/// Builder for fetching workspaces
pub struct FetchWorkspacesBuilder {
    client: WachtClient,
    options: ListOptions,
}

impl FetchWorkspacesBuilder {
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

    pub async fn send(self) -> Result<WorkspaceListResponse> {
        let client = self.client.http_client();
        let url = format!("{}/workspaces", self.client.config().base_url);

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
                "Failed to fetch workspaces",
                &error_body,
            ))
        }
    }
}

/// Builder for fetching a workspace
pub struct FetchWorkspaceBuilder {
    client: WachtClient,
    workspace_id: String,
}

impl FetchWorkspaceBuilder {
    pub fn new(client: WachtClient, workspace_id: &str) -> Self {
        Self {
            client,
            workspace_id: workspace_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<Workspace> {
        let client = self.client.http_client();
        let url = format!(
            "{}/workspaces/{}",
            self.client.config().base_url,
            self.workspace_id
        );

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to fetch workspace",
                &error_body,
            ))
        }
    }
}

/// Builder for updating a workspace
pub struct UpdateWorkspaceBuilder {
    client: WachtClient,
    workspace_id: String,
    request: UpdateWorkspaceRequest,
}

impl UpdateWorkspaceBuilder {
    pub fn new(client: WachtClient, workspace_id: &str, request: UpdateWorkspaceRequest) -> Self {
        Self {
            client,
            workspace_id: workspace_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<Workspace> {
        let client = self.client.http_client();
        let url = format!(
            "{}/workspaces/{}",
            self.client.config().base_url,
            self.workspace_id
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
        if let Some(image_bytes) = &self.request.workspace_image {
            let part = reqwest::multipart::Part::bytes(image_bytes.clone())
                .file_name("workspace_image.jpg")
                .mime_str("image/jpeg")
                .map_err(|e| {
                    Error::InvalidRequest(format!("Failed to create multipart payload: {e}"))
                })?;
            form = form.part("workspace_image", part);
        }

        let response = client.patch(&url).multipart(form).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to update workspace",
                &error_body,
            ))
        }
    }
}

/// Builder for deleting a workspace
pub struct DeleteWorkspaceBuilder {
    client: WachtClient,
    workspace_id: String,
}

impl DeleteWorkspaceBuilder {
    pub fn new(client: WachtClient, workspace_id: &str) -> Self {
        Self {
            client,
            workspace_id: workspace_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/workspaces/{}",
            self.client.config().base_url,
            self.workspace_id
        );

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to delete workspace",
                &error_body,
            ))
        }
    }
}
