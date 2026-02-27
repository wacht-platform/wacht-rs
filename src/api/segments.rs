use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{CreateSegmentRequest, PaginatedResponse, Segment, UpdateSegmentRequest},
};
use serde::{Deserialize, Serialize};

pub type SegmentListResponse = PaginatedResponse<Segment>;

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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrganizationFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkspaceFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSegmentDataRequest {
    pub target_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<SegmentDataFilters>,
}

pub type SegmentDataResponse = PaginatedResponse<AnalyzedEntity>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignSegmentRequest {
    pub entity_id: String,
}

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

#[derive(Debug, Clone)]
pub struct SegmentsApi {
    client: WachtClient,
}

impl SegmentsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn fetch_segments(&self) -> FetchSegmentsBuilder {
        FetchSegmentsBuilder::new(self.client.clone())
    }

    pub fn create_segment(&self, request: CreateSegmentRequest) -> CreateSegmentBuilder {
        CreateSegmentBuilder::new(self.client.clone(), request)
    }

    pub fn update_segment(
        &self,
        segment_id: impl Into<String>,
        request: UpdateSegmentRequest,
    ) -> UpdateSegmentBuilder {
        UpdateSegmentBuilder::new(self.client.clone(), segment_id, request)
    }

    pub fn delete_segment(&self, segment_id: impl Into<String>) -> DeleteSegmentBuilder {
        DeleteSegmentBuilder::new(self.client.clone(), segment_id)
    }

    pub fn assign_segment(
        &self,
        segment_id: impl Into<String>,
        entity_id: impl Into<String>,
    ) -> AssignSegmentBuilder {
        AssignSegmentBuilder::new(self.client.clone(), segment_id, entity_id)
    }

    pub fn remove_segment(
        &self,
        segment_id: impl Into<String>,
        entity_id: impl Into<String>,
    ) -> RemoveSegmentBuilder {
        RemoveSegmentBuilder::new(self.client.clone(), segment_id, entity_id)
    }

    pub fn get_segment_data(
        &self,
        segment_id: impl Into<String>,
        target_type: impl Into<String>,
    ) -> GetSegmentDataBuilder {
        GetSegmentDataBuilder::new(self.client.clone(), segment_id, target_type)
    }
}

#[derive(Debug, Clone)]
pub struct FetchSegmentsBuilder {
    client: WachtClient,
    options: Option<ListSegmentsOptions>,
}

impl FetchSegmentsBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
            options: None,
        }
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
        let client = self.client.http_client();
        let url = format!("{}/segments", self.client.config().base_url);

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
                message: format!("Failed to fetch segments: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateSegmentBuilder {
    client: WachtClient,
    request: CreateSegmentRequest,
}

impl CreateSegmentBuilder {
    pub fn new(client: WachtClient, request: CreateSegmentRequest) -> Self {
        Self { client, request }
    }

    pub async fn send(self) -> Result<Segment> {
        let client = self.client.http_client();
        let url = format!("{}/segments", self.client.config().base_url);

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

#[derive(Debug, Clone)]
pub struct UpdateSegmentBuilder {
    client: WachtClient,
    segment_id: String,
    request: UpdateSegmentRequest,
}

impl UpdateSegmentBuilder {
    pub fn new(
        client: WachtClient,
        segment_id: impl Into<String>,
        request: UpdateSegmentRequest,
    ) -> Self {
        Self {
            client,
            segment_id: segment_id.into(),
            request,
        }
    }

    pub async fn send(self) -> Result<Segment> {
        let client = self.client.http_client();
        let url = format!("{}/segments/{}", self.client.config().base_url, self.segment_id);

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

#[derive(Debug, Clone)]
pub struct DeleteSegmentBuilder {
    client: WachtClient,
    segment_id: String,
}

impl DeleteSegmentBuilder {
    pub fn new(client: WachtClient, segment_id: impl Into<String>) -> Self {
        Self {
            client,
            segment_id: segment_id.into(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!("{}/segments/{}", self.client.config().base_url, self.segment_id);

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

#[derive(Debug, Clone)]
pub struct AssignSegmentBuilder {
    client: WachtClient,
    segment_id: String,
    entity_id: String,
}

impl AssignSegmentBuilder {
    pub fn new(
        client: WachtClient,
        segment_id: impl Into<String>,
        entity_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            segment_id: segment_id.into(),
            entity_id: entity_id.into(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!("{}/segments/{}/assign", self.client.config().base_url, self.segment_id);

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

#[derive(Debug, Clone)]
pub struct RemoveSegmentBuilder {
    client: WachtClient,
    segment_id: String,
    entity_id: String,
}

impl RemoveSegmentBuilder {
    pub fn new(
        client: WachtClient,
        segment_id: impl Into<String>,
        entity_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            segment_id: segment_id.into(),
            entity_id: entity_id.into(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!("{}/segments/{}/remove", self.client.config().base_url, self.segment_id);

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

#[derive(Debug, Clone)]
pub struct GetSegmentDataBuilder {
    client: WachtClient,
    segment_id: String,
    target_type: String,
    filters: Option<SegmentDataFilters>,
}

impl GetSegmentDataBuilder {
    pub fn new(
        client: WachtClient,
        segment_id: impl Into<String>,
        target_type: impl Into<String>,
    ) -> Self {
        Self {
            client,
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
        let client = self.client.http_client();
        let url = format!("{}/segments/data", self.client.config().base_url);

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
