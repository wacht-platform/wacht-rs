use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateJwtTemplateRequest {
    pub name: String,
    pub token_lifetime: i64,
    #[serde(default = "default_allowed_clock_skew")]
    pub allowed_clock_skew: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_signing_key: Option<CustomSigningKey>,
    pub template: Value,
}

fn default_allowed_clock_skew() -> i64 {
    0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomSigningKey {
    pub key_id: String,
    pub key_value: String,
}

impl CreateJwtTemplateRequest {
    pub fn new(name: String, token_lifetime: i64, template: Value) -> CreateJwtTemplateRequest {
        CreateJwtTemplateRequest {
            name,
            token_lifetime,
            allowed_clock_skew: 0,
            custom_signing_key: None,
            template,
        }
    }

    pub fn with_allowed_clock_skew(mut self, skew: i64) -> Self {
        self.allowed_clock_skew = skew;
        self
    }

    pub fn with_custom_signing_key(mut self, key: CustomSigningKey) -> Self {
        self.custom_signing_key = Some(key);
        self
    }
}

