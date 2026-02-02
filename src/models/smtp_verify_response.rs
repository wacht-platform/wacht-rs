use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmtpVerifyResponse {
    pub success: bool,
    pub message: Option<String>,
}
