//! User Phone Management Module
//!
//! Handles phone management for users using builder pattern.

use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{UserPhone, AddPhoneRequest, UpdatePhoneRequest},
};

/// Builder for adding a phone to a user
pub struct AddPhoneBuilder {
    user_id: String,
    request: AddPhoneRequest,
}

impl AddPhoneBuilder {
    pub fn new(user_id: &str, request: AddPhoneRequest) -> Self {
        Self {
            user_id: user_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<UserPhone> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/users/{}/phones", config.base_url, self.user_id);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to add phone: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Add a phone to a user using builder pattern
pub fn add_phone(user_id: &str, request: AddPhoneRequest) -> AddPhoneBuilder {
    AddPhoneBuilder::new(user_id, request)
}

/// Builder for updating a user phone
pub struct UpdatePhoneBuilder {
    user_id: String,
    phone_id: String,
    request: UpdatePhoneRequest,
}

impl UpdatePhoneBuilder {
    pub fn new(user_id: &str, phone_id: &str, request: UpdatePhoneRequest) -> Self {
        Self {
            user_id: user_id.to_string(),
            phone_id: phone_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<UserPhone> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/users/{}/phones/{}", config.base_url, self.user_id, self.phone_id);

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update phone: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Update a user phone using builder pattern
pub fn update_phone(user_id: &str, phone_id: &str, request: UpdatePhoneRequest) -> UpdatePhoneBuilder {
    UpdatePhoneBuilder::new(user_id, phone_id, request)
}

/// Builder for deleting a user phone
pub struct DeletePhoneBuilder {
    user_id: String,
    phone_id: String,
}

impl DeletePhoneBuilder {
    pub fn new(user_id: &str, phone_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            phone_id: phone_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/users/{}/phones/{}", config.base_url, self.user_id, self.phone_id);

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete phone: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Delete a user phone using builder pattern
pub fn delete_phone(user_id: &str, phone_id: &str) -> DeletePhoneBuilder {
    DeletePhoneBuilder::new(user_id, phone_id)
}
