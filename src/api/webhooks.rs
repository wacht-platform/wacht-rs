use crate::{
    client::{get_client, get_config},
    error::Error,
    Result,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::form_urlencoded;

/// Create a webhook app
#[derive(Debug, Serialize)]
pub struct CreateWebhookAppRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<WebhookEventDefinition>>,
}

/// Update a webhook app
#[derive(Debug, Serialize)]
pub struct UpdateWebhookAppRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

/// Create a webhook endpoint
#[derive(Debug, Serialize)]
pub struct CreateWebhookEndpointRequest {
    pub app_name: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Value>,
    pub subscriptions: Vec<EventSubscription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSubscription {
    pub event_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_rules: Option<Value>,
}

/// Update a webhook endpoint
#[derive(Debug, Serialize)]
pub struct UpdateWebhookEndpointRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<EventSubscription>>,
}

/// Trigger a webhook event
#[derive(Debug, Serialize)]
pub struct TriggerWebhookEventRequest {
    pub app_name: String,
    pub event_name: String,
    pub payload: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_context: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct TriggerWebhookEventResponse {
    pub delivery_ids: Vec<String>,
    pub filtered_count: usize,
    pub delivered_count: usize,
}

#[derive(Debug, Deserialize)]
pub struct TestWebhookEndpointResponse {
    pub success: bool,
    pub status_code: u16,
    pub response_time_ms: u64,
    pub response_body: Option<String>,
    pub response_headers: Option<Value>,
    pub error_message: Option<String>,
}

/// Webhook app
#[derive(Debug, Deserialize)]
pub struct WebhookApp {
    pub deployment_id: String,
    pub name: String,
    pub description: Option<String>,
    pub signing_secret: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Webhook delivery (list response)
#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookDelivery {
    pub deployment_id: String,
    pub delivery_id: String,
    pub app_name: String,
    pub endpoint_id: String,
    pub endpoint_url: String,
    pub event_name: String,
    pub status: String,
    pub http_status_code: Option<i32>,
    pub response_time_ms: Option<i32>,
    pub attempt_number: i32,
    pub max_attempts: i32,
    pub error_message: Option<String>,
    pub filtered_reason: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Webhook delivery details (includes payload info)
#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookDeliveryDetails {
    pub deployment_id: String,
    pub delivery_id: String,
    pub app_name: String,
    pub endpoint_id: String,
    pub endpoint_url: String,
    pub event_name: String,
    pub status: String,
    pub http_status_code: Option<i32>,
    pub response_time_ms: Option<i32>,
    pub attempt_number: i32,
    pub max_attempts: i32,
    pub error_message: Option<String>,
    pub filtered_reason: Option<String>,
    pub payload_s3_key: String,
    pub response_body: Option<String>,
    pub response_headers: Option<Value>,
    pub timestamp: DateTime<Utc>,
    pub payload: Option<Value>,
}

/// Webhook stats response
#[derive(Debug, Deserialize)]
pub struct WebhookStats {
    pub total_events: i64,
    pub total_deliveries: i64,
    pub successful_deliveries: i64,
    pub failed_deliveries: i64,
    pub filtered_deliveries: i64,
    pub avg_response_time_ms: Option<f64>,
    pub p50_response_time_ms: Option<f64>,
    pub p95_response_time_ms: Option<f64>,
    pub p99_response_time_ms: Option<f64>,
    pub success_rate: f64,
    pub top_events: Vec<EventCount>,
    pub endpoint_performance: Vec<EndpointPerformance>,
    pub failure_reasons: Vec<FailureReason>,
}

#[derive(Debug, Deserialize)]
pub struct EventCount {
    pub event_name: String,
    pub count: i64,
}

#[derive(Debug, Deserialize)]
pub struct EndpointPerformance {
    pub endpoint_id: i64,
    pub endpoint_url: String,
    pub total_attempts: i64,
    pub successful_attempts: i64,
    pub failed_attempts: i64,
    pub avg_response_time_ms: Option<f64>,
    pub success_rate: f64,
}

#[derive(Debug, Deserialize)]
pub struct FailureReason {
    pub reason: String,
    pub count: i64,
}

/// Webhook endpoint
#[derive(Debug, Deserialize)]
pub struct WebhookEndpoint {
    pub id: String,
    pub deployment_id: String,
    pub app_name: String,
    pub url: String,
    pub description: Option<String>,
    pub headers: Option<Value>,
    pub is_active: bool,
    pub signing_secret: Option<String>,
    pub max_retries: i32,
    pub timeout_seconds: i32,
    pub failure_count: i32,
    pub last_failure_at: Option<DateTime<Utc>>,
    pub auto_disabled: bool,
    pub auto_disabled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub subscriptions: Vec<EventSubscription>,
}

#[derive(Debug, Deserialize)]
pub struct WebhookEndpointSubscription {
    pub endpoint_id: String,
    pub deployment_id: String,
    pub app_name: String,
    pub event_name: String,
    pub filter_rules: Option<Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct WebhookEndpointWithSubscriptions {
    pub endpoint: WebhookEndpoint,
    pub subscribed_events: Vec<String>,
}

/// Webhook event definition (for creating/updating apps)
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookEventDefinition {
    pub name: String,
    pub description: String,
    pub schema: Option<Value>,
}

/// Webhook event from backend
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookAppEvent {
    pub deployment_id: String,
    pub app_name: String,
    pub event_name: String,
    pub description: Option<String>,
    pub schema: Option<Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct EventsResponse {
    events: Vec<WebhookAppEvent>,
}

/// Get webhook app by name
pub async fn get_webhook_app(app_name: &str) -> Result<WebhookApp> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/webhooks/apps/{}", config.base_url, app_name);
    
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        let app = response.json().await?;
        Ok(app)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to get webhook app: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// List webhook apps
pub async fn list_webhook_apps(include_inactive: Option<bool>) -> Result<Vec<WebhookApp>> {
    let config = get_config();
    let client = get_client();
    let mut url = format!("{}/webhooks/apps", config.base_url);
    
    if let Some(inactive) = include_inactive {
        url.push_str(&format!("?include_inactive={}", inactive));
    }
    
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        let apps = response.json().await?;
        Ok(apps)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to list webhook apps: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Create a webhook app
pub async fn create_webhook_app(request: CreateWebhookAppRequest) -> Result<WebhookApp> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/webhooks/apps", config.base_url);
    
    let response = client.post(&url).json(&request).send().await?;
    
    if response.status().is_success() {
        let app = response.json().await?;
        Ok(app)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to create webhook app: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Update a webhook app
pub async fn update_webhook_app(
    app_name: &str,
    request: UpdateWebhookAppRequest,
) -> Result<WebhookApp> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/webhooks/apps/{}", config.base_url, app_name);
    
    let response = client.patch(&url).json(&request).send().await?;
    
    if response.status().is_success() {
        let app = response.json().await?;
        Ok(app)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to update webhook app: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Delete a webhook app
pub async fn delete_webhook_app(app_name: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/webhooks/apps/{}", config.base_url, app_name);
    
    let response = client.delete(&url).send().await?;
    
    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to delete webhook app: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Rotate webhook secret
pub async fn rotate_webhook_secret(app_name: &str) -> Result<WebhookApp> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/webhooks/apps/{}/rotate-secret", config.base_url, app_name);
    
    let response = client.post(&url).send().await?;
    
    if response.status().is_success() {
        let result = response.json().await?;
        Ok(result)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to rotate webhook secret: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Get webhook events for an app
pub async fn get_webhook_events(app_name: &str) -> Result<Vec<WebhookAppEvent>> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/webhooks/apps/{}/events", config.base_url, app_name);
    
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        let response_data: EventsResponse = response.json().await?;
        Ok(response_data.events)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to get webhook events: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct EndpointsResponse {
    pub data: Vec<WebhookEndpoint>,
    #[serde(default)]
    pub limit: Option<i32>,
    #[serde(default)]
    pub offset: Option<i32>,
    pub has_more: bool,
}

#[derive(Debug, Deserialize)]
pub struct PaginatedEndpointsResponse {
    pub endpoints: Vec<WebhookEndpointWithSubscriptions>,
    pub count: usize,
    pub limit: i32,
    pub offset: i32,
    pub has_more: bool,
}

/// Get webhook endpoints with subscriptions
pub async fn get_webhook_endpoints_with_subscriptions(
    app_name: &str,
    include_inactive: Option<bool>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<PaginatedEndpointsResponse> {
    let config = get_config();
    let client = get_client();
    let mut url = format!("{}/webhooks/apps/{}/endpoints", config.base_url, app_name);
    
    let mut params = Vec::new();
    if let Some(inactive) = include_inactive {
        params.push(format!("include_inactive={}", inactive));
    }
    if let Some(lim) = limit {
        params.push(format!("limit={}", lim));
    }
    if let Some(off) = offset {
        params.push(format!("offset={}", off));
    }
    
    if !params.is_empty() {
        url.push_str("?");
        url.push_str(&params.join("&"));
    }
    
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        let response_data: EndpointsResponse = response.json().await?;
        
        // Convert WebhookEndpoint to WebhookEndpointWithSubscriptions
        let endpoints_with_subs: Vec<WebhookEndpointWithSubscriptions> = response_data.data.into_iter().map(|mut endpoint| {
            let subscribed_events = endpoint.subscriptions.iter()
                .map(|sub| sub.event_name.clone())
                .collect();
            
            // Clear subscriptions from endpoint to avoid duplication
            endpoint.subscriptions.clear();
            
            WebhookEndpointWithSubscriptions {
                endpoint,
                subscribed_events,
            }
        }).collect();
        
        let count = endpoints_with_subs.len();
        Ok(PaginatedEndpointsResponse {
            endpoints: endpoints_with_subs,
            count,
            limit: response_data.limit.unwrap_or(100),
            offset: response_data.offset.unwrap_or(0),
            has_more: response_data.has_more,
        })
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to get webhook endpoints with subscriptions: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// List webhook endpoints
pub async fn list_webhook_endpoints(
    app_name: Option<&str>,
    include_inactive: Option<bool>,
) -> Result<Vec<WebhookEndpoint>> {
    let config = get_config();
    let client = get_client();
    let mut url = format!("{}/webhooks/endpoints", config.base_url);
    
    let mut params = Vec::new();
    if let Some(name) = app_name {
        params.push(format!("app_name={}", name));
    }
    if let Some(inactive) = include_inactive {
        params.push(format!("include_inactive={}", inactive));
    }
    
    if !params.is_empty() {
        url.push_str("?");
        url.push_str(&params.join("&"));
    }
    
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        let endpoints = response.json().await?;
        Ok(endpoints)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to list webhook endpoints: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Create a webhook endpoint
pub async fn create_webhook_endpoint(request: CreateWebhookEndpointRequest) -> Result<WebhookEndpoint> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/webhooks/endpoints", config.base_url);
    
    let response = client.post(&url).json(&request).send().await?;
    
    if response.status().is_success() {
        let endpoint = response.json().await?;
        Ok(endpoint)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to create webhook endpoint: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Update a webhook endpoint
pub async fn update_webhook_endpoint(
    endpoint_id: String,
    request: UpdateWebhookEndpointRequest,
) -> Result<WebhookEndpoint> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/webhooks/endpoints/{}", config.base_url, endpoint_id);
    
    let response = client.patch(&url).json(&request).send().await?;
    
    if response.status().is_success() {
        let endpoint = response.json().await?;
        Ok(endpoint)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to update webhook endpoint: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Delete a webhook endpoint
pub async fn delete_webhook_endpoint(endpoint_id: String) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/webhooks/endpoints/{}", config.base_url, endpoint_id);
    
    let response = client.delete(&url).send().await?;
    
    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to delete webhook endpoint: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Trigger a webhook event
pub async fn trigger_webhook_event(
    request: TriggerWebhookEventRequest,
) -> Result<TriggerWebhookEventResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/webhooks/trigger", config.base_url);
    
    let response = client.post(&url).json(&request).send().await?;
    
    if response.status().is_success() {
        let result = response.json().await?;
        Ok(result)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to trigger webhook event: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Get webhook deliveries response
#[derive(Debug, Deserialize)]
pub struct GetWebhookDeliveriesResponse {
    pub data: Vec<WebhookDelivery>,
    #[serde(default)]
    pub limit: Option<i32>,
    #[serde(default)]
    pub offset: Option<i32>,
    pub has_more: bool,
}

/// Get webhook deliveries
pub async fn get_webhook_deliveries(
    app_name: &str,
    endpoint_id: Option<i64>,
    event_name: Option<&str>,
    status: Option<&str>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<GetWebhookDeliveriesResponse> {
    let config = get_config();
    let client = get_client();
    let mut url = format!("{}/webhooks/apps/{}/deliveries", config.base_url, app_name);
    
    let mut params = Vec::new();
    if let Some(id) = endpoint_id {
        params.push(format!("endpoint_id={}", id));
    }
    if let Some(event) = event_name {
        params.push(format!("event_name={}", event));
    }
    if let Some(s) = status {
        params.push(format!("status={}", s));
    }
    if let Some(l) = limit {
        params.push(format!("limit={}", l));
    }
    if let Some(o) = offset {
        params.push(format!("offset={}", o));
    }
    
    if !params.is_empty() {
        url.push_str("?");
        url.push_str(&params.join("&"));
    }
    
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        let result = response.json().await?;
        Ok(result)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to get webhook deliveries: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Get webhook delivery details
pub async fn get_webhook_delivery_details(
    delivery_id: String,
    status: Option<&str>,
) -> Result<WebhookDeliveryDetails> {
    let config = get_config();
    let client = get_client();
    let mut url = format!("{}/webhooks/deliveries/{}", config.base_url, delivery_id);
    
    // Add status query parameter if provided
    if let Some(status) = status {
        url.push_str(&format!("?status={}", urlencoding::encode(status)));
    }
    
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        let delivery = response.json().await?;
        Ok(delivery)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to get webhook delivery details: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}


/// Replay webhook deliveries
pub async fn replay_webhook_deliveries(app_name: String, delivery_ids: Vec<String>, include_successful: bool) -> Result<Value> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/webhooks/apps/{}/deliveries/replay", config.base_url, app_name);
    
    let request_body = serde_json::json!({
        "delivery_ids": delivery_ids,
        "include_successful": include_successful
    });
    
    let response = client.post(&url).json(&request_body).send().await?;
    
    if response.status().is_success() {
        let result = response.json().await?;
        Ok(result)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to replay webhook deliveries: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Reactivate a webhook endpoint
pub async fn reactivate_webhook_endpoint(endpoint_id: String) -> Result<Value> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/webhooks/endpoints/{}/reactivate", config.base_url, endpoint_id);
    
    let response = client.post(&url).send().await?;
    
    if response.status().is_success() {
        let result = response.json().await?;
        Ok(result)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to reactivate webhook endpoint: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Test webhook endpoint
pub async fn test_webhook_endpoint(
    app_name: &str,
    endpoint_id: String,
    event_name: String,
    payload: Option<Value>,
) -> Result<TestWebhookEndpointResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/webhooks/apps/{}/endpoints/{}/test", config.base_url, app_name, endpoint_id);
    
    let request_body = serde_json::json!({
        "event_name": event_name,
        "payload": payload
    });
    
    let response = client.post(&url).json(&request_body).send().await?;
    
    if response.status().is_success() {
        let result: TestWebhookEndpointResponse = response.json().await?;
        Ok(result)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to test webhook endpoint: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Get webhook stats
pub async fn get_webhook_stats(app_name: &str) -> Result<WebhookStats> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/webhooks/apps/{}/stats", config.base_url, app_name);
    
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        let stats = response.json().await?;
        Ok(stats)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to get webhook stats: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Webhook analytics result
#[derive(Debug, Deserialize, Serialize)]
pub struct AnalyticsResult {
    pub total_events: i64,
    pub total_deliveries: i64,
    pub successful_deliveries: i64,
    pub failed_deliveries: i64,
    pub filtered_deliveries: i64,
    pub avg_response_time_ms: Option<f64>,
    pub p50_response_time_ms: Option<f64>,
    pub p95_response_time_ms: Option<f64>,
    pub p99_response_time_ms: Option<f64>,
    pub success_rate: f64,
    pub top_events: Vec<AnalyticsEventCount>,
    pub endpoint_performance: Vec<AnalyticsEndpointPerformance>,
    pub failure_reasons: Vec<AnalyticsFailureReason>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnalyticsEventCount {
    pub event_name: String,
    pub count: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnalyticsEndpointPerformance {
    pub endpoint_id: i64,
    pub endpoint_url: String,
    pub total_attempts: i64,
    pub successful_attempts: i64,
    pub failed_attempts: i64,
    pub avg_response_time_ms: Option<f64>,
    pub success_rate: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnalyticsFailureReason {
    pub reason: String,
    pub count: i64,
}

/// Webhook timeseries result
#[derive(Debug, Deserialize, Serialize)]
pub struct TimeseriesResult {
    pub data: Vec<TimeseriesDataPoint>,
    pub interval: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeseriesDataPoint {
    pub timestamp: DateTime<Utc>,
    pub total_events: i64,
    pub total_deliveries: i64,
    pub successful_deliveries: i64,
    pub failed_deliveries: i64,
    pub filtered_deliveries: i64,
    pub avg_response_time_ms: Option<f64>,
    pub success_rate: f64,
}

/// Get webhook timeseries
pub async fn get_webhook_timeseries(
    app_name: &str,
    interval: &str,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<TimeseriesResult> {
    let config = get_config();
    let client = get_client();
    let mut url = format!("{}/webhooks/apps/{}/timeseries", config.base_url, app_name);
    
    // Build query parameters
    let mut query_params = Vec::new();
    query_params.push(format!("interval={}", urlencoding::encode(interval)));
    if let Some(start) = start_date {
        query_params.push(format!("start_date={}", urlencoding::encode(start)));
    }
    if let Some(end) = end_date {
        query_params.push(format!("end_date={}", urlencoding::encode(end)));
    }
    
    if !query_params.is_empty() {
        url.push_str("?");
        url.push_str(&query_params.join("&"));
    }
    
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        let timeseries: TimeseriesResult = response.json().await?;
        Ok(timeseries)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to get webhook timeseries: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Get webhook analytics
pub async fn get_webhook_analytics(
    app_name: &str,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<AnalyticsResult> {
    let config = get_config();
    let client = get_client();
    let mut url = format!("{}/webhooks/apps/{}/analytics", config.base_url, app_name);
    
    // Build query parameters
    let mut query_params = Vec::new();
    if let Some(start) = start_date {
        query_params.push(format!("start_date={}", urlencoding::encode(start)));
    }
    if let Some(end) = end_date {
        query_params.push(format!("end_date={}", urlencoding::encode(end)));
    }
    
    if !query_params.is_empty() {
        url.push_str("?");
        url.push_str(&query_params.join("&"));
    }
    
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        let analytics: AnalyticsResult = response.json().await?;
        Ok(analytics)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to get webhook analytics: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}