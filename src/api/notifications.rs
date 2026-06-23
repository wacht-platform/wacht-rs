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

#[derive(serde::Deserialize)]
struct CreateNotificationsResponse {
    #[serde(default)]
    data: Vec<Notification>,
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

    pub async fn send(self) -> Result<Vec<Notification>> {
        let client = self.client.http_client();
        let url = format!("{}/notifications", self.client.config().base_url);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            let body: CreateNotificationsResponse = response.json().await?;
            Ok(body.data)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to create notification",
                &error_body,
            ))
        }
    }
}
