//! Organization Members Module
//!
//! Handles member management within organizations using builder pattern.

use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{
        AddOrganizationMemberRequest, ListOptions, OrganizationMember, PaginatedResponse,
        UpdateOrganizationMemberRequest,
    },
};

pub type OrganizationMemberListResponse = PaginatedResponse<OrganizationMember>;

#[derive(Debug, Clone)]
pub struct OrganizationMembersApi {
    client: WachtClient,
}

impl OrganizationMembersApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn fetch_members(&self, organization_id: &str) -> FetchMembersBuilder {
        FetchMembersBuilder::new(self.client.clone(), organization_id)
    }

    pub fn add_member(
        &self,
        organization_id: &str,
        request: AddOrganizationMemberRequest,
    ) -> AddMemberBuilder {
        AddMemberBuilder::new(self.client.clone(), organization_id, request)
    }

    pub fn update_member(
        &self,
        organization_id: &str,
        membership_id: &str,
        request: UpdateOrganizationMemberRequest,
    ) -> UpdateMemberBuilder {
        UpdateMemberBuilder::new(self.client.clone(), organization_id, membership_id, request)
    }

    pub fn remove_member(&self, organization_id: &str, membership_id: &str) -> RemoveMemberBuilder {
        RemoveMemberBuilder::new(self.client.clone(), organization_id, membership_id)
    }
}

/// Builder for fetching organization members
pub struct FetchMembersBuilder {
    client: WachtClient,
    organization_id: String,
    options: ListOptions,
}

impl FetchMembersBuilder {
    pub fn new(client: WachtClient, organization_id: &str) -> Self {
        Self {
            client,
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
        let client = self.client.http_client();
        let url = format!(
            "{}/organizations/{}/members",
            self.client.config().base_url,
            self.organization_id
        );

        let mut request = client.get(&url);
        request = request.query(&self.options);

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to fetch organization members",
                &error_body,
            ))
        }
    }
}

/// Builder for adding a member to organization
pub struct AddMemberBuilder {
    client: WachtClient,
    organization_id: String,
    request: AddOrganizationMemberRequest,
}

impl AddMemberBuilder {
    pub fn new(
        client: WachtClient,
        organization_id: &str,
        request: AddOrganizationMemberRequest,
    ) -> Self {
        Self {
            client,
            organization_id: organization_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<OrganizationMember> {
        let client = self.client.http_client();
        let url = format!(
            "{}/organizations/{}/members",
            self.client.config().base_url,
            self.organization_id
        );

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                format!(
                    "Failed to add member to organization {}",
                    self.organization_id
                ),
                &error_body,
            ))
        }
    }
}

/// Builder for updating organization member
pub struct UpdateMemberBuilder {
    client: WachtClient,
    organization_id: String,
    membership_id: String,
    request: UpdateOrganizationMemberRequest,
}

impl UpdateMemberBuilder {
    pub fn new(
        client: WachtClient,
        organization_id: &str,
        membership_id: &str,
        request: UpdateOrganizationMemberRequest,
    ) -> Self {
        Self {
            client,
            organization_id: organization_id.to_string(),
            membership_id: membership_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/organizations/{}/members/{}",
            self.client.config().base_url,
            self.organization_id,
            self.membership_id
        );

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                format!(
                    "Failed to update member {} in organization {}",
                    self.membership_id, self.organization_id
                ),
                &error_body,
            ))
        }
    }
}

/// Builder for removing organization member
pub struct RemoveMemberBuilder {
    client: WachtClient,
    organization_id: String,
    membership_id: String,
}

impl RemoveMemberBuilder {
    pub fn new(client: WachtClient, organization_id: &str, membership_id: &str) -> Self {
        Self {
            client,
            organization_id: organization_id.to_string(),
            membership_id: membership_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/organizations/{}/members/{}",
            self.client.config().base_url,
            self.organization_id,
            self.membership_id
        );

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                format!(
                    "Failed to remove member {} from organization {}",
                    self.membership_id, self.organization_id
                ),
                &error_body,
            ))
        }
    }
}
