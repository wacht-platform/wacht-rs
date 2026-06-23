use crate::{Result, client::WachtClient, error::Error};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct CreateApiAuthAppRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    pub app_slug: String,
    pub name: String,
    pub key_prefix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_scheme_slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct UpdateApiAuthAppRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct RevokeApiKeyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RateLimitMode {
    PerApp,
    PerKey,
    PerKeyAndIp,
    PerAppAndIp,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RateLimitUnit {
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
    CalendarDay,
    Month,
    CalendarMonth,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RateLimit {
    pub unit: RateLimitUnit,
    pub duration: i32,
    pub max_requests: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<RateLimitMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<String>>,
    #[serde(default)]
    pub priority: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiAuthApp {
    pub deployment_id: String,
    pub user_id: Option<String>,
    pub organization_id: Option<String>,
    pub workspace_id: Option<String>,
    pub app_slug: String,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub key_prefix: String,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default)]
    pub resources: Vec<String>,
    #[serde(default)]
    pub rate_limits: Vec<RateLimit>,
    pub rate_limit_scheme_slug: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiKey {
    pub id: String,
    pub deployment_id: String,
    pub app_slug: String,
    pub name: String,
    pub key_prefix: String,
    pub key_suffix: String,
    #[serde(default)]
    pub permissions: Vec<String>,
    pub metadata: serde_json::Value,
    #[serde(default)]
    pub rate_limits: Vec<RateLimit>,
    pub rate_limit_scheme_slug: Option<String>,
    pub owner_user_id: Option<String>,
    pub organization_id: Option<String>,
    pub workspace_id: Option<String>,
    pub organization_membership_id: Option<String>,
    pub workspace_membership_id: Option<String>,
    #[serde(default)]
    pub org_role_permissions: Vec<String>,
    #[serde(default)]
    pub workspace_role_permissions: Vec<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateRateLimitSchemeRequest {
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub rules: Vec<RateLimit>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateRateLimitSchemeRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub rules: Option<Vec<RateLimit>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateLimitScheme {
    pub id: String,
    pub deployment_id: String,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub rules: Vec<RateLimit>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListRateLimitSchemesResponse {
    pub schemes: Vec<RateLimitScheme>,
    pub total: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListApiAuditLogsQuery {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub outcome: Option<String>,
    pub key_id: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub cursor: Option<String>,
    pub cursor_ts: Option<DateTime<Utc>>,
    pub cursor_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetApiAuditAnalyticsQuery {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub key_id: Option<String>,
    pub include_top_keys: Option<bool>,
    pub include_top_paths: Option<bool>,
    pub include_blocked_reasons: Option<bool>,
    pub include_rate_limits: Option<bool>,
    pub top_limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetApiAuditTimeseriesQuery {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub interval: Option<String>,
    pub key_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuditLog {
    pub request_id: String,
    pub deployment_id: String,
    pub app_slug: String,
    pub key_id: String,
    pub key_name: String,
    pub outcome: String,
    pub blocked_by_rule: Option<String>,
    pub client_ip: String,
    pub path: String,
    pub user_agent: Option<String>,
    pub rate_limits: Option<Value>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuditLogsResponse {
    #[serde(default)]
    pub data: Vec<ApiAuditLog>,
    pub limit: u32,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuditTopKey {
    pub key_id: String,
    pub key_name: String,
    pub total_requests: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuditTopPath {
    pub path: String,
    pub total_requests: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuditBlockedReason {
    pub blocked_by_rule: String,
    pub count: i64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuditRateLimitRule {
    pub rule: String,
    pub hit_count: i64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuditRateLimitBreakdown {
    pub total_hits: i64,
    pub percentage_of_blocked: f64,
    #[serde(default)]
    pub top_rules: Vec<ApiAuditRateLimitRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuditAnalyticsResponse {
    pub total_requests: u64,
    pub allowed_requests: u64,
    pub blocked_requests: u64,
    pub success_rate: f64,
    pub keys_used_24h: u64,
    pub top_keys: Option<Vec<ApiAuditTopKey>>,
    pub top_paths: Option<Vec<ApiAuditTopPath>>,
    pub blocked_reasons: Option<Vec<ApiAuditBlockedReason>>,
    pub rate_limit_stats: Option<ApiAuditRateLimitBreakdown>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuditTimeseriesPoint {
    pub timestamp: DateTime<Utc>,
    pub total_requests: i64,
    pub allowed_requests: i64,
    pub blocked_requests: i64,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuditTimeseriesResponse {
    #[serde(default)]
    pub data: Vec<ApiAuditTimeseriesPoint>,
    pub interval: String,
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

    pub fn create_api_key(
        &self,
        app_name: &str,
        request: CreateApiKeyRequest,
    ) -> CreateApiKeyBuilder {
        CreateApiKeyBuilder::new(self.client.clone(), app_name, request)
    }

    pub fn revoke_api_key(
        &self,
        app_name: &str,
        key_id: i64,
        request: RevokeApiKeyRequest,
    ) -> RevokeApiKeyBuilder {
        RevokeApiKeyBuilder::new(self.client.clone(), app_name, key_id, request)
    }

    pub fn rotate_api_key(&self, app_name: &str, key_id: i64) -> RotateApiKeyBuilder {
        RotateApiKeyBuilder::new(self.client.clone(), app_name, key_id)
    }

    pub fn list_rate_limit_schemes(&self) -> ListRateLimitSchemesBuilder {
        ListRateLimitSchemesBuilder::new(self.client.clone())
    }

    pub fn get_rate_limit_scheme(&self, slug: &str) -> GetRateLimitSchemeBuilder {
        GetRateLimitSchemeBuilder::new(self.client.clone(), slug)
    }

    pub fn create_rate_limit_scheme(
        &self,
        request: CreateRateLimitSchemeRequest,
    ) -> CreateRateLimitSchemeBuilder {
        CreateRateLimitSchemeBuilder::new(self.client.clone(), request)
    }

    pub fn update_rate_limit_scheme(
        &self,
        slug: &str,
        request: UpdateRateLimitSchemeRequest,
    ) -> UpdateRateLimitSchemeBuilder {
        UpdateRateLimitSchemeBuilder::new(self.client.clone(), slug, request)
    }

    pub fn delete_rate_limit_scheme(&self, slug: &str) -> DeleteRateLimitSchemeBuilder {
        DeleteRateLimitSchemeBuilder::new(self.client.clone(), slug)
    }

    pub fn get_api_audit_logs(
        &self,
        app_name: &str,
        query: ListApiAuditLogsQuery,
    ) -> GetApiAuditLogsBuilder {
        GetApiAuditLogsBuilder::new(self.client.clone(), app_name, query)
    }

    pub fn get_api_audit_analytics(
        &self,
        app_name: &str,
        query: GetApiAuditAnalyticsQuery,
    ) -> GetApiAuditAnalyticsBuilder {
        GetApiAuditAnalyticsBuilder::new(self.client.clone(), app_name, query)
    }

    pub fn get_api_audit_timeseries(
        &self,
        app_name: &str,
        query: GetApiAuditTimeseriesQuery,
    ) -> GetApiAuditTimeseriesBuilder {
        GetApiAuditTimeseriesBuilder::new(self.client.clone(), app_name, query)
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
        let url = format!(
            "{}/api-auth/apps/{}",
            self.client.config().base_url,
            self.app_name
        );

        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::api_from_text(
                status,
                "Failed to get API auth app",
                &error_text,
            ))
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
            Err(Error::api_from_text(
                status,
                "Failed to list API auth apps",
                &error_text,
            ))
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
            Err(Error::api_from_text(
                status,
                "Failed to create API auth app",
                &error_text,
            ))
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
        let url = format!(
            "{}/api-auth/apps/{}",
            self.client.config().base_url,
            self.app_name
        );

        let response = client.patch(&url).json(&self.request).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::api_from_text(
                status,
                "Failed to update API auth app",
                &error_text,
            ))
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
        let url = format!(
            "{}/api-auth/apps/{}",
            self.client.config().base_url,
            self.app_name
        );

        let response = client.delete(&url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::api_from_text(
                status,
                "Failed to delete API auth app",
                &error_text,
            ))
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
            Err(Error::api_from_text(
                status,
                "Failed to list API keys",
                &error_text,
            ))
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
            Err(Error::api_from_text(
                status,
                "Failed to create API key",
                &error_text,
            ))
        }
    }
}

pub struct RevokeApiKeyBuilder {
    client: WachtClient,
    app_name: String,
    key_id: i64,
    request: RevokeApiKeyRequest,
}

impl RevokeApiKeyBuilder {
    pub fn new(
        client: WachtClient,
        app_name: &str,
        key_id: i64,
        request: RevokeApiKeyRequest,
    ) -> Self {
        Self {
            client,
            app_name: app_name.to_string(),
            key_id,
            request,
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/api-auth/apps/{}/keys/{}/revoke",
            self.client.config().base_url,
            self.app_name,
            self.key_id
        );

        let response = client.post(&url).json(&self.request).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::api_from_text(
                status,
                "Failed to revoke API key",
                &error_text,
            ))
        }
    }
}

pub struct RotateApiKeyBuilder {
    client: WachtClient,
    app_name: String,
    key_id: i64,
}

impl RotateApiKeyBuilder {
    pub fn new(client: WachtClient, app_name: &str, key_id: i64) -> Self {
        Self {
            client,
            app_name: app_name.to_string(),
            key_id,
        }
    }

    pub async fn send(self) -> Result<ApiKeyWithSecret> {
        let client = self.client.http_client();
        let url = format!(
            "{}/api-auth/apps/{}/keys/{}/rotate",
            self.client.config().base_url,
            self.app_name,
            self.key_id
        );

        let response = client.post(&url).send().await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::api_from_text(
                status,
                "Failed to rotate API key",
                &error_text,
            ))
        }
    }
}

pub struct ListRateLimitSchemesBuilder {
    client: WachtClient,
}

impl ListRateLimitSchemesBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub async fn send(self) -> Result<Vec<RateLimitScheme>> {
        let client = self.client.http_client();
        let url = format!(
            "{}/api-auth/rate-limit-schemes",
            self.client.config().base_url
        );

        let response = client.get(&url).send().await?;
        if response.status().is_success() {
            let response_data: ListRateLimitSchemesResponse = response.json().await?;
            Ok(response_data.schemes)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::api_from_text(
                status,
                "Failed to list rate limit schemes",
                &error_text,
            ))
        }
    }
}

pub struct GetRateLimitSchemeBuilder {
    client: WachtClient,
    slug: String,
}

impl GetRateLimitSchemeBuilder {
    pub fn new(client: WachtClient, slug: &str) -> Self {
        Self {
            client,
            slug: slug.to_string(),
        }
    }

    pub async fn send(self) -> Result<RateLimitScheme> {
        let client = self.client.http_client();
        let url = format!(
            "{}/api-auth/rate-limit-schemes/{}",
            self.client.config().base_url,
            self.slug
        );

        let response = client.get(&url).send().await?;
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::api_from_text(
                status,
                "Failed to get rate limit scheme",
                &error_text,
            ))
        }
    }
}

pub struct CreateRateLimitSchemeBuilder {
    client: WachtClient,
    request: CreateRateLimitSchemeRequest,
}

impl CreateRateLimitSchemeBuilder {
    pub fn new(client: WachtClient, request: CreateRateLimitSchemeRequest) -> Self {
        Self { client, request }
    }

    pub async fn send(self) -> Result<RateLimitScheme> {
        let client = self.client.http_client();
        let url = format!(
            "{}/api-auth/rate-limit-schemes",
            self.client.config().base_url
        );

        let response = client.post(&url).json(&self.request).send().await?;
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::api_from_text(
                status,
                "Failed to create rate limit scheme",
                &error_text,
            ))
        }
    }
}

pub struct UpdateRateLimitSchemeBuilder {
    client: WachtClient,
    slug: String,
    request: UpdateRateLimitSchemeRequest,
}

impl UpdateRateLimitSchemeBuilder {
    pub fn new(client: WachtClient, slug: &str, request: UpdateRateLimitSchemeRequest) -> Self {
        Self {
            client,
            slug: slug.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<RateLimitScheme> {
        let client = self.client.http_client();
        let url = format!(
            "{}/api-auth/rate-limit-schemes/{}",
            self.client.config().base_url,
            self.slug
        );

        let response = client.patch(&url).json(&self.request).send().await?;
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::api_from_text(
                status,
                "Failed to update rate limit scheme",
                &error_text,
            ))
        }
    }
}

pub struct DeleteRateLimitSchemeBuilder {
    client: WachtClient,
    slug: String,
}

impl DeleteRateLimitSchemeBuilder {
    pub fn new(client: WachtClient, slug: &str) -> Self {
        Self {
            client,
            slug: slug.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/api-auth/rate-limit-schemes/{}",
            self.client.config().base_url,
            self.slug
        );

        let response = client.delete(&url).send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::api_from_text(
                status,
                "Failed to delete rate limit scheme",
                &error_text,
            ))
        }
    }
}

pub struct GetApiAuditLogsBuilder {
    client: WachtClient,
    app_name: String,
    query: ListApiAuditLogsQuery,
}

impl GetApiAuditLogsBuilder {
    pub fn new(client: WachtClient, app_name: &str, query: ListApiAuditLogsQuery) -> Self {
        Self {
            client,
            app_name: app_name.to_string(),
            query,
        }
    }

    pub async fn send(self) -> Result<ApiAuditLogsResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/api-auth/apps/{}/audit/logs",
            self.client.config().base_url,
            self.app_name
        );

        let response = client.get(&url).query(&self.query).send().await?;
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::api_from_text(
                status,
                "Failed to get API audit logs",
                &error_text,
            ))
        }
    }
}

pub struct GetApiAuditAnalyticsBuilder {
    client: WachtClient,
    app_name: String,
    query: GetApiAuditAnalyticsQuery,
}

impl GetApiAuditAnalyticsBuilder {
    pub fn new(client: WachtClient, app_name: &str, query: GetApiAuditAnalyticsQuery) -> Self {
        Self {
            client,
            app_name: app_name.to_string(),
            query,
        }
    }

    pub async fn send(self) -> Result<ApiAuditAnalyticsResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/api-auth/apps/{}/audit/analytics",
            self.client.config().base_url,
            self.app_name
        );

        let response = client.get(&url).query(&self.query).send().await?;
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::api_from_text(
                status,
                "Failed to get API audit analytics",
                &error_text,
            ))
        }
    }
}

pub struct GetApiAuditTimeseriesBuilder {
    client: WachtClient,
    app_name: String,
    query: GetApiAuditTimeseriesQuery,
}

impl GetApiAuditTimeseriesBuilder {
    pub fn new(client: WachtClient, app_name: &str, query: GetApiAuditTimeseriesQuery) -> Self {
        Self {
            client,
            app_name: app_name.to_string(),
            query,
        }
    }

    pub async fn send(self) -> Result<ApiAuditTimeseriesResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/api-auth/apps/{}/audit/timeseries",
            self.client.config().base_url,
            self.app_name
        );

        let response = client.get(&url).query(&self.query).send().await?;
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::api_from_text(
                status,
                "Failed to get API audit timeseries",
                &error_text,
            ))
        }
    }
}
