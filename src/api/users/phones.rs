//! User Phone Management Module
//!
//! Handles phone management for users using builder pattern.

use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{AddPhoneRequest, UpdatePhoneRequest, UserPhone},
};

#[derive(Debug, Clone)]
pub struct UserPhonesApi {
    client: WachtClient,
}

impl UserPhonesApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn add_phone(&self, user_id: &str, request: AddPhoneRequest) -> AddPhoneBuilder {
        AddPhoneBuilder::new(self.client.clone(), user_id, request)
    }

    pub fn update_phone(
        &self,
        user_id: &str,
        phone_id: &str,
        request: UpdatePhoneRequest,
    ) -> UpdatePhoneBuilder {
        UpdatePhoneBuilder::new(self.client.clone(), user_id, phone_id, request)
    }

    pub fn delete_phone(&self, user_id: &str, phone_id: &str) -> DeletePhoneBuilder {
        DeletePhoneBuilder::new(self.client.clone(), user_id, phone_id)
    }
}

/// Builder for adding a phone to a user
pub struct AddPhoneBuilder {
    client: WachtClient,
    user_id: String,
    request: AddPhoneRequest,
}

impl AddPhoneBuilder {
    pub fn new(client: WachtClient, user_id: &str, request: AddPhoneRequest) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<UserPhone> {
        let client = self.client.http_client();
        let url = format!("{}/users/{}/phones", self.client.config().base_url, self.user_id);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to add phone: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for updating a user phone
pub struct UpdatePhoneBuilder {
    client: WachtClient,
    user_id: String,
    phone_id: String,
    request: UpdatePhoneRequest,
}

impl UpdatePhoneBuilder {
    pub fn new(
        client: WachtClient,
        user_id: &str,
        phone_id: &str,
        request: UpdatePhoneRequest,
    ) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
            phone_id: phone_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<UserPhone> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/phones/{}",
            self.client.config().base_url,
            self.user_id,
            self.phone_id
        );

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update phone: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for deleting a user phone
pub struct DeletePhoneBuilder {
    client: WachtClient,
    user_id: String,
    phone_id: String,
}

impl DeletePhoneBuilder {
    pub fn new(client: WachtClient, user_id: &str, phone_id: &str) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
            phone_id: phone_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/phones/{}",
            self.client.config().base_url,
            self.user_id,
            self.phone_id
        );

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete phone: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}
