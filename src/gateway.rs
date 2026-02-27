use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{client::WachtClient, error::{Error, Result}};

const GATEWAY_URL: &str = "https://gateway.wacht.dev";

#[derive(Debug, Clone)]
pub struct GatewayApi {
    client: WachtClient,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GatewayPrincipalType {
    ApiKey,
    OauthAccessToken,
}

#[derive(Debug, Clone, Default)]
pub struct GatewayAuthzOptions {
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    pub required_permissions: Option<Vec<String>>,
}

impl GatewayApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub async fn verify_request(
        &self,
        api_key: &str,
        method: &str,
        resource: &str,
    ) -> Result<GatewayCheckResponse> {
        self.verify_request_with_principal_type(
            GatewayPrincipalType::ApiKey,
            api_key,
            method,
            resource,
        )
        .await
    }

    pub async fn verify_request_with_principal_type(
        &self,
        principal_type: GatewayPrincipalType,
        principal_value: &str,
        method: &str,
        resource: &str,
    ) -> Result<GatewayCheckResponse> {
        self.check_authz_with_principal_type(
            principal_type,
            principal_value,
            method,
            resource,
            GatewayAuthzOptions::default(),
        )
        .await
    }

    pub async fn verify_oauth_access_token_request(
        &self,
        access_token: &str,
        method: &str,
        resource: &str,
    ) -> Result<GatewayCheckResponse> {
        self.verify_request_with_principal_type(
            GatewayPrincipalType::OauthAccessToken,
            access_token,
            method,
            resource,
        )
        .await
    }

    pub async fn check_authz_with_principal_type(
        &self,
        principal_type: GatewayPrincipalType,
        principal_value: &str,
        method: &str,
        resource: &str,
        options: GatewayAuthzOptions,
    ) -> Result<GatewayCheckResponse> {
        let client = self.client.http_client();
        let url = format!("{GATEWAY_URL}/v1/authz/check");
        let payload = GatewayAuthzCheckRequest {
            principal: GatewayPrincipal {
                principal_type,
                value: principal_value.to_string(),
            },
            resource: resource.to_string(),
            method: method.to_string(),
            client_ip: options.client_ip,
            user_agent: options.user_agent,
            required_permissions: options.required_permissions,
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
                app_slug: identity.app_slug,
                key_name: identity.key_name,
                owner_user_id: parse_optional_i64_field(identity.owner_user_id, "owner_user_id")?,
                principal_type: identity
                    .principal_type
                    .or_else(|| parsed.metadata.as_ref().and_then(extract_principal_type_from_metadata))
                    .unwrap_or(GatewayPrincipalType::ApiKey),
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
}

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
    pub principal_type: GatewayPrincipalType,
    pub key_id: i64,
    pub deployment_id: i64,
    pub app_slug: String,
    pub key_name: String,
    pub owner_user_id: Option<i64>,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GatewayTokenType {
    ApiKey,
    OauthToken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayPrincipalIdentity {
    pub principal_type: GatewayPrincipalType,
    pub key_id: i64,
    pub deployment_id: i64,
    pub app_slug: String,
    pub key_name: String,
    pub owner_user_id: Option<i64>,
    pub organization_id: Option<i64>,
    pub workspace_id: Option<i64>,
    pub organization_membership_id: Option<i64>,
    pub workspace_membership_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayPrincipalMetadata {
    pub principal_type: GatewayTokenType,
    pub permissions_checked: Vec<String>,
    pub organization_permissions: Vec<String>,
    pub workspace_permissions: Vec<String>,
    pub scopes: Vec<String>,
    pub resource: Option<String>,
    pub expires_at: Option<String>,
    pub raw: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedGatewayPrincipalContext {
    pub token_type: GatewayTokenType,
    pub identity: GatewayPrincipalIdentity,
    pub metadata: GatewayPrincipalMetadata,
    pub owner_user_id: Option<i64>,
    pub organization_id: Option<i64>,
    pub workspace_id: Option<i64>,
    pub permissions: Vec<String>,
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
    principal_type: GatewayPrincipalType,
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
    app_slug: String,
    key_name: String,
    owner_user_id: Option<String>,
    #[serde(default)]
    principal_type: Option<GatewayPrincipalType>,
    organization_id: Option<String>,
    workspace_id: Option<String>,
    organization_membership_id: Option<String>,
    workspace_membership_id: Option<String>,
}

fn extract_principal_type_from_metadata(metadata: &Value) -> Option<GatewayPrincipalType> {
    let principal = metadata.get("principal_type")?.as_str()?;
    match principal {
        "api_key" => Some(GatewayPrincipalType::ApiKey),
        "oauth_access_token" => Some(GatewayPrincipalType::OauthAccessToken),
        _ => None,
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

fn extract_string_array(metadata: &Value, key: &str) -> Vec<String> {
    metadata
        .get(key)
        .and_then(|value| value.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(ToOwned::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

fn extract_optional_string(metadata: &Value, key: &str) -> Option<String> {
    metadata
        .get(key)
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned)
}

impl GatewayCheckResponse {
    pub fn resolve_principal_context(&self) -> ResolvedGatewayPrincipalContext {
        let token_type = match self.principal_type {
            GatewayPrincipalType::ApiKey => GatewayTokenType::ApiKey,
            GatewayPrincipalType::OauthAccessToken => GatewayTokenType::OauthToken,
        };
        let organization_permissions = if self.organization_id.is_some() {
            self.permissions.clone()
        } else {
            Vec::new()
        };
        let workspace_permissions = if self.workspace_id.is_some() {
            self.permissions.clone()
        } else {
            Vec::new()
        };
        let scopes = extract_string_array(&self.metadata, "scopes");
        let resource = extract_optional_string(&self.metadata, "resource");
        let expires_at = extract_optional_string(&self.metadata, "expires_at");
        let metadata = GatewayPrincipalMetadata {
            principal_type: token_type,
            permissions_checked: self.permissions.clone(),
            organization_permissions,
            workspace_permissions,
            scopes,
            resource,
            expires_at,
            raw: self.metadata.clone(),
        };

        ResolvedGatewayPrincipalContext {
            token_type,
            identity: GatewayPrincipalIdentity {
                principal_type: self.principal_type,
                key_id: self.key_id,
                deployment_id: self.deployment_id,
                app_slug: self.app_slug.clone(),
                key_name: self.key_name.clone(),
                owner_user_id: self.owner_user_id,
                organization_id: self.organization_id,
                workspace_id: self.workspace_id,
                organization_membership_id: self.organization_membership_id,
                workspace_membership_id: self.workspace_membership_id,
            },
            metadata,
            owner_user_id: self.owner_user_id,
            organization_id: self.organization_id,
            workspace_id: self.workspace_id,
            permissions: self.permissions.clone(),
        }
    }
}
