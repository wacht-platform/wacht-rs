use serde::{Deserialize, Serialize};
use crate::models::JwtClaims;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateJwtTemplateRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub claims: JwtClaims,
    pub ttl: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

impl CreateJwtTemplateRequest {
    pub fn new(name: String, claims: JwtClaims, ttl: i32) -> CreateJwtTemplateRequest {
        CreateJwtTemplateRequest {
            name,
            description: None,
            claims,
            ttl,
            is_default: None,
        }
    }
}

