use serde::{Deserialize, Serialize};

/// Request for generating a JWT token
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenerateTokenRequest {
    /// The session ID to generate a token for
    pub session_id: i64,
    /// Name of the JWT template to use. If not provided, 'default' template will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

impl GenerateTokenRequest {
    /// Create a new GenerateTokenRequest
    pub fn new(session_id: i64) -> GenerateTokenRequest {
        GenerateTokenRequest {
            session_id,
            template: None,
        }
    }

    /// Set the JWT template name
    pub fn with_template(mut self, template: String) -> Self {
        self.template = Some(template);
        self
    }
}