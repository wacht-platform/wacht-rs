use serde::{Deserialize, Serialize};

/// One OIDC signing key for an OAuth app. The private half stays in the
/// database — only the public PEM is returned, for external verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthAppSigningKey {
    pub kid: String,
    pub algorithm: String,
    /// `active` signs new id_tokens; `retired` stays in JWKS for grace.
    pub status: String,
    pub public_key_pem: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthAppSigningKeysListResponse {
    pub keys: Vec<OAuthAppSigningKey>,
}

/// Returned from a rotate. The previously active key is automatically
/// retired (stays in JWKS); `new` takes over for signing new id_tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthAppSigningKeyRotatedResponse {
    pub new: OAuthAppSigningKey,
}
