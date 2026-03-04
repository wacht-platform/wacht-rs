use crate::{
    Result,
    client::WachtClient,
    error::Error,
    models::{
        CreateOAuthAppRequest, CreateOAuthClientRequest, ListOAuthAppsResponse,
        ListOAuthClientsResponse, ListOAuthGrantsResponse, OAuthApp, OAuthClient,
        OAuthDomainVerificationResponse, OAuthGrant, RotateOAuthClientSecretResponse,
        SetOAuthScopeMappingRequest, UpdateOAuthAppRequest, UpdateOAuthClientRequest,
        UpdateOAuthScopeRequest,
    },
};
use serde_json::Value;

fn unwrap_data(value: Value) -> Value {
    value
        .get("data")
        .cloned()
        .unwrap_or(value)
}

fn api_error(status: reqwest::StatusCode, prefix: &str, error_text: &str) -> Error {
    Error::Api {
        status,
        message: format!("{prefix}: {error_text}"),
        details: serde_json::from_str(error_text).ok(),
    }
}

#[derive(Debug, Clone)]
pub struct OauthApi {
    client: WachtClient,
}

impl OauthApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn list_oauth_apps(&self, deployment_id: &str) -> ListOAuthAppsBuilder {
        ListOAuthAppsBuilder::new(self.client.clone(), deployment_id)
    }

    pub fn create_oauth_app(
        &self,
        deployment_id: &str,
        request: CreateOAuthAppRequest,
    ) -> CreateOAuthAppBuilder {
        CreateOAuthAppBuilder::new(self.client.clone(), deployment_id, request)
    }

    pub fn update_oauth_app(
        &self,
        deployment_id: &str,
        oauth_app_slug: &str,
        request: UpdateOAuthAppRequest,
    ) -> UpdateOAuthAppBuilder {
        UpdateOAuthAppBuilder::new(self.client.clone(), deployment_id, oauth_app_slug, request)
    }

    pub fn verify_oauth_app_domain(
        &self,
        deployment_id: &str,
        oauth_app_slug: &str,
    ) -> VerifyOAuthAppDomainBuilder {
        VerifyOAuthAppDomainBuilder::new(self.client.clone(), deployment_id, oauth_app_slug)
    }

    pub fn update_oauth_scope(
        &self,
        deployment_id: &str,
        oauth_app_slug: &str,
        scope: &str,
        request: UpdateOAuthScopeRequest,
    ) -> UpdateOAuthScopeBuilder {
        UpdateOAuthScopeBuilder::new(
            self.client.clone(),
            deployment_id,
            oauth_app_slug,
            scope,
            request,
        )
    }

    pub fn archive_oauth_scope(
        &self,
        deployment_id: &str,
        oauth_app_slug: &str,
        scope: &str,
    ) -> ArchiveOAuthScopeBuilder {
        ArchiveOAuthScopeBuilder::new(self.client.clone(), deployment_id, oauth_app_slug, scope)
    }

    pub fn unarchive_oauth_scope(
        &self,
        deployment_id: &str,
        oauth_app_slug: &str,
        scope: &str,
    ) -> UnarchiveOAuthScopeBuilder {
        UnarchiveOAuthScopeBuilder::new(self.client.clone(), deployment_id, oauth_app_slug, scope)
    }

    pub fn set_oauth_scope_mapping(
        &self,
        deployment_id: &str,
        oauth_app_slug: &str,
        scope: &str,
        request: SetOAuthScopeMappingRequest,
    ) -> SetOAuthScopeMappingBuilder {
        SetOAuthScopeMappingBuilder::new(
            self.client.clone(),
            deployment_id,
            oauth_app_slug,
            scope,
            request,
        )
    }

    pub fn list_oauth_clients(
        &self,
        deployment_id: &str,
        oauth_app_slug: &str,
    ) -> ListOAuthClientsBuilder {
        ListOAuthClientsBuilder::new(self.client.clone(), deployment_id, oauth_app_slug)
    }

    pub fn create_oauth_client(
        &self,
        deployment_id: &str,
        oauth_app_slug: &str,
        request: CreateOAuthClientRequest,
    ) -> CreateOAuthClientBuilder {
        CreateOAuthClientBuilder::new(self.client.clone(), deployment_id, oauth_app_slug, request)
    }

    pub fn update_oauth_client(
        &self,
        deployment_id: &str,
        oauth_app_slug: &str,
        oauth_client_id: &str,
        request: UpdateOAuthClientRequest,
    ) -> UpdateOAuthClientBuilder {
        UpdateOAuthClientBuilder::new(
            self.client.clone(),
            deployment_id,
            oauth_app_slug,
            oauth_client_id,
            request,
        )
    }

    pub fn deactivate_oauth_client(
        &self,
        deployment_id: &str,
        oauth_app_slug: &str,
        oauth_client_id: &str,
    ) -> DeactivateOAuthClientBuilder {
        DeactivateOAuthClientBuilder::new(
            self.client.clone(),
            deployment_id,
            oauth_app_slug,
            oauth_client_id,
        )
    }

    pub fn rotate_oauth_client_secret(
        &self,
        deployment_id: &str,
        oauth_app_slug: &str,
        oauth_client_id: &str,
    ) -> RotateOAuthClientSecretBuilder {
        RotateOAuthClientSecretBuilder::new(
            self.client.clone(),
            deployment_id,
            oauth_app_slug,
            oauth_client_id,
        )
    }

    pub fn list_oauth_grants(
        &self,
        deployment_id: &str,
        oauth_app_slug: &str,
        oauth_client_id: &str,
    ) -> ListOAuthGrantsBuilder {
        ListOAuthGrantsBuilder::new(
            self.client.clone(),
            deployment_id,
            oauth_app_slug,
            oauth_client_id,
        )
    }

    pub fn revoke_oauth_grant(
        &self,
        deployment_id: &str,
        oauth_app_slug: &str,
        oauth_client_id: &str,
        grant_id: &str,
    ) -> RevokeOAuthGrantBuilder {
        RevokeOAuthGrantBuilder::new(
            self.client.clone(),
            deployment_id,
            oauth_app_slug,
            oauth_client_id,
            grant_id,
        )
    }
}

pub struct ListOAuthAppsBuilder {
    client: WachtClient,
    deployment_id: String,
}

impl ListOAuthAppsBuilder {
    pub fn new(client: WachtClient, deployment_id: &str) -> Self {
        Self {
            client,
            deployment_id: deployment_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<Vec<OAuthApp>> {
        let client = self.client.http_client();
        let url = format!(
            "{}/deployments/{}/oauth/apps",
            self.client.config().base_url,
            self.deployment_id
        );

        let response = client.get(&url).send().await?;
        if response.status().is_success() {
            let payload: Value = response.json().await?;
            let data = unwrap_data(payload);
            let parsed: ListOAuthAppsResponse = serde_json::from_value(data).unwrap_or_default();
            Ok(parsed.apps)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(api_error(status, "Failed to list OAuth apps", &error_text))
        }
    }
}

pub struct CreateOAuthAppBuilder {
    client: WachtClient,
    deployment_id: String,
    request: CreateOAuthAppRequest,
}

impl CreateOAuthAppBuilder {
    pub fn new(client: WachtClient, deployment_id: &str, request: CreateOAuthAppRequest) -> Self {
        Self {
            client,
            deployment_id: deployment_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<OAuthApp> {
        let client = self.client.http_client();
        let url = format!(
            "{}/deployments/{}/oauth/apps",
            self.client.config().base_url,
            self.deployment_id
        );

        let mut request = self.request;
        let response = if let Some(bytes) = request.logo_file.take() {
            let mut form = reqwest::multipart::Form::new()
                .text("slug", request.slug)
                .text("name", request.name);

            if let Some(description) = request.description {
                form = form.text("description", description);
            }
            if let Some(fqdn) = request.fqdn {
                form = form.text("fqdn", fqdn);
            }
            if let Some(scopes) = request.supported_scopes {
                form = form.text("supported_scopes", scopes.join(","));
            }
            if let Some(scope_definitions) = request.scope_definitions {
                form = form.text("scope_definitions", serde_json::to_string(&scope_definitions)?);
            }
            if let Some(allow_dynamic_client_registration) =
                request.allow_dynamic_client_registration
            {
                form = form.text(
                    "allow_dynamic_client_registration",
                    if allow_dynamic_client_registration {
                        "true".to_string()
                    } else {
                        "false".to_string()
                    },
                );
            }

            let logo_part = reqwest::multipart::Part::bytes(bytes)
                .file_name(request.logo_filename.unwrap_or_else(|| "logo.png".to_string()));
            form = form.part("logo", logo_part);

            client.post(&url).multipart(form).send().await?
        } else {
            client.post(&url).json(&request).send().await?
        };
        if response.status().is_success() {
            let payload: Value = response.json().await?;
            let data = unwrap_data(payload);
            Ok(serde_json::from_value(data)?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(api_error(status, "Failed to create OAuth app", &error_text))
        }
    }
}

pub struct UpdateOAuthAppBuilder {
    client: WachtClient,
    deployment_id: String,
    oauth_app_slug: String,
    request: UpdateOAuthAppRequest,
}

impl UpdateOAuthAppBuilder {
    pub fn new(
        client: WachtClient,
        deployment_id: &str,
        oauth_app_slug: &str,
        request: UpdateOAuthAppRequest,
    ) -> Self {
        Self {
            client,
            deployment_id: deployment_id.to_string(),
            oauth_app_slug: oauth_app_slug.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<OAuthApp> {
        let client = self.client.http_client();
        let url = format!(
            "{}/deployments/{}/oauth/apps/{}",
            self.client.config().base_url,
            self.deployment_id,
            self.oauth_app_slug
        );

        let response = client.patch(&url).json(&self.request).send().await?;
        if response.status().is_success() {
            let payload: Value = response.json().await?;
            let data = unwrap_data(payload);
            Ok(serde_json::from_value(data)?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(api_error(status, "Failed to update OAuth app", &error_text))
        }
    }
}

pub struct VerifyOAuthAppDomainBuilder {
    client: WachtClient,
    deployment_id: String,
    oauth_app_slug: String,
}

impl VerifyOAuthAppDomainBuilder {
    pub fn new(client: WachtClient, deployment_id: &str, oauth_app_slug: &str) -> Self {
        Self {
            client,
            deployment_id: deployment_id.to_string(),
            oauth_app_slug: oauth_app_slug.to_string(),
        }
    }

    pub async fn send(self) -> Result<OAuthDomainVerificationResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/deployments/{}/oauth/apps/{}/verify-domain",
            self.client.config().base_url,
            self.deployment_id,
            self.oauth_app_slug
        );

        let response = client.post(&url).send().await?;
        if response.status().is_success() {
            let payload: Value = response.json().await?;
            let data = unwrap_data(payload);
            Ok(serde_json::from_value(data)?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(api_error(status, "Failed to verify OAuth app domain", &error_text))
        }
    }
}

pub struct UpdateOAuthScopeBuilder {
    client: WachtClient,
    deployment_id: String,
    oauth_app_slug: String,
    scope: String,
    request: UpdateOAuthScopeRequest,
}

impl UpdateOAuthScopeBuilder {
    pub fn new(
        client: WachtClient,
        deployment_id: &str,
        oauth_app_slug: &str,
        scope: &str,
        request: UpdateOAuthScopeRequest,
    ) -> Self {
        Self {
            client,
            deployment_id: deployment_id.to_string(),
            oauth_app_slug: oauth_app_slug.to_string(),
            scope: urlencoding::encode(scope).to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<OAuthApp> {
        let client = self.client.http_client();
        let url = format!(
            "{}/deployments/{}/oauth/apps/{}/scopes/{}",
            self.client.config().base_url,
            self.deployment_id,
            self.oauth_app_slug,
            self.scope
        );

        let response = client.patch(&url).json(&self.request).send().await?;
        if response.status().is_success() {
            let payload: Value = response.json().await?;
            let data = unwrap_data(payload);
            Ok(serde_json::from_value(data)?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(api_error(status, "Failed to update OAuth scope", &error_text))
        }
    }
}

pub struct ArchiveOAuthScopeBuilder {
    client: WachtClient,
    deployment_id: String,
    oauth_app_slug: String,
    scope: String,
}

impl ArchiveOAuthScopeBuilder {
    pub fn new(client: WachtClient, deployment_id: &str, oauth_app_slug: &str, scope: &str) -> Self {
        Self {
            client,
            deployment_id: deployment_id.to_string(),
            oauth_app_slug: oauth_app_slug.to_string(),
            scope: urlencoding::encode(scope).to_string(),
        }
    }

    pub async fn send(self) -> Result<OAuthApp> {
        let client = self.client.http_client();
        let url = format!(
            "{}/deployments/{}/oauth/apps/{}/scopes/{}/archive",
            self.client.config().base_url,
            self.deployment_id,
            self.oauth_app_slug,
            self.scope
        );

        let response = client.post(&url).send().await?;
        if response.status().is_success() {
            let payload: Value = response.json().await?;
            let data = unwrap_data(payload);
            Ok(serde_json::from_value(data)?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(api_error(status, "Failed to archive OAuth scope", &error_text))
        }
    }
}

pub struct UnarchiveOAuthScopeBuilder {
    client: WachtClient,
    deployment_id: String,
    oauth_app_slug: String,
    scope: String,
}

impl UnarchiveOAuthScopeBuilder {
    pub fn new(client: WachtClient, deployment_id: &str, oauth_app_slug: &str, scope: &str) -> Self {
        Self {
            client,
            deployment_id: deployment_id.to_string(),
            oauth_app_slug: oauth_app_slug.to_string(),
            scope: urlencoding::encode(scope).to_string(),
        }
    }

    pub async fn send(self) -> Result<OAuthApp> {
        let client = self.client.http_client();
        let url = format!(
            "{}/deployments/{}/oauth/apps/{}/scopes/{}/unarchive",
            self.client.config().base_url,
            self.deployment_id,
            self.oauth_app_slug,
            self.scope
        );

        let response = client.post(&url).send().await?;
        if response.status().is_success() {
            let payload: Value = response.json().await?;
            let data = unwrap_data(payload);
            Ok(serde_json::from_value(data)?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(api_error(status, "Failed to unarchive OAuth scope", &error_text))
        }
    }
}

pub struct SetOAuthScopeMappingBuilder {
    client: WachtClient,
    deployment_id: String,
    oauth_app_slug: String,
    scope: String,
    request: SetOAuthScopeMappingRequest,
}

impl SetOAuthScopeMappingBuilder {
    pub fn new(
        client: WachtClient,
        deployment_id: &str,
        oauth_app_slug: &str,
        scope: &str,
        request: SetOAuthScopeMappingRequest,
    ) -> Self {
        Self {
            client,
            deployment_id: deployment_id.to_string(),
            oauth_app_slug: oauth_app_slug.to_string(),
            scope: urlencoding::encode(scope).to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<OAuthApp> {
        let client = self.client.http_client();
        let url = format!(
            "{}/deployments/{}/oauth/apps/{}/scopes/{}/mapping",
            self.client.config().base_url,
            self.deployment_id,
            self.oauth_app_slug,
            self.scope
        );

        let response = client.post(&url).json(&self.request).send().await?;
        if response.status().is_success() {
            let payload: Value = response.json().await?;
            let data = unwrap_data(payload);
            Ok(serde_json::from_value(data)?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(api_error(status, "Failed to set OAuth scope mapping", &error_text))
        }
    }
}

pub struct ListOAuthClientsBuilder {
    client: WachtClient,
    deployment_id: String,
    oauth_app_slug: String,
}

impl ListOAuthClientsBuilder {
    pub fn new(client: WachtClient, deployment_id: &str, oauth_app_slug: &str) -> Self {
        Self {
            client,
            deployment_id: deployment_id.to_string(),
            oauth_app_slug: oauth_app_slug.to_string(),
        }
    }

    pub async fn send(self) -> Result<Vec<OAuthClient>> {
        let client = self.client.http_client();
        let url = format!(
            "{}/deployments/{}/oauth/apps/{}/clients",
            self.client.config().base_url,
            self.deployment_id,
            self.oauth_app_slug
        );

        let response = client.get(&url).send().await?;
        if response.status().is_success() {
            let payload: Value = response.json().await?;
            let data = unwrap_data(payload);
            let parsed: ListOAuthClientsResponse = serde_json::from_value(data).unwrap_or_default();
            Ok(parsed.clients)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(api_error(status, "Failed to list OAuth clients", &error_text))
        }
    }
}

pub struct CreateOAuthClientBuilder {
    client: WachtClient,
    deployment_id: String,
    oauth_app_slug: String,
    request: CreateOAuthClientRequest,
}

impl CreateOAuthClientBuilder {
    pub fn new(
        client: WachtClient,
        deployment_id: &str,
        oauth_app_slug: &str,
        request: CreateOAuthClientRequest,
    ) -> Self {
        Self {
            client,
            deployment_id: deployment_id.to_string(),
            oauth_app_slug: oauth_app_slug.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<OAuthClient> {
        let client = self.client.http_client();
        let url = format!(
            "{}/deployments/{}/oauth/apps/{}/clients",
            self.client.config().base_url,
            self.deployment_id,
            self.oauth_app_slug
        );

        let response = client.post(&url).json(&self.request).send().await?;
        if response.status().is_success() {
            let payload: Value = response.json().await?;
            let data = unwrap_data(payload);
            Ok(serde_json::from_value(data)?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(api_error(status, "Failed to create OAuth client", &error_text))
        }
    }
}

pub struct UpdateOAuthClientBuilder {
    client: WachtClient,
    deployment_id: String,
    oauth_app_slug: String,
    oauth_client_id: String,
    request: UpdateOAuthClientRequest,
}

impl UpdateOAuthClientBuilder {
    pub fn new(
        client: WachtClient,
        deployment_id: &str,
        oauth_app_slug: &str,
        oauth_client_id: &str,
        request: UpdateOAuthClientRequest,
    ) -> Self {
        Self {
            client,
            deployment_id: deployment_id.to_string(),
            oauth_app_slug: oauth_app_slug.to_string(),
            oauth_client_id: oauth_client_id.to_string(),
            request,
        }
    }

    pub async fn send(self) -> Result<OAuthClient> {
        let client = self.client.http_client();
        let url = format!(
            "{}/deployments/{}/oauth/apps/{}/clients/{}",
            self.client.config().base_url,
            self.deployment_id,
            self.oauth_app_slug,
            self.oauth_client_id
        );

        let response = client.patch(&url).json(&self.request).send().await?;
        if response.status().is_success() {
            let payload: Value = response.json().await?;
            let data = unwrap_data(payload);
            Ok(serde_json::from_value(data)?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(api_error(status, "Failed to update OAuth client", &error_text))
        }
    }
}

pub struct DeactivateOAuthClientBuilder {
    client: WachtClient,
    deployment_id: String,
    oauth_app_slug: String,
    oauth_client_id: String,
}

impl DeactivateOAuthClientBuilder {
    pub fn new(
        client: WachtClient,
        deployment_id: &str,
        oauth_app_slug: &str,
        oauth_client_id: &str,
    ) -> Self {
        Self {
            client,
            deployment_id: deployment_id.to_string(),
            oauth_app_slug: oauth_app_slug.to_string(),
            oauth_client_id: oauth_client_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/deployments/{}/oauth/apps/{}/clients/{}",
            self.client.config().base_url,
            self.deployment_id,
            self.oauth_app_slug,
            self.oauth_client_id
        );

        let response = client.delete(&url).send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(api_error(status, "Failed to deactivate OAuth client", &error_text))
        }
    }
}

pub struct RotateOAuthClientSecretBuilder {
    client: WachtClient,
    deployment_id: String,
    oauth_app_slug: String,
    oauth_client_id: String,
}

impl RotateOAuthClientSecretBuilder {
    pub fn new(
        client: WachtClient,
        deployment_id: &str,
        oauth_app_slug: &str,
        oauth_client_id: &str,
    ) -> Self {
        Self {
            client,
            deployment_id: deployment_id.to_string(),
            oauth_app_slug: oauth_app_slug.to_string(),
            oauth_client_id: oauth_client_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<RotateOAuthClientSecretResponse> {
        let client = self.client.http_client();
        let url = format!(
            "{}/deployments/{}/oauth/apps/{}/clients/{}/rotate-secret",
            self.client.config().base_url,
            self.deployment_id,
            self.oauth_app_slug,
            self.oauth_client_id
        );

        let response = client.post(&url).send().await?;
        if response.status().is_success() {
            let payload: Value = response.json().await?;
            let data = unwrap_data(payload);
            Ok(serde_json::from_value(data)?)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(api_error(status, "Failed to rotate OAuth client secret", &error_text))
        }
    }
}

pub struct ListOAuthGrantsBuilder {
    client: WachtClient,
    deployment_id: String,
    oauth_app_slug: String,
    oauth_client_id: String,
}

impl ListOAuthGrantsBuilder {
    pub fn new(
        client: WachtClient,
        deployment_id: &str,
        oauth_app_slug: &str,
        oauth_client_id: &str,
    ) -> Self {
        Self {
            client,
            deployment_id: deployment_id.to_string(),
            oauth_app_slug: oauth_app_slug.to_string(),
            oauth_client_id: oauth_client_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<Vec<OAuthGrant>> {
        let client = self.client.http_client();
        let url = format!(
            "{}/deployments/{}/oauth/apps/{}/clients/{}/grants",
            self.client.config().base_url,
            self.deployment_id,
            self.oauth_app_slug,
            self.oauth_client_id
        );

        let response = client.get(&url).send().await?;
        if response.status().is_success() {
            let payload: Value = response.json().await?;
            let data = unwrap_data(payload);
            let parsed: ListOAuthGrantsResponse = serde_json::from_value(data).unwrap_or_default();
            Ok(parsed.grants)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(api_error(status, "Failed to list OAuth grants", &error_text))
        }
    }
}

pub struct RevokeOAuthGrantBuilder {
    client: WachtClient,
    deployment_id: String,
    oauth_app_slug: String,
    oauth_client_id: String,
    grant_id: String,
}

impl RevokeOAuthGrantBuilder {
    pub fn new(
        client: WachtClient,
        deployment_id: &str,
        oauth_app_slug: &str,
        oauth_client_id: &str,
        grant_id: &str,
    ) -> Self {
        Self {
            client,
            deployment_id: deployment_id.to_string(),
            oauth_app_slug: oauth_app_slug.to_string(),
            oauth_client_id: oauth_client_id.to_string(),
            grant_id: grant_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/deployments/{}/oauth/apps/{}/clients/{}/grants/{}/revoke",
            self.client.config().base_url,
            self.deployment_id,
            self.oauth_app_slug,
            self.oauth_client_id,
            self.grant_id
        );

        let response = client.post(&url).send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(api_error(status, "Failed to revoke OAuth grant", &error_text))
        }
    }
}
