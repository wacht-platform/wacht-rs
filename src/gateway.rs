use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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
    pub allowed: bool,
    pub key_id: i64,
    pub deployment_id: i64,
    pub app_id: i64,
    pub app_name: String,
    pub key_name: String,
    pub permissions: Vec<String>,
    pub metadata: Value,
    pub rate_limits: Vec<RateLimitInfo>,
    pub retry_after: Option<u32>,
}

pub async fn verify_request(api_key: &str, identifier: &str) -> Result<GatewayCheckResponse> {
    let client = reqwest::Client::new();
    let url = format!("{GATEWAY_URL}/check/{identifier}");

    let mut headers = HeaderMap::new();
    headers.insert(
        "X-API-Key",
        HeaderValue::from_str(api_key).map_err(|_| Error::InvalidRequest("Invalid API key format".to_string()))?,
    );

    let response = client.get(&url).headers(headers).send().await?;

    let status = response.status();
    let response_headers = response.headers().clone();

    if status.is_success() || status.as_u16() == 429 {
        let allowed = status.is_success();

        let key_id = parse_header_i64(&response_headers, "x-wacht-key-id")?;
        let deployment_id = parse_header_i64(&response_headers, "x-wacht-deployment-id")?;
        let app_id = parse_header_i64(&response_headers, "x-wacht-app-id")?;
        let app_name = parse_header_string(&response_headers, "x-wacht-app-name")?;
        let key_name = parse_header_string(&response_headers, "x-wacht-key-name")?;
        let permissions = parse_header_json(&response_headers, "x-wacht-permissions")?;
        let metadata = parse_header_json(&response_headers, "x-wacht-metadata")?;

        let rate_limits = parse_rate_limit_headers(&response_headers);

        let retry_after = if !allowed {
            response_headers
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok())
        } else {
            None
        };

        Ok(GatewayCheckResponse {
            allowed,
            key_id,
            deployment_id,
            app_id,
            app_name,
            key_name,
            permissions,
            metadata,
            rate_limits,
            retry_after,
        })
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: error_body.clone(),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

fn parse_header_i64(headers: &HeaderMap, key: &str) -> Result<i64> {
    headers
        .get(key)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| Error::InvalidRequest(format!("Missing or invalid header: {key}")))
}

fn parse_header_string(headers: &HeaderMap, key: &str) -> Result<String> {
    headers
        .get(key)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| Error::InvalidRequest(format!("Missing or invalid header: {key}")))
}

fn parse_header_json<T: serde::de::DeserializeOwned>(headers: &HeaderMap, key: &str) -> Result<T> {
    let json_str = parse_header_string(headers, key)?;
    serde_json::from_str(&json_str).map_err(|e| Error::InvalidRequest(format!("Failed to parse JSON from header {key}: {e}")))
}

fn parse_rate_limit_headers(headers: &HeaderMap) -> Vec<RateLimitInfo> {
    let mut limits_map: HashMap<u64, RateLimitInfo> = HashMap::new();

    for (key, value) in headers.iter() {
        let key_str = key.as_str();
        if key_str.starts_with("x-ratelimit-") && key_str.ends_with("s-limit") {
            if let Some(window_str) = key_str.strip_prefix("x-ratelimit-").and_then(|s| s.strip_suffix("s-limit")) {
                if let Ok(window) = window_str.parse::<u64>() {
                    if let Ok(limit_str) = value.to_str() {
                        if let Ok(limit) = limit_str.parse::<u32>() {
                        let remaining_key = format!("x-ratelimit-{window}s-remaining");
                        let reset_key = format!("x-ratelimit-{window}s-reset");

                        let remaining = headers
                            .get(&remaining_key)
                            .and_then(|v| v.to_str().ok())
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0);

                        let reset = headers
                            .get(&reset_key)
                            .and_then(|v| v.to_str().ok())
                            .and_then(|s| s.parse().ok());

                        limits_map.insert(
                            window,
                            RateLimitInfo {
                                window_seconds: window,
                                limit,
                                remaining,
                                reset,
                            },
                        );
                        }
                    }
                }
            }
        }
    }

    let mut limits: Vec<RateLimitInfo> = limits_map.into_values().collect();
    limits.sort_by_key(|l| l.window_seconds);
    limits
}
