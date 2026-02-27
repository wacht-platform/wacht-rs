//! User Social Connections Module
//!
//! Handles social connection management for users using builder pattern.

use crate::{
    client::WachtClient,
    error::{Error, Result},
};

#[derive(Debug, Clone)]
pub struct UserSocialConnectionsApi {
    client: WachtClient,
}

impl UserSocialConnectionsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn delete_social_connection(
        &self,
        user_id: &str,
        connection_id: &str,
    ) -> DeleteSocialConnectionBuilder {
        DeleteSocialConnectionBuilder::new(self.client.clone(), user_id, connection_id)
    }
}

/// Builder for deleting a user's social connection
pub struct DeleteSocialConnectionBuilder {
    client: WachtClient,
    user_id: String,
    connection_id: String,
}

impl DeleteSocialConnectionBuilder {
    pub fn new(client: WachtClient, user_id: &str, connection_id: &str) -> Self {
        Self {
            client,
            user_id: user_id.to_string(),
            connection_id: connection_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!(
            "{}/users/{}/social-connections/{}",
            self.client.config().base_url,
            self.user_id,
            self.connection_id
        );

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete social connection: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}
