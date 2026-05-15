//! User Passkeys Module
//!
//! Admin operations on a user's registered passkeys: list, rename, delete.
//! Credential bytes are never exposed — only descriptive metadata.

use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{PaginatedResponse, RenamePasskeyRequest, UserPasskey},
};

pub type UserPasskeyListResponse = PaginatedResponse<UserPasskey>;

#[derive(Debug, Clone)]
pub struct UserPasskeysApi {
    client: WachtClient,
}

impl UserPasskeysApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn list(&self, user_id: &str) -> ListUserPasskeysBuilder {
        ListUserPasskeysBuilder::new(self.client.clone(), user_id)
    }

    pub fn rename(
        &self,
        user_id: &str,
        passkey_id: &str,
        name: impl Into<String>,
    ) -> RenameUserPasskeyBuilder {
        RenameUserPasskeyBuilder::new(self.client.clone(), user_id, passkey_id, name.into())
    }

    pub fn delete(&self, user_id: &str, passkey_id: &str) -> DeleteUserPasskeyBuilder {
        DeleteUserPasskeyBuilder::new(self.client.clone(), user_id, passkey_id)
    }
}

pub struct ListUserPasskeysBuilder {
    client: WachtClient,
    user_id: String,
}

impl ListUserPasskeysBuilder {
    pub fn new(client: WachtClient, user_id: &str) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<UserPasskeyListResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/passkeys",
            self.client.config().base_url,
            self.user_id
        );
        let response = client.get(&url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to list user passkeys",
                &error_body,
            ))
        }
    }
}

pub struct RenameUserPasskeyBuilder {
    client: WachtClient,
    user_id: String,
    passkey_id: String,
    name: String,
}

impl RenameUserPasskeyBuilder {
    pub fn new(client: WachtClient, user_id: &str, passkey_id: &str, name: String) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
            passkey_id: passkey_id.to_string(),
            name,
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/passkeys/{}",
            self.client.config().base_url,
            self.user_id,
            self.passkey_id
        );
        let body = RenamePasskeyRequest { name: self.name };
        let response = client.patch(&url).json(&body).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to rename passkey",
                &error_body,
            ))
        }
    }
}

pub struct DeleteUserPasskeyBuilder {
    client: WachtClient,
    user_id: String,
    passkey_id: String,
}

impl DeleteUserPasskeyBuilder {
    pub fn new(client: WachtClient, user_id: &str, passkey_id: &str) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
            passkey_id: passkey_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/passkeys/{}",
            self.client.config().base_url,
            self.user_id,
            self.passkey_id
        );
        let response = client.delete(&url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to delete passkey",
                &error_body,
            ))
        }
    }
}
