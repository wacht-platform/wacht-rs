use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{
        User, CreateUserRequest, UpdateUserRequest, UpdatePasswordRequest,
        InviteUserRequest, WaitlistUser, UserEmail, UserPhone
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListResponse {
    pub data: Vec<User>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDetailsResponse {
    pub user: User,
    pub organizations: Vec<serde_json::Value>, // Organization details
    pub workspaces: Vec<serde_json::Value>,    // Workspace details
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitationListResponse {
    pub data: Vec<UserInvitation>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInvitation {
    pub id: String,
    pub email: String,
    pub role: Option<String>,
    pub organization_id: Option<String>,
    pub invited_at: String,
    pub expires_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitlistResponse {
    pub data: Vec<WaitlistUser>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListUsersOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

/// Fetch users
pub async fn fetch_users(options: Option<ListUsersOptions>) -> Result<UserListResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/users", config.base_url);
    
    let mut request = client.get(&url);
    
    if let Some(opts) = options {
        request = request.query(&opts);
    }
    
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

/// Create user
pub async fn create_user(request: CreateUserRequest) -> Result<User> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/users", config.base_url);
    
    // Build multipart form
    let mut form = reqwest::multipart::Form::new()
        .text("first_name", request.first_name.clone())
        .text("last_name", request.last_name.clone());
    
    if let Some(email) = request.email_address {
        form = form.text("email_address", email);
    }
    
    if let Some(phone) = request.phone_number {
        form = form.text("phone_number", phone);
    }
    
    if let Some(username) = request.username {
        form = form.text("username", username);
    }
    
    if let Some(password) = request.password {
        form = form.text("password", password);
    }
    
    if let Some(image_data) = request.profile_image {
        let part = reqwest::multipart::Part::bytes(image_data)
            .file_name("profile.png")
            .mime_str("image/png")
            .map_err(|e| Error::InvalidRequest(format!("Invalid image MIME type: {}", e)))?;
        form = form.part("profile_image", part);
    }
    
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

/// Fetch user details including organizations and workspaces
pub async fn fetch_user_details(user_id: &str) -> Result<UserDetailsResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/users/{}/details", config.base_url, user_id);
    
    let response = client.get(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to get user details {}: {}", user_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Update user
pub async fn update_user(user_id: &str, request: UpdateUserRequest) -> Result<User> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/users/{}", config.base_url, user_id);
    
    // Build multipart form
    let mut form = reqwest::multipart::Form::new();
    
    if let Some(first_name) = request.first_name {
        form = form.text("first_name", first_name);
    }
    
    if let Some(last_name) = request.last_name {
        form = form.text("last_name", last_name);
    }
    
    if let Some(username) = request.username {
        form = form.text("username", username);
    }
    
    if let Some(public_metadata) = request.public_metadata {
        let metadata_str = serde_json::to_string(&public_metadata)
            .map_err(|e| Error::InvalidRequest(format!("Failed to serialize public_metadata: {}", e)))?;
        form = form.text("public_metadata", metadata_str);
    }
    
    if let Some(private_metadata) = request.private_metadata {
        let metadata_str = serde_json::to_string(&private_metadata)
            .map_err(|e| Error::InvalidRequest(format!("Failed to serialize private_metadata: {}", e)))?;
        form = form.text("private_metadata", metadata_str);
    }
    
    if let Some(image_data) = request.profile_image {
        let part = reqwest::multipart::Part::bytes(image_data)
            .file_name("profile.png")
            .mime_str("image/png")
            .map_err(|e| Error::InvalidRequest(format!("Invalid image MIME type: {}", e)))?;
        form = form.part("profile_image", part);
    }
    
    let response = client.patch(&url).multipart(form).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to update user {}: {}", user_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Update user password
pub async fn update_password(user_id: &str, new_password: String) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/users/{}/password", config.base_url, user_id);
    
    let response = client.patch(&url).json(&new_password).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to update password for user {}: {}", user_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Add email to user
pub async fn add_email(
    user_id: &str,
    email: String,
    verified: Option<bool>,
    is_primary: Option<bool>,
) -> Result<UserEmail> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/users/{}/emails", config.base_url, user_id);
    
    #[derive(Serialize)]
    struct AddEmailRequest {
        email: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        verified: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_primary: Option<bool>,
    }
    
    let response = client.post(&url)
        .json(&AddEmailRequest { email, verified, is_primary })
        .send()
        .await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to add email to user {}: {}", user_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Update user email
pub async fn update_email(user_id: &str, email_id: &str, is_primary: bool, is_verified: bool) -> Result<UserEmail> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/users/{}/emails/{}", config.base_url, user_id, email_id);
    
    #[derive(Serialize)]
    struct UpdateEmailRequest {
        is_primary: bool,
        is_verified: bool,
    }
    
    let response = client.patch(&url)
        .json(&UpdateEmailRequest { is_primary, is_verified })
        .send()
        .await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to update email {} for user {}: {}", email_id, user_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Delete user email
pub async fn delete_email(user_id: &str, email_id: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/users/{}/emails/{}", config.base_url, user_id, email_id);
    
    let response = client.delete(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to delete email {} for user {}: {}", email_id, user_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Add phone to user
pub async fn add_phone(
    user_id: &str,
    phone_number: String,
    country_code: String,
    verified: Option<bool>,
    is_primary: Option<bool>,
) -> Result<UserPhone> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/users/{}/phones", config.base_url, user_id);
    
    #[derive(Serialize)]
    struct AddPhoneRequest {
        phone_number: String,
        country_code: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        verified: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_primary: Option<bool>,
    }
    
    let response = client.post(&url)
        .json(&AddPhoneRequest { phone_number, country_code, verified, is_primary })
        .send()
        .await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to add phone to user {}: {}", user_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Update user phone
pub async fn update_phone(user_id: &str, phone_id: &str, is_primary: bool, is_verified: bool) -> Result<UserPhone> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/users/{}/phones/{}", config.base_url, user_id, phone_id);
    
    #[derive(Serialize)]
    struct UpdatePhoneRequest {
        is_primary: bool,
        is_verified: bool,
    }
    
    let response = client.patch(&url)
        .json(&UpdatePhoneRequest { is_primary, is_verified })
        .send()
        .await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to update phone {} for user {}: {}", phone_id, user_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Delete user phone
pub async fn delete_phone(user_id: &str, phone_id: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/users/{}/phones/{}", config.base_url, user_id, phone_id);
    
    let response = client.delete(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to delete phone {} for user {}: {}", phone_id, user_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Delete user social connection
pub async fn delete_social_connection(user_id: &str, connection_id: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/users/{}/social-connections/{}", config.base_url, user_id, connection_id);
    
    let response = client.delete(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to delete social connection {} for user {}: {}", connection_id, user_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Fetch invited users
pub async fn fetch_invited_users() -> Result<InvitationListResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/invited-users", config.base_url);
    
    let response = client.get(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to fetch invited users: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Invite user
pub async fn invite_user(request: InviteUserRequest) -> Result<UserInvitation> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/invited-users", config.base_url);
    
    let response = client.post(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to invite user: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Fetch waitlist
pub async fn fetch_waitlist() -> Result<WaitlistResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/user-waitlist", config.base_url);
    
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

/// Approve waitlist user
pub async fn approve_waitlist_user(waitlist_user_id: &str) -> Result<User> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/user-waitlist/{}/approve", config.base_url, waitlist_user_id);
    
    let response = client.post(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to approve waitlist user {}: {}", waitlist_user_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}