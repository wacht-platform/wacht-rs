use crate::{
    client::{get_client, get_config},
    error::Error,
    Result,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub delivery_ids: Vec<i64>,
    pub filtered_count: usize,
    pub delivered_count: usize,
}

/// Webhook app
#[derive(Debug, Deserialize)]
pub struct WebhookApp {
    pub deployment_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub signing_secret: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct WebhookAppEvent {
    pub deployment_id: i64,
    pub app_name: String,
    pub event_name: String,
    pub description: Option<String>,
    pub schema: Option<Value>,
    pub created_at: String,
}

/// Webhook endpoint
#[derive(Debug, Deserialize)]
pub struct WebhookEndpoint {
    pub id: i64,
    pub deployment_id: i64,
    pub app_name: String,
    pub url: String,
    pub description: Option<String>,
    pub headers: Option<Value>,
    pub is_active: bool,
    pub signing_secret: Option<String>,
    pub max_retries: i32,
    pub timeout_seconds: i32,
    pub failure_count: i32,
    pub last_failure_at: Option<String>,
    pub auto_disabled: bool,
    pub auto_disabled_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct WebhookEndpointSubscription {
    pub endpoint_id: i64,
    pub deployment_id: i64,
    pub app_name: String,
    pub event_name: String,
    pub filter_rules: Option<Value>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct WebhookEndpointWithSubscriptions {
    pub endpoint: WebhookEndpoint,
    pub subscribed_events: Vec<String>,
}

/// Webhook event definition
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookEventDefinition {
    pub name: String,
    pub description: String,
    pub schema: Option<Value>,
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
pub async fn get_webhook_events(app_name: &str) -> Result<Vec<WebhookEventDefinition>> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/webhooks/apps/{}/events", config.base_url, app_name);
    
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        let events = response.json().await?;
        Ok(events)
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

/// Get webhook endpoints with subscriptions
pub async fn get_webhook_endpoints_with_subscriptions(
    app_name: &str,
    include_inactive: Option<bool>,
) -> Result<Vec<WebhookEndpointWithSubscriptions>> {
    let config = get_config();
    let client = get_client();
    let mut url = format!("{}/webhooks/apps/{}/endpoints", config.base_url, app_name);
    
    if let Some(inactive) = include_inactive {
        url.push_str(&format!("?include_inactive={}", inactive));
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
    endpoint_id: i64,
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
pub async fn delete_webhook_endpoint(endpoint_id: i64) -> Result<()> {
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

/// Get webhook deliveries
pub async fn get_webhook_deliveries(
    app_name: Option<&str>,
    endpoint_id: Option<i64>,
    event_name: Option<&str>,
    status: Option<&str>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<Value>> {
    let config = get_config();
    let client = get_client();
    let mut url = format!("{}/webhooks/deliveries", config.base_url);
    
    let mut params = Vec::new();
    if let Some(name) = app_name {
        params.push(format!("app_name={}", name));
    }
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
        let deliveries = response.json().await?;
        Ok(deliveries)
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
pub async fn get_webhook_delivery_details(delivery_id: i64) -> Result<Value> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/webhooks/deliveries/{}", config.base_url, delivery_id);
    
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

/// Retry a webhook delivery
pub async fn retry_webhook_delivery(delivery_id: i64) -> Result<Value> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/webhooks/deliveries/{}/retry", config.base_url, delivery_id);
    
    let response = client.post(&url).send().await?;
    
    if response.status().is_success() {
        let result = response.json().await?;
        Ok(result)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(Error::Api {
            status,
            message: format!("Failed to retry webhook delivery: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Reactivate a webhook endpoint
pub async fn reactivate_webhook_endpoint(endpoint_id: i64) -> Result<Value> {
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
    endpoint_id: i64,
    event_name: String,
    payload: Option<Value>,
) -> Result<Value> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/webhooks/endpoints/{}/test", config.base_url, endpoint_id);
    
    let test_payload = payload.unwrap_or_else(|| {
        serde_json::json!({
            "event": event_name,
            "test": true,
            "timestamp": chrono::Utc::now().to_rfc3339()
        })
    });
    
    let request_body = serde_json::json!({
        "event_name": event_name,
        "payload": test_payload
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
            message: format!("Failed to test webhook endpoint: {}", error_text),
            details: serde_json::from_str(&error_text).ok(),
        })
    }
}

/// Get webhook stats
pub async fn get_webhook_stats(app_name: &str) -> Result<Value> {
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

/// Get webhook timeseries
pub async fn get_webhook_timeseries(
    app_name: &str,
    interval: &str,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<Value> {
    let config = get_config();
    let client = get_client();
    let mut url = format!("{}/webhooks/apps/{}/timeseries", config.base_url, app_name);
    
    let mut params = vec![format!("interval={}", interval)];
    if let Some(start) = start_date {
        params.push(format!("start_date={}", start));
    }
    if let Some(end) = end_date {
        params.push(format!("end_date={}", end));
    }
    
    if !params.is_empty() {
        url.push_str("?");
        url.push_str(&params.join("&"));
    }
    
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        let timeseries = response.json().await?;
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
    app_name: Option<&str>,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<Value> {
    let config = get_config();
    let client = get_client();
    let mut url = format!("{}/webhooks/analytics", config.base_url);
    
    let mut params = Vec::new();
    if let Some(name) = app_name {
        params.push(format!("app_name={}", name));
    }
    if let Some(start) = start_date {
        params.push(format!("start_date={}", start));
    }
    if let Some(end) = end_date {
        params.push(format!("end_date={}", end));
    }
    
    if !params.is_empty() {
        url.push_str("?");
        url.push_str(&params.join("&"));
    }
    
    let response = client.get(&url).send().await?;
    
    if response.status().is_success() {
        let analytics = response.json().await?;
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