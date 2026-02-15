//! Workspace Members Module
//!
//! Handles member management within workspaces using builder pattern.

use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{ListOptions, PaginatedResponse, WorkspaceMember},
};

pub type WorkspaceMemberListResponse = PaginatedResponse<WorkspaceMember>;

/// Builder for fetching workspace members
pub struct FetchMembersBuilder {
    workspace_id: String,
    options: ListOptions,
}

impl FetchMembersBuilder {
    pub fn new(workspace_id: &str) -> Self {
        Self {
            workspace_id: workspace_id.to_string(),
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

    pub async fn send(self) -> Result<WorkspaceMemberListResponse> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/workspaces/{}/members",
            config.base_url, self.workspace_id
        );

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
                message: format!(
                    "Failed to fetch members for workspace {}: {}",
                    self.workspace_id, error_body
                ),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Fetch workspace members using builder pattern
pub fn fetch_members(workspace_id: &str) -> FetchMembersBuilder {
    FetchMembersBuilder::new(workspace_id)
}

/// Builder for adding a member to workspace
pub struct AddMemberBuilder {
    workspace_id: String,
    user_id: String,
    role_ids: Vec<String>,
}

impl AddMemberBuilder {
    pub fn new(workspace_id: &str, user_id: &str, role_ids: Vec<String>) -> Self {
        Self {
            workspace_id: workspace_id.to_string(),
            user_id: user_id.to_string(),
            role_ids,
        }
    }

    pub async fn send(self) -> Result<WorkspaceMember> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/workspaces/{}/members",
            config.base_url, self.workspace_id
        );

        let payload = serde_json::json!({
            "user_id": self.user_id,
            "role_ids": self.role_ids,
        });

        let response = client.post(&url).json(&payload).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!(
                    "Failed to add member to workspace {}: {}",
                    self.workspace_id, error_body
                ),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Add member to workspace using builder pattern
pub fn add_member(workspace_id: &str, user_id: &str, role_ids: Vec<String>) -> AddMemberBuilder {
    AddMemberBuilder::new(workspace_id, user_id, role_ids)
}

/// Builder for updating workspace member
pub struct UpdateMemberBuilder {
    workspace_id: String,
    membership_id: String,
    role_ids: Option<Vec<String>>,
    public_metadata: Option<serde_json::Value>,
}

impl UpdateMemberBuilder {
    pub fn new(workspace_id: &str, membership_id: &str) -> Self {
        Self {
            workspace_id: workspace_id.to_string(),
            membership_id: membership_id.to_string(),
            role_ids: None,
            public_metadata: None,
        }
    }

    pub fn role_ids(mut self, role_ids: Vec<String>) -> Self {
        self.role_ids = Some(role_ids);
        self
    }

    pub fn public_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.public_metadata = Some(metadata);
        self
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/workspaces/{}/members/{}",
            config.base_url, self.workspace_id, self.membership_id
        );

        let mut payload = serde_json::json!({});

        if let Some(roles) = self.role_ids {
            payload["role_ids"] = serde_json::to_value(roles)?;
        }
        if let Some(metadata) = self.public_metadata {
            payload["public_metadata"] = metadata;
        }

        let response = client.patch(&url).json(&payload).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update workspace member: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Update workspace member using builder pattern
pub fn update_member(workspace_id: &str, membership_id: &str) -> UpdateMemberBuilder {
    UpdateMemberBuilder::new(workspace_id, membership_id)
}

/// Builder for removing workspace member
pub struct RemoveMemberBuilder {
    workspace_id: String,
    membership_id: String,
}

impl RemoveMemberBuilder {
    pub fn new(workspace_id: &str, membership_id: &str) -> Self {
        Self {
            workspace_id: workspace_id.to_string(),
            membership_id: membership_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/workspaces/{}/members/{}",
            config.base_url, self.workspace_id, self.membership_id
        );

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to remove workspace member: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Remove workspace member using builder pattern
pub fn remove_member(workspace_id: &str, membership_id: &str) -> RemoveMemberBuilder {
    RemoveMemberBuilder::new(workspace_id, membership_id)
}
