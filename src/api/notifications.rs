use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{CreateNotificationRequest, Notification},
};

#[derive(Debug, Clone)]
pub struct NotificationsApi {
    client: WachtClient,
}

impl NotificationsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn create(&self, request: CreateNotificationRequest) -> CreateBuilder {
        CreateBuilder::new(self.client.clone(), request)
    }
}

/// Builder for creating a notification
pub struct CreateBuilder {
    client: WachtClient,
    request: CreateNotificationRequest,
}

impl CreateBuilder {
    pub fn new(client: WachtClient, request: CreateNotificationRequest) -> Self {
        Self { client, request }
    }

    pub async fn send(self) -> Result<Notification> {
        let client = self.client.http_client();
        let url = format!("{}/notifications", self.client.config().base_url);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to create notification: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}
