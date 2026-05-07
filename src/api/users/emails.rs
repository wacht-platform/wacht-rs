//! User Email Management Module
//!
//! Handles email management for users using builder pattern.

use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{AddEmailRequest, UpdateEmailRequest, UserEmail},
};

#[derive(Debug, Clone)]
pub struct UserEmailsApi {
    client: WachtClient,
}

impl UserEmailsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn add_email(&self, user_id: &str, request: AddEmailRequest) -> AddEmailBuilder {
        AddEmailBuilder::new(self.client.clone(), user_id, request)
    }

    pub fn update_email(
        &self,
        user_id: &str,
        email_id: &str,
        request: UpdateEmailRequest,
    ) -> UpdateEmailBuilder {
        UpdateEmailBuilder::new(self.client.clone(), user_id, email_id, request)
    }

    pub fn delete_email(&self, user_id: &str, email_id: &str) -> DeleteEmailBuilder {
        DeleteEmailBuilder::new(self.client.clone(), user_id, email_id)
    }
}

/// Builder for adding an email to a user
pub struct AddEmailBuilder {
    client: WachtClient,
    user_id: String,
    request: AddEmailRequest,
}

impl AddEmailBuilder {
    pub fn new(client: WachtClient, user_id: &str, request: AddEmailRequest) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<UserEmail> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/emails",
            self.client.config().base_url,
            self.user_id
        );

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to add email",
                &error_body,
            ))
        }
    }
}

/// Builder for updating a user email
pub struct UpdateEmailBuilder {
    client: WachtClient,
    user_id: String,
    email_id: String,
    request: UpdateEmailRequest,
}

impl UpdateEmailBuilder {
    pub fn new(
        client: WachtClient,
        user_id: &str,
        email_id: &str,
        request: UpdateEmailRequest,
    ) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
            email_id: email_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<UserEmail> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/emails/{}",
            self.client.config().base_url,
            self.user_id,
            self.email_id
        );

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to update email",
                &error_body,
            ))
        }
    }
}

/// Builder for deleting a user email
pub struct DeleteEmailBuilder {
    client: WachtClient,
    user_id: String,
    email_id: String,
}

impl DeleteEmailBuilder {
    pub fn new(client: WachtClient, user_id: &str, email_id: &str) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
            email_id: email_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/emails/{}",
            self.client.config().base_url,
            self.user_id,
            self.email_id
        );

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to delete email",
                &error_body,
            ))
        }
    }
}
