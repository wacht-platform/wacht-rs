use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{
        AuthenticationSettings, B2BSettings, DisplaySettings,
        DeploymentRestrictions, JwtTemplate, CreateJwtTemplateRequest,
        UpdateJwtTemplateRequest, EmailTemplate, SocialConnection,
        ImageUploadResponse, SmtpConfigRequest, SmtpConfigResponse, SmtpVerifyResponse,
        PaginatedResponse,
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
    pub b2b_settings: Option<B2BSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<DeploymentRestrictions>,
}

/// Builder for fetching deployment settings
pub struct FetchDeploymentSettingsBuilder {
    client: Option<reqwest::Client>,
}

impl Default for FetchDeploymentSettingsBuilder {
    fn default() -> Self {
        Self { client: None }
    }
}

impl FetchDeploymentSettingsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<DeploymentSettingsResponse> {
        let config = get_config();
        let client = if let Some(c) = self.client {
            c
        } else {
            get_client()
        };
        let url = format!("{}/", config.base_url);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch deployment settings: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for updating authentication settings
pub struct UpdateAuthenticationSettingsBuilder {
    client: Option<reqwest::Client>,
    settings: AuthenticationSettings,
}

impl UpdateAuthenticationSettingsBuilder {
    pub fn new(settings: AuthenticationSettings) -> Self {
        Self {
            client: None,
            settings,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = if let Some(c) = self.client {
            c
        } else {
            get_client()
        };
        let url = format!("{}/settings/auth", config.base_url);

        let response = client.patch(&url).json(&self.settings).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update authentication settings: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for updating display settings
pub struct UpdateDisplaySettingsBuilder {
    client: Option<reqwest::Client>,
    settings: DisplaySettings,
}

impl UpdateDisplaySettingsBuilder {
    pub fn new(settings: DisplaySettings) -> Self {
        Self {
            client: None,
            settings,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = if let Some(c) = self.client {
            c
        } else {
            get_client()
        };
        let url = format!("{}/settings/display", config.base_url);

        let response = client.patch(&url).json(&self.settings).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update display settings: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for updating B2B settings
pub struct UpdateB2BSettingsBuilder {
    client: Option<reqwest::Client>,
    settings: B2BSettings,
}

impl UpdateB2BSettingsBuilder {
    pub fn new(settings: B2BSettings) -> Self {
        Self {
            client: None,
            settings,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = if let Some(c) = self.client {
            c
        } else {
            get_client()
        };
        let url = format!("{}/settings/b2b", config.base_url);

        let response = client.patch(&url).json(&self.settings).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update B2B settings: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for updating deployment restrictions
pub struct UpdateDeploymentRestrictionsBuilder {
    client: Option<reqwest::Client>,
    restrictions: DeploymentRestrictions,
}

impl UpdateDeploymentRestrictionsBuilder {
    pub fn new(restrictions: DeploymentRestrictions) -> Self {
        Self {
            client: None,
            restrictions,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = if let Some(c) = self.client {
            c
        } else {
            get_client()
        };
        let url = format!("{}/settings/restrictions", config.base_url);

        let response = client.patch(&url).json(&self.restrictions).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update deployment restrictions: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for fetching JWT templates
pub struct FetchJwtTemplatesBuilder {
    client: Option<reqwest::Client>,
}

impl Default for FetchJwtTemplatesBuilder {
    fn default() -> Self {
        Self { client: None }
    }
}

impl FetchJwtTemplatesBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<JwtTemplateListResponse> {
        let config = get_config();
        let client = if let Some(c) = self.client {
            c
        } else {
            get_client()
        };
        let url = format!("{}/jwt-templates", config.base_url);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch JWT templates: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for creating JWT template
pub struct CreateJwtTemplateBuilder {
    client: Option<reqwest::Client>,
    request: CreateJwtTemplateRequest,
}

impl CreateJwtTemplateBuilder {
    pub fn new(request: CreateJwtTemplateRequest) -> Self {
        Self {
            client: None,
            request,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<JwtTemplate> {
        let config = get_config();
        let client = if let Some(c) = self.client {
            c
        } else {
            get_client()
        };
        let url = format!("{}/jwt-templates", config.base_url);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to create JWT template: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for updating JWT template
pub struct UpdateJwtTemplateBuilder {
    client: Option<reqwest::Client>,
    template_id: String,
    request: UpdateJwtTemplateRequest,
}

impl UpdateJwtTemplateBuilder {
    pub fn new(template_id: &str, request: UpdateJwtTemplateRequest) -> Self {
        Self {
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
        let config = get_config();
        let client = if let Some(c) = self.client {
            c
        } else {
            get_client()
        };
        let url = format!("{}/jwt-templates/{}", config.base_url, self.template_id);

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update JWT template {}: {error_body}", self.template_id),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for deleting JWT template
pub struct DeleteJwtTemplateBuilder {
    client: Option<reqwest::Client>,
    template_id: String,
}

impl DeleteJwtTemplateBuilder {
    pub fn new(template_id: &str) -> Self {
        Self {
            client: None,
            template_id: template_id.to_string(),
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = if let Some(c) = self.client {
            c
        } else {
            get_client()
        };
        let url = format!("{}/jwt-templates/{}", config.base_url, self.template_id);

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete JWT template {}: {error_body}", self.template_id),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for updating SMTP configuration
pub struct UpdateSmtpConfigBuilder {
    client: Option<reqwest::Client>,
    config_data: SmtpConfigRequest,
}

impl UpdateSmtpConfigBuilder {
    pub fn new(config_data: SmtpConfigRequest) -> Self {
        Self {
            client: None,
            config_data,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<SmtpConfigResponse> {
        let config = get_config();
        let client = if let Some(c) = self.client {
            c
        } else {
            get_client()
        };
        let url = format!("{}/settings/email/smtp", config.base_url);

        let response = client.post(&url).json(&self.config_data).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update SMTP configuration: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for removing SMTP configuration
pub struct RemoveSmtpConfigBuilder {
    client: Option<reqwest::Client>,
}

impl Default for RemoveSmtpConfigBuilder {
    fn default() -> Self {
        Self { client: None }
    }
}

impl RemoveSmtpConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = if let Some(c) = self.client {
            c
        } else {
            get_client()
        };
        let url = format!("{}/settings/email/smtp", config.base_url);

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to remove SMTP configuration: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for verifying SMTP connection
pub struct VerifySmtpConnectionBuilder {
    client: Option<reqwest::Client>,
    config_data: SmtpConfigRequest,
}

impl VerifySmtpConnectionBuilder {
    pub fn new(config_data: SmtpConfigRequest) -> Self {
        Self {
            client: None,
            config_data,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<SmtpVerifyResponse> {
        let config = get_config();
        let client = if let Some(c) = self.client {
            c
        } else {
            get_client()
        };
        let url = format!("{}/settings/email/smtp/verify", config.base_url);

        let response = client.post(&url).json(&self.config_data).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to verify SMTP connection: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for fetching email template
pub struct FetchEmailTemplateBuilder {
    client: Option<reqwest::Client>,
    template_name: String,
}

impl FetchEmailTemplateBuilder {
    pub fn new(template_name: &str) -> Self {
        Self {
            client: None,
            template_name: template_name.to_string(),
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<EmailTemplate> {
        let config = get_config();
        let client = if let Some(c) = self.client {
            c
        } else {
            get_client()
        };
        let url = format!("{}/settings/email-templates/{}", config.base_url, self.template_name);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch email template {}: {error_body}", self.template_name),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for updating email template
pub struct UpdateEmailTemplateBuilder {
    client: Option<reqwest::Client>,
    template_name: String,
    template: EmailTemplate,
}

impl UpdateEmailTemplateBuilder {
    pub fn new(template_name: &str, template: EmailTemplate) -> Self {
        Self {
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
        let config = get_config();
        let client = if let Some(c) = self.client {
            c
        } else {
            get_client()
        };
        let url = format!("{}/settings/email-templates/{}", config.base_url, self.template_name);

        let response = client.patch(&url).json(&self.template).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update email template {}: {error_body}", self.template_name),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for fetching social connections
pub struct FetchSocialConnectionsBuilder {
    client: Option<reqwest::Client>,
}

impl Default for FetchSocialConnectionsBuilder {
    fn default() -> Self {
        Self { client: None }
    }
}

impl FetchSocialConnectionsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<SocialConnectionsResponse> {
        let config = get_config();
        let client = if let Some(c) = self.client {
            c
        } else {
            get_client()
        };
        let url = format!("{}/settings/social-connections", config.base_url);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to fetch social connections: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for upsert social connection
pub struct UpsertSocialConnectionBuilder {
    client: Option<reqwest::Client>,
    connection: SocialConnection,
}

impl UpsertSocialConnectionBuilder {
    pub fn new(connection: SocialConnection) -> Self {
        Self {
            client: None,
            connection,
        }
    }

    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub async fn send(self) -> Result<SocialConnection> {
        let config = get_config();
        let client = if let Some(c) = self.client {
            c
        } else {
            get_client()
        };
        let url = format!("{}/settings/social-connections", config.base_url);

        let response = client.put(&url).json(&self.connection).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to upsert social connection: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Builder for uploading image
pub struct UploadImageBuilder {
    client: Option<reqwest::Client>,
    image_type: String,
    file_content: Vec<u8>,
    file_name: String,
}

impl UploadImageBuilder {
    pub fn new(image_type: &str, file_content: Vec<u8>, file_name: String) -> Self {
        Self {
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
        let config = get_config();
        let client = if let Some(c) = self.client {
            c
        } else {
            get_client()
        };
        let url = format!("{}/settings/upload/{}", config.base_url, self.image_type);

        // Determine mime type from file extension
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

        let form = reqwest::multipart::Form::new()
            .part("file", part);

        let response = client.post(&url)
            .multipart(form)
            .send()
            .await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to upload {} image: {error_body}", self.image_type),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

// Convenience functions to create builders
pub fn fetch_deployment_settings() -> FetchDeploymentSettingsBuilder {
    FetchDeploymentSettingsBuilder::new()
}

pub fn update_authentication_settings(settings: AuthenticationSettings) -> UpdateAuthenticationSettingsBuilder {
    UpdateAuthenticationSettingsBuilder::new(settings)
}

pub fn update_display_settings(settings: DisplaySettings) -> UpdateDisplaySettingsBuilder {
    UpdateDisplaySettingsBuilder::new(settings)
}

pub fn update_b2b_settings(settings: B2BSettings) -> UpdateB2BSettingsBuilder {
    UpdateB2BSettingsBuilder::new(settings)
}

pub fn update_deployment_restrictions(restrictions: DeploymentRestrictions) -> UpdateDeploymentRestrictionsBuilder {
    UpdateDeploymentRestrictionsBuilder::new(restrictions)
}

pub fn fetch_jwt_templates() -> FetchJwtTemplatesBuilder {
    FetchJwtTemplatesBuilder::new()
}

pub fn create_jwt_template(request: CreateJwtTemplateRequest) -> CreateJwtTemplateBuilder {
    CreateJwtTemplateBuilder::new(request)
}

pub fn update_jwt_template(template_id: &str, request: UpdateJwtTemplateRequest) -> UpdateJwtTemplateBuilder {
    UpdateJwtTemplateBuilder::new(template_id, request)
}

pub fn delete_jwt_template(template_id: &str) -> DeleteJwtTemplateBuilder {
    DeleteJwtTemplateBuilder::new(template_id)
}

pub fn update_smtp_config(config_data: SmtpConfigRequest) -> UpdateSmtpConfigBuilder {
    UpdateSmtpConfigBuilder::new(config_data)
}

pub fn remove_smtp_config() -> RemoveSmtpConfigBuilder {
    RemoveSmtpConfigBuilder::new()
}

pub fn verify_smtp_connection(config_data: SmtpConfigRequest) -> VerifySmtpConnectionBuilder {
    VerifySmtpConnectionBuilder::new(config_data)
}

pub fn fetch_email_template(template_name: &str) -> FetchEmailTemplateBuilder {
    FetchEmailTemplateBuilder::new(template_name)
}

pub fn update_email_template(template_name: &str, template: EmailTemplate) -> UpdateEmailTemplateBuilder {
    UpdateEmailTemplateBuilder::new(template_name, template)
}

pub fn fetch_social_connections() -> FetchSocialConnectionsBuilder {
    FetchSocialConnectionsBuilder::new()
}

pub fn upsert_social_connection(connection: SocialConnection) -> UpsertSocialConnectionBuilder {
    UpsertSocialConnectionBuilder::new(connection)
}

pub fn upload_image(image_type: &str, file_content: Vec<u8>, file_name: String) -> UploadImageBuilder {
    UploadImageBuilder::new(image_type, file_content, file_name)
}