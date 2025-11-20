use std::fmt;

#[derive(Debug)]
pub enum Error {
    /// HTTP request error
    Request(reqwest::Error),
    /// JSON serialization/deserialization error
    Json(serde_json::Error),
    /// IO error
    Io(std::io::Error),
    /// API error response
    Api {
        status: reqwest::StatusCode,
        message: String,
        details: Option<serde_json::Value>,
    },
    /// Configuration error
    Config(String),
    /// Authentication error
    Auth(String),
    /// Invalid request error (validation, serialization, etc.)
    InvalidRequest(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Request(e) => write!(f, "Request error: {}", e),
            Error::Json(e) => write!(f, "JSON error: {}", e),
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Api { status, message, .. } => {
                write!(f, "API error ({}): {}", status, message)
            }
            Error::Config(msg) => write!(f, "Configuration error: {}", msg),
            Error::Auth(msg) => write!(f, "Authentication error: {}", msg),
            Error::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Request(e) => Some(e),
            Error::Json(e) => Some(e),
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Request(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;