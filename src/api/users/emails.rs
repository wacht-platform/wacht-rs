//! User Email Management Module
//!
//! Handles email management for users using builder pattern.

use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{AddEmailRequest, UpdateEmailRequest, UserEmail},
};

/// Builder for adding an email to a user
pub struct AddEmailBuilder {
    user_id: String,
    request: AddEmailRequest,
}

impl AddEmailBuilder {
    pub fn new(user_id: &str, request: AddEmailRequest) -> Self {
        Self {
            user_id: user_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<UserEmail> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/users/{}/emails", config.base_url, self.user_id);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to add email: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Add an email to a user using builder pattern
pub fn add_email(user_id: &str, request: AddEmailRequest) -> AddEmailBuilder {
    AddEmailBuilder::new(user_id, request)
}

/// Builder for updating a user email
pub struct UpdateEmailBuilder {
    user_id: String,
    email_id: String,
    request: UpdateEmailRequest,
}

impl UpdateEmailBuilder {
    pub fn new(user_id: &str, email_id: &str, request: UpdateEmailRequest) -> Self {
        Self {
            user_id: user_id.to_string(),
            email_id: email_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<UserEmail> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/users/{}/emails/{}",
            config.base_url, self.user_id, self.email_id
        );

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update email: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Update a user email using builder pattern
pub fn update_email(
    user_id: &str,
    email_id: &str,
    request: UpdateEmailRequest,
) -> UpdateEmailBuilder {
    UpdateEmailBuilder::new(user_id, email_id, request)
}

/// Builder for deleting a user email
pub struct DeleteEmailBuilder {
    user_id: String,
    email_id: String,
}

impl DeleteEmailBuilder {
    pub fn new(user_id: &str, email_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            email_id: email_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/users/{}/emails/{}",
            config.base_url, self.user_id, self.email_id
        );

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete email: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Delete a user email using builder pattern
pub fn delete_email(user_id: &str, email_id: &str) -> DeleteEmailBuilder {
    DeleteEmailBuilder::new(user_id, email_id)
}
