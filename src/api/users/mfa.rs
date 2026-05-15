//! User MFA Module
//!
//! Administer a user's TOTP authenticator (create / delete) and regenerate
//! backup codes. The TOTP secret and freshly regenerated backup codes are
//! returned exactly once — store/display them immediately.

use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{
        CreateAuthenticatorRequest, CreateAuthenticatorResponse, RegeneratedBackupCodesResponse,
    },
};

#[derive(Debug, Clone)]
pub struct UserMfaApi {
    client: WachtClient,
}

impl UserMfaApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    /// Set up a TOTP authenticator on behalf of the user using an admin-
    /// provided base32 secret. Fails with 409 if the user already has an
    /// active authenticator — call `delete_authenticator` first to re-enroll.
    pub fn create_authenticator(
        &self,
        user_id: &str,
        request: CreateAuthenticatorRequest,
    ) -> CreateAuthenticatorBuilder {
        CreateAuthenticatorBuilder::new(self.client.clone(), user_id, request)
    }

    pub fn delete_authenticator(&self, user_id: &str) -> DeleteAuthenticatorBuilder {
        DeleteAuthenticatorBuilder::new(self.client.clone(), user_id)
    }

    /// Regenerate the user's backup codes. The new set is returned exactly
    /// once and replaces any prior codes.
    pub fn regenerate_backup_codes(&self, user_id: &str) -> RegenerateBackupCodesBuilder {
        RegenerateBackupCodesBuilder::new(self.client.clone(), user_id)
    }
}

pub struct CreateAuthenticatorBuilder {
    client: WachtClient,
    user_id: String,
    request: CreateAuthenticatorRequest,
}

impl CreateAuthenticatorBuilder {
    pub fn new(client: WachtClient, user_id: &str, request: CreateAuthenticatorRequest) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<CreateAuthenticatorResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/authenticators",
            self.client.config().base_url,
            self.user_id
        );
        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to create authenticator",
                &error_body,
            ))
        }
    }
}

pub struct DeleteAuthenticatorBuilder {
    client: WachtClient,
    user_id: String,
}

impl DeleteAuthenticatorBuilder {
    pub fn new(client: WachtClient, user_id: &str) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/authenticators",
            self.client.config().base_url,
            self.user_id
        );
        let response = client.delete(&url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to delete authenticator",
                &error_body,
            ))
        }
    }
}

pub struct RegenerateBackupCodesBuilder {
    client: WachtClient,
    user_id: String,
}

impl RegenerateBackupCodesBuilder {
    pub fn new(client: WachtClient, user_id: &str) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<RegeneratedBackupCodesResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/backup-codes/regenerate",
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
                "Failed to regenerate backup codes",
                &error_body,
            ))
        }
    }
}
