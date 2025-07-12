use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{AnalyticsStats, RecentSignup},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentSignupsResponse {
    pub data: Vec<RecentSignup>,
    pub total: u32,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AnalyticsStatsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
}

/// Fetch analytics statistics
pub async fn fetch_analytics_stats(options: Option<AnalyticsStatsOptions>) -> Result<AnalyticsStats> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/analytics/stats", config.base_url);
    
    let mut request = client.get(&url);
    
    if let Some(opts) = options {
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

/// Fetch recent signups
pub async fn fetch_recent_signups(limit: Option<u32>) -> Result<RecentSignupsResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/analytics/recent-signups", config.base_url);
    
    let mut request = client.get(&url);
    
    if let Some(limit) = limit {
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