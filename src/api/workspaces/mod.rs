//! Workspace Management Module
//!
//! This module contains all workspace-related functionality including core operations,
//! member management, and role management.

pub mod members;
pub mod roles;

// Core workspace functions
use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{Workspace, CreateWorkspaceRequest, UpdateWorkspaceRequest, PaginatedResponse, ListOptions},
};

pub type WorkspaceListResponse = PaginatedResponse<Workspace>;

/// Builder for fetching workspaces
pub struct FetchWorkspacesBuilder {
    options: ListOptions,
}

impl FetchWorkspacesBuilder {
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

    pub async fn send(self) -> Result<WorkspaceListResponse> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/workspaces", config.base_url);

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
                message: format!("Failed to fetch workspaces: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// List all workspaces using builder pattern
pub fn fetch_workspaces() -> FetchWorkspacesBuilder {
    FetchWorkspacesBuilder::new()
}

/// Builder for creating a workspace
pub struct CreateWorkspaceBuilder {
    request: CreateWorkspaceRequest,
}

impl CreateWorkspaceBuilder {
    pub fn new(request: CreateWorkspaceRequest) -> Self {
        Self { request }
    }

    pub async fn send(self) -> Result<Workspace> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/workspaces", config.base_url);

        let mut form = reqwest::multipart::Form::new();
        form = form.text("name", self.request.name.clone());
        if let Some(description) = &self.request.description {
            form = form.text("description", description.clone());
        }
        if let Some(public_metadata) = &self.request.public_metadata {
            form = form.text("public_metadata", serde_json::to_string(public_metadata).unwrap_or_default());
        }
        if let Some(private_metadata) = &self.request.private_metadata {
            form = form.text("private_metadata", serde_json::to_string(private_metadata).unwrap_or_default());
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
                message: format!("Failed to create workspace: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Create a new workspace using builder pattern
pub fn create_workspace(request: CreateWorkspaceRequest) -> CreateWorkspaceBuilder {
    CreateWorkspaceBuilder::new(request)
}

/// Builder for fetching a workspace
pub struct FetchWorkspaceBuilder {
    workspace_id: String,
}

impl FetchWorkspaceBuilder {
    pub fn new(workspace_id: &str) -> Self {
        Self {
            workspace_id: workspace_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<Workspace> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/workspaces/{}", config.base_url, self.workspace_id);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch workspace: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Fetch workspace details using builder pattern
pub fn fetch_workspace(workspace_id: &str) -> FetchWorkspaceBuilder {
    FetchWorkspaceBuilder::new(workspace_id)
}

/// Builder for updating a workspace
pub struct UpdateWorkspaceBuilder {
    workspace_id: String,
    request: UpdateWorkspaceRequest,
}

impl UpdateWorkspaceBuilder {
    pub fn new(workspace_id: &str, request: UpdateWorkspaceRequest) -> Self {
        Self {
            workspace_id: workspace_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<Workspace> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/workspaces/{}", config.base_url, self.workspace_id);

        let mut form = reqwest::multipart::Form::new();
        if let Some(name) = &self.request.name {
            form = form.text("name", name.clone());
        }
        if let Some(description) = &self.request.description {
            form = form.text("description", description.clone());
        }
        if let Some(public_metadata) = &self.request.public_metadata {
            form = form.text("public_metadata", serde_json::to_string(public_metadata).unwrap_or_default());
        }
        if let Some(private_metadata) = &self.request.private_metadata {
            form = form.text("private_metadata", serde_json::to_string(private_metadata).unwrap_or_default());
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

        let response = client.patch(&url).multipart(form).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update workspace: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Update a workspace using builder pattern
pub fn update_workspace(workspace_id: &str, request: UpdateWorkspaceRequest) -> UpdateWorkspaceBuilder {
    UpdateWorkspaceBuilder::new(workspace_id, request)
}

/// Builder for deleting a workspace
pub struct DeleteWorkspaceBuilder {
    workspace_id: String,
}

impl DeleteWorkspaceBuilder {
    pub fn new(workspace_id: &str) -> Self {
        Self {
            workspace_id: workspace_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/workspaces/{}", config.base_url, self.workspace_id);

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete workspace: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Delete a workspace using builder pattern
pub fn delete_workspace(workspace_id: &str) -> DeleteWorkspaceBuilder {
    DeleteWorkspaceBuilder::new(workspace_id)
}
