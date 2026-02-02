use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Issuer (iss) claim - identifies the principal that issued the JWT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    /// Subject (sub) claim - identifies the principal that is the subject of the JWT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,
    /// Audience (aud) claim - identifies the recipients that the JWT is intended for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,
    /// Expiration time (exp) claim - identifies the expiration time on or after which the JWT MUST NOT be accepted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    /// Not before (nbf) claim - identifies the time before which the JWT MUST NOT be accepted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<i64>,
    /// Issued at (iat) claim - identifies the time at which the JWT was issued
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat: Option<i64>,
    /// JWT ID (jti) claim - provides a unique identifier for the JWT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
    /// Custom claims - additional key-value pairs for application-specific data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_claims: Option<HashMap<String, serde_json::Value>>,
}

impl JwtClaims {
    /// Create a new empty JwtClaims structure
    pub fn new() -> Self {
        Self {
            iss: None,
            sub: None,
            aud: None,
            exp: None,
            nbf: None,
            iat: None,
            jti: None,
            custom_claims: None,
        }
    }

    /// Create a new JwtClaims with subject and expiration time
    pub fn with_subject(sub: String, exp: i64) -> Self {
        Self {
            sub: Some(sub),
            exp: Some(exp),
            ..Default::default()
        }
    }

    /// Set the issuer claim
    pub fn with_issuer(mut self, iss: String) -> Self {
        self.iss = Some(iss);
        self
    }

    /// Set the audience claim
    pub fn with_audience(mut self, aud: String) -> Self {
        self.aud = Some(aud);
        self
    }

    /// Set the issued at claim
    pub fn with_issued_at(mut self, iat: i64) -> Self {
        self.iat = Some(iat);
        self
    }

    /// Set the not before claim
    pub fn with_not_before(mut self, nbf: i64) -> Self {
        self.nbf = Some(nbf);
        self
    }

    /// Set the JWT ID claim
    pub fn with_id(mut self, jti: String) -> Self {
        self.jti = Some(jti);
        self
    }

    /// Add a custom claim
    pub fn with_custom_claim(mut self, key: String, value: serde_json::Value) -> Self {
        self.custom_claims
            .get_or_insert_with(HashMap::new)
            .insert(key, value);
        self
    }

    /// Add multiple custom claims
    pub fn with_custom_claims(mut self, claims: HashMap<String, serde_json::Value>) -> Self {
        self.custom_claims = Some(claims);
        self
    }
}

impl Default for JwtClaims {
    fn default() -> Self {
        Self::new()
    }
}
