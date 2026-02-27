//! Invitation Management Module
//!
//! Handles user invitations for the platform using builder pattern.

use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{InviteUserRequest, PaginatedResponse, UserInvitation},
};

#[derive(Debug, Clone)]
pub struct InvitationsApi {
    client: WachtClient,
}

impl InvitationsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn fetch_users(&self) -> FetchUsersBuilder {
        FetchUsersBuilder::new(self.client.clone())
    }

    pub fn create(&self, request: InviteUserRequest) -> CreateBuilder {
        CreateBuilder::new(self.client.clone(), request)
    }

    pub fn delete(&self, invitation_id: &str) -> DeleteBuilder {
        DeleteBuilder::new(self.client.clone(), invitation_id)
    }
}

/// Builder for fetching invited users
pub struct FetchUsersBuilder {
    client: WachtClient,
}

impl FetchUsersBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub async fn send(self) -> Result<PaginatedResponse<UserInvitation>> {
        let client = self.client.http_client();
        let url = format!("{}/invitations", self.client.config().base_url);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch invited users: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for inviting a new user
pub struct CreateBuilder {
    client: WachtClient,
    request: InviteUserRequest,
}

impl CreateBuilder {
    pub fn new(client: WachtClient, request: InviteUserRequest) -> Self {
        Self { client, request }
    }

    pub async fn send(self) -> Result<UserInvitation> {
        let client = self.client.http_client();
        let url = format!("{}/invitations", self.client.config().base_url);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to invite user: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for deleting an invitation
pub struct DeleteBuilder {
    client: WachtClient,
    invitation_id: String,
}

impl DeleteBuilder {
    pub fn new(client: WachtClient, invitation_id: &str) -> Self {
        Self {
            client,
            invitation_id: invitation_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/invitations/{}",
            self.client.config().base_url,
            self.invitation_id
        );

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete invitation: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}
