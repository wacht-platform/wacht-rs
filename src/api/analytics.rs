use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::RecentSignup,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentSignupsResponse {
    pub signups: Vec<RecentSignup>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AnalyticsStatsOptions {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsStatsResponse {
    pub unique_signins: i64,
    pub signups: i64,
    pub organizations_created: i64,
    pub workspaces_created: i64,
    pub total_signups: i64,
    pub unique_signins_change: Option<f64>,
    pub signups_change: Option<f64>,
    pub organizations_created_change: Option<f64>,
    pub workspaces_created_change: Option<f64>,
}

/// Builder for fetching analytics statistics
pub struct FetchStatsBuilder {
    options: Option<AnalyticsStatsOptions>,
}

impl FetchStatsBuilder {
    pub fn new() -> Self {
        Self { options: None }
    }

    pub fn options(mut self, options: AnalyticsStatsOptions) -> Self {
        self.options = Some(options);
        self
    }

    pub async fn send(self) -> Result<AnalyticsStatsResponse> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/analytics/summary", config.base_url);

        let mut request = client.get(&url);

        if let Some(opts) = self.options {
            request = request.query(&opts);
        }

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch analytics statistics: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Fetch analytics statistics using builder pattern
pub fn fetch_stats() -> FetchStatsBuilder {
    FetchStatsBuilder::new()
}

/// Builder for fetching recent signups
pub struct FetchRecentSignupsBuilder {
    limit: Option<u32>,
}

impl FetchRecentSignupsBuilder {
    pub fn new() -> Self {
        Self { limit: None }
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub async fn send(self) -> Result<RecentSignupsResponse> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/analytics/recent-signups", config.base_url);

        let mut request = client.get(&url);

        if let Some(limit) = self.limit {
            request = request.query(&[("limit", limit)]);
        }

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch recent signups: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Fetch recent signups using builder pattern
pub fn fetch_recent_signups() -> FetchRecentSignupsBuilder {
    FetchRecentSignupsBuilder::new()
}

/// Builder for fetching recent signins
pub struct FetchRecentSigninsBuilder {
    limit: Option<u32>,
}

impl FetchRecentSigninsBuilder {
    pub fn new() -> Self {
        Self { limit: None }
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub async fn send(self) -> Result<RecentSignupsResponse> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/analytics/recent-signins", config.base_url);

        let mut request = client.get(&url);

        if let Some(limit) = self.limit {
            request = request.query(&[("limit", limit)]);
        }

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch recent signins: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Fetch recent signins using builder pattern
pub fn fetch_recent_signins() -> FetchRecentSigninsBuilder {
    FetchRecentSigninsBuilder::new()
}
