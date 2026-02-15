//! User Management Module
//!
//! This module contains all user-related functionality including core user operations,
//! email management, phone management, and social connections.

pub mod emails;
pub mod phones;
pub mod social_connections;

// Core user functions
use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{
        CreateUserRequest, ListOptions, PaginatedResponse, UpdatePasswordRequest,
        UpdateUserRequest, User, UserDetails,
    },
};

/// Builder for fetching users
pub struct FetchUsersBuilder {
    options: ListOptions,
}

impl FetchUsersBuilder {
    pub fn new() -> Self {
        Self {
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
        let config = get_config();
        let client = get_client();
        let url = format!("{}/users", config.base_url);

        let mut request = client.get(&url);
        request = request.query(&self.options);

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch users: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Fetch all users using builder pattern
pub fn fetch_users() -> FetchUsersBuilder {
    FetchUsersBuilder::new()
}

/// Builder for creating a user
pub struct CreateUserBuilder {
    request: CreateUserRequest,
}

impl CreateUserBuilder {
    pub fn new(request: CreateUserRequest) -> Self {
        Self { request }
    }

    pub async fn send(self) -> Result<User> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/users", config.base_url);

        // Convert request to multipart form
        let mut form = reqwest::multipart::Form::new();

        // Add text fields
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

        let response = client.post(&url).multipart(form).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to create user: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Create a new user using builder pattern
pub fn create_user(request: CreateUserRequest) -> CreateUserBuilder {
    CreateUserBuilder::new(request)
}

/// Builder for fetching user details
pub struct FetchUserDetailsBuilder {
    user_id: String,
}

impl FetchUserDetailsBuilder {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<UserDetails> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/users/{}/details", config.base_url, self.user_id);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch user details: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Fetch detailed information about a user using builder pattern
pub fn fetch_user_details(user_id: &str) -> FetchUserDetailsBuilder {
    FetchUserDetailsBuilder::new(user_id)
}

/// Builder for updating a user
pub struct UpdateUserBuilder {
    user_id: String,
    request: UpdateUserRequest,
}

impl UpdateUserBuilder {
    pub fn new(user_id: &str, request: UpdateUserRequest) -> Self {
        Self {
            user_id: user_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<User> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/users/{}", config.base_url, self.user_id);

        // Convert request to multipart form
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

        let response = client.patch(&url).multipart(form).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update user: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Update a user using builder pattern
pub fn update_user(user_id: &str, request: UpdateUserRequest) -> UpdateUserBuilder {
    UpdateUserBuilder::new(user_id, request)
}

/// Builder for updating user password
pub struct UpdatePasswordBuilder {
    user_id: String,
    request: UpdatePasswordRequest,
}

impl UpdatePasswordBuilder {
    pub fn new(user_id: &str, request: UpdatePasswordRequest) -> Self {
        Self {
            user_id: user_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/users/{}/password", config.base_url, self.user_id);

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update password: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Update user password using builder pattern
pub fn update_password(user_id: &str, request: UpdatePasswordRequest) -> UpdatePasswordBuilder {
    UpdatePasswordBuilder::new(user_id, request)
}

/// Builder for deleting a user
pub struct DeleteUserBuilder {
    user_id: String,
}

impl DeleteUserBuilder {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/users/{}", config.base_url, self.user_id);

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete user: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Delete a user using builder pattern
pub fn delete_user(user_id: &str) -> DeleteUserBuilder {
    DeleteUserBuilder::new(user_id)
}
