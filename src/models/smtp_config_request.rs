use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmtpConfigRequest {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_email: String,
    #[serde(default = "default_use_tls")]
    pub use_tls: bool,
}

fn default_use_tls() -> bool {
    false
}
