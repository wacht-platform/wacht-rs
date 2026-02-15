//! Invitation Management Module
//!
//! Handles user invitations for the platform using builder pattern.

use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{PaginatedResponse, InviteUserRequest, UserInvitation},
};

/// Builder for fetching invited users
pub struct FetchUsersBuilder;

impl FetchUsersBuilder {
    pub fn new() -> Self {
        Self
    }

    pub async fn send(self) -> Result<PaginatedResponse<UserInvitation>> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/invitations", config.base_url);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch invited users: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Fetch all invited users using builder pattern
pub fn fetch_users() -> FetchUsersBuilder {
    FetchUsersBuilder::new()
}

/// Builder for inviting a new user
pub struct CreateBuilder {
    request: InviteUserRequest,
}

impl CreateBuilder {
    pub fn new(request: InviteUserRequest) -> Self {
        Self { request }
    }

    pub async fn send(self) -> Result<UserInvitation> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/invitations", config.base_url);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to invite user: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Invite a new user using builder pattern
pub fn create(request: InviteUserRequest) -> CreateBuilder {
    CreateBuilder::new(request)
}

/// Builder for deleting an invitation
pub struct DeleteBuilder {
    invitation_id: String,
}

impl DeleteBuilder {
    pub fn new(invitation_id: &str) -> Self {
        Self {
        invitation_id: invitation_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/invitations/{}", config.base_url, self.invitation_id);

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete invitation: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Delete an invitation using builder pattern
pub fn delete(invitation_id: &str) -> DeleteBuilder {
    DeleteBuilder::new(invitation_id)
}
