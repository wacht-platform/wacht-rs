use crate::{
    client::WachtClient,
    error::{Error, Result},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    #[serde(default)]
    pub daily_metrics: Vec<DailyAuthMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyAuthMetric {
    pub day: String,
    pub signins: i64,
    pub signups: i64,
}

#[derive(Debug, Clone)]
pub struct AnalyticsApi {
    client: WachtClient,
}

impl AnalyticsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn fetch_stats(&self) -> FetchStatsBuilder {
        FetchStatsBuilder::new(self.client.clone())
    }
}

/// Builder for fetching analytics statistics
pub struct FetchStatsBuilder {
    client: WachtClient,
    options: Option<AnalyticsStatsOptions>,
}

impl FetchStatsBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
            options: None,
        }
    }

    pub fn options(mut self, options: AnalyticsStatsOptions) -> Self {
        self.options = Some(options);
        self
    }

    pub async fn send(self) -> Result<AnalyticsStatsResponse> {
        let client = self.client.http_client();
        let url = format!("{}/analytics", self.client.config().base_url);

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
            Err(Error::api_from_text(
                status,
                "Failed to fetch analytics statistics",
                &error_body,
            ))
        }
    }
}
