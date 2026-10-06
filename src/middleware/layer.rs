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
        if public_key.trim().is_empty() && public_jwks.is_none() {
            panic!(
                "Public signing material must be configured in Wacht SDK. Initialize SDK with WachtConfig::with_public_key() or load_public_key()"
            );
        }

        Self {
            config: Arc::new(AuthConfig {
                public_key,
                public_jwks,
                ..AuthConfig::default()
            }),
        }
    }

    /// Try to create a new authentication layer using the public key from global SDK configuration.
    ///
    /// Returns None if the SDK hasn't been initialized with a public key.
    pub fn try_new() -> Option<Self> {
        let public_key = crate::get_public_signing_key().unwrap_or_default();
        let public_jwks = crate::get_public_signing_jwks();
        if public_key.trim().is_empty() && public_jwks.is_none() {
            return None;
        }

        Some(Self {
            config: Arc::new(AuthConfig {
                public_key,
                public_jwks,
                ..AuthConfig::default()
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
                ..AuthConfig::default()
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

    /// Set the required audience claim value.
    pub fn required_audience(mut self, audience: impl Into<String>) -> Self {
        Arc::make_mut(&mut self.config).required_audience = Some(audience.into());
        self
    }

    /// Restrict accepted algorithms (intersected with those implied by the key type).
    pub fn allowed_algorithms(mut self, algorithms: impl IntoIterator<Item = Algorithm>) -> Self {
        Arc::make_mut(&mut self.config).allowed_algorithms = Some(algorithms.into_iter().collect());
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

    if matches!(
        header.alg,
        Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512
    ) {
        return Err(error_response(
            StatusCode::UNAUTHORIZED,
            "Unsupported algorithm",
        ));
    }

    let (decoding_key, algorithm) = resolve_verification_key(&header, config)?;

    let mut validation = Validation::new(algorithm);
    validation.algorithms = vec![algorithm];
    validation.leeway = config.allowed_clock_skew;
    validation.validate_exp = config.validate_exp;
    validation.validate_nbf = config.validate_nbf;

    let mut required_claims = vec!["exp"];
    if let Some(ref issuer) = config.required_issuer {
        validation.set_issuer(&[issuer]);
        required_claims.push("iss");
    }
    if let Some(ref audience) = config.required_audience {
        validation.set_audience(&[audience]);
        required_claims.push("aud");
    }
    validation.set_required_spec_claims(&required_claims);

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

const RSA_ALGORITHMS: &[Algorithm] = &[
    Algorithm::RS256,
    Algorithm::RS384,
    Algorithm::RS512,
    Algorithm::PS256,
    Algorithm::PS384,
    Algorithm::PS512,
];

/// Resolves the key and the algorithm pinned by its key type; the header alg is only
/// accepted if it matches. If a JWKS is configured, the PEM key is never consulted.
fn resolve_verification_key(
    header: &jsonwebtoken::Header,
    config: &AuthConfig,
) -> Result<(DecodingKey, Algorithm), Response> {
    let (decoding_key, key_algorithms) = match config.public_jwks.as_ref() {
        Some(jwks) => {
            let jwk = jwks
                .keys
                .iter()
                .find(|key| {
                    header
                        .kid
                        .as_ref()
                        .is_none_or(|kid| key.kid.as_ref() == Some(kid))
                        && jwk_algorithms(key).contains(&header.alg)
                })
                .ok_or_else(|| {
                    error_response(StatusCode::UNAUTHORIZED, "No matching verification key")
                })?;
            let parsed = serde_json::to_value(jwk)
                .ok()
                .and_then(|value| serde_json::from_value::<Jwk>(value).ok())
                .ok_or_else(|| {
                    error_response(StatusCode::INTERNAL_SERVER_ERROR, "Invalid JWK")
                })?;
            let key = DecodingKey::from_jwk(&parsed).map_err(|e| {
                error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &format!("Invalid JWK for token verification: {e}"),
                )
            })?;
            (key, jwk_algorithms(jwk))
        }
        None => pem_decoding_key(&config.public_key)?,
    };

    let allowed_by_config = config
        .allowed_algorithms
        .as_ref()
        .is_none_or(|allowed| allowed.contains(&header.alg));
    if !allowed_by_config || !key_algorithms.contains(&header.alg) {
        return Err(error_response(
            StatusCode::UNAUTHORIZED,
            "Token algorithm not allowed for verification key",
        ));
    }

    Ok((decoding_key, header.alg))
}

fn jwk_algorithms(jwk: &crate::models::Jwk) -> Vec<Algorithm> {
    let by_type: &[Algorithm] = match (jwk.kty.as_str(), jwk.crv.as_deref()) {
        ("EC", Some("P-256")) => &[Algorithm::ES256],
        ("EC", Some("P-384")) => &[Algorithm::ES384],
        ("RSA", _) => RSA_ALGORITHMS,
        ("OKP", Some("Ed25519")) => &[Algorithm::EdDSA],
        _ => &[],
    };
    by_type
        .iter()
        .copied()
        .filter(|alg| jwk.alg.as_deref().is_none_or(|a| a == algorithm_name(*alg)))
        .collect()
}

fn pem_decoding_key(pem: &str) -> Result<(DecodingKey, Vec<Algorithm>), Response> {
    const OID_P256: &[u8] = &[0x06, 0x08, 0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x03, 0x01, 0x07];
    const OID_P384: &[u8] = &[0x06, 0x05, 0x2B, 0x81, 0x04, 0x00, 0x22];
    const OID_RSA: &[u8] = &[0x06, 0x09, 0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01];
    const OID_ED25519: &[u8] = &[0x06, 0x03, 0x2B, 0x65, 0x70];

    let misconfigured = |msg: &str| error_response(StatusCode::INTERNAL_SERVER_ERROR, msg);
    let pem = pem.trim();
    if pem.is_empty() {
        return Err(misconfigured("No verification key configured"));
    }

    use base64::Engine;
    let body: String = pem
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .map(str::trim)
        .collect();
    let der = base64::engine::general_purpose::STANDARD
        .decode(body)
        .map_err(|_| misconfigured("Invalid public key PEM"))?;
    let has = |oid: &[u8]| der.windows(oid.len()).any(|w| w == oid);

    let (key, algorithms) = if has(OID_P256) {
        (DecodingKey::from_ec_pem(pem.as_bytes()), vec![Algorithm::ES256])
    } else if has(OID_P384) {
        (DecodingKey::from_ec_pem(pem.as_bytes()), vec![Algorithm::ES384])
    } else if has(OID_RSA) || pem.starts_with("-----BEGIN RSA PUBLIC KEY-----") {
        (DecodingKey::from_rsa_pem(pem.as_bytes()), RSA_ALGORITHMS.to_vec())
    } else if has(OID_ED25519) {
        (DecodingKey::from_ed_pem(pem.as_bytes()), vec![Algorithm::EdDSA])
    } else {
        return Err(misconfigured("Unsupported public key type"));
    };
    let key = key.map_err(|e| misconfigured(&format!("Invalid public key: {e}")))?;
    Ok((key, algorithms))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Jwk as ModelJwk, JwksDocument};
    use base64::Engine;
    use jsonwebtoken::{EncodingKey, Header, encode};

    const PRIVATE_PEM: &str = "-----BEGIN PRIVATE KEY-----
MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgAPJlZ5isUZONbIII
kkVAQoVmh0hWR8WxfhkM+JjKTQuhRANCAATuXhF7Bk6lePn6kzVAC7qIum5roTPV
PqXuI/JIen2YOxazEucwpThE5CylEvMqS+j7BRoEf+ZHYJJbhlPWOGDx
-----END PRIVATE KEY-----";
    const PUBLIC_PEM: &str = "-----BEGIN PUBLIC KEY-----
MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE7l4RewZOpXj5+pM1QAu6iLpua6Ez
1T6l7iPySHp9mDsWsxLnMKU4ROQspRLzKkvo+wUaBH/mR2CSW4ZT1jhg8Q==
-----END PUBLIC KEY-----";

    fn jwks_config() -> AuthConfig {
        let jwk: ModelJwk = serde_json::from_value(serde_json::json!({
            "kty": "EC",
            "kid": "k1",
            "alg": "ES256",
            "crv": "P-256",
            "x": "7l4RewZOpXj5-pM1QAu6iLpua6Ez1T6l7iPySHp9mDs",
            "y": "FrMS5zClOETkLKUS8ypL6PsFGgR_5kdgkluGU9Y4YPE",
        }))
        .unwrap();
        AuthConfig {
            public_jwks: Some(JwksDocument { keys: vec![jwk] }),
            ..AuthConfig::default()
        }
    }

    fn pem_config() -> AuthConfig {
        AuthConfig {
            public_key: PUBLIC_PEM.to_string(),
            ..AuthConfig::default()
        }
    }

    fn claims(iss: &str) -> serde_json::Value {
        let now = chrono::Utc::now().timestamp();
        serde_json::json!({
            "iss": iss, "sub": "user_1", "sid": "sess_1", "iat": now, "exp": now + 300,
            "organization": "org_victim",
        })
    }

    fn hs256(secret: &[u8], kid: Option<&str>) -> String {
        let mut header = Header::new(Algorithm::HS256);
        header.kid = kid.map(str::to_string);
        encode(&header, &claims("https://issuer"), &EncodingKey::from_secret(secret)).unwrap()
    }

    fn es256(kid: Option<&str>, iss: &str) -> String {
        let mut header = Header::new(Algorithm::ES256);
        header.kid = kid.map(str::to_string);
        let key = EncodingKey::from_ec_pem(PRIVATE_PEM.as_bytes()).unwrap();
        encode(&header, &claims(iss), &key).unwrap()
    }

    async fn check(token: &str, config: &AuthConfig) -> Result<AuthContext, StatusCode> {
        let req = Request::builder()
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .body(Body::empty())
            .unwrap();
        validate_token(req, config)
            .await
            .map(|(_, ctx)| ctx)
            .map_err(|resp| resp.status())
    }

    #[tokio::test]
    async fn valid_es256_accepted() {
        assert!(check(&es256(Some("k1"), "x"), &jwks_config()).await.is_ok());
        assert!(check(&es256(None, "x"), &pem_config()).await.is_ok());
    }

    #[tokio::test]
    async fn hs256_with_empty_secret_rejected() {
        for kid in [None, Some("k1"), Some("unknown")] {
            let token = hs256(b"", kid);
            assert!(check(&token, &jwks_config()).await.is_err());
            assert!(check(&token, &pem_config()).await.is_err());
        }
        let empty_key = AuthConfig::default();
        assert!(check(&hs256(b"", None), &empty_key).await.is_err());
    }

    #[tokio::test]
    async fn hs256_with_public_pem_as_secret_rejected() {
        let token = hs256(PUBLIC_PEM.as_bytes(), None);
        assert!(check(&token, &pem_config()).await.is_err());
        let mut both = jwks_config();
        both.public_key = PUBLIC_PEM.to_string();
        assert!(check(&hs256(PUBLIC_PEM.as_bytes(), Some("unknown")), &both).await.is_err());
    }

    #[tokio::test]
    async fn unknown_kid_rejected_without_pem_fallback() {
        let mut both = jwks_config();
        both.public_key = PUBLIC_PEM.to_string();
        let token = es256(Some("unknown"), "x");
        assert_eq!(check(&token, &both).await.unwrap_err(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn header_alg_mismatching_key_type_rejected() {
        let token = es256(Some("k1"), "x");
        let mut parts: Vec<&str> = token.split('.').collect();
        let forged = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(r#"{"alg":"ES384","typ":"JWT","kid":"k1"}"#);
        parts[0] = &forged;
        let forged_token = parts.join(".");
        assert!(check(&forged_token, &jwks_config()).await.is_err());
        assert!(check(&forged_token, &pem_config()).await.is_err());
    }

    #[tokio::test]
    async fn empty_public_key_rejected() {
        let config = AuthConfig {
            public_key: "  ".to_string(),
            ..AuthConfig::default()
        };
        assert!(check(&es256(None, "x"), &config).await.is_err());
    }

    #[tokio::test]
    async fn issuer_audience_and_allowlist_enforced() {
        let mut config = jwks_config();
        config.required_issuer = Some("https://issuer".to_string());
        assert!(check(&es256(Some("k1"), "https://issuer"), &config).await.is_ok());
        assert!(check(&es256(Some("k1"), "https://evil"), &config).await.is_err());

        config.required_audience = Some("console".to_string());
        assert!(check(&es256(Some("k1"), "https://issuer"), &config).await.is_err());

        let mut config = jwks_config();
        config.allowed_algorithms = Some(vec![Algorithm::RS256]);
        assert!(check(&es256(Some("k1"), "x"), &config).await.is_err());
    }
}
