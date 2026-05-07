//! User Management Module
//!
//! This module contains all user-related functionality including core user operations,
//! email management, phone management, and social connections.

pub mod emails;
pub mod phones;
pub mod social_connections;

use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{
        CreateUserRequest, ListOptions, PaginatedResponse, UpdatePasswordRequest,
        UpdateUserRequest, User, UserDetails,
    },
};

#[derive(Debug, Clone)]
pub struct UsersApi {
    client: WachtClient,
}

impl UsersApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn fetch_users(&self) -> FetchUsersBuilder {
        FetchUsersBuilder::new(self.client.clone())
    }

    pub fn create_user(&self, request: CreateUserRequest) -> CreateUserBuilder {
        CreateUserBuilder::new(self.client.clone(), request)
    }

    pub fn fetch_user_details(&self, user_id: &str) -> FetchUserDetailsBuilder {
        FetchUserDetailsBuilder::new(self.client.clone(), user_id)
    }

    pub fn update_user(&self, user_id: &str, request: UpdateUserRequest) -> UpdateUserBuilder {
        UpdateUserBuilder::new(self.client.clone(), user_id, request)
    }

    pub fn update_password(
        &self,
        user_id: &str,
        request: UpdatePasswordRequest,
    ) -> UpdatePasswordBuilder {
        UpdatePasswordBuilder::new(self.client.clone(), user_id, request)
    }

    pub fn delete_user(&self, user_id: &str) -> DeleteUserBuilder {
        DeleteUserBuilder::new(self.client.clone(), user_id)
    }

    pub fn emails(&self) -> emails::UserEmailsApi {
        emails::UserEmailsApi::new(self.client.clone())
    }

    pub fn phones(&self) -> phones::UserPhonesApi {
        phones::UserPhonesApi::new(self.client.clone())
    }

    pub fn social_connections(&self) -> social_connections::UserSocialConnectionsApi {
        social_connections::UserSocialConnectionsApi::new(self.client.clone())
    }
}

/// Builder for fetching users
pub struct FetchUsersBuilder {
    client: WachtClient,
    options: ListOptions,
}

impl FetchUsersBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
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

    pub async fn send(self) -> Result<PaginatedResponse<User>> {
        let client = self.client.http_client();
        let url = format!("{}/users", self.client.config().base_url);

        let mut request = client.get(&url);
        request = request.query(&self.options);

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to fetch users",
                &error_body,
            ))
        }
    }
}

/// Builder for creating a user
pub struct CreateUserBuilder {
    client: WachtClient,
    request: CreateUserRequest,
}

impl CreateUserBuilder {
    pub fn new(client: WachtClient, request: CreateUserRequest) -> Self {
        Self { client, request }
    }

    pub async fn send(self) -> Result<User> {
        let client = self.client.http_client();
        let url = format!("{}/users", self.client.config().base_url);

        let mut form = reqwest::multipart::Form::new();
        form = form.text("first_name", self.request.first_name.clone());
        form = form.text("last_name", self.request.last_name.clone());

        if let Some(email) = &self.request.email_address {
            form = form.text("email_address", email.clone());
        }
        if let Some(phone) = &self.request.phone_number {
            form = form.text("phone_number", phone.clone());
        }
        if let Some(username) = &self.request.username {
            form = form.text("username", username.clone());
        }
        if let Some(password) = &self.request.password {
            form = form.text("password", password.clone());
        }
        form = form.text(
            "skip_password_check",
            self.request.skip_password_check.to_string(),
        );
        if let Some(profile_image) = &self.request.profile_image {
            let part = reqwest::multipart::Part::bytes(profile_image.clone())
                .file_name("profile_image.jpg")
                .mime_str("image/jpeg")
                .map_err(|e| {
                    Error::InvalidRequest(format!("Failed to create multipart payload: {e}"))
                })?;
            form = form.part("profile_image", part);
        }

        let response = client.post(&url).multipart(form).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to create user",
                &error_body,
            ))
        }
    }
}

/// Builder for fetching user details
pub struct FetchUserDetailsBuilder {
    client: WachtClient,
    user_id: String,
}

impl FetchUserDetailsBuilder {
    pub fn new(client: WachtClient, user_id: &str) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<UserDetails> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/details",
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
                "Failed to fetch user details",
                &error_body,
            ))
        }
    }
}

/// Builder for updating a user
pub struct UpdateUserBuilder {
    client: WachtClient,
    user_id: String,
    request: UpdateUserRequest,
}

impl UpdateUserBuilder {
    pub fn new(client: WachtClient, user_id: &str, request: UpdateUserRequest) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<User> {
        let client = self.client.http_client();
        let url = format!("{}/users/{}", self.client.config().base_url, self.user_id);

        let mut form = reqwest::multipart::Form::new();

        if let Some(first_name) = &self.request.first_name {
            form = form.text("first_name", first_name.clone());
        }
        if let Some(last_name) = &self.request.last_name {
            form = form.text("last_name", last_name.clone());
        }
        if let Some(username) = &self.request.username {
            form = form.text("username", username.clone());
        }
        if let Some(public_metadata) = &self.request.public_metadata {
            form = form.text(
                "public_metadata",
                serde_json::to_string(public_metadata).unwrap_or_default(),
            );
        }
        if let Some(private_metadata) = &self.request.private_metadata {
            form = form.text(
                "private_metadata",
                serde_json::to_string(private_metadata).unwrap_or_default(),
            );
        }
        if let Some(disabled) = self.request.disabled {
            form = form.text("disabled", disabled.to_string());
        }
        if let Some(remove_profile_image) = self.request.remove_profile_image {
            form = form.text("remove_profile_image", remove_profile_image.to_string());
        }
        if let Some(profile_image) = &self.request.profile_image {
            let part = reqwest::multipart::Part::bytes(profile_image.clone())
                .file_name("profile_image.jpg")
                .mime_str("image/jpeg")
                .map_err(|e| {
                    Error::InvalidRequest(format!("Failed to create multipart payload: {e}"))
                })?;
            form = form.part("profile_image", part);
        }

        let response = client.patch(&url).multipart(form).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to update user",
                &error_body,
            ))
        }
    }
}

/// Builder for updating user password
pub struct UpdatePasswordBuilder {
    client: WachtClient,
    user_id: String,
    request: UpdatePasswordRequest,
}

impl UpdatePasswordBuilder {
    pub fn new(client: WachtClient, user_id: &str, request: UpdatePasswordRequest) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/password",
            self.client.config().base_url,
            self.user_id
        );

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to update password",
                &error_body,
            ))
        }
    }
}

/// Builder for deleting a user
pub struct DeleteUserBuilder {
    client: WachtClient,
    user_id: String,
}

impl DeleteUserBuilder {
    pub fn new(client: WachtClient, user_id: &str) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!("{}/users/{}", self.client.config().base_url, self.user_id);

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to delete user",
                &error_body,
            ))
        }
    }
}
