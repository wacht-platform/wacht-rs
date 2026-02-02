use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{
        AuthenticationSettings, B2BSettings, DisplaySettings,
        DeploymentRestrictions, JwtTemplate, CreateJwtTemplateRequest,
        UpdateJwtTemplateRequest, EmailTemplate, SocialConnection,
        ImageUploadResponse, SmtpConfigRequest, SmtpConfigResponse, SmtpVerifyResponse
    },
};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtTemplateListResponse {
    pub data: Vec<JwtTemplate>,
}

/// Fetch deployment settings
pub async fn fetch_deployment_settings() -> Result<DeploymentSettingsResponse> {
    let config = get_config();
    let client = get_client();
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

/// Update authentication settings
pub async fn update_authentication_settings(settings: AuthenticationSettings) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/settings/auth", config.base_url);
    
    let response = client.patch(&url).json(&settings).send().await?;
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

/// Update display settings
pub async fn update_display_settings(settings: DisplaySettings) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/settings/display", config.base_url);
    
    let response = client.patch(&url).json(&settings).send().await?;
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

/// Update B2B settings
pub async fn update_b2b_settings(settings: B2BSettings) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/settings/b2b", config.base_url);
    
    let response = client.patch(&url).json(&settings).send().await?;
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

/// Update deployment restrictions
pub async fn update_deployment_restrictions(restrictions: DeploymentRestrictions) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/settings/restrictions", config.base_url);
    
    let response = client.patch(&url).json(&restrictions).send().await?;
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

/// Fetch JWT templates
pub async fn fetch_jwt_templates() -> Result<JwtTemplateListResponse> {
    let config = get_config();
    let client = get_client();
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

/// Create JWT template
pub async fn create_jwt_template(request: CreateJwtTemplateRequest) -> Result<JwtTemplate> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/jwt-templates", config.base_url);
    
    let response = client.post(&url).json(&request).send().await?;
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

/// Update JWT template
pub async fn update_jwt_template(template_id: &str, request: UpdateJwtTemplateRequest) -> Result<JwtTemplate> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/jwt-templates/{}", config.base_url, template_id);
    
    let response = client.patch(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to update JWT template {template_id}: {error_body}"),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Delete JWT template
pub async fn delete_jwt_template(template_id: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/jwt-templates/{}", config.base_url, template_id);
    
    let response = client.delete(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to delete JWT template {template_id}: {error_body}"),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Update SMTP configuration
pub async fn update_smtp_config(config_data: SmtpConfigRequest) -> Result<SmtpConfigResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/settings/email/smtp", config.base_url);

    let response = client.post(&url).json(&config_data).send().await?;
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

/// Remove SMTP configuration
pub async fn remove_smtp_config() -> Result<()> {
    let config = get_config();
    let client = get_client();
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

/// Verify SMTP connection
pub async fn verify_smtp_connection() -> Result<SmtpVerifyResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/settings/email/smtp/verify", config.base_url);

    let response = client.post(&url).send().await?;
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

/// Fetch email template
pub async fn fetch_email_template(template_name: &str) -> Result<EmailTemplate> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/settings/email-templates/{}", config.base_url, template_name);

    let response = client.get(&url).send().await?;
    let status = response.status();

    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to fetch email template {template_name}: {error_body}"),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Update email template
pub async fn update_email_template(template_name: &str, template: EmailTemplate) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/settings/email-templates/{}", config.base_url, template_name);
    
    let response = client.patch(&url).json(&template).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to update email template {template_name}: {error_body}"),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Fetch social connections
pub async fn fetch_social_connections() -> Result<Vec<SocialConnection>> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/social-connections", config.base_url);
    
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

/// Upsert social connection
pub async fn upsert_social_connection(connection: SocialConnection) -> Result<SocialConnection> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/social-connections", config.base_url);
    
    let response = client.put(&url).json(&connection).send().await?;
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

/// Upload image
pub async fn upload_image(image_type: &str, file_content: Vec<u8>, file_name: String) -> Result<ImageUploadResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/upload/{}", config.base_url, image_type);
    
    let part = reqwest::multipart::Part::bytes(file_content)
        .file_name(file_name);
    
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
            message: format!("Failed to upload {image_type} image: {error_body}"),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}