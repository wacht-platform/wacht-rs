//! Organization Members Module
//!
//! Handles member management within organizations using builder pattern.

use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{PaginatedResponse, OrganizationMember, AddOrganizationMemberRequest, UpdateOrganizationMemberRequest, ListOptions},
};

pub type OrganizationMemberListResponse = PaginatedResponse<OrganizationMember>;

/// Builder for fetching organization members
pub struct FetchMembersBuilder {
    organization_id: String,
    options: ListOptions,
}

impl FetchMembersBuilder {
    pub fn new(organization_id: &str) -> Self {
        Self {
            organization_id: organization_id.to_string(),
            options: ListOptions::default(),
        }
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.options.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.options.offset = Some(offset);
        self
    }

    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.options.search = Some(search.into());
        self
    }

    pub fn sort_key(mut self, sort_key: impl Into<String>) -> Self {
        self.options.sort_key = Some(sort_key.into());
        self
    }

    pub fn sort_order(mut self, sort_order: impl Into<String>) -> Self {
        self.options.sort_order = Some(sort_order.into());
        self
    }

    pub async fn send(self) -> Result<OrganizationMemberListResponse> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/organizations/{}/members", config.base_url, self.organization_id);

        let mut request = client.get(&url);
        request = request.query(&self.options);

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch organization members: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Fetch organization members using builder pattern
pub fn fetch_members(organization_id: &str) -> FetchMembersBuilder {
    FetchMembersBuilder::new(organization_id)
}

/// Builder for adding a member to organization
pub struct AddMemberBuilder {
    organization_id: String,
    request: AddOrganizationMemberRequest,
}

impl AddMemberBuilder {
    pub fn new(organization_id: &str, request: AddOrganizationMemberRequest) -> Self {
        Self {
            organization_id: organization_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<OrganizationMember> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/organizations/{}/members", config.base_url, self.organization_id);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to add member to organization {}: {}", self.organization_id, error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Add member to organization using builder pattern
pub fn add_member(organization_id: &str, request: AddOrganizationMemberRequest) -> AddMemberBuilder {
    AddMemberBuilder::new(organization_id, request)
}

/// Builder for updating organization member
pub struct UpdateMemberBuilder {
    organization_id: String,
    membership_id: String,
    request: UpdateOrganizationMemberRequest,
}

impl UpdateMemberBuilder {
    pub fn new(organization_id: &str, membership_id: &str, request: UpdateOrganizationMemberRequest) -> Self {
        Self {
            organization_id: organization_id.to_string(),
            membership_id: membership_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/organizations/{}/members/{}",
            config.base_url, self.organization_id, self.membership_id
        );

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update member {} in organization {}: {}", self.membership_id, self.organization_id, error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Update organization member using builder pattern
pub fn update_member(organization_id: &str, membership_id: &str, request: UpdateOrganizationMemberRequest) -> UpdateMemberBuilder {
    UpdateMemberBuilder::new(organization_id, membership_id, request)
}

/// Builder for removing organization member
pub struct RemoveMemberBuilder {
    organization_id: String,
    membership_id: String,
}

impl RemoveMemberBuilder {
    pub fn new(organization_id: &str, membership_id: &str) -> Self {
        Self {
            organization_id: organization_id.to_string(),
            membership_id: membership_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/organizations/{}/members/{}",
            config.base_url, self.organization_id, self.membership_id
        );

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to remove member {} from organization {}: {}", self.membership_id, self.organization_id, error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Remove organization member using builder pattern
pub fn remove_member(organization_id: &str, membership_id: &str) -> RemoveMemberBuilder {
    RemoveMemberBuilder::new(organization_id, membership_id)
}
