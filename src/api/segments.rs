
use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{
        Segment, CreateSegmentRequest, UpdateSegmentRequest, SegmentListResponse,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListSegmentsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

/// Fetch segments
pub async fn fetch_segments(options: Option<ListSegmentsOptions>) -> Result<SegmentListResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/segments", config.base_url);
    
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
            message: format!("Failed to fetch segments: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Create segment
pub async fn create_segment(request: CreateSegmentRequest) -> Result<Segment> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/segments", config.base_url);
    
    let response = client.post(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to create segment: {}", error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Update segment
pub async fn update_segment(segment_id: &str, request: UpdateSegmentRequest) -> Result<Segment> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/segments/{}", config.base_url, segment_id);
    
    let response = client.patch(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to update segment {}: {}", segment_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Delete segment
pub async fn delete_segment(segment_id: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/segments/{}", config.base_url, segment_id);
    
    let response = client.delete(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to delete segment {}: {}", segment_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Assign segment to organization
pub async fn assign_organization_segment(organization_id: &str, segment_id: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/organizations/{}/segments", config.base_url, organization_id);
    
    #[derive(Serialize)]
    struct AssignSegmentRequest {
        segment_id: String,
    }

    let response = client.post(&url)
        .json(&AssignSegmentRequest { segment_id: segment_id.to_string() })
        .send()
        .await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to assign segment {} to organization {}: {}", segment_id, organization_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Remove segment from organization
pub async fn remove_organization_segment(organization_id: &str, segment_id: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/organizations/{}/segments/{}", config.base_url, organization_id, segment_id);
    
    let response = client.delete(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to remove segment {} from organization {}: {}", segment_id, organization_id, error_body),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}
