use serde::{Deserialize, Serialize};

/// Response containing a generated JWT token
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenerateTokenResponse {
    /// The generated JWT token
    pub token: String,
    /// Token expiration timestamp in milliseconds
    pub expires: i64,
}

impl GenerateTokenResponse {
    /// Create a new GenerateTokenResponse
    pub fn new(token: String, expires: i64) -> GenerateTokenResponse {
        GenerateTokenResponse { token, expires }
    }
}