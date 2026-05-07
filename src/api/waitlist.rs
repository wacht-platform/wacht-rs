//! Waitlist Management Module
//!
//! Handles user waitlist management using builder pattern.

use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{PaginatedResponse, UserInvitation, WaitlistUser},
};

#[derive(Debug, Clone)]
pub struct WaitlistApi {
    client: WachtClient,
}

impl WaitlistApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn fetch(&self) -> FetchBuilder {
        FetchBuilder::new(self.client.clone())
    }

    pub fn approve(&self, waitlist_user_id: &str) -> ApproveBuilder {
        ApproveBuilder::new(self.client.clone(), waitlist_user_id)
    }
}

/// Builder for fetching users on waitlist
pub struct FetchBuilder {
    client: WachtClient,
}

impl FetchBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub async fn send(self) -> Result<PaginatedResponse<WaitlistUser>> {
        let client = self.client.http_client();
        let url = format!("{}/waitlist", self.client.config().base_url);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to fetch waitlist",
                &error_body,
            ))
        }
    }
}

/// Builder for approving a waitlist user
pub struct ApproveBuilder {
    client: WachtClient,
    waitlist_user_id: String,
}

impl ApproveBuilder {
    pub fn new(client: WachtClient, waitlist_user_id: &str) -> Self {
        Self {
            client,
            waitlist_user_id: waitlist_user_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<UserInvitation> {
        let client = self.client.http_client();
        let url = format!(
            "{}/waitlist/{}/approve",
            self.client.config().base_url,
            self.waitlist_user_id
        );

        let response = client.post(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to approve waitlist user",
                &error_body,
            ))
        }
    }
}
