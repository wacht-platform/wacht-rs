use crate::{
    client::WachtClient,
    error::Error,
    Result,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct CreateApiAuthAppRequest {
    pub app_slug: String,
    pub name: String,
    pub key_prefix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_scheme_slug: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateApiAuthAppRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_scheme_slug: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_membership_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_membership_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct RevokeApiKeyRequest {
    pub key_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RotateApiKeyRequest {
    pub key_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RateLimitMode {
    PerKey,
    PerIp,
    PerKeyAndIp,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RateLimitUnit {
    Second,
    Minute,
    Hour,
    Day,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RateLimit {
    pub unit: RateLimitUnit,
    pub duration: i32,
    pub max_requests: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<RateLimitMode>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiAuthApp {
    pub id: String,
    pub deployment_id: String,
    pub app_slug: String,
    pub name: String,
    pub key_prefix: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub rate_limits: Vec<RateLimit>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiKey {
    pub id: String,
    pub app_id: String,
    pub deployment_id: String,
    pub app_slug: String,
    pub name: String,
    pub key_prefix: String,
    pub key_suffix: String,
    pub permissions: Vec<String>,
    pub org_role_permissions: Vec<String>,
    pub workspace_role_permissions: Vec<String>,
    pub metadata: serde_json::Value,
    pub organization_id: Option<String>,
    pub workspace_id: Option<String>,
    pub organization_membership_id: Option<String>,
    pub workspace_membership_id: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoked_reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiKeyWithSecret {
    #[serde(flatten)]
    pub key: ApiKey,
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct ListApiAuthAppsResponse {
    pub total: usize,
    pub apps: Vec<ApiAuthApp>,
}

#[derive(Debug, Deserialize)]
pub struct ListApiKeysResponse {
    pub keys: Vec<ApiKey>,
}

#[derive(Debug, Clone)]
pub struct ApiKeysApi {
    client: WachtClient,
}

impl ApiKeysApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn get_api_auth_app(&self, app_name: &str) -> GetApiAuthAppBuilder {
        GetApiAuthAppBuilder::new(self.client.clone(), app_name)
    }

    pub fn list_api_auth_apps(&self) -> ListApiAuthAppsBuilder {
        ListApiAuthAppsBuilder::new(self.client.clone())
    }

    pub fn create_api_auth_app(&self, request: CreateApiAuthAppRequest) -> CreateApiAuthAppBuilder {
        CreateApiAuthAppBuilder::new(self.client.clone(), request)
    }

    pub fn update_api_auth_app(
        &self,
        app_name: &str,
        request: UpdateApiAuthAppRequest,
    ) -> UpdateApiAuthAppBuilder {
        UpdateApiAuthAppBuilder::new(self.client.clone(), app_name, request)
    }

    pub fn delete_api_auth_app(&self, app_name: &str) -> DeleteApiAuthAppBuilder {
        DeleteApiAuthAppBuilder::new(self.client.clone(), app_name)
    }

    pub fn list_api_keys(&self, app_name: &str) -> ListApiKeysBuilder {
        ListApiKeysBuilder::new(self.client.clone(), app_name)
    }

    pub fn create_api_key(&self, app_name: &str, request: CreateApiKeyRequest) -> CreateApiKeyBuilder {
        CreateApiKeyBuilder::new(self.client.clone(), app_name, request)
    }

    pub fn revoke_api_key(&self, request: RevokeApiKeyRequest) -> RevokeApiKeyBuilder {
        RevokeApiKeyBuilder::new(self.client.clone(), request)
    }

    pub fn rotate_api_key(&self, request: RotateApiKeyRequest) -> RotateApiKeyBuilder {
        RotateApiKeyBuilder::new(self.client.clone(), request)
    }
}

pub struct GetApiAuthAppBuilder {
    client: WachtClient,
    app_name: String,
}

impl GetApiAuthAppBuilder {
    pub fn new(client: WachtClient, app_name: &str) -> Self {
        Self {
            client,
            app_name: app_name.to_string(),
        }
    }

    pub async fn send(self) -> Result<ApiAuthApp> {
        let client = self.client.http_client();
        let url = format!("{}/api-auth/apps/{}", self.client.config().base_url, self.app_name);

        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
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

pub struct ListApiAuthAppsBuilder {
    client: WachtClient,
    include_inactive: Option<bool>,
}

impl ListApiAuthAppsBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
            include_inactive: None,
        }
    }

    pub fn include_inactive(mut self, include_inactive: bool) -> Self {
        self.include_inactive = Some(include_inactive);
        self
    }

    pub async fn send(self) -> Result<Vec<ApiAuthApp>> {
        let client = self.client.http_client();
        let mut url = format!("{}/api-auth/apps", self.client.config().base_url);

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

pub struct CreateApiAuthAppBuilder {
    client: WachtClient,
    request: CreateApiAuthAppRequest,
}

impl CreateApiAuthAppBuilder {
    pub fn new(client: WachtClient, request: CreateApiAuthAppRequest) -> Self {
        Self { client, request }
    }

    pub async fn send(self) -> Result<ApiAuthApp> {
        let client = self.client.http_client();
        let url = format!("{}/api-auth/apps", self.client.config().base_url);

        let response = client.post(&url).json(&self.request).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
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

pub struct UpdateApiAuthAppBuilder {
    client: WachtClient,
    app_name: String,
    request: UpdateApiAuthAppRequest,
}

impl UpdateApiAuthAppBuilder {
    pub fn new(client: WachtClient, app_name: &str, request: UpdateApiAuthAppRequest) -> Self {
        Self {
            client,
            app_name: app_name.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<ApiAuthApp> {
        let client = self.client.http_client();
        let url = format!("{}/api-auth/apps/{}", self.client.config().base_url, self.app_name);

        let response = client.patch(&url).json(&self.request).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
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

pub struct DeleteApiAuthAppBuilder {
    client: WachtClient,
    app_name: String,
}

impl DeleteApiAuthAppBuilder {
    pub fn new(client: WachtClient, app_name: &str) -> Self {
        Self {
            client,
            app_name: app_name.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!("{}/api-auth/apps/{}", self.client.config().base_url, self.app_name);

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

pub struct ListApiKeysBuilder {
    client: WachtClient,
    app_name: String,
    include_inactive: Option<bool>,
}

impl ListApiKeysBuilder {
    pub fn new(client: WachtClient, app_name: &str) -> Self {
        Self {
            client,
            app_name: app_name.to_string(),
            include_inactive: None,
        }
    }

    pub fn include_inactive(mut self, include_inactive: bool) -> Self {
        self.include_inactive = Some(include_inactive);
        self
    }

    pub async fn send(self) -> Result<Vec<ApiKey>> {
        let client = self.client.http_client();
        let mut url = format!(
            "{}/api-auth/apps/{}/keys",
            self.client.config().base_url,
            self.app_name
        );

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

pub struct CreateApiKeyBuilder {
    client: WachtClient,
    app_name: String,
    request: CreateApiKeyRequest,
}

impl CreateApiKeyBuilder {
    pub fn new(client: WachtClient, app_name: &str, request: CreateApiKeyRequest) -> Self {
        Self {
            client,
            app_name: app_name.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<ApiKeyWithSecret> {
        let client = self.client.http_client();
        let url = format!(
            "{}/api-auth/apps/{}/keys",
            self.client.config().base_url,
            self.app_name
        );

        let response = client.post(&url).json(&self.request).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
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

pub struct RevokeApiKeyBuilder {
    client: WachtClient,
    request: RevokeApiKeyRequest,
}

impl RevokeApiKeyBuilder {
    pub fn new(client: WachtClient, request: RevokeApiKeyRequest) -> Self {
        Self { client, request }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!("{}/api-auth/keys/revoke", self.client.config().base_url);

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

pub struct RotateApiKeyBuilder {
    client: WachtClient,
    request: RotateApiKeyRequest,
}

impl RotateApiKeyBuilder {
    pub fn new(client: WachtClient, request: RotateApiKeyRequest) -> Self {
        Self { client, request }
    }

    pub async fn send(self) -> Result<ApiKeyWithSecret> {
        let client = self.client.http_client();
        let url = format!("{}/api-auth/keys/rotate", self.client.config().base_url);

        let response = client.post(&url).json(&self.request).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
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
