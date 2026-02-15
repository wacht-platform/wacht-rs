use crate::{
    Result,
    client::{get_client, get_config},
    error::Error,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::form_urlencoded;

/// List webhook apps response
#[derive(Debug, Deserialize)]
pub struct ListWebhookAppsResponse {
    pub data: Vec<WebhookApp>,
    #[serde(default)]
    pub limit: Option<i32>,
    #[serde(default)]
    pub offset: Option<i32>,
    pub has_more: bool,
}

/// Query parameters for listing webhook endpoints (matches backend ListWebhookEndpointsQuery)
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListWebhookEndpointsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_inactive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

impl ListWebhookEndpointsQuery {
    pub fn is_empty(&self) -> bool {
        self.include_inactive.is_none() && self.limit.is_none() && self.offset.is_none()
    }
}

/// Query parameters for listing webhook deliveries (matches backend GetAppWebhookDeliveriesQuery)
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetAppWebhookDeliveriesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
}

/// Query parameters for webhook analytics (matches backend WebhookAnalyticsQuery)
#[derive(Debug, Clone, Serialize, Default)]
pub struct WebhookAnalyticsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

/// Query parameters for webhook timeseries (matches backend WebhookTimeseriesQuery)
#[derive(Debug, Clone, Serialize, Default)]
pub struct WebhookTimeseriesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    pub interval: String,
}

/// Create a webhook app
#[derive(Debug, Serialize)]
pub struct CreateWebhookAppRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_catalog_slug: Option<String>,
}

/// Update a webhook app
#[derive(Debug, Serialize)]
pub struct UpdateWebhookAppRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_catalog_slug: Option<String>,
}

/// Create a webhook endpoint
#[derive(Debug, Serialize)]
pub struct CreateWebhookEndpointRequest {
    pub app_slug: String,
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
    pub app_slug: String,
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
    pub app_slug: String,
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
    pub app_slug: String,
    pub endpoint_id: String,

    pub event_name: String,
    pub status: String,
    pub http_status_code: Option<i32>,
    pub response_time_ms: Option<i32>,
    pub attempt_number: i32,
    pub max_attempts: i32,

    pub timestamp: DateTime<Utc>,
}

/// Webhook delivery details (includes payload info)
#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookDeliveryDetails {
    pub deployment_id: String,
    pub delivery_id: String,
    pub app_slug: String,
    pub endpoint_id: String,

    pub event_name: String,
    pub status: String,
    pub http_status_code: Option<i32>,
    pub response_time_ms: Option<i32>,
    pub attempt_number: i32,
    pub max_attempts: i32,

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
    pub app_slug: String,
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
    pub rate_limit_config: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub subscriptions: Vec<EventSubscription>,
}

#[derive(Debug, Deserialize)]
pub struct WebhookEndpointSubscription {
    pub endpoint_id: String,
    pub deployment_id: String,
    pub app_slug: String,
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
    pub app_slug: String,
    pub event_name: String,
    pub description: Option<String>,
    pub schema: Option<Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct EventsResponse {
    events: Vec<WebhookAppEvent>,
}

/// Builder for list_webhook_apps
pub struct ListWebhookAppsBuilder {
    include_inactive: Option<bool>,
}

/// Builder for get_webhook_app
pub struct GetWebhookAppBuilder {
    app_name: String,
}

impl GetWebhookAppBuilder {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
        }
    }

    pub async fn send(self) -> Result<WebhookApp> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/webhooks/apps/{}", config.base_url, self.app_name);

        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            let app = response.json().await?;
            Ok(app)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to get webhook app: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

impl ListWebhookAppsBuilder {
    pub fn new() -> Self {
        Self {
            include_inactive: None,
        }
    }

    pub fn include_inactive(mut self, include: bool) -> Self {
        self.include_inactive = Some(include);
        self
    }

    pub async fn send(self) -> Result<Vec<WebhookApp>> {
        let config = get_config();
        let client = get_client();
        let mut url = format!("{}/webhooks/apps", config.base_url);

        if let Some(inactive) = self.include_inactive {
            url.push_str(&format!("?include_inactive={inactive}"));
        }

        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            let result: ListWebhookAppsResponse = response.json().await?;
            Ok(result.data)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to list webhook apps: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Builder for create_webhook_app
pub struct CreateWebhookAppBuilder {
    request: CreateWebhookAppRequest,
}

impl CreateWebhookAppBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            request: CreateWebhookAppRequest {
                name: name.to_string(),
                description: None,
                is_active: None,
                event_catalog_slug: None,
            },
        }
    }

    pub fn description(mut self, description: &str) -> Self {
        self.request.description = Some(description.to_string());
        self
    }

    pub fn is_active(mut self, is_active: bool) -> Self {
        self.request.is_active = Some(is_active);
        self
    }

    pub fn event_catalog_slug(mut self, slug: &str) -> Self {
        self.request.event_catalog_slug = Some(slug.to_string());
        self
    }

    pub async fn send(self) -> Result<WebhookApp> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/webhooks/apps", config.base_url);

        let response = client.post(&url).json(&self.request).send().await?;

        if response.status().is_success() {
            let app = response.json().await?;
            Ok(app)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to create webhook app: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Create a webhook app (legacy function for backward compatibility)
/// Builder for update_webhook_app
pub struct UpdateWebhookAppBuilder {
    app_name: String,
    description: Option<String>,
    is_active: Option<bool>,
    event_catalog_slug: Option<String>,
}

impl UpdateWebhookAppBuilder {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            description: None,
            is_active: None,
            event_catalog_slug: None,
        }
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }

    pub fn event_catalog_slug(mut self, slug: &str) -> Self {
        self.event_catalog_slug = Some(slug.to_string());
        self
    }

    pub async fn send(self) -> Result<WebhookApp> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/webhooks/apps/{}", config.base_url, self.app_name);

        let request = UpdateWebhookAppRequest {
            description: self.description,
            is_active: self.is_active,
            event_catalog_slug: self.event_catalog_slug,
        };

        let response = client.patch(&url).json(&request).send().await?;

        if response.status().is_success() {
            let app = response.json().await?;
            Ok(app)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to update webhook app: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Builder for delete_webhook_app
pub struct DeleteWebhookAppBuilder {
    app_name: String,
}

impl DeleteWebhookAppBuilder {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/webhooks/apps/{}", config.base_url, self.app_name);

        let response = client.delete(&url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to delete webhook app: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Delete a webhook app (legacy function for backward compatibility)
pub struct RotateWebhookSecretBuilder {
    app_name: String,
}

impl RotateWebhookSecretBuilder {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
        }
    }

    pub async fn send(self) -> Result<WebhookApp> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/webhooks/apps/{}/rotate-secret",
            config.base_url, self.app_name
        );

        let response = client.post(&url).send().await?;

        if response.status().is_success() {
            let result = response.json().await?;
            Ok(result)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to rotate webhook secret: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Rotate webhook secret (legacy function for backward compatibility)
pub struct GetWebhookEventsBuilder {
    app_name: String,
}

impl GetWebhookEventsBuilder {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
        }
    }

    pub async fn send(self) -> Result<Vec<WebhookAppEvent>> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/webhooks/apps/{}/events", config.base_url, self.app_name);

        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            let response_data: EventsResponse = response.json().await?;
            Ok(response_data.events)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to get webhook events: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Get webhook events for an app (legacy function for backward compatibility)
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

/// Builder for list_webhook_endpoints
pub struct ListWebhookEndpointsBuilder {
    app_name: Option<String>,
    include_inactive: Option<bool>,
    limit: Option<i32>,
    offset: Option<i32>,
}

impl ListWebhookEndpointsBuilder {
    pub fn new() -> Self {
        Self {
            app_name: None,
            include_inactive: None,
            limit: None,
            offset: None,
        }
    }

    pub fn app_name(mut self, app_name: &str) -> Self {
        self.app_name = Some(app_name.to_string());
        self
    }

    pub fn include_inactive(mut self, include: bool) -> Self {
        self.include_inactive = Some(include);
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub async fn send(self) -> Result<Vec<WebhookEndpoint>> {
        let config = get_config();
        let client = get_client();

        if let Some(app_name) = &self.app_name {
            let mut url = format!("{}/webhooks/apps/{}/endpoints", config.base_url, app_name);

            let mut params = Vec::new();
            if let Some(inactive) = self.include_inactive {
                params.push(format!("include_inactive={inactive}"));
            }
            if let Some(limit) = self.limit {
                params.push(format!("limit={limit}"));
            }
            if let Some(offset) = self.offset {
                params.push(format!("offset={offset}"));
            }

            if !params.is_empty() {
                url.push('?');
                url.push_str(&params.join("&"));
            }

            let response = client.get(&url).send().await?;

            if response.status().is_success() {
                let result: EndpointsResponse = response.json().await?;
                Ok(result.data)
            } else {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                Err(Error::Api {
                    status,
                    message: format!("Failed to list webhook endpoints: {error_text}"),
                    details: serde_json::from_str(&error_text).ok(),
                })
            }
        } else {
            Err(Error::Api {
                status: reqwest::StatusCode::BAD_REQUEST,
                message: "app_name is required for listing webhook endpoints".to_string(),
                details: None,
            })
        }
    }
}

/// Builder for get_webhook_endpoints_with_subscriptions
pub struct GetWebhookEndpointsWithSubscriptionsBuilder {
    app_name: String,
    include_inactive: Option<bool>,
    limit: Option<i32>,
    offset: Option<i32>,
}

impl GetWebhookEndpointsWithSubscriptionsBuilder {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            include_inactive: None,
            limit: None,
            offset: None,
        }
    }

    pub fn include_inactive(mut self, include: bool) -> Self {
        self.include_inactive = Some(include);
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub async fn send(self) -> Result<PaginatedEndpointsResponse> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/webhooks/apps/{}/endpoints",
            config.base_url, self.app_name
        );

        let mut options = ListWebhookEndpointsQuery::default();
        options.include_inactive = self.include_inactive;
        options.limit = self.limit;
        options.offset = self.offset;

        let mut request = client.get(&url);

        if !options.is_empty() {
            request = request.query(&options);
        }

        let response = request.send().await?;

        if response.status().is_success() {
            let response_data: EndpointsResponse = response.json().await?;

            // Convert WebhookEndpoint to WebhookEndpointWithSubscriptions
            let endpoints_with_subs: Vec<WebhookEndpointWithSubscriptions> = response_data
                .data
                .into_iter()
                .map(|mut endpoint| {
                    let subscribed_events = endpoint
                        .subscriptions
                        .iter()
                        .map(|sub| sub.event_name.clone())
                        .collect();

                    // Clear subscriptions from endpoint to avoid duplication
                    endpoint.subscriptions.clear();

                    WebhookEndpointWithSubscriptions {
                        endpoint,
                        subscribed_events,
                    }
                })
                .collect();

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
                message: format!(
                    "Failed to get webhook endpoints with subscriptions: {error_text}"
                ),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Builder for create_webhook_endpoint
pub struct CreateWebhookEndpointBuilder {
    app_slug: String,
    url: String,
    description: Option<String>,
    headers: Option<Value>,
    subscriptions: Vec<EventSubscription>,
    max_retries: Option<i32>,
    timeout_seconds: Option<i32>,
}

impl CreateWebhookEndpointBuilder {
    pub fn new(app_slug: &str, url: &str) -> Self {
        Self {
            app_slug: app_slug.to_string(),
            url: url.to_string(),
            description: None,
            headers: None,
            subscriptions: Vec::new(),
            max_retries: None,
            timeout_seconds: None,
        }
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn headers(mut self, headers: Value) -> Self {
        self.headers = Some(headers);
        self
    }

    pub fn subscriptions(mut self, subscriptions: Vec<EventSubscription>) -> Self {
        self.subscriptions = subscriptions;
        self
    }

    pub fn add_subscription(mut self, subscription: EventSubscription) -> Self {
        self.subscriptions.push(subscription);
        self
    }

    pub fn add_event(mut self, event_name: &str, filter_rules: Value) -> Self {
        self.subscriptions.push(EventSubscription {
            event_name: event_name.to_string(),
            filter_rules: Some(filter_rules),
        });
        self
    }

    pub fn max_retries(mut self, max_retries: i32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    pub fn timeout_seconds(mut self, timeout_seconds: i32) -> Self {
        self.timeout_seconds = Some(timeout_seconds);
        self
    }

    pub async fn send(self) -> Result<WebhookEndpoint> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/webhooks/endpoints", config.base_url);

        let request = CreateWebhookEndpointRequest {
            app_slug: self.app_slug,
            url: self.url,
            description: self.description,
            headers: self.headers,
            subscriptions: self.subscriptions,
            max_retries: self.max_retries,
            timeout_seconds: self.timeout_seconds,
        };

        let response = client.post(&url).json(&request).send().await?;

        if response.status().is_success() {
            let endpoint = response.json().await?;
            Ok(endpoint)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to create webhook endpoint: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Builder for update_webhook_endpoint
pub struct UpdateWebhookEndpointBuilder {
    endpoint_id: String,
    url: Option<String>,
    description: Option<String>,
    headers: Option<Value>,
    is_active: Option<bool>,
    max_retries: Option<i32>,
    timeout_seconds: Option<i32>,
    subscriptions: Option<Vec<EventSubscription>>,
}

impl UpdateWebhookEndpointBuilder {
    pub fn new(endpoint_id: &str) -> Self {
        Self {
            endpoint_id: endpoint_id.to_string(),
            url: None,
            description: None,
            headers: None,
            is_active: None,
            max_retries: None,
            timeout_seconds: None,
            subscriptions: None,
        }
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn headers(mut self, headers: Value) -> Self {
        self.headers = Some(headers);
        self
    }

    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }

    pub fn max_retries(mut self, max_retries: i32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    pub fn timeout_seconds(mut self, timeout_seconds: i32) -> Self {
        self.timeout_seconds = Some(timeout_seconds);
        self
    }

    pub fn subscriptions(mut self, subscriptions: Vec<EventSubscription>) -> Self {
        self.subscriptions = Some(subscriptions);
        self
    }

    pub async fn send(self) -> Result<WebhookEndpoint> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/webhooks/endpoints/{}",
            config.base_url, self.endpoint_id
        );

        let request = UpdateWebhookEndpointRequest {
            url: self.url,
            description: self.description,
            headers: self.headers,
            is_active: self.is_active,
            max_retries: self.max_retries,
            timeout_seconds: self.timeout_seconds,
            subscriptions: self.subscriptions,
        };

        let response = client.patch(&url).json(&request).send().await?;

        if response.status().is_success() {
            let endpoint = response.json().await?;
            Ok(endpoint)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to update webhook endpoint: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Builder for delete_webhook_endpoint
pub struct DeleteWebhookEndpointBuilder {
    endpoint_id: String,
}

impl DeleteWebhookEndpointBuilder {
    pub fn new(endpoint_id: &str) -> Self {
        Self {
            endpoint_id: endpoint_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/webhooks/endpoints/{}",
            config.base_url, self.endpoint_id
        );

        let response = client.delete(&url).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to delete webhook endpoint: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Builder for trigger_webhook_event
pub struct TriggerWebhookEventBuilder {
    app_slug: String,
    event_name: String,
    payload: Value,
    filter_context: Option<Value>,
}

impl TriggerWebhookEventBuilder {
    pub fn new(app_slug: &str, event_name: &str, payload: Value) -> Self {
        Self {
            app_slug: app_slug.to_string(),
            event_name: event_name.to_string(),
            payload,
            filter_context: None,
        }
    }

    pub fn filter_context(mut self, filter_context: Value) -> Self {
        self.filter_context = Some(filter_context);
        self
    }

    pub async fn send(self) -> Result<TriggerWebhookEventResponse> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/webhooks/apps/{}/trigger",
            config.base_url, self.app_slug
        );

        let request = TriggerWebhookEventRequest {
            app_slug: self.app_slug.clone(),
            event_name: self.event_name,
            payload: self.payload,
            filter_context: self.filter_context,
        };

        let response = client.post(&url).json(&request).send().await?;

        if response.status().is_success() {
            let result = response.json().await?;
            Ok(result)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to trigger webhook event: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Webhook event trigger for batch operations
#[derive(Debug, Serialize)]
pub struct WebhookEventTrigger {
    pub event_name: String,
    pub payload: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_context: Option<Value>,
}

/// Builder for get_webhook_deliveries
pub struct GetWebhookDeliveriesBuilder {
    app_name: String,
    endpoint_id: Option<i64>,
    event_name: Option<String>,
    status: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
    since: Option<String>,
    until: Option<String>,
}

impl GetWebhookDeliveriesBuilder {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            endpoint_id: None,
            event_name: None,
            status: None,
            limit: None,
            offset: None,
            since: None,
            until: None,
        }
    }

    pub fn endpoint_id(mut self, endpoint_id: i64) -> Self {
        self.endpoint_id = Some(endpoint_id);
        self
    }

    pub fn event_name(mut self, event_name: &str) -> Self {
        self.event_name = Some(event_name.to_string());
        self
    }

    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_string());
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn since(mut self, since: &str) -> Self {
        self.since = Some(since.to_string());
        self
    }

    pub fn until(mut self, until: &str) -> Self {
        self.until = Some(until.to_string());
        self
    }

    pub async fn send(self) -> Result<GetWebhookDeliveriesResponse> {
        let config = get_config();
        let client = get_client();
        let mut url = format!(
            "{}/webhooks/apps/{}/deliveries",
            config.base_url, self.app_name
        );

        let mut params = Vec::new();
        if let Some(id) = self.endpoint_id {
            params.push(format!("endpoint_id={id}"));
        }
        if let Some(event) = &self.event_name {
            params.push(format!("event_name={event}"));
        }
        if let Some(s) = &self.status {
            params.push(format!("status={s}"));
        }
        if let Some(l) = self.limit {
            params.push(format!("limit={l}"));
        }
        if let Some(o) = self.offset {
            params.push(format!("offset={o}"));
        }
        if let Some(since) = &self.since {
            params.push(format!("since={since}"));
        }
        if let Some(until) = &self.until {
            params.push(format!("until={until}"));
        }

        if !params.is_empty() {
            url.push('?');
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
                message: format!("Failed to get webhook deliveries: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Response for get_webhook_deliveries
#[derive(Debug, Deserialize)]
pub struct GetWebhookDeliveriesResponse {
    pub data: Vec<WebhookDelivery>,
    pub has_more: bool,
    #[serde(default)]
    pub limit: Option<i32>,
    #[serde(default)]
    pub offset: Option<i32>,
}

/// Builder for get_webhook_delivery_details
pub struct GetWebhookDeliveryDetailsBuilder {
    delivery_id: String,
}

impl GetWebhookDeliveryDetailsBuilder {
    pub fn new(delivery_id: &str) -> Self {
        Self {
            delivery_id: delivery_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<WebhookDeliveryDetails> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/webhooks/deliveries/{}",
            config.base_url, self.delivery_id
        );

        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            let result = response.json().await?;
            Ok(result)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to get webhook delivery details: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Builder for replay_webhook_deliveries
pub struct ReplayWebhookDeliveriesBuilder {
    app_name: String,
    request: ReplayWebhookDeliveriesRequest,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ReplayWebhookDeliveriesRequest {
    ByIds {
        delivery_ids: Vec<String>,
    },
    ByDateRange {
        start_date: DateTime<Utc>,
        #[serde(skip_serializing_if = "Option::is_none")]
        end_date: Option<DateTime<Utc>>,
    },
}

impl ReplayWebhookDeliveriesBuilder {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            request: ReplayWebhookDeliveriesRequest::ByIds {
                delivery_ids: Vec::new(),
            },
        }
    }

    pub fn by_ids(mut self, delivery_ids: Vec<String>) -> Self {
        self.request = ReplayWebhookDeliveriesRequest::ByIds { delivery_ids };
        self
    }

    pub fn by_date_range(
        mut self,
        start_date: DateTime<Utc>,
        end_date: Option<DateTime<Utc>>,
    ) -> Self {
        self.request = ReplayWebhookDeliveriesRequest::ByDateRange {
            start_date,
            end_date,
        };
        self
    }

    pub async fn send(self) -> Result<ReplayWebhookDeliveriesResponse> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/webhooks/apps/{}/deliveries/replay",
            config.base_url, self.app_name
        );

        let response = client.post(&url).json(&self.request).send().await?;

        if response.status().is_success() {
            let result = response.json().await?;
            Ok(result)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to replay webhook deliveries: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ReplayWebhookDeliveriesResponse {
    pub status: String,
    pub message: String,
}

/// Builder for reactivate_webhook_endpoint
pub struct ReactivateWebhookEndpointBuilder {
    endpoint_id: String,
}

impl ReactivateWebhookEndpointBuilder {
    pub fn new(endpoint_id: &str) -> Self {
        Self {
            endpoint_id: endpoint_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<ReactivateEndpointResponse> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/webhooks/endpoints/{}/reactivate",
            config.base_url, self.endpoint_id
        );

        let response = client.post(&url).send().await?;

        if response.status().is_success() {
            let result = response.json().await?;
            Ok(result)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to reactivate webhook endpoint: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ReactivateEndpointResponse {
    pub success: bool,
    pub message: String,
}

/// Builder for test_webhook_endpoint
pub struct TestWebhookEndpointBuilder {
    app_name: String,
    endpoint_id: String,
    event_name: String,
    payload: Option<Value>,
}

impl TestWebhookEndpointBuilder {
    pub fn new(app_name: &str, endpoint_id: &str, event_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            endpoint_id: endpoint_id.to_string(),
            event_name: event_name.to_string(),
            payload: None,
        }
    }

    pub fn payload(mut self, payload: Value) -> Self {
        self.payload = Some(payload);
        self
    }

    pub async fn send(self) -> Result<TestWebhookEndpointResponse> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/webhooks/apps/{}/endpoints/{}/test",
            config.base_url, self.app_name, self.endpoint_id
        );

        let request_body = serde_json::json!({
            "event_name": self.event_name,
            "payload": self.payload
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
                message: format!("Failed to test webhook endpoint: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Analytics result
#[derive(Debug, Deserialize)]
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

/// Builder for get_webhook_timeseries
pub struct GetWebhookTimeseriesBuilder {
    app_name: String,
    interval: String,
    start_date: Option<String>,
    end_date: Option<String>,
}

impl GetWebhookTimeseriesBuilder {
    pub fn new(app_name: &str, interval: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            interval: interval.to_string(),
            start_date: None,
            end_date: None,
        }
    }

    pub fn start_date(mut self, start_date: &str) -> Self {
        self.start_date = Some(start_date.to_string());
        self
    }

    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_string());
        self
    }

    pub async fn send(self) -> Result<TimeseriesResult> {
        let config = get_config();
        let client = get_client();
        let mut url = format!(
            "{}/webhooks/apps/{}/timeseries",
            config.base_url, self.app_name
        );

        // Build query parameters
        let mut query_params = Vec::new();
        query_params.push(format!("interval={}", urlencoding::encode(&self.interval)));
        if let Some(start) = &self.start_date {
            query_params.push(format!("start_date={}", urlencoding::encode(start)));
        }
        if let Some(end) = &self.end_date {
            query_params.push(format!("end_date={}", urlencoding::encode(end)));
        }

        if !query_params.is_empty() {
            url.push('?');
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
                message: format!("Failed to get webhook timeseries: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Builder for get_webhook_analytics
pub struct GetWebhookAnalyticsBuilder {
    app_name: String,
    start_date: Option<String>,
    end_date: Option<String>,
    endpoint_id: Option<i64>,
}

impl GetWebhookAnalyticsBuilder {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            start_date: None,
            end_date: None,
            endpoint_id: None,
        }
    }

    pub fn start_date(mut self, start_date: &str) -> Self {
        self.start_date = Some(start_date.to_string());
        self
    }

    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_string());
        self
    }

    pub fn endpoint_id(mut self, endpoint_id: i64) -> Self {
        self.endpoint_id = Some(endpoint_id);
        self
    }

    pub async fn send(self) -> Result<AnalyticsResult> {
        let config = get_config();
        let client = get_client();
        let mut url = format!(
            "{}/webhooks/apps/{}/analytics",
            config.base_url, self.app_name
        );

        // Build query parameters
        let mut query_params = Vec::new();
        if let Some(start) = &self.start_date {
            query_params.push(format!("start_date={}", urlencoding::encode(start)));
        }
        if let Some(end) = &self.end_date {
            query_params.push(format!("end_date={}", urlencoding::encode(end)));
        }
        if let Some(endpoint_id) = self.endpoint_id {
            query_params.push(format!("endpoint_id={}", endpoint_id));
        }

        if !query_params.is_empty() {
            url.push('?');
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
                message: format!("Failed to get webhook analytics: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// Builder for get_webhook_stats
pub struct GetWebhookStatsBuilder {
    app_name: String,
}

impl GetWebhookStatsBuilder {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
        }
    }

    pub async fn send(self) -> Result<AnalyticsResult> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/webhooks/apps/{}/stats", config.base_url, self.app_name);

        let response = client.get(&url).send().await?;

        if response.status().is_success() {
            let stats: AnalyticsResult = response.json().await?;
            Ok(stats)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: format!("Failed to get webhook stats: {error_text}"),
                details: serde_json::from_str(&error_text).ok(),
            })
        }
    }
}

/// List webhook apps
pub fn list_webhook_apps() -> ListWebhookAppsBuilder {
    ListWebhookAppsBuilder::new()
}

/// Get webhook app details
pub fn get_webhook_app(app_name: &str) -> GetWebhookAppBuilder {
    GetWebhookAppBuilder::new(app_name)
}

/// Create a webhook app
pub fn create_webhook_app(name: &str) -> CreateWebhookAppBuilder {
    CreateWebhookAppBuilder::new(name)
}

/// Update a webhook app
pub fn update_webhook_app(app_name: &str) -> UpdateWebhookAppBuilder {
    UpdateWebhookAppBuilder::new(app_name)
}

/// Delete a webhook app
pub fn delete_webhook_app(app_name: &str) -> DeleteWebhookAppBuilder {
    DeleteWebhookAppBuilder::new(app_name)
}

/// Rotate webhook secret
pub fn rotate_webhook_secret(app_name: &str) -> RotateWebhookSecretBuilder {
    RotateWebhookSecretBuilder::new(app_name)
}

/// Get webhook events
pub fn get_webhook_events(app_name: &str) -> GetWebhookEventsBuilder {
    GetWebhookEventsBuilder::new(app_name)
}

/// List webhook endpoints
pub fn list_webhook_endpoints(app_name: &str) -> ListWebhookEndpointsBuilder {
    ListWebhookEndpointsBuilder::new().app_name(app_name)
}

/// Get webhook endpoints with subscriptions
pub fn get_webhook_endpoints_with_subscriptions(
    app_name: &str,
) -> GetWebhookEndpointsWithSubscriptionsBuilder {
    GetWebhookEndpointsWithSubscriptionsBuilder::new(app_name)
}

/// Create a webhook endpoint
pub fn create_webhook_endpoint(app_name: &str, url: &str) -> CreateWebhookEndpointBuilder {
    CreateWebhookEndpointBuilder::new(app_name, url)
}

/// Update a webhook endpoint
pub fn update_webhook_endpoint(endpoint_id: &str) -> UpdateWebhookEndpointBuilder {
    UpdateWebhookEndpointBuilder::new(endpoint_id)
}

/// Delete a webhook endpoint
pub fn delete_webhook_endpoint(endpoint_id: &str) -> DeleteWebhookEndpointBuilder {
    DeleteWebhookEndpointBuilder::new(endpoint_id)
}

/// Trigger a webhook event
pub fn trigger_webhook_event(
    app_name: &str,
    event_name: &str,
    payload: Value,
) -> TriggerWebhookEventBuilder {
    TriggerWebhookEventBuilder::new(app_name, event_name, payload)
}

/// Batch trigger webhook events

/// List webhook deliveries
pub fn list_webhook_deliveries(app_name: &str) -> GetWebhookDeliveriesBuilder {
    GetWebhookDeliveriesBuilder::new(app_name)
}

/// Get webhook delivery details
pub fn get_webhook_delivery_details(delivery_id: &str) -> GetWebhookDeliveryDetailsBuilder {
    GetWebhookDeliveryDetailsBuilder::new(delivery_id)
}

/// Replay webhook deliveries
pub fn replay_webhook_deliveries(app_name: &str) -> ReplayWebhookDeliveriesBuilder {
    ReplayWebhookDeliveriesBuilder::new(app_name)
}

/// Reactivate a webhook endpoint
pub fn reactivate_webhook_endpoint(endpoint_id: &str) -> ReactivateWebhookEndpointBuilder {
    ReactivateWebhookEndpointBuilder::new(endpoint_id)
}

/// Test a webhook endpoint
pub fn test_webhook_endpoint(
    app_name: &str,
    endpoint_id: &str,
    event_name: &str,
) -> TestWebhookEndpointBuilder {
    TestWebhookEndpointBuilder::new(app_name, endpoint_id, event_name)
}

/// Get webhook stats
pub fn get_webhook_stats(app_name: &str) -> GetWebhookStatsBuilder {
    GetWebhookStatsBuilder::new(app_name)
}

/// Get webhook timeseries
pub fn get_webhook_timeseries(app_name: &str, interval: &str) -> GetWebhookTimeseriesBuilder {
    GetWebhookTimeseriesBuilder::new(app_name, interval)
}

/// Get webhook analytics
pub fn get_webhook_analytics(app_name: &str) -> GetWebhookAnalyticsBuilder {
    GetWebhookAnalyticsBuilder::new(app_name)
}
