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
    /// SDK was used before calling init()
    Uninitialized(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Request(e) => write!(f, "Request error: {e}"),
            Error::Json(e) => write!(f, "JSON error: {e}"),
            Error::Io(e) => write!(f, "IO error: {e}"),
            Error::Api {
                status, message, ..
            } => {
                write!(f, "API error ({status}): {message}")
            }
            Error::Config(msg) => write!(f, "Configuration error: {msg}"),
            Error::Auth(msg) => write!(f, "Authentication error: {msg}"),
            Error::InvalidRequest(msg) => write!(f, "Invalid request: {msg}"),
            Error::Uninitialized(msg) => write!(f, "Uninitialized SDK: {msg}"),
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

impl Error {
    /// Convenience helper for constructing API errors with parsed JSON details when available.
    pub fn api_from_text(
        status: reqwest::StatusCode,
        message: impl Into<String>,
        body_text: &str,
    ) -> Self {
        Self::Api {
            status,
            message: format!("{}: {}", message.into(), body_text),
            details: serde_json::from_str(body_text).ok(),
        }
    }

    /// Returns HTTP status code when the error originated from an API response.
    pub fn status_code(&self) -> Option<reqwest::StatusCode> {
        match self {
            Self::Api { status, .. } => Some(*status),
            _ => None,
        }
    }

    /// Returns true for transport and server-side failures that are often retryable.
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Request(e) => e.is_timeout() || e.is_connect(),
            Self::Api { status, .. } => {
                status.is_server_error() || *status == reqwest::StatusCode::TOO_MANY_REQUESTS
            }
            _ => false,
        }
    }

    /// Returns true when an error clearly indicates authn/authz failure.
    pub fn is_authentication_error(&self) -> bool {
        match self {
            Self::Auth(_) => true,
            Self::Api { status, .. } => {
                *status == reqwest::StatusCode::UNAUTHORIZED
                    || *status == reqwest::StatusCode::FORBIDDEN
            }
            _ => false,
        }
    }
}
