use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{CreateNotificationRequest, Notification},
};

/// Create a notification for a specific user
pub async fn create_notification(request: CreateNotificationRequest) -> Result<Notification> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/notifications", config.base_url);

    let response = client
        .post(&url)
        .json(&request)
        .send()
        .await?;
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
