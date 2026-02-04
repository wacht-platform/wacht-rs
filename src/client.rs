use reqwest::{
    Client, ClientBuilder,
    header::{AUTHORIZATION, HeaderMap, HeaderName, HeaderValue},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::OnceLock;
use std::time::Duration;

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
        let frontend_host =
            std::env::var("WACHT_FRONTEND_HOST").map_err(|_| "WACHT_FRONTEND_HOST must be set")?;

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

pub(crate) struct GlobalClient {
    config: WachtConfig,
    client: Arc<Client>,
    headers: HeaderMap,
}

// Global client with lazy initialization using OnceLock for thread safety
static GLOBAL_CONFIG: OnceLock<WachtConfig> = OnceLock::new();
static GLOBAL_HEADERS: OnceLock<HeaderMap> = OnceLock::new();

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
    let config = GLOBAL_CONFIG.get()
        .expect("Wacht SDK not initialized. Call init() first");
    let headers = GLOBAL_HEADERS.get()
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
    GLOBAL_CONFIG.get()
        .expect("Wacht SDK not initialized. Call init() first")
        .clone()
}

/// Check if the SDK has been initialized
pub fn is_initialized() -> bool {
    GLOBAL_CONFIG.get().is_some()
}

/// Get the public key if one is configured
pub fn get_public_signing_key() -> Option<String> {
    GLOBAL_CONFIG.get()
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
