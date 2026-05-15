//! User Memberships Module
//!
//! List a user's organization and workspace memberships from the admin view.
//! Each response includes the resolved org/workspace plus the user's roles
//! within it, so callers don't have to chain extra lookups.

use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{PaginatedResponse, UserOrganizationMembership, UserWorkspaceMembership},
};

pub type UserOrganizationMembershipListResponse = PaginatedResponse<UserOrganizationMembership>;
pub type UserWorkspaceMembershipListResponse = PaginatedResponse<UserWorkspaceMembership>;

#[derive(Debug, Clone)]
pub struct UserMembershipsApi {
    client: WachtClient,
}

impl UserMembershipsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn list_organizations(&self, user_id: &str) -> ListUserOrganizationMembershipsBuilder {
        ListUserOrganizationMembershipsBuilder::new(self.client.clone(), user_id)
    }

    pub fn list_workspaces(&self, user_id: &str) -> ListUserWorkspaceMembershipsBuilder {
        ListUserWorkspaceMembershipsBuilder::new(self.client.clone(), user_id)
    }
}

pub struct ListUserOrganizationMembershipsBuilder {
    client: WachtClient,
    user_id: String,
}

impl ListUserOrganizationMembershipsBuilder {
    pub fn new(client: WachtClient, user_id: &str) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<UserOrganizationMembershipListResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/organization-memberships",
            self.client.config().base_url,
            self.user_id
        );
        let response = client.get(&url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to list user organization memberships",
                &error_body,
            ))
        }
    }
}

pub struct ListUserWorkspaceMembershipsBuilder {
    client: WachtClient,
    user_id: String,
}

impl ListUserWorkspaceMembershipsBuilder {
    pub fn new(client: WachtClient, user_id: &str) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<UserWorkspaceMembershipListResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/workspace-memberships",
            self.client.config().base_url,
            self.user_id
        );
        let response = client.get(&url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to list user workspace memberships",
                &error_body,
            ))
        }
    }
}
