#[cfg(feature = "axum")]
pub mod auth;

#[cfg(feature = "axum")]
pub mod layer;

#[cfg(feature = "axum")]
pub mod extractors;

#[cfg(feature = "axum")]
pub mod permission_layer;

// Core auth types
#[cfg(feature = "axum")]
pub use auth::{
    extract_auth_context, require_permission_middleware, AuthConfig, AuthContext, PermissionScope,
};

// Authentication layer
#[cfg(feature = "axum")]
pub use layer::AuthLayer;

// Handler extractors
#[cfg(feature = "axum")]
pub use extractors::{OptionalAuth, Permission, RequireAuth, RequirePermission};

// Permission layers
#[cfg(feature = "axum")]
pub use permission_layer::{MultiplePermissionLayers, PermissionLayer, RequireAnyPermissionLayer};
