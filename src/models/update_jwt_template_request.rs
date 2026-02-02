use serde::{Deserialize, Serialize};
use crate::models::JwtClaims;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateJwtTemplateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims: Option<JwtClaims>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

impl UpdateJwtTemplateRequest {
    pub fn new() -> UpdateJwtTemplateRequest {
        UpdateJwtTemplateRequest {
            name: None,
            description: None,
            claims: None,
            ttl: None,
            is_default: None,
        }
    }
}

