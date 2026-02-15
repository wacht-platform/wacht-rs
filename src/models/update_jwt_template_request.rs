use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Custom signing key configuration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomSigningKey {
    /// Whether the custom signing key is enabled
    pub enabled: bool,
    /// The key value
    pub key: String,
    /// The algorithm used for signing
    pub algorithm: String,
}

/// Request to update a JWT template
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateJwtTemplateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_lifetime: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_clock_skew: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_signing_key: Option<CustomSigningKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<Value>,
}

impl UpdateJwtTemplateRequest {
    pub fn new() -> UpdateJwtTemplateRequest {
        UpdateJwtTemplateRequest {
            name: None,
            token_lifetime: None,
            allowed_clock_skew: None,
            custom_signing_key: None,
            template: None,
        }
    }
}
