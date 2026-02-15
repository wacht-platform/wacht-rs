use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{CreateSegmentRequest, PaginatedResponse, Segment, UpdateSegmentRequest},
};
use serde::{Deserialize, Serialize};

pub type SegmentListResponse = PaginatedResponse<Segment>;

/// Analyzed entity from segment query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzedEntity {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

/// Filter options for user entities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

/// Filter options for organization entities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrganizationFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Filter options for workspace entities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkspaceFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Filters for segment data query
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SegmentDataFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<OrganizationFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<WorkspaceFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
}

/// Request to get segment data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSegmentDataRequest {
    pub target_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<SegmentDataFilters>,
}

/// Response from segment data query
pub type SegmentDataResponse = PaginatedResponse<AnalyzedEntity>;

/// Request to assign entity to segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignSegmentRequest {
    pub entity_id: String,
}

/// Request to remove entity from segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveSegmentRequest {
    pub entity_id: String,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListSegmentsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

/// Builder for listing segments
#[derive(Debug, Clone)]
pub struct FetchSegmentsBuilder {
    options: Option<ListSegmentsOptions>,
}

impl FetchSegmentsBuilder {
    pub fn new() -> Self {
        Self { options: None }
    }

    pub fn limit(mut self, limit: i32) -> Self {
        let mut options = self.options.unwrap_or_default();
        options.limit = Some(limit);
        self.options = Some(options);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        let mut options = self.options.unwrap_or_default();
        options.offset = Some(offset);
        self.options = Some(options);
        self
    }

    pub fn search(mut self, search: impl Into<String>) -> Self {
        let mut options = self.options.unwrap_or_default();
        options.search = Some(search.into());
        self.options = Some(options);
        self
    }

    pub fn sort_key(mut self, sort_key: impl Into<String>) -> Self {
        let mut options = self.options.unwrap_or_default();
        options.sort_key = Some(sort_key.into());
        self.options = Some(options);
        self
    }

    pub fn sort_order(mut self, sort_order: impl Into<String>) -> Self {
        let mut options = self.options.unwrap_or_default();
        options.sort_order = Some(sort_order.into());
        self.options = Some(options);
        self
    }

    pub fn options(mut self, options: ListSegmentsOptions) -> Self {
        self.options = Some(options);
        self
    }

    pub async fn send(self) -> Result<SegmentListResponse> {
        let options = self.options;
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
                message: format!("Failed to fetch segments: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

impl Default for FetchSegmentsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating segments
#[derive(Debug, Clone)]
pub struct CreateSegmentBuilder {
    request: CreateSegmentRequest,
}

impl CreateSegmentBuilder {
    pub fn new(request: CreateSegmentRequest) -> Self {
        Self { request }
    }

    pub async fn send(self) -> Result<Segment> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/segments", config.base_url);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to create segment: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for updating segments
#[derive(Debug, Clone)]
pub struct UpdateSegmentBuilder {
    segment_id: String,
    request: UpdateSegmentRequest,
}

impl UpdateSegmentBuilder {
    pub fn new(segment_id: impl Into<String>, request: UpdateSegmentRequest) -> Self {
        Self {
            segment_id: segment_id.into(),
            request,
        }
    }

    pub async fn send(self) -> Result<Segment> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/segments/{}", config.base_url, self.segment_id);

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update segment {}: {error_body}", self.segment_id),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for deleting segments
#[derive(Debug, Clone)]
pub struct DeleteSegmentBuilder {
    segment_id: String,
}

impl DeleteSegmentBuilder {
    pub fn new(segment_id: impl Into<String>) -> Self {
        Self {
            segment_id: segment_id.into(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/segments/{}", config.base_url, self.segment_id);

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete segment {}: {error_body}", self.segment_id),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for assigning entities to segments
#[derive(Debug, Clone)]
pub struct AssignSegmentBuilder {
    segment_id: String,
    entity_id: String,
}

impl AssignSegmentBuilder {
    pub fn new(segment_id: impl Into<String>, entity_id: impl Into<String>) -> Self {
        Self {
            segment_id: segment_id.into(),
            entity_id: entity_id.into(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/segments/{}/assign", config.base_url, self.segment_id);

        let response = client
            .post(&url)
            .json(&AssignSegmentRequest {
                entity_id: self.entity_id.clone(),
            })
            .send()
            .await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!(
                    "Failed to assign entity to segment {}: {error_body}",
                    self.segment_id
                ),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for removing entities from segments
#[derive(Debug, Clone)]
pub struct RemoveSegmentBuilder {
    segment_id: String,
    entity_id: String,
}

impl RemoveSegmentBuilder {
    pub fn new(segment_id: impl Into<String>, entity_id: impl Into<String>) -> Self {
        Self {
            segment_id: segment_id.into(),
            entity_id: entity_id.into(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/segments/{}/remove", config.base_url, self.segment_id);

        let response = client
            .post(&url)
            .json(&RemoveSegmentRequest {
                entity_id: self.entity_id.clone(),
            })
            .send()
            .await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!(
                    "Failed to remove entity from segment {}: {error_body}",
                    self.segment_id
                ),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for getting segment data
#[derive(Debug, Clone)]
pub struct GetSegmentDataBuilder {
    segment_id: String,
    target_type: String,
    filters: Option<SegmentDataFilters>,
}

impl GetSegmentDataBuilder {
    pub fn new(segment_id: impl Into<String>, target_type: impl Into<String>) -> Self {
        Self {
            segment_id: segment_id.into(),
            target_type: target_type.into(),
            filters: None,
        }
    }

    pub fn filters(mut self, filters: SegmentDataFilters) -> Self {
        self.filters = Some(filters);
        self
    }

    pub fn user_filter(mut self, filter: UserFilter) -> Self {
        let mut filters = self.filters.unwrap_or_default();
        filters.user = Some(filter);
        self.filters = Some(filters);
        self
    }

    pub fn organization_filter(mut self, filter: OrganizationFilter) -> Self {
        let mut filters = self.filters.unwrap_or_default();
        filters.organization = Some(filter);
        self.filters = Some(filters);
        self
    }

    pub fn workspace_filter(mut self, filter: WorkspaceFilter) -> Self {
        let mut filters = self.filters.unwrap_or_default();
        filters.workspace = Some(filter);
        self.filters = Some(filters);
        self
    }

    pub async fn send(self) -> Result<SegmentDataResponse> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/segments/data", config.base_url);

        // Build filters with segment_id included
        let mut final_filters = self.filters.unwrap_or_default();
        if final_filters.segment_id.is_none() {
            final_filters.segment_id = Some(self.segment_id.clone());
        }

        let request = GetSegmentDataRequest {
            target_type: self.target_type.clone(),
            filters: Some(final_filters),
        };

        let response = client.post(&url).json(&request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to get segment data: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Fetch segments
pub async fn fetch_segments(options: Option<ListSegmentsOptions>) -> Result<SegmentListResponse> {
    FetchSegmentsBuilder::new()
        .options(options.unwrap_or_default())
        .send()
        .await
}

/// Create segment
pub async fn create_segment(request: CreateSegmentRequest) -> Result<Segment> {
    CreateSegmentBuilder::new(request).send().await
}

/// Update segment
pub async fn update_segment(segment_id: &str, request: UpdateSegmentRequest) -> Result<Segment> {
    UpdateSegmentBuilder::new(segment_id, request).send().await
}

/// Delete segment
pub async fn delete_segment(segment_id: &str) -> Result<()> {
    DeleteSegmentBuilder::new(segment_id).send().await
}

/// Assign entity to segment
pub async fn assign_segment(segment_id: &str, entity_id: &str) -> Result<()> {
    AssignSegmentBuilder::new(segment_id, entity_id)
        .send()
        .await
}

/// Remove entity from segment
pub async fn remove_segment(segment_id: &str, entity_id: &str) -> Result<()> {
    RemoveSegmentBuilder::new(segment_id, entity_id)
        .send()
        .await
}

/// Get segment data
pub async fn get_segment_data(
    segment_id: &str,
    target_type: &str,
    filters: Option<SegmentDataFilters>,
) -> Result<SegmentDataResponse> {
    let mut builder = GetSegmentDataBuilder::new(segment_id, target_type);

    if let Some(filters) = filters {
        builder = builder.filters(filters);
    }

    builder.send().await
}
