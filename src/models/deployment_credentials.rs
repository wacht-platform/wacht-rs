use serde::{Deserialize, Serialize};

/// API key portion of a deployment credentials response. The `secret` is
/// only returned at creation time — store it immediately, the SDK has no
/// way to recover it later.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentCredentialsApiKey {
    pub id: String,
    pub secret: String,
    pub prefix: String,
    pub suffix: String,
    /// API auth app the key belongs to.
    pub app_slug: String,
}

/// Fresh deployment credentials minted by `POST /credentials`. Typically
/// consumed by bootstrap scripts and the `wacht` CLI, not on every request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentCredentialsResponse {
    pub publishable_key: String,
    pub frontend_host: String,
    pub backend_host: String,
    pub api_key: DeploymentCredentialsApiKey,
}
