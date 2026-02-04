use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomSigningKey {
    pub key_id: String,
    pub key_value: String,
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

