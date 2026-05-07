//! JWT authentication middleware for Axum web framework.
//!
//! This module provides middleware for validating JWT tokens and extracting
//! authentication context from requests.

#![cfg(feature = "axum")]

use axum::{
    Extension, body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response,
};
use serde::{Deserialize, Serialize};

use crate::models::JwksDocument;

/// JWT token claims structure.
///
/// Contains standard JWT claims plus Wacht-specific claims for
/// session, organization, and workspace context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    /// Token issuer (iss)
    pub iss: String,
    /// Subject - user ID (sub)
    pub sub: String,
    /// Issued at timestamp (iat)
    pub iat: i64,
    /// Expiration timestamp (exp)
    pub exp: i64,
    /// Session ID
    pub sid: String,
    /// Organization ID if user is in organization context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// Workspace ID if user is in workspace context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    /// Permissions grouped by scope
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<TokenPermissions>,
    /// Custom claims object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims: Option<serde_json::Map<String, serde_json::Value>>,
    /// Metadata object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Additional custom claims (flattened)
    #[serde(flatten)]
    pub custom_claims: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<Vec<String>>,
}

/// Authentication context extracted from JWT token.
///
/// This struct is inserted into the request extensions after successful
/// token validation and can be extracted in request handlers.
#[derive(Debug, Clone)]
pub struct AuthContext {
    /// User ID from token subject
    pub user_id: String,
    /// Session ID
    pub session_id: String,
    /// Organization ID if present
    pub organization_id: Option<String>,
    /// Workspace ID if present
    pub workspace_id: Option<String>,
    /// Permissions
    pub permissions: Option<TokenPermissions>,
    /// Full token claims
    pub claims: TokenClaims,
}

/// Configuration for JWT authentication.
#[derive(Clone)]
pub struct AuthConfig {
    /// Public key for token verification (PEM format)
    pub public_key: String,
    /// JWKS document for token verification
    pub public_jwks: Option<JwksDocument>,
    /// Allowed clock skew in seconds (default: 5)
    pub allowed_clock_skew: u64,
    /// Whether to validate token expiration (default: true)
    pub validate_exp: bool,
    /// Whether to validate not-before claim (default: true)
    pub validate_nbf: bool,
    /// Required issuer claim value
    pub required_issuer: Option<String>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            public_key: String::new(),
            public_jwks: None,
            allowed_clock_skew: 5,
            validate_exp: true,
            validate_nbf: true,
            required_issuer: None,
        }
    }
}

/// Extract authentication context from request extensions.
///
/// Use this in request handlers to access the authenticated user's context.
///
/// # Example
/// ```ignore
/// async fn handler(req: Request<Body>) -> Result<Response, StatusCode> {
///     let auth = extract_auth_context(&req)
///         .ok_or(StatusCode::UNAUTHORIZED)?;
///
///     println!("User ID: {}", auth.user_id);
///     // ... handle request
/// }
/// ```
pub fn extract_auth_context(req: &Request<Body>) -> Option<&AuthContext> {
    req.extensions().get::<AuthContext>()
}

/// Configuration for permission-based authorization middleware.
#[derive(Debug, Clone)]
pub struct RequirePermission {
    /// The permission string to check
    pub permission: String,
    /// The scope in which to check the permission
    pub scope: PermissionScope,
}

/// Scope for permission checking.
#[derive(Debug, Clone)]
pub enum PermissionScope {
    /// Check permission in organization context
    Organization,
    /// Check permission in workspace context
    Workspace,
}

impl RequirePermission {
    /// Create a permission requirement for organization scope.
    pub fn organization(permission: impl Into<String>) -> Self {
        Self {
            permission: permission.into(),
            scope: PermissionScope::Organization,
        }
    }

    /// Create a permission requirement for workspace scope.
    pub fn workspace(permission: impl Into<String>) -> Self {
        Self {
            permission: permission.into(),
            scope: PermissionScope::Workspace,
        }
    }
}

/// Middleware to check if authenticated user has required permission.
///
/// This middleware should be used after `auth_middleware` to check
/// specific permissions in organization or workspace context.
///
/// # Example
/// ```ignore
/// let app = Router::new()
///     .route("/admin", get(handler))
///     .route_layer(middleware::from_fn(auth_middleware))
///     .route_layer(middleware::from_fn(move |req, next| {
///         require_permission_middleware(
///             Extension(RequirePermission::organization("admin")),
///             req,
///             next
///         )
///     }));
/// ```
pub async fn require_permission_middleware(
    Extension(required): Extension<RequirePermission>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    let auth_context = req.extensions().get::<AuthContext>().ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            "No auth context found".to_string(),
        )
    })?;

    let has_permission = if let Some(permissions) = &auth_context.permissions {
        match required.scope {
            PermissionScope::Organization => permissions
                .organization
                .as_ref()
                .map(|perms| perms.contains(&required.permission))
                .unwrap_or(false),
            PermissionScope::Workspace => permissions
                .workspace
                .as_ref()
                .map(|perms| perms.contains(&required.permission))
                .unwrap_or(false),
        }
    } else {
        false
    };

    if !has_permission {
        return Err((
            StatusCode::FORBIDDEN,
            format!("Missing required permission: {}", required.permission),
        ));
    }

    Ok(next.run(req).await)
}
