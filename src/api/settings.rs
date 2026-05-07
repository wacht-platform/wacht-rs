use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{
        AuthenticationSettings, CreateJwtTemplateRequest, DeploymentB2bSettingsUpdates,
        DeploymentOrganizationRole, DeploymentRestrictionsUpdates, DeploymentWorkspaceRole,
        DisplaySettings, EmailTemplate, ImageUploadResponse, JwtTemplate, PaginatedResponse,
        SmtpConfigRequest, SmtpConfigResponse, SmtpVerifyResponse, SocialConnection,
        UpdateJwtTemplateRequest,
    },
};
use serde::{Deserialize, Serialize};

pub type JwtTemplateListResponse = PaginatedResponse<JwtTemplate>;
pub type SocialConnectionsResponse = PaginatedResponse<SocialConnection>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentSettingsResponse {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub maintenance_mode: bool,
    pub backend_host: String,
    pub frontend_host: String,
    pub mail_from_host: String,
    pub publishable_key: String,
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_settings: Option<AuthenticationSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_settings: Option<DisplaySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b2b_settings: Option<DeploymentB2bSettingsUpdates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<DeploymentRestrictionsUpdates>,
}

#[derive(Debug, Clone)]
struct SettingsContext {
    base_url: String,
    default_client: reqwest::Client,
}

impl SettingsContext {
    fn from_client(client: &WachtClient) -> Self {
        Self {
            base_url: client.config().base_url.clone(),
            default_client: client.http_client(),
        }
    }

    fn resolve_client(&self, override_client: Option<reqwest::Client>) -> reqwest::Client {
        override_client.unwrap_or_else(|| self.default_client.clone())
    }
}

#[derive(Debug, Clone)]
pub struct SettingsApi {
    ctx: SettingsContext,
}

impl SettingsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self {
            ctx: SettingsContext::from_client(&client),
        }
    }

    pub fn fetch_deployment_settings(&self) -> FetchDeploymentSettingsBuilder {
        FetchDeploymentSettingsBuilder::new(self.ctx.clone())
    }

    pub fn update_authentication_settings(
        &self,
        settings: AuthenticationSettings,
    ) -> UpdateAuthenticationSettingsBuilder {
        UpdateAuthenticationSettingsBuilder::new(self.ctx.clone(), settings)
    }

    pub fn update_display_settings(
        &self,
        settings: DisplaySettings,
    ) -> UpdateDisplaySettingsBuilder {
        UpdateDisplaySettingsBuilder::new(self.ctx.clone(), settings)
    }

    pub fn update_b2b_settings(
        &self,
        settings: DeploymentB2bSettingsUpdates,
    ) -> UpdateB2BSettingsBuilder {
        UpdateB2BSettingsBuilder::new(self.ctx.clone(), settings)
    }

    pub fn fetch_deployment_organization_roles(&self) -> FetchDeploymentOrganizationRolesBuilder {
        FetchDeploymentOrganizationRolesBuilder::new(self.ctx.clone())
    }

    pub fn fetch_deployment_workspace_roles(&self) -> FetchDeploymentWorkspaceRolesBuilder {
        FetchDeploymentWorkspaceRolesBuilder::new(self.ctx.clone())
    }

    pub fn update_deployment_restrictions(
        &self,
        restrictions: DeploymentRestrictionsUpdates,
    ) -> UpdateDeploymentRestrictionsBuilder {
        UpdateDeploymentRestrictionsBuilder::new(self.ctx.clone(), restrictions)
    }

    pub fn fetch_jwt_templates(&self) -> FetchJwtTemplatesBuilder {
        FetchJwtTemplatesBuilder::new(self.ctx.clone())
    }

    pub fn create_jwt_template(
        &self,
        request: CreateJwtTemplateRequest,
    ) -> CreateJwtTemplateBuilder {
        CreateJwtTemplateBuilder::new(self.ctx.clone(), request)
    }

    pub fn update_jwt_template(
        &self,
        template_id: &str,
        request: UpdateJwtTemplateRequest,
    ) -> UpdateJwtTemplateBuilder {
        UpdateJwtTemplateBuilder::new(self.ctx.clone(), template_id, request)
    }

    pub fn delete_jwt_template(&self, template_id: &str) -> DeleteJwtTemplateBuilder {
        DeleteJwtTemplateBuilder::new(self.ctx.clone(), template_id)
    }

    pub fn update_smtp_config(&self, config_data: SmtpConfigRequest) -> UpdateSmtpConfigBuilder {
        UpdateSmtpConfigBuilder::new(self.ctx.clone(), config_data)
    }

    pub fn remove_smtp_config(&self) -> RemoveSmtpConfigBuilder {
        RemoveSmtpConfigBuilder::new(self.ctx.clone())
    }

    pub fn verify_smtp_connection(
        &self,
        config_data: SmtpConfigRequest,
    ) -> VerifySmtpConnectionBuilder {
        VerifySmtpConnectionBuilder::new(self.ctx.clone(), config_data)
    }

    pub fn fetch_email_template(&self, template_name: &str) -> FetchEmailTemplateBuilder {
        FetchEmailTemplateBuilder::new(self.ctx.clone(), template_name)
    }

    pub fn update_email_template(
        &self,
        template_name: &str,
        template: EmailTemplate,
    ) -> UpdateEmailTemplateBuilder {
        UpdateEmailTemplateBuilder::new(self.ctx.clone(), template_name, template)
    }

    pub fn fetch_social_connections(&self) -> FetchSocialConnectionsBuilder {
        FetchSocialConnectionsBuilder::new(self.ctx.clone())
    }

    pub fn upsert_social_connection(
        &self,
        connection: SocialConnection,
    ) -> UpsertSocialConnectionBuilder {
        UpsertSocialConnectionBuilder::new(self.ctx.clone(), connection)
    }

    pub fn upload_image(
        &self,
        image_type: &str,
        file_content: Vec<u8>,
        file_name: String,
    ) -> UploadImageBuilder {
        UploadImageBuilder::new(self.ctx.clone(), image_type, file_content, file_name)
    }
}

pub struct FetchDeploymentSettingsBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
}

impl FetchDeploymentSettingsBuilder {
    fn new(ctx: SettingsContext) -> Self {
        Self { ctx, client: None }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<DeploymentSettingsResponse> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!("{}/", self.ctx.base_url);
        let response = client.get(&url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to fetch deployment settings",
                &error_body,
            ))
        }
    }
}

pub struct UpdateAuthenticationSettingsBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
    settings: AuthenticationSettings,
}

impl UpdateAuthenticationSettingsBuilder {
    fn new(ctx: SettingsContext, settings: AuthenticationSettings) -> Self {
        Self {
            ctx,
            client: None,
            settings,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<()> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!("{}/settings/auth", self.ctx.base_url);
        let response = client.patch(&url).json(&self.settings).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to update authentication settings",
                &error_body,
            ))
        }
    }
}

pub struct UpdateDisplaySettingsBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
    settings: DisplaySettings,
}

impl UpdateDisplaySettingsBuilder {
    fn new(ctx: SettingsContext, settings: DisplaySettings) -> Self {
        Self {
            ctx,
            client: None,
            settings,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<()> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!("{}/settings/display", self.ctx.base_url);
        let response = client.patch(&url).json(&self.settings).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to update display settings",
                &error_body,
            ))
        }
    }
}

pub struct UpdateB2BSettingsBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
    settings: DeploymentB2bSettingsUpdates,
}

impl UpdateB2BSettingsBuilder {
    fn new(ctx: SettingsContext, settings: DeploymentB2bSettingsUpdates) -> Self {
        Self {
            ctx,
            client: None,
            settings,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<()> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!("{}/settings/b2b", self.ctx.base_url);
        let response = client.patch(&url).json(&self.settings).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to update B2B settings",
                &error_body,
            ))
        }
    }
}

pub struct FetchDeploymentOrganizationRolesBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
}

impl FetchDeploymentOrganizationRolesBuilder {
    fn new(ctx: SettingsContext) -> Self {
        Self { ctx, client: None }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<PaginatedResponse<DeploymentOrganizationRole>> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!("{}/settings/b2b/organization-roles", self.ctx.base_url);
        let response = client.get(&url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to fetch deployment organization roles",
                &error_body,
            ))
        }
    }
}

pub struct FetchDeploymentWorkspaceRolesBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
}

impl FetchDeploymentWorkspaceRolesBuilder {
    fn new(ctx: SettingsContext) -> Self {
        Self { ctx, client: None }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<PaginatedResponse<DeploymentWorkspaceRole>> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!("{}/settings/b2b/workspace-roles", self.ctx.base_url);
        let response = client.get(&url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to fetch deployment workspace roles",
                &error_body,
            ))
        }
    }
}

pub struct UpdateDeploymentRestrictionsBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
    restrictions: DeploymentRestrictionsUpdates,
}

impl UpdateDeploymentRestrictionsBuilder {
    fn new(ctx: SettingsContext, restrictions: DeploymentRestrictionsUpdates) -> Self {
        Self {
            ctx,
            client: None,
            restrictions,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<()> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!("{}/settings/restrictions", self.ctx.base_url);
        let response = client.patch(&url).json(&self.restrictions).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to update deployment restrictions",
                &error_body,
            ))
        }
    }
}

pub struct FetchJwtTemplatesBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
}

impl FetchJwtTemplatesBuilder {
    fn new(ctx: SettingsContext) -> Self {
        Self { ctx, client: None }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<JwtTemplateListResponse> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!("{}/jwt-templates", self.ctx.base_url);
        let response = client.get(&url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to fetch JWT templates",
                &error_body,
            ))
        }
    }
}

pub struct CreateJwtTemplateBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
    request: CreateJwtTemplateRequest,
}

impl CreateJwtTemplateBuilder {
    fn new(ctx: SettingsContext, request: CreateJwtTemplateRequest) -> Self {
        Self {
            ctx,
            client: None,
            request,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<JwtTemplate> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!("{}/jwt-templates", self.ctx.base_url);
        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to create JWT template",
                &error_body,
            ))
        }
    }
}

pub struct UpdateJwtTemplateBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
    template_id: String,
    request: UpdateJwtTemplateRequest,
}

impl UpdateJwtTemplateBuilder {
    fn new(ctx: SettingsContext, template_id: &str, request: UpdateJwtTemplateRequest) -> Self {
        Self {
            ctx,
            client: None,
            template_id: template_id.to_string(),
            request,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<JwtTemplate> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!("{}/jwt-templates/{}", self.ctx.base_url, self.template_id);
        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                format!("Failed to update JWT template {}", self.template_id),
                &error_body,
            ))
        }
    }
}

pub struct DeleteJwtTemplateBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
    template_id: String,
}

impl DeleteJwtTemplateBuilder {
    fn new(ctx: SettingsContext, template_id: &str) -> Self {
        Self {
            ctx,
            client: None,
            template_id: template_id.to_string(),
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<()> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!("{}/jwt-templates/{}", self.ctx.base_url, self.template_id);
        let response = client.delete(&url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                format!("Failed to delete JWT template {}", self.template_id),
                &error_body,
            ))
        }
    }
}

pub struct UpdateSmtpConfigBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
    config_data: SmtpConfigRequest,
}

impl UpdateSmtpConfigBuilder {
    fn new(ctx: SettingsContext, config_data: SmtpConfigRequest) -> Self {
        Self {
            ctx,
            client: None,
            config_data,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<SmtpConfigResponse> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!("{}/settings/email/smtp", self.ctx.base_url);
        let response = client.post(&url).json(&self.config_data).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to update SMTP configuration",
                &error_body,
            ))
        }
    }
}

pub struct RemoveSmtpConfigBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
}

impl RemoveSmtpConfigBuilder {
    fn new(ctx: SettingsContext) -> Self {
        Self { ctx, client: None }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<()> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!("{}/settings/email/smtp", self.ctx.base_url);
        let response = client.delete(&url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to remove SMTP configuration",
                &error_body,
            ))
        }
    }
}

pub struct VerifySmtpConnectionBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
    config_data: SmtpConfigRequest,
}

impl VerifySmtpConnectionBuilder {
    fn new(ctx: SettingsContext, config_data: SmtpConfigRequest) -> Self {
        Self {
            ctx,
            client: None,
            config_data,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<SmtpVerifyResponse> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!("{}/settings/email/smtp/verify", self.ctx.base_url);
        let response = client.post(&url).json(&self.config_data).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to verify SMTP connection",
                &error_body,
            ))
        }
    }
}

pub struct FetchEmailTemplateBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
    template_name: String,
}

impl FetchEmailTemplateBuilder {
    fn new(ctx: SettingsContext, template_name: &str) -> Self {
        Self {
            ctx,
            client: None,
            template_name: template_name.to_string(),
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<EmailTemplate> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!(
            "{}/settings/email-templates/{}",
            self.ctx.base_url, self.template_name
        );
        let response = client.get(&url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                format!("Failed to fetch email template {}", self.template_name),
                &error_body,
            ))
        }
    }
}

pub struct UpdateEmailTemplateBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
    template_name: String,
    template: EmailTemplate,
}

impl UpdateEmailTemplateBuilder {
    fn new(ctx: SettingsContext, template_name: &str, template: EmailTemplate) -> Self {
        Self {
            ctx,
            client: None,
            template_name: template_name.to_string(),
            template,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<()> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!(
            "{}/settings/email-templates/{}",
            self.ctx.base_url, self.template_name
        );
        let response = client.patch(&url).json(&self.template).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                format!("Failed to update email template {}", self.template_name),
                &error_body,
            ))
        }
    }
}

pub struct FetchSocialConnectionsBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
}

impl FetchSocialConnectionsBuilder {
    fn new(ctx: SettingsContext) -> Self {
        Self { ctx, client: None }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<SocialConnectionsResponse> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!("{}/settings/social-connections", self.ctx.base_url);
        let response = client.get(&url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to fetch social connections",
                &error_body,
            ))
        }
    }
}

pub struct UpsertSocialConnectionBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
    connection: SocialConnection,
}

impl UpsertSocialConnectionBuilder {
    fn new(ctx: SettingsContext, connection: SocialConnection) -> Self {
        Self {
            ctx,
            client: None,
            connection,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<SocialConnection> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!("{}/settings/social-connections", self.ctx.base_url);
        let response = client.put(&url).json(&self.connection).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to upsert social connection",
                &error_body,
            ))
        }
    }
}

pub struct UploadImageBuilder {
    ctx: SettingsContext,
    client: Option<reqwest::Client>,
    image_type: String,
    file_content: Vec<u8>,
    file_name: String,
}

impl UploadImageBuilder {
    fn new(
        ctx: SettingsContext,
        image_type: &str,
        file_content: Vec<u8>,
        file_name: String,
    ) -> Self {
        Self {
            ctx,
            client: None,
            image_type: image_type.to_string(),
            file_content,
            file_name,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<ImageUploadResponse> {
        let client = self.ctx.resolve_client(self.client);
        let url = format!("{}/settings/upload/{}", self.ctx.base_url, self.image_type);

        let mime_type = match self.file_name.split('.').last() {
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("png") => "image/png",
            Some("gif") => "image/gif",
            Some("webp") => "image/webp",
            Some("ico") => "image/x-icon",
            _ => "image/png",
        };

        let part = reqwest::multipart::Part::bytes(self.file_content)
            .file_name(self.file_name)
            .mime_str(mime_type)?;

        let form = reqwest::multipart::Form::new().part("file", part);

        let response = client.post(&url).multipart(form).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                format!("Failed to upload {} image", self.image_type),
                &error_body,
            ))
        }
    }
}
