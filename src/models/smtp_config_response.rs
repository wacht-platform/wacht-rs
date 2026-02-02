use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmtpConfigResponse {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub from_email: String,
    pub use_tls: bool,
    pub verified: bool,
}
