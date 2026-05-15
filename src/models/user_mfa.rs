use serde::{Deserialize, Serialize};

/// Admin-provided base32 TOTP secret + optional label for the authenticator
/// app. Whitespace and `-` separators in the secret are stripped before
/// validation; the secret must decode to at least 16 bytes (128 bits).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAuthenticatorRequest {
    pub secret: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuthenticatorResponse {
    pub id: String,
    /// otpauth:// URL with the secret embedded — render as a QR code, or
    /// copy as-is and share out-of-band.
    pub otp_url: String,
}

/// Returned by `POST /users/{id}/backup-codes/regenerate` — the freshly
/// generated codes are returned exactly once and should be displayed to the
/// user immediately.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegeneratedBackupCodesResponse {
    pub backup_codes: Vec<String>,
}
