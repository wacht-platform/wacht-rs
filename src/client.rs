use base64::{Engine as _, engine::general_purpose};
use reqwest::{
    Client, ClientBuilder,
    header::{AUTHORIZATION, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use url::Url;

#[derive(Debug, Clone)]
pub struct Auth {
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct WachtConfig {
    pub base_url: String,
    pub auth: Auth,
    pub frontend_url: String,
    pub public_signing_key: Option<String>,
    pub timeout: Duration,
    pub user_agent: String,
}

impl WachtConfig {
    pub fn new(token: impl Into<String>, frontend_url: impl Into<String>) -> Self {
        Self {
            base_url: "https://api.wacht.dev".to_string(),
            auth: Auth {
                token: token.into(),
            },
            frontend_url: frontend_url.into(),
            public_signing_key: None,
            timeout: Duration::from_secs(30),
            user_agent: format!("wacht/{}", env!("CARGO_PKG_VERSION")),
        }
    }

    pub fn with_public_key(mut self, public_key: impl Into<String>) -> Self {
        self.public_signing_key = Some(public_key.into());
        self
    }

    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let token = std::env::var("WACHT_API_KEY").map_err(|_| "WACHT_API_KEY must be set")?;
        let frontend_host = std::env::var("WACHT_FRONTEND_HOST")
            .ok()
            .or_else(|| {
                std::env::var("WACHT_PUBLISHABLE_KEY")
                    .ok()
                    .and_then(|pk| parse_frontend_api_url_from_publishable_key(&pk))
            })
            .ok_or(
                "Either WACHT_FRONTEND_HOST or WACHT_PUBLISHABLE_KEY must be set",
            )?;

        let mut config = Self::new(token, frontend_host);

        if let Ok(public_signing_key) = std::env::var("WACHT_PUBLIC_SIGNING_KEY") {
            config.public_signing_key = Some(public_signing_key);
        }

        Ok(config)
    }

    pub async fn load_public_key(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        self.public_signing_key = Some(fetch_public_key(&self.frontend_url).await?);
        Ok(self)
    }
}

fn parse_frontend_api_url_from_publishable_key(publishable_key: &str) -> Option<String> {
    let encoded = publishable_key
        .strip_prefix("pk_test_")
        .or_else(|| publishable_key.strip_prefix("pk_live_"))?;

    if encoded.is_empty() {
        return None;
    }

    let decoded = general_purpose::STANDARD
        .decode(encoded)
        .or_else(|_| general_purpose::URL_SAFE.decode(encoded))
        .or_else(|_| general_purpose::URL_SAFE_NO_PAD.decode(encoded))
        .ok()?;

    let decoded = String::from_utf8(decoded).ok()?;
    let url = Url::parse(&decoded).ok()?;

    Some(url.origin().ascii_serialization())
}

static GLOBAL_CONFIG: OnceLock<WachtConfig> = OnceLock::new();
static GLOBAL_HEADERS: OnceLock<HeaderMap> = OnceLock::new();

#[derive(Debug)]
struct WachtClientInner {
    config: WachtConfig,
    client: Client,
}

#[derive(Debug, Clone)]
pub struct WachtClient {
    inner: Arc<WachtClientInner>,
}

impl WachtClient {
    pub fn new(config: WachtConfig) -> Result<Self, String> {
        let mut headers = HeaderMap::new();
        let auth_value = HeaderValue::from_str(&format!("Bearer {}", config.auth.token))
            .map_err(|_| "Invalid authentication token")?;
        headers.insert(AUTHORIZATION, auth_value);

        let client = ClientBuilder::new()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .default_headers(headers)
            .pool_max_idle_per_host(500)
            .pool_idle_timeout(Duration::from_secs(90))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {e}"))?;

        Ok(Self {
            inner: Arc::new(WachtClientInner { config, client }),
        })
    }

    pub async fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let config = WachtConfig::from_env()?.load_public_key().await?;
        Self::new(config).map_err(|e| e.into())
    }

    pub fn config(&self) -> &WachtConfig {
        &self.inner.config
    }

    pub fn http_client(&self) -> Client {
        self.inner.client.clone()
    }

    pub fn users(&self) -> crate::api::users::UsersApi {
        crate::api::users::UsersApi::new(self.clone())
    }

    pub fn organizations(&self) -> crate::api::organizations::OrganizationsApi {
        crate::api::organizations::OrganizationsApi::new(self.clone())
    }

    pub fn ai(&self) -> crate::api::ai::AiApi {
        crate::api::ai::AiApi::new(self.clone())
    }

    pub fn workspaces(&self) -> crate::api::workspaces::WorkspacesApi {
        crate::api::workspaces::WorkspacesApi::new(self.clone())
    }

    pub fn health(&self) -> crate::api::health::HealthApi {
        crate::api::health::HealthApi::new(self.clone())
    }

    pub fn analytics(&self) -> crate::api::analytics::AnalyticsApi {
        crate::api::analytics::AnalyticsApi::new(self.clone())
    }

    pub fn integrations(&self) -> crate::api::integrations::IntegrationsApi {
        crate::api::integrations::IntegrationsApi::new(self.clone())
    }

    pub fn segments(&self) -> crate::api::segments::SegmentsApi {
        crate::api::segments::SegmentsApi::new(self.clone())
    }

    pub fn ai_settings(&self) -> crate::api::ai_settings::AiSettingsApi {
        crate::api::ai_settings::AiSettingsApi::new(self.clone())
    }

    pub fn settings(&self) -> crate::api::settings::SettingsApi {
        crate::api::settings::SettingsApi::new(self.clone())
    }

    pub fn api_keys(&self) -> crate::api::api_keys::ApiKeysApi {
        crate::api::api_keys::ApiKeysApi::new(self.clone())
    }

    pub fn waitlist(&self) -> crate::api::waitlist::WaitlistApi {
        crate::api::waitlist::WaitlistApi::new(self.clone())
    }

    pub fn invitations(&self) -> crate::api::invitations::InvitationsApi {
        crate::api::invitations::InvitationsApi::new(self.clone())
    }

    pub fn notifications(&self) -> crate::api::notifications::NotificationsApi {
        crate::api::notifications::NotificationsApi::new(self.clone())
    }

    pub fn oauth(&self) -> crate::api::oauth::OauthApi {
        crate::api::oauth::OauthApi::new(self.clone())
    }

    pub fn session(&self) -> crate::api::session::SessionApi {
        crate::api::session::SessionApi::new(self.clone())
    }

    pub fn webhooks(&self) -> crate::api::webhooks::WebhooksApi {
        crate::api::webhooks::WebhooksApi::new(self.clone())
    }

    pub fn gateway(&self) -> crate::gateway::GatewayApi {
        crate::gateway::GatewayApi::new(self.clone())
    }
}

/// Initialize the Wacht SDK with configuration
/// This MUST be called before using any API methods
pub fn init(config: WachtConfig) -> Result<(), String> {
    let mut headers = HeaderMap::new();

    // Add authentication header - API key is passed as Bearer token
    let auth_value = HeaderValue::from_str(&format!("Bearer {}", config.auth.token))
        .map_err(|_| "Invalid authentication token")?;
    headers.insert(AUTHORIZATION, auth_value);

    // Store config and headers globally (ignores if already set)
    let _ = GLOBAL_CONFIG.set(config);
    let _ = GLOBAL_HEADERS.set(headers);

    Ok(())
}

/// Initialize from environment variables
/// Requires  WACHT_API_KEY to be set
pub async fn init_from_env() -> Result<(), Box<dyn std::error::Error>> {
    let config = WachtConfig::from_env()?.load_public_key().await?;
    init(config)?;
    Ok(())
}

/// Get the global HTTP client
/// Creates a new client per-call to avoid cross-runtime issues
/// Panics if init() hasn't been called
pub fn get_client() -> Client {
    let config = GLOBAL_CONFIG
        .get()
        .expect("Wacht SDK not initialized. Call init() first");
    let headers = GLOBAL_HEADERS
        .get()
        .expect("Wacht SDK not initialized. Call init() first");

    ClientBuilder::new()
        .timeout(config.timeout)
        .user_agent(&config.user_agent)
        .default_headers(headers.clone())
        .pool_max_idle_per_host(500)
        .pool_idle_timeout(Duration::from_secs(90))
        .build()
        .expect("Failed to create HTTP client")
}

/// Get the current configuration
/// Panics if init() hasn't been called
pub fn get_config() -> WachtConfig {
    GLOBAL_CONFIG
        .get()
        .expect("Wacht SDK not initialized. Call init() first")
        .clone()
}

/// Check if the SDK has been initialized
pub fn is_initialized() -> bool {
    GLOBAL_CONFIG.get().is_some()
}

/// Get the public key if one is configured
pub fn get_public_signing_key() -> Option<String> {
    GLOBAL_CONFIG
        .get()
        .and_then(|config| config.public_signing_key.clone())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKeyResponse {
    pub data: PublicKeyData,
    pub errors: Option<serde_json::Value>,
    pub message: String,
    pub session: Option<serde_json::Value>,
    pub status: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKeyData {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub deployment_id: i64,
    pub public_key: String,
}

pub async fn fetch_public_key(base_url: &str) -> Result<String, crate::Error> {
    let client = reqwest::Client::new();
    let url = format!("{base_url}/.well-known/jwk");

    let response = client.get(&url).send().await?;
    let status = response.status();

    if status.is_success() {
        let key_response: PublicKeyResponse = response.json().await?;
        Ok(key_response.data.public_key)
    } else {
        let error_body = response.text().await?;
        Err(crate::Error::Api {
            status,
            message: format!("Failed to fetch public key: {error_body}"),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}
