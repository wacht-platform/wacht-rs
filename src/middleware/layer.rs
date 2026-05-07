//! Tower Layer implementation for JWT authentication.
//!
//! This module provides a Tower-compatible layer that can be used with Axum
//! to add JWT authentication to routes.

#![cfg(feature = "axum")]

use axum::{
    body::Body,
    extract::Request,
    http::{StatusCode, header},
    response::Response,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header, jwk::Jwk};
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::{debug, error, info, warn};

use super::auth::{AuthConfig, AuthContext, TokenClaims};

/// Tower Layer for JWT authentication.
///
/// This layer validates JWT tokens in the Authorization header and adds
/// authentication context to request extensions.
///
/// # Example
/// ```ignore
/// use wacht::middleware::AuthLayer;
///
/// let app = Router::new()
///     .route("/protected", get(handler))
///     .layer(AuthLayer::new());
/// ```
#[derive(Clone)]
pub struct AuthLayer {
    config: Arc<AuthConfig>,
}

impl Default for AuthLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthLayer {
    /// Create a new authentication layer using the public key from global SDK configuration.
    ///
    /// # Panics
    /// Panics if the SDK hasn't been initialized with a public key.
    pub fn new() -> Self {
        let public_key = crate::get_public_signing_key().unwrap_or_default();
        let public_jwks = crate::get_public_signing_jwks();
        if public_key.is_empty() && public_jwks.is_none() {
            panic!(
                "Public signing material must be configured in Wacht SDK. Initialize SDK with WachtConfig::with_public_key() or load_public_key()"
            );
        }

        Self {
            config: Arc::new(AuthConfig {
                public_key,
                public_jwks,
                allowed_clock_skew: 5,
                validate_exp: true,
                validate_nbf: true,
                required_issuer: None,
            }),
        }
    }

    /// Try to create a new authentication layer using the public key from global SDK configuration.
    ///
    /// Returns None if the SDK hasn't been initialized with a public key.
    pub fn try_new() -> Option<Self> {
        let public_key = crate::get_public_signing_key().unwrap_or_default();
        let public_jwks = crate::get_public_signing_jwks();
        if public_key.is_empty() && public_jwks.is_none() {
            return None;
        }

        Some(Self {
            config: Arc::new(AuthConfig {
                public_key,
                public_jwks,
                allowed_clock_skew: 5,
                validate_exp: true,
                validate_nbf: true,
                required_issuer: None,
            }),
        })
    }

    /// Create a new authentication layer with a specific public key.
    ///
    /// Use this when you need to override the global SDK configuration.
    pub fn with_public_key(key: impl Into<String>) -> Self {
        Self {
            config: Arc::new(AuthConfig {
                public_key: key.into(),
                public_jwks: None,
                allowed_clock_skew: 5,
                validate_exp: true,
                validate_nbf: true,
                required_issuer: None,
            }),
        }
    }

    /// Set the public key for token verification.
    pub fn public_key(mut self, key: impl Into<String>) -> Self {
        Arc::make_mut(&mut self.config).public_key = key.into();
        self
    }

    /// Set the allowed clock skew in seconds (default: 5).
    pub fn allowed_clock_skew(mut self, skew: u64) -> Self {
        Arc::make_mut(&mut self.config).allowed_clock_skew = skew;
        self
    }

    /// Set the required issuer claim value.
    pub fn required_issuer(mut self, issuer: impl Into<String>) -> Self {
        Arc::make_mut(&mut self.config).required_issuer = Some(issuer.into());
        self
    }

    /// Set whether to validate token expiration (default: true).
    pub fn validate_exp(mut self, validate: bool) -> Self {
        Arc::make_mut(&mut self.config).validate_exp = validate;
        self
    }

    /// Set whether to validate not-before claim (default: true).
    pub fn validate_nbf(mut self, validate: bool) -> Self {
        Arc::make_mut(&mut self.config).validate_nbf = validate;
        self
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthService {
            inner,
            config: self.config.clone(),
        }
    }
}

/// Tower Service that applies JWT authentication to requests.
///
/// This service validates JWT tokens and adds authentication context
/// to request extensions before forwarding to the inner service.
#[derive(Clone)]
pub struct AuthService<S> {
    inner: S,
    config: Arc<AuthConfig>,
}

impl<S> Service<Request<Body>> for AuthService<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static + Clone,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = std::convert::Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match self.inner.poll_ready(cx) {
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Ready(Err(_)) => {
                // Log error and convert to our error type
                error!("Inner service poll_ready returned error");
                Poll::Ready(Ok(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let config = self.config.clone();
        let mut inner = self.inner.clone();
        let method = req.method().clone();
        let uri = req.uri().clone();

        Box::pin(async move {
            debug!(method = %method, uri = %uri, "Processing request");

            // Validate token and extract claims
            match validate_token(req, &config).await {
                Ok((mut req, auth_context)) => {
                    req.extensions_mut().insert(auth_context);

                    // Call the inner service
                    match inner.call(req).await {
                        Ok(response) => Ok(response),
                        Err(_) => {
                            error!("Inner service call failed");
                            Ok(error_response(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "Internal server error",
                            ))
                        }
                    }
                }
                Err(response) => {
                    debug!("Token validation failed");
                    Ok(response)
                }
            }
        })
    }
}

/// Validate JWT token and return the request with auth context.
///
/// # Arguments
/// * `req` - The incoming HTTP request
/// * `config` - Authentication configuration
///
/// # Returns
/// * `Ok((Request, AuthContext))` - Valid token, returns request and auth context
/// * `Err(Response)` - Invalid token, returns error response
async fn validate_token(
    req: Request<Body>,
    config: &AuthConfig,
) -> Result<(Request<Body>, AuthContext), Response> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            debug!("Missing authorization header");
            error_response(StatusCode::UNAUTHORIZED, "Missing authorization header")
        })?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| error_response(StatusCode::UNAUTHORIZED, "Invalid authorization format"))?;

    let header = decode_header(token).map_err(|e| {
        error_response(
            StatusCode::UNAUTHORIZED,
            &format!("Invalid token header: {e}"),
        )
    })?;

    let algorithm = match header.alg {
        Algorithm::HS256 => Algorithm::HS256,
        Algorithm::HS384 => Algorithm::HS384,
        Algorithm::HS512 => Algorithm::HS512,
        Algorithm::RS256 => Algorithm::RS256,
        Algorithm::RS384 => Algorithm::RS384,
        Algorithm::RS512 => Algorithm::RS512,
        Algorithm::ES256 => Algorithm::ES256,
        Algorithm::ES384 => Algorithm::ES384,
        _ => {
            return Err(error_response(
                StatusCode::UNAUTHORIZED,
                "Unsupported algorithm",
            ));
        }
    };

    let decoding_key = build_decoding_key(token, &header, algorithm, config)?;

    let mut validation = Validation::new(algorithm);
    validation.leeway = config.allowed_clock_skew;
    validation.validate_exp = config.validate_exp;
    validation.validate_nbf = config.validate_nbf;

    if let Some(ref issuer) = config.required_issuer {
        validation.set_issuer(&[issuer]);
    }

    // Decode and validate token
    let token_data = decode::<TokenClaims>(token, &decoding_key, &validation)
        .map_err(|e| error_response(StatusCode::UNAUTHORIZED, &format!("Invalid token: {e}")))?;

    // Return request and auth context
    Ok((
        req,
        AuthContext {
            user_id: token_data.claims.sub.clone(),
            session_id: token_data.claims.sid.clone(),
            organization_id: token_data.claims.organization.clone(),
            workspace_id: token_data.claims.workspace.clone(),
            permissions: token_data.claims.permissions.clone(),
            claims: token_data.claims,
        },
    ))
}

fn build_decoding_key(
    token: &str,
    header: &jsonwebtoken::Header,
    algorithm: Algorithm,
    config: &AuthConfig,
) -> Result<DecodingKey, Response> {
    if let Some(jwk) = select_jwk_for_token(header, config) {
        return DecodingKey::from_jwk(&jwk).map_err(|e| {
            error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("Invalid JWK for token verification: {e}"),
            )
        });
    }

    match algorithm {
        Algorithm::ES256 | Algorithm::ES384 => {
            DecodingKey::from_ec_pem(config.public_key.as_bytes()).map_err(|e| {
                error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &format!("Invalid EC public key: {e}"),
                )
            })
        }
        Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => {
            DecodingKey::from_rsa_pem(config.public_key.as_bytes()).map_err(|e| {
                error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &format!("Invalid RSA public key: {e}"),
                )
            })
        }
        Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => {
            Ok(DecodingKey::from_secret(config.public_key.as_bytes()))
        }
        _ => Err(error_response(
            StatusCode::UNAUTHORIZED,
            &format!(
                "Unsupported algorithm for token: {}",
                token.split('.').next().unwrap_or("unknown")
            ),
        )),
    }
}

fn select_jwk_for_token(header: &jsonwebtoken::Header, config: &AuthConfig) -> Option<Jwk> {
    let jwks = config.public_jwks.as_ref()?;

    let matching = jwks.keys.iter().find(|key| {
        if let Some(header_kid) = header.kid.as_ref() {
            if key.kid.as_ref() != Some(header_kid) {
                return false;
            }
        }

        if let Some(alg) = key.alg.as_ref() {
            if alg != algorithm_name(header.alg) {
                return false;
            }
        }

        true
    })?;

    serde_json::to_value(matching)
        .ok()
        .and_then(|value| serde_json::from_value::<Jwk>(value).ok())
}

fn algorithm_name(algorithm: Algorithm) -> &'static str {
    match algorithm {
        Algorithm::HS256 => "HS256",
        Algorithm::HS384 => "HS384",
        Algorithm::HS512 => "HS512",
        Algorithm::ES256 => "ES256",
        Algorithm::ES384 => "ES384",
        Algorithm::RS256 => "RS256",
        Algorithm::RS384 => "RS384",
        Algorithm::RS512 => "RS512",
        Algorithm::PS256 => "PS256",
        Algorithm::PS384 => "PS384",
        Algorithm::PS512 => "PS512",
        Algorithm::EdDSA => "EdDSA",
    }
}

/// Create an HTTP error response with the given status and message.
/// The error message is included in both the body and X-Auth-Error header.
fn error_response(status: StatusCode, message: &str) -> Response {
    // Sanitize message for use in headers (remove non-ASCII and control characters)
    let sanitized_message = message
        .chars()
        .filter(|c| c.is_ascii() && !c.is_control())
        .take(1000) // Limit header length
        .collect::<String>();

    match Response::builder()
        .status(status)
        .header("X-Auth-Error", sanitized_message)
        .header("WWW-Authenticate", "Bearer")
        .body(Body::from(message.to_string()))
    {
        Ok(response) => response,
        Err(e) => {
            // Fallback response if headers fail
            error!(error = %e, "Failed to build auth error response");
            Response::builder()
                .status(status)
                .body(Body::from(format!("Authentication error: {status}")))
                .unwrap_or_else(|_| {
                    // Last resort fallback
                    Response::new(Body::from("Authentication error"))
                })
        }
    }
}
