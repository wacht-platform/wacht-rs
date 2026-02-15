use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{CreateNotificationRequest, Notification},
};

/// Builder for creating a notification
pub struct CreateBuilder {
    request: CreateNotificationRequest,
}

impl CreateBuilder {
    pub fn new(request: CreateNotificationRequest) -> Self {
        Self { request }
    }

    pub async fn send(self) -> Result<Notification> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/notifications", config.base_url);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to create notification: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Create a notification for a specific user using builder pattern
pub fn create(request: CreateNotificationRequest) -> CreateBuilder {
    CreateBuilder::new(request)
}
