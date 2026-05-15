//! Organization Invitations Module
//!
//! Lists, creates, and discards pending invitations to an organization.
//! Accepted-or-discarded invitations are soft-deleted in the same column,
//! so the data can't distinguish them — pass `include_deleted` on list to
//! see both.

use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{
        CreateOrganizationInvitationRequest, OrganizationInvitation,
        OrganizationInvitationSummary, PaginatedResponse,
    },
};

pub type OrganizationInvitationListResponse = PaginatedResponse<OrganizationInvitation>;

#[derive(Debug, Clone)]
pub struct OrganizationInvitationsApi {
    client: WachtClient,
}

impl OrganizationInvitationsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn list(&self, organization_id: &str) -> ListOrganizationInvitationsBuilder {
        ListOrganizationInvitationsBuilder::new(self.client.clone(), organization_id)
    }

    pub fn create(
        &self,
        organization_id: &str,
        request: CreateOrganizationInvitationRequest,
    ) -> CreateOrganizationInvitationBuilder {
        CreateOrganizationInvitationBuilder::new(self.client.clone(), organization_id, request)
    }

    pub fn discard(
        &self,
        organization_id: &str,
        invitation_id: &str,
    ) -> DiscardOrganizationInvitationBuilder {
        DiscardOrganizationInvitationBuilder::new(
            self.client.clone(),
            organization_id,
            invitation_id,
        )
    }
}

#[derive(Debug, Default, serde::Serialize)]
struct ListInvitationsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    workspace_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_deleted: Option<bool>,
}

pub struct ListOrganizationInvitationsBuilder {
    client: WachtClient,
    organization_id: String,
    query: ListInvitationsQuery,
}

impl ListOrganizationInvitationsBuilder {
    pub fn new(client: WachtClient, organization_id: &str) -> Self {
        Self {
            client,
            organization_id: organization_id.to_string(),
            query: ListInvitationsQuery::default(),
        }
    }

    /// Restrict results to invitations targeting a specific workspace within
    /// the organization.
    pub fn workspace_id(mut self, workspace_id: impl Into<String>) -> Self {
        self.query.workspace_id = Some(workspace_id.into());
        self
    }

    /// Include rows that were soft-deleted (by accept or admin discard).
    pub fn include_deleted(mut self, include_deleted: bool) -> Self {
        self.query.include_deleted = Some(include_deleted);
        self
    }

    pub async fn send(self) -> Result<OrganizationInvitationListResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/organizations/{}/invitations",
            self.client.config().base_url,
            self.organization_id
        );
        let response = client.get(&url).query(&self.query).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to list organization invitations",
                &error_body,
            ))
        }
    }
}

pub struct CreateOrganizationInvitationBuilder {
    client: WachtClient,
    organization_id: String,
    request: CreateOrganizationInvitationRequest,
}

impl CreateOrganizationInvitationBuilder {
    pub fn new(
        client: WachtClient,
        organization_id: &str,
        request: CreateOrganizationInvitationRequest,
    ) -> Self {
        Self {
            client,
            organization_id: organization_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<OrganizationInvitationSummary> {
        let client = self.client.http_client();
        let url = format!(
            "{}/organizations/{}/invitations",
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
                "Failed to create organization invitation",
                &error_body,
            ))
        }
    }
}

pub struct DiscardOrganizationInvitationBuilder {
    client: WachtClient,
    organization_id: String,
    invitation_id: String,
}

impl DiscardOrganizationInvitationBuilder {
    pub fn new(client: WachtClient, organization_id: &str, invitation_id: &str) -> Self {
        Self {
            client,
            organization_id: organization_id.to_string(),
            invitation_id: invitation_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/organizations/{}/invitations/{}/discard",
            self.client.config().base_url,
            self.organization_id,
            self.invitation_id
        );
        let response = client.post(&url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to discard organization invitation",
                &error_body,
            ))
        }
    }
}
