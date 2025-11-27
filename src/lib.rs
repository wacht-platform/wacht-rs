#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_repr;
extern crate url;

pub mod api;
pub mod client;
pub mod error;
pub mod gateway;
#[cfg(feature = "axum")]
pub mod middleware;
pub mod models;

// Re-export commonly used types
pub use client::{get_public_signing_key, init, init_from_env, is_initialized, WachtConfig};
pub use error::{Error, Result};

// Re-export the API modules for easy access
pub use api::{
    agents, analytics, api_keys, deployments, execution_context, health, knowledge_bases, 
    organizations, settings, tools, users, webhooks, workflows, workspaces,
};