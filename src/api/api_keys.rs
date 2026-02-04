use crate::{
    client::{get_client, get_config},
    error::Error,
    Result,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Create an API auth app
#[derive(Debug, Serialize)]
pub struct CreateApiAuthAppRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limits: Option<Vec<RateLimit>>,
}

/// Update an API auth app
#[derive(Debug, Serialize)]
pub struct UpdateApiAuthAppRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limits: Option<Vec<RateLimit>>,
}

/// Create an API key
#[derive(Debug, Serialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub key_prefix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

/// Revoke an API key
#[derive(Debug, Serialize)]
pub struct RevokeApiKeyRequest {
    pub key_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Rotate an API key
#[derive(Debug, Serialize)]
pub struct RotateApiKeyRequest {
    pub key_id: String,
}

/// Rate limit mode for API key app
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RateLimitMode {
    PerKey,
    PerIp,
    PerKeyAndIp,
}

/// Rate limit unit (second, minute, hour, day)
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RateLimitUnit {
    Second,
    Minute,
    Hour,
    Day,
}

/// Individual rate limit configuration
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RateLimit {
    pub unit: RateLimitUnit,
    pub duration: i32,
    pub max_requests: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<RateLimitMode>,
}

/// API auth app data
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiAuthApp {
    pub id: String,
    pub deployment_id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub rate_limits: Vec<RateLimit>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// API key data
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiKey {
    pub id: String,
    pub app_id: String,
    pub deployment_id: String,
    pub name: String,
    pub key_prefix: String,
    pub key_suffix: String,
    pub permissions: Vec<String>,
    pub metadata: serde_json::Value,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoked_reason: Option<String>,
}

/// API key with secret (returned on creation/rotation)
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiKeyWithSecret {
    #[serde(flatten)]
    pub key: ApiKey,
    pub secret: String,
}

/// Response for listing API auth apps
#[derive(Debug, Deserialize)]
pub struct ListApiAuthAppsResponse {
    pub total: usize,
    pub apps: Vec<ApiAuthApp>,
}

/// Response for listing API keys
#[derive(Debug, Deserialize)]
pub struct ListApiKeysResponse {
    pub keys: Vec<ApiKey>,
}

/// Get a single API auth app by name
pub fn get_api_auth_app(app_name: &str) -> GetApiAuthAppBuilder {
    GetApiAuthAppBuilder::new(app_name)
}

/// Builder for get_api_auth_app
pub struct GetApiAuthAppBuilder {
    app_name: String,
}

impl GetApiAuthAppBuilder {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
        }
    }

    pub async fn send(self) -> Result<ApiAuthApp> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/api-auth/apps/{}", config.base_url, self.app_name);

        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            let app = response.json().await?;
            Ok(app)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to get API auth app: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// List API auth apps
pub fn list_api_auth_apps() -> ListApiAuthAppsBuilder {
    ListApiAuthAppsBuilder::new()
}

/// Builder for list_api_auth_apps
pub struct ListApiAuthAppsBuilder {
    include_inactive: Option<bool>,
}

impl ListApiAuthAppsBuilder {
    pub fn new() -> Self {
        Self {
            include_inactive: None,
        }
    }

    pub fn include_inactive(mut self, include_inactive: bool) -> Self {
        self.include_inactive = Some(include_inactive);
        self
    }

    pub async fn send(self) -> Result<Vec<ApiAuthApp>> {
        let config = get_config();
        let client = get_client();
        let mut url = format!("{}/api-auth/apps", config.base_url);

        if let Some(inactive) = self.include_inactive {
            url.push_str(&format!("?include_inactive={inactive}"));
        }

        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            let response_data: ListApiAuthAppsResponse = response.json().await?;
            Ok(response_data.apps)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to list API auth apps: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Create an API auth app
pub fn create_api_auth_app(request: CreateApiAuthAppRequest) -> CreateApiAuthAppBuilder {
    CreateApiAuthAppBuilder::new(request)
}

/// Builder for create_api_auth_app
pub struct CreateApiAuthAppBuilder {
    request: CreateApiAuthAppRequest,
}

impl CreateApiAuthAppBuilder {
    pub fn new(request: CreateApiAuthAppRequest) -> Self {
        Self { request }
    }

    pub async fn send(self) -> Result<ApiAuthApp> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/api-auth/apps", config.base_url);

        let response = client.post(&url).json(&self.request).send().await?;

        if response.status().is_success() {
            let app = response.json().await?;
            Ok(app)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to create API auth app: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Update an API auth app
pub fn update_api_auth_app(app_name: &str, request: UpdateApiAuthAppRequest) -> UpdateApiAuthAppBuilder {
    UpdateApiAuthAppBuilder::new(app_name, request)
}

/// Builder for update_api_auth_app
pub struct UpdateApiAuthAppBuilder {
    app_name: String,
    request: UpdateApiAuthAppRequest,
}

impl UpdateApiAuthAppBuilder {
    pub fn new(app_name: &str, request: UpdateApiAuthAppRequest) -> Self {
        Self {
            app_name: app_name.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<ApiAuthApp> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/api-auth/apps/{}", config.base_url, self.app_name);

        let response = client.patch(&url).json(&self.request).send().await?;

        if response.status().is_success() {
            let app = response.json().await?;
            Ok(app)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to update API auth app: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Delete an API auth app
pub fn delete_api_auth_app(app_name: &str) -> DeleteApiAuthAppBuilder {
    DeleteApiAuthAppBuilder::new(app_name)
}

/// Builder for delete_api_auth_app
pub struct DeleteApiAuthAppBuilder {
    app_name: String,
}

impl DeleteApiAuthAppBuilder {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/api-auth/apps/{}", config.base_url, self.app_name);

        let response = client.delete(&url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to delete API auth app: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// List API keys for an app
pub fn list_api_keys(app_name: &str) -> ListApiKeysBuilder {
    ListApiKeysBuilder::new(app_name)
}

/// Builder for list_api_keys
pub struct ListApiKeysBuilder {
    app_name: String,
    include_inactive: Option<bool>,
}

impl ListApiKeysBuilder {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            include_inactive: None,
        }
    }

    pub fn include_inactive(mut self, include_inactive: bool) -> Self {
        self.include_inactive = Some(include_inactive);
        self
    }

    pub async fn send(self) -> Result<Vec<ApiKey>> {
        let config = get_config();
        let client = get_client();
        let mut url = format!("{}/api-auth/apps/{}/keys", config.base_url, self.app_name);

        if let Some(inactive) = self.include_inactive {
            url.push_str(&format!("?include_inactive={inactive}"));
        }

        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            let response_data: ListApiKeysResponse = response.json().await?;
            Ok(response_data.keys)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to list API keys: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Create an API key
pub fn create_api_key(app_name: &str, request: CreateApiKeyRequest) -> CreateApiKeyBuilder {
    CreateApiKeyBuilder::new(app_name, request)
}

/// Builder for create_api_key
pub struct CreateApiKeyBuilder {
    app_name: String,
    request: CreateApiKeyRequest,
}

impl CreateApiKeyBuilder {
    pub fn new(app_name: &str, request: CreateApiKeyRequest) -> Self {
        Self {
            app_name: app_name.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<ApiKeyWithSecret> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/api-auth/apps/{}/keys", config.base_url, self.app_name);

        let response = client.post(&url).json(&self.request).send().await?;

        if response.status().is_success() {
            let key = response.json().await?;
            Ok(key)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to create API key: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Revoke an API key
pub fn revoke_api_key(request: RevokeApiKeyRequest) -> RevokeApiKeyBuilder {
    RevokeApiKeyBuilder::new(request)
}

/// Builder for revoke_api_key
pub struct RevokeApiKeyBuilder {
    request: RevokeApiKeyRequest,
}

impl RevokeApiKeyBuilder {
    pub fn new(request: RevokeApiKeyRequest) -> Self {
        Self { request }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/api-auth/keys/revoke", config.base_url);

        let response = client.post(&url).json(&self.request).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to revoke API key: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Rotate an API key
pub fn rotate_api_key(request: RotateApiKeyRequest) -> RotateApiKeyBuilder {
    RotateApiKeyBuilder::new(request)
}

/// Builder for rotate_api_key
pub struct RotateApiKeyBuilder {
    request: RotateApiKeyRequest,
}

impl RotateApiKeyBuilder {
    pub fn new(request: RotateApiKeyRequest) -> Self {
        Self { request }
    }

    pub async fn send(self) -> Result<ApiKeyWithSecret> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/api-auth/keys/rotate", config.base_url);

        let response = client.post(&url).json(&self.request).send().await?;

        if response.status().is_success() {
            let key = response.json().await?;
            Ok(key)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to rotate API key: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}