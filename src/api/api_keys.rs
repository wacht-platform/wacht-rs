use crate::{
    client::{get_client, get_config},
    error::Error,
    Result,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Create an API key app
#[derive(Debug, Serialize)]
pub struct CreateApiKeyAppRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limits: Option<Vec<RateLimit>>,
}

/// Update an API key app
#[derive(Debug, Serialize)]
pub struct UpdateApiKeyAppRequest {
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

/// API key app data
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiKeyApp {
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

/// Response for listing API key apps
#[derive(Debug, Deserialize)]
pub struct ListApiKeyAppsResponse {
    pub total: usize,
    pub apps: Vec<ApiKeyApp>,
}

/// Response for listing API keys
#[derive(Debug, Deserialize)]
pub struct ListApiKeysResponse {
    pub keys: Vec<ApiKey>,
}

/// Get a single API key app by name
pub async fn get_api_key_app(app_name: &str) -> Result<ApiKeyApp> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/api-keys/apps/{}", config.base_url, app_name);
    
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        let app = response.json().await?;
        Ok(app)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to get API key app: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// List API key apps
pub async fn list_api_key_apps(
    include_inactive: Option<bool>,
) -> Result<Vec<ApiKeyApp>> {
    let config = get_config();
    let client = get_client();
    let mut url = format!("{}/api-keys/apps", config.base_url);
    
    if let Some(inactive) = include_inactive {
        url.push_str(&format!("?include_inactive={}", inactive));
    }
    
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        let response_data: ListApiKeyAppsResponse = response.json().await?;
        Ok(response_data.apps)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to list API key apps: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Create an API key app
pub async fn create_api_key_app(
    request: CreateApiKeyAppRequest,
) -> Result<ApiKeyApp> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/api-keys/apps", config.base_url);
    
    let response = client.post(&url).json(&request).send().await?;
    
    if response.status().is_success() {
        let app = response.json().await?;
        Ok(app)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to create API key app: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Update an API key app
pub async fn update_api_key_app(
    app_name: &str,
    request: UpdateApiKeyAppRequest,
) -> Result<ApiKeyApp> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/api-keys/apps/{}", config.base_url, app_name);
    
    let response = client.patch(&url).json(&request).send().await?;
    
    if response.status().is_success() {
        let app = response.json().await?;
        Ok(app)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to update API key app: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Delete an API key app
pub async fn delete_api_key_app(app_name: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/api-keys/apps/{}", config.base_url, app_name);
    
    let response = client.delete(&url).send().await?;
    
    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to delete API key app: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// List API keys for an app
pub async fn list_api_keys(
    app_name: &str,
    include_inactive: Option<bool>,
) -> Result<Vec<ApiKey>> {
    let config = get_config();
    let client = get_client();
    let mut url = format!("{}/api-keys/apps/{}/keys", config.base_url, app_name);
    
    if let Some(inactive) = include_inactive {
        url.push_str(&format!("?include_inactive={}", inactive));
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
            message: format!("Failed to list API keys: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Create an API key
pub async fn create_api_key(
    app_name: &str,
    request: CreateApiKeyRequest,
) -> Result<ApiKeyWithSecret> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/api-keys/apps/{}/keys", config.base_url, app_name);
    
    let response = client.post(&url).json(&request).send().await?;
    
    if response.status().is_success() {
        let key = response.json().await?;
        Ok(key)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to create API key: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Revoke an API key
pub async fn revoke_api_key(
    request: RevokeApiKeyRequest,
) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/api-keys/revoke", config.base_url);
    
    let response = client.post(&url).json(&request).send().await?;
    
    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to revoke API key: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Rotate an API key
pub async fn rotate_api_key(
    request: RotateApiKeyRequest,
) -> Result<ApiKeyWithSecret> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/api-keys/rotate", config.base_url);
    
    let response = client.post(&url).json(&request).send().await?;
    
    if response.status().is_success() {
        let key = response.json().await?;
        Ok(key)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to rotate API key: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}