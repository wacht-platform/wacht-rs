use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{Error, Result};

const GATEWAY_URL: &str = "https://gateway.wacht.dev";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    pub window_seconds: u64,
    pub limit: u32,
    pub remaining: u32,
    pub reset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayCheckResponse {
    pub request_id: String,
    pub allowed: bool,
    pub reason: Option<GatewayDenyReason>,
    pub blocked_rule: Option<String>,
    pub key_id: i64,
    pub deployment_id: i64,
    pub app_id: i64,
    pub app_slug: String,
    pub key_name: String,
    pub permissions: Vec<String>,
    pub metadata: Value,
    pub organization_id: Option<i64>,
    pub workspace_id: Option<i64>,
    pub organization_membership_id: Option<i64>,
    pub workspace_membership_id: Option<i64>,
    pub rate_limits: Vec<RateLimitInfo>,
    pub retry_after: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GatewayDenyReason {
    PermissionDenied,
    RateLimited,
}

#[derive(Debug, Serialize)]
struct GatewayAuthzCheckRequest {
    principal: GatewayPrincipal,
    resource: String,
    method: String,
    client_ip: Option<String>,
    user_agent: Option<String>,
    required_permissions: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct GatewayPrincipal {
    #[serde(rename = "type")]
    principal_type: &'static str,
    value: String,
}

#[derive(Debug, Deserialize)]
struct GatewayAuthzCheckEnvelope {
    request_id: String,
    allowed: bool,
    reason: Option<GatewayDenyReason>,
    blocked_rule: Option<String>,
    identity: Option<GatewayIdentity>,
    permissions: Vec<String>,
    metadata: Option<Value>,
    rate_limits: Vec<RateLimitInfo>,
    retry_after: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct GatewayIdentity {
    key_id: String,
    deployment_id: String,
    app_id: String,
    app_slug: String,
    key_name: String,
    organization_id: Option<String>,
    workspace_id: Option<String>,
    organization_membership_id: Option<String>,
    workspace_membership_id: Option<String>,
}

pub async fn verify_request(
    api_key: &str,
    method: &str,
    resource: &str,
) -> Result<GatewayCheckResponse> {
    let client = reqwest::Client::new();
    let url = format!("{GATEWAY_URL}/v1/authz/check");
    let payload = GatewayAuthzCheckRequest {
        principal: GatewayPrincipal {
            principal_type: "api_key",
            value: api_key.to_string(),
        },
        resource: resource.to_string(),
        method: method.to_string(),
        client_ip: None,
        user_agent: None,
        required_permissions: None,
    };

    let response = client.post(&url).json(&payload).send().await?;

    let status = response.status();
    let body = response.text().await?;
    if status.is_success() {
        let parsed: GatewayAuthzCheckEnvelope = serde_json::from_str(&body)?;
        let identity = parsed.identity.ok_or_else(|| {
            Error::InvalidRequest("Missing identity in gateway response".to_string())
        })?;

        Ok(GatewayCheckResponse {
            request_id: parsed.request_id,
            allowed: parsed.allowed,
            reason: parsed.reason,
            blocked_rule: parsed.blocked_rule,
            key_id: parse_i64_field(&identity.key_id, "key_id")?,
            deployment_id: parse_i64_field(&identity.deployment_id, "deployment_id")?,
            app_id: parse_i64_field(&identity.app_id, "app_id")?,
            app_slug: identity.app_slug,
            key_name: identity.key_name,
            permissions: parsed.permissions,
            metadata: parsed.metadata.unwrap_or(Value::Object(Default::default())),
            organization_id: parse_optional_i64_field(identity.organization_id, "organization_id")?,
            workspace_id: parse_optional_i64_field(identity.workspace_id, "workspace_id")?,
            organization_membership_id: parse_optional_i64_field(
                identity.organization_membership_id,
                "organization_membership_id",
            )?,
            workspace_membership_id: parse_optional_i64_field(
                identity.workspace_membership_id,
                "workspace_membership_id",
            )?,
            rate_limits: parsed.rate_limits,
            retry_after: parsed.retry_after,
        })
    } else {
        Err(Error::Api {
            status,
            message: body.clone(),
            details: serde_json::from_str(&body).ok(),
        })
    }
}

fn parse_i64_field(input: &str, field: &str) -> Result<i64> {
    input
        .parse::<i64>()
        .map_err(|_| Error::InvalidRequest(format!("Invalid field {field}: expected i64 string")))
}

fn parse_optional_i64_field(input: Option<String>, field: &str) -> Result<Option<i64>> {
    input
        .map(|value| parse_i64_field(&value, field))
        .transpose()
}
