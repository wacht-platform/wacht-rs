//! Axum extractors for authentication and authorization.
//!
//! These extractors can be used in handler functions to require authentication
//! or specific permissions.

#![cfg(feature = "axum")]

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use std::marker::PhantomData;

use super::auth::{AuthContext, PermissionScope};

/// Extractor that requires authentication.
///
/// If authentication fails, returns 401 Unauthorized.
///
/// # Example
/// ```ignore
/// async fn protected_handler(
///     auth: RequireAuth,
///     // other parameters...
/// ) -> impl IntoResponse {
///     println!("User ID: {}", auth.user_id);
///     // handle request...
/// }
/// ```
#[derive(Debug, Clone)]
pub struct RequireAuth(pub AuthContext);

impl std::ops::Deref for RequireAuth {
    type Target = AuthContext;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthContext>()
            .cloned()
            .map(RequireAuth)
            .ok_or((StatusCode::UNAUTHORIZED, "Authentication required"))
    }
}

/// Marker trait for permission requirements.
pub trait Permission: Send + Sync + 'static {
    const PERMISSION: &'static str;
    const SCOPE: PermissionScope;
}

/// Extractor that requires a specific permission.
///
/// The permission type parameter determines what permission to check.
///
/// # Example
/// ```ignore
/// // Define a permission
/// struct AdminRead;
/// impl Permission for AdminRead {
///     const PERMISSION: &'static str = "admin:read";
///     const SCOPE: PermissionScope = PermissionScope::Organization;
/// }
///
/// // Use in handler
/// async fn admin_handler(
///     _perm: RequirePermission<AdminRead>,
///     auth: RequireAuth,
/// ) -> impl IntoResponse {
///     // User has admin:read permission in organization
///     format!("Admin access for user {}", auth.user_id)
/// }
/// ```
#[derive(Debug)]
pub struct RequirePermission<P: Permission> {
    pub auth: AuthContext,
    _phantom: PhantomData<P>,
}

impl<P: Permission> std::ops::Deref for RequirePermission<P> {
    type Target = AuthContext;

    fn deref(&self) -> &Self::Target {
        &self.auth
    }
}

impl<S, P> FromRequestParts<S> for RequirePermission<P>
where
    S: Send + Sync,
    P: Permission,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // First check authentication
        let auth = parts
            .extensions
            .get::<AuthContext>()
            .cloned()
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Authentication required").into_response())?;

        // Then check permission
        let has_permission = match P::SCOPE {
            PermissionScope::Organization => auth
                .organization_permissions
                .as_ref()
                .map(|perms| perms.contains(&P::PERMISSION.to_string()))
                .unwrap_or(false),
            PermissionScope::Workspace => auth
                .workspace_permissions
                .as_ref()
                .map(|perms| perms.contains(&P::PERMISSION.to_string()))
                .unwrap_or(false),
        };

        if has_permission {
            Ok(RequirePermission {
                auth,
                _phantom: PhantomData,
            })
        } else {
            Err((
                StatusCode::FORBIDDEN,
                format!("Missing required permission: {}", P::PERMISSION),
            )
                .into_response())
        }
    }
}

/// Helper macro to define permissions.
///
/// # Example
/// ```ignore
/// require_permission!(CanReadUsers, "users:read", Organization);
/// require_permission!(CanManageWorkspace, "workspace:manage", Workspace);
/// ```
#[macro_export]
macro_rules! require_permission {
    ($name:ident, $permission:expr, $scope:ident) => {
        pub struct $name;

        impl $crate::middleware::extractors::Permission for $name {
            const PERMISSION: &'static str = $permission;
            const SCOPE: $crate::middleware::PermissionScope =
                $crate::middleware::PermissionScope::$scope;
        }
    };
}

/// Extractor that optionally requires authentication.
///
/// Unlike `RequireAuth`, this won't fail if no auth is present.
///
/// # Example
/// ```ignore
/// async fn maybe_protected_handler(
///     auth: OptionalAuth,
/// ) -> impl IntoResponse {
///     if let Some(auth) = auth.0 {
///         format!("Hello, user {}", auth.user_id)
///     } else {
///         "Hello, anonymous".to_string()
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct OptionalAuth(pub Option<AuthContext>);

impl<S> FromRequestParts<S> for OptionalAuth
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(OptionalAuth(parts.extensions.get::<AuthContext>().cloned()))
    }
}
