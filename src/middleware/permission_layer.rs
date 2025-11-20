//! Permission-based authorization layer for Axum.
//!
//! This module provides a Tower Layer for checking permissions after authentication.

#![cfg(feature = "axum")]

use axum::{body::Body, extract::Request, http::StatusCode, response::Response};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::{debug, error, warn};

use super::auth::{AuthContext, PermissionScope};

/// Helper function to build error responses with sanitized headers
fn build_error_response(status: StatusCode, message: &str) -> Response {
    // Sanitize message for use in headers
    let sanitized_message = message
        .chars()
        .filter(|c| c.is_ascii() && !c.is_control())
        .take(1000)
        .collect::<String>();

    let mut builder = Response::builder().status(status);

    // Add auth headers
    if status == StatusCode::UNAUTHORIZED {
        builder = builder.header("WWW-Authenticate", "Bearer");
    }

    // Try to add X-Auth-Error header
    builder = builder.header("X-Auth-Error", sanitized_message);

    // Build response with body
    builder
        .body(Body::from(message.to_string()))
        .unwrap_or_else(|_| {
            // Fallback response
            Response::builder()
                .status(status)
                .body(Body::from(format!("Error: {}", status)))
                .unwrap_or_else(|_| Response::new(Body::from("Error")))
        })
}

/// Layer that checks if authenticated user has required permission.
///
/// This layer should be applied after `AuthLayer` to check specific permissions.
///
/// # Example
/// ```ignore
/// use wacht::middleware::{AuthLayer, PermissionLayer};
///
/// let app = Router::new()
///     .route("/admin", get(admin_handler))
///     .layer(PermissionLayer::organization("admin:read"))
///     .layer(AuthLayer::new());
/// ```
#[derive(Clone)]
pub struct PermissionLayer {
    permission: String,
    scope: PermissionScope,
}

impl PermissionLayer {
    /// Create a permission layer that checks organization-level permission.
    pub fn organization(permission: impl Into<String>) -> Self {
        Self {
            permission: permission.into(),
            scope: PermissionScope::Organization,
        }
    }

    /// Create a permission layer that checks workspace-level permission.
    pub fn workspace(permission: impl Into<String>) -> Self {
        Self {
            permission: permission.into(),
            scope: PermissionScope::Workspace,
        }
    }

    /// Create a permission layer with custom scope.
    pub fn new(permission: impl Into<String>, scope: PermissionScope) -> Self {
        Self {
            permission: permission.into(),
            scope,
        }
    }
}

impl<S> Layer<S> for PermissionLayer {
    type Service = PermissionService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        PermissionService {
            inner,
            permission: self.permission.clone(),
            scope: self.scope.clone(),
        }
    }
}

/// Service that checks permissions before forwarding requests.
#[derive(Clone)]
pub struct PermissionService<S> {
    inner: S,
    permission: String,
    scope: PermissionScope,
}

impl<S> Service<Request<Body>> for PermissionService<S>
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
                error!("PermissionService: Inner service poll_ready returned error");
                Poll::Ready(Ok(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let permission = self.permission.clone();
        let scope = self.scope.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Extract auth context from request extensions
            let auth_context = match req.extensions().get::<AuthContext>() {
                Some(ctx) => ctx,
                None => {
                    return Ok(build_error_response(
                        StatusCode::UNAUTHORIZED,
                        "No authentication context found",
                    ));
                }
            };

            // Check permission based on scope
            let has_permission = match scope {
                PermissionScope::Organization => auth_context
                    .permissions
                    .as_ref()
                    .map(|perms| {
                        perms
                            .organization
                            .as_ref()
                            .map(|perms| perms.contains(&permission))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false),
                PermissionScope::Workspace => auth_context
                    .permissions
                    .as_ref()
                    .map(|perms| {
                        perms
                            .workspace
                            .as_ref()
                            .map(|perms| perms.contains(&permission))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false),
            };

            if !has_permission {
                let error_msg = format!("Missing required permission: {}", permission);
                return Ok(build_error_response(StatusCode::FORBIDDEN, &error_msg));
            }

            // User has permission, forward to inner service
            match inner.call(req).await {
                Ok(response) => Ok(response),
                Err(_) => {
                    error!("PermissionService: Inner service call failed");
                    Ok(build_error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error",
                    ))
                }
            }
        })
    }
}

/// Create multiple permission layers that all must be satisfied.
///
/// All permissions must be satisfied for the request to proceed.
///
/// # Example
/// ```ignore
/// use wacht::middleware::{AuthLayer, MultiplePermissionLayers};
///
/// let app = Router::new()
///     .route("/super-admin", get(handler))
///     .layer(MultiplePermissionLayers::all(vec![
///         ("admin:read", PermissionScope::Organization),
///         ("admin:write", PermissionScope::Organization),
///     ]))
///     .layer(AuthLayer::new());
/// ```
#[derive(Clone)]
pub struct MultiplePermissionLayers {
    permissions: Vec<(String, PermissionScope)>,
    require_all: bool,
}

impl MultiplePermissionLayers {
    /// Create layers that require all permissions.
    pub fn all(permissions: Vec<(&str, PermissionScope)>) -> Self {
        Self {
            permissions: permissions
                .into_iter()
                .map(|(p, s)| (p.to_string(), s))
                .collect(),
            require_all: true,
        }
    }

    /// Create layers that require any permission.
    pub fn any(permissions: Vec<(&str, PermissionScope)>) -> Self {
        Self {
            permissions: permissions
                .into_iter()
                .map(|(p, s)| (p.to_string(), s))
                .collect(),
            require_all: false,
        }
    }
}

impl<S> Layer<S> for MultiplePermissionLayers {
    type Service = MultiplePermissionService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        MultiplePermissionService {
            inner,
            permissions: self.permissions.clone(),
            require_all: self.require_all,
        }
    }
}

/// Service that checks multiple permissions.
#[derive(Clone)]
pub struct MultiplePermissionService<S> {
    inner: S,
    permissions: Vec<(String, PermissionScope)>,
    require_all: bool,
}

impl<S> Service<Request<Body>> for MultiplePermissionService<S>
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
                error!("PermissionService: Inner service poll_ready returned error");
                Poll::Ready(Ok(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let permissions = self.permissions.clone();
        let require_all = self.require_all;
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Extract auth context from request extensions
            let auth_context = match req.extensions().get::<AuthContext>() {
                Some(ctx) => ctx,
                None => {
                    return Ok(build_error_response(
                        StatusCode::UNAUTHORIZED,
                        "No authentication context found",
                    ));
                }
            };

            // Check permissions
            let check_permission = |permission: &str, scope: &PermissionScope| -> bool {
                match scope {
                    PermissionScope::Organization => auth_context
                        .permissions
                        .as_ref()
                        .map(|perms| {
                            perms
                                .organization
                                .as_ref()
                                .map(|perms| perms.contains(&permission.to_string()))
                                .unwrap_or(false)
                        })
                        .unwrap_or(false),
                    PermissionScope::Workspace => auth_context
                        .permissions
                        .as_ref()
                        .map(|perms| {
                            perms
                                .workspace
                                .as_ref()
                                .map(|perms| perms.contains(&permission.to_string()))
                                .unwrap_or(false)
                        })
                        .unwrap_or(false),
                }
            };

            let has_permission = if require_all {
                permissions.iter().all(|(p, s)| check_permission(p, s))
            } else {
                permissions.iter().any(|(p, s)| check_permission(p, s))
            };

            if !has_permission {
                let message = if require_all {
                    format!(
                        "Missing required permissions: {}",
                        permissions
                            .iter()
                            .map(|(p, _)| p.as_str())
                            .collect::<Vec<_>>()
                            .join(" AND ")
                    )
                } else {
                    format!(
                        "Missing required permission: {}",
                        permissions
                            .iter()
                            .map(|(p, _)| p.as_str())
                            .collect::<Vec<_>>()
                            .join(" OR ")
                    )
                };

                return Ok(build_error_response(StatusCode::FORBIDDEN, &message));
            }

            // User has permission, forward to inner service
            match inner.call(req).await {
                Ok(response) => Ok(response),
                Err(_) => {
                    error!("PermissionService: Inner service call failed");
                    Ok(build_error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error",
                    ))
                }
            }
        })
    }
}

/// Create a layer that requires at least one of the given permissions.
///
/// # Example
/// ```ignore
/// use wacht::middleware::{AuthLayer, RequireAnyPermissionLayer};
///
/// let app = Router::new()
///     .route("/content", get(handler))
///     .layer(RequireAnyPermissionLayer::new(vec![
///         ("content:read", PermissionScope::Organization),
///         ("content:manage", PermissionScope::Organization),
///     ]))
///     .layer(AuthLayer::new());
/// ```
#[derive(Clone)]
pub struct RequireAnyPermissionLayer {
    permissions: Vec<(String, PermissionScope)>,
}

impl RequireAnyPermissionLayer {
    /// Create a new layer that requires any of the given permissions.
    pub fn new(permissions: Vec<(&str, PermissionScope)>) -> Self {
        Self {
            permissions: permissions
                .into_iter()
                .map(|(p, s)| (p.to_string(), s))
                .collect(),
        }
    }
}

impl<S> Layer<S> for RequireAnyPermissionLayer {
    type Service = RequireAnyPermissionService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequireAnyPermissionService {
            inner,
            permissions: self.permissions.clone(),
        }
    }
}

/// Service that checks if user has any of the required permissions.
#[derive(Clone)]
pub struct RequireAnyPermissionService<S> {
    inner: S,
    permissions: Vec<(String, PermissionScope)>,
}

impl<S> Service<Request<Body>> for RequireAnyPermissionService<S>
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
                error!("PermissionService: Inner service poll_ready returned error");
                Poll::Ready(Ok(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let permissions = self.permissions.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Extract auth context from request extensions
            let auth_context = match req.extensions().get::<AuthContext>() {
                Some(ctx) => ctx,
                None => {
                    return Ok(build_error_response(
                        StatusCode::UNAUTHORIZED,
                        "No authentication context found",
                    ));
                }
            };

            // Check if user has any of the required permissions
            let has_any_permission = permissions.iter().any(|(permission, scope)| match scope {
                PermissionScope::Organization => auth_context
                    .permissions
                    .as_ref()
                    .map(|perms| {
                        perms
                            .organization
                            .as_ref()
                            .map(|perms| perms.contains(permission))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false),
                PermissionScope::Workspace => auth_context
                    .permissions
                    .as_ref()
                    .map(|perms| {
                        perms
                            .workspace
                            .as_ref()
                            .map(|perms| perms.contains(permission))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false),
            });

            if !has_any_permission {
                let permission_list = permissions
                    .iter()
                    .map(|(p, _)| p.as_str())
                    .collect::<Vec<_>>()
                    .join(" OR ");

                let error_msg = format!("Missing required permission: {}", permission_list);
                return Ok(build_error_response(StatusCode::FORBIDDEN, &error_msg));
            }

            // User has at least one permission, forward to inner service
            match inner.call(req).await {
                Ok(response) => Ok(response),
                Err(_) => {
                    error!("PermissionService: Inner service call failed");
                    Ok(build_error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error",
                    ))
                }
            }
        })
    }
}
