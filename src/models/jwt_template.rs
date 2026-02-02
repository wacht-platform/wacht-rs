use serde::{Deserialize, Serialize};
use crate::models::JwtClaims;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JwtTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims: Option<JwtClaims>,
    /// Token time-to-live in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl JwtTemplate {
    pub fn new() -> JwtTemplate {
        JwtTemplate {
            id: None,
            name: None,
            description: None,
            claims: None,
            ttl: None,
            is_default: None,
            created_at: None,
            updated_at: None,
        }
    }
}

