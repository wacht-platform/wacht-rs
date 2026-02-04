//! Waitlist Management Module
//!
//! Handles user waitlist management using builder pattern.

use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{PaginatedResponse, WaitlistUser, UserInvitation},
};

/// Builder for fetching users on waitlist
pub struct FetchBuilder;

impl FetchBuilder {
    pub fn new() -> Self {
        Self
    }

    pub async fn send(self) -> Result<PaginatedResponse<WaitlistUser>> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/waitlist", config.base_url);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch waitlist: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Fetch all users on waitlist using builder pattern
pub fn fetch() -> FetchBuilder {
    FetchBuilder::new()
}

/// Builder for approving a waitlist user
pub struct ApproveBuilder {
    waitlist_user_id: String,
}

impl ApproveBuilder {
    pub fn new(waitlist_user_id: &str) -> Self {
        Self {
            waitlist_user_id: waitlist_user_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<UserInvitation> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/waitlist/{}/approve", config.base_url, self.waitlist_user_id);

        let response = client.post(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to approve waitlist user: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Approve a waitlist user and create an invitation using builder pattern
pub fn approve(waitlist_user_id: &str) -> ApproveBuilder {
    ApproveBuilder::new(waitlist_user_id)
}
