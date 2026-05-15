//! User Sessions Module
//!
//! Lists and revokes a user's active sign-ins. The "revoke-all" endpoint
//! returns a count so callers can confirm something actually happened.

use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{PaginatedResponse, RevokeAllSigninsResponse, UserSignin},
};

pub type UserSigninListResponse = PaginatedResponse<UserSignin>;

#[derive(Debug, Clone)]
pub struct UserSessionsApi {
    client: WachtClient,
}

impl UserSessionsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn list(&self, user_id: &str) -> ListUserSigninsBuilder {
        ListUserSigninsBuilder::new(self.client.clone(), user_id)
    }

    pub fn revoke(&self, user_id: &str, signin_id: &str) -> RevokeSigninBuilder {
        RevokeSigninBuilder::new(self.client.clone(), user_id, signin_id)
    }

    pub fn revoke_all(&self, user_id: &str) -> RevokeAllSigninsBuilder {
        RevokeAllSigninsBuilder::new(self.client.clone(), user_id)
    }
}

#[derive(Debug, Default, serde::Serialize)]
struct ListSigninsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_expired: Option<bool>,
}

pub struct ListUserSigninsBuilder {
    client: WachtClient,
    user_id: String,
    query: ListSigninsQuery,
}

impl ListUserSigninsBuilder {
    pub fn new(client: WachtClient, user_id: &str) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
            query: ListSigninsQuery::default(),
        }
    }

    /// Include expired sign-ins in the result. Defaults to false.
    pub fn include_expired(mut self, include_expired: bool) -> Self {
        self.query.include_expired = Some(include_expired);
        self
    }

    pub async fn send(self) -> Result<UserSigninListResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/sessions",
            self.client.config().base_url,
            self.user_id
        );
        let response = client.get(&url).query(&self.query).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to list user sessions",
                &error_body,
            ))
        }
    }
}

pub struct RevokeSigninBuilder {
    client: WachtClient,
    user_id: String,
    signin_id: String,
}

impl RevokeSigninBuilder {
    pub fn new(client: WachtClient, user_id: &str, signin_id: &str) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
            signin_id: signin_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/sessions/{}/revoke",
            self.client.config().base_url,
            self.user_id,
            self.signin_id
        );
        let response = client.post(&url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to revoke session",
                &error_body,
            ))
        }
    }
}

pub struct RevokeAllSigninsBuilder {
    client: WachtClient,
    user_id: String,
}

impl RevokeAllSigninsBuilder {
    pub fn new(client: WachtClient, user_id: &str) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<RevokeAllSigninsResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/sessions/revoke-all",
            self.client.config().base_url,
            self.user_id
        );
        let response = client.post(&url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to revoke all user sessions",
                &error_body,
            ))
        }
    }
}
