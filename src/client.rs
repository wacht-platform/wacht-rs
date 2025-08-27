use once_cell::sync::Lazy;
use reqwest::{
    Client, ClientBuilder,
    header::{AUTHORIZATION, HeaderMap, HeaderName, HeaderValue},
};
use serde::{Deserialize, Serialize};
use std::sync::RwLock;
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
    client: Client,
}

// Global client with lazy initialization
pub(crate) static GLOBAL_CLIENT: Lazy<RwLock<Option<GlobalClient>>> =
    Lazy::new(|| RwLock::new(None));

/// Initialize the Wacht SDK with configuration
/// This MUST be called before using any API methods
pub fn init(config: WachtConfig) -> Result<(), String> {
    let mut headers = HeaderMap::new();

    // Add authentication header - API key is passed as Bearer token
    let auth_value = HeaderValue::from_str(&format!("Bearer {}", config.auth.token))
        .map_err(|_| "Invalid authentication token")?;
    headers.insert(AUTHORIZATION, auth_value);

    // Build HTTP client with authentication headers
    let client = ClientBuilder::new()
        .timeout(config.timeout)
        .user_agent(&config.user_agent)
        .default_headers(headers)
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let mut global_client = GLOBAL_CLIENT.write().unwrap();
    *global_client = Some(GlobalClient { config, client });

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
/// Panics if init() hasn't been called
pub fn get_client() -> Client {
    let global_client = GLOBAL_CLIENT.read().unwrap();
    global_client
        .as_ref()
        .expect("Wacht SDK not initialized. Call init() first")
        .client
        .clone()
}

/// Get the current configuration
/// Panics if init() hasn't been called
pub fn get_config() -> WachtConfig {
    let global_client = GLOBAL_CLIENT.read().unwrap();
    global_client
        .as_ref()
        .expect("Wacht SDK not initialized. Call init() first")
        .config
        .clone()
}

/// Check if the SDK has been initialized
pub fn is_initialized() -> bool {
    GLOBAL_CLIENT.read().unwrap().is_some()
}

/// Get the public key if one is configured
pub fn get_public_signing_key() -> Option<String> {
    let global_client = GLOBAL_CLIENT.read().unwrap();
    global_client
        .as_ref()
        .and_then(|client| client.config.public_signing_key.clone())
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
    let url = format!("{}/.well-known/jwk", base_url);

    let response = client.get(&url).send().await?;
    let status = response.status();

    if status.is_success() {
        let key_response: PublicKeyResponse = response.json().await?;
        Ok(key_response.data.public_key)
    } else {
        let error_body = response.text().await?;
        Err(crate::Error::Api {
            status,
            message: format!("Failed to fetch public key: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}
