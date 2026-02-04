//! User Social Connections Module
//!
//! Handles social connection management for users using builder pattern.

use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
};

/// Builder for deleting a user's social connection
pub struct DeleteSocialConnectionBuilder {
    user_id: String,
    connection_id: String,
}

impl DeleteSocialConnectionBuilder {
    pub fn new(user_id: &str, connection_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            connection_id: connection_id.to_string(),
        }
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!(
            "{}/users/{}/social-connections/{}",
            config.base_url, self.user_id, self.connection_id
        );

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete social connection: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Delete a user's social connection using builder pattern
pub fn delete_social_connection(user_id: &str, connection_id: &str) -> DeleteSocialConnectionBuilder {
    DeleteSocialConnectionBuilder::new(user_id, connection_id)
}
