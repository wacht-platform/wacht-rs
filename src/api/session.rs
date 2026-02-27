//! Session Management Module
//!
//! Handles session tickets for authentication using builder pattern.

use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{CreateSessionTicketRequest, SessionTicketResponse},
};

#[derive(Debug, Clone)]
pub struct SessionApi {
    client: WachtClient,
}

impl SessionApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn create(&self, request: CreateSessionTicketRequest) -> CreateTicketBuilder {
        CreateTicketBuilder::new(self.client.clone(), request)
    }
}

/// Builder for creating a session ticket
pub struct CreateTicketBuilder {
    client: WachtClient,
    request: CreateSessionTicketRequest,
}

impl CreateTicketBuilder {
    pub fn new(client: WachtClient, request: CreateSessionTicketRequest) -> Self {
        Self { client, request }
    }

    pub async fn send(self) -> Result<SessionTicketResponse> {
        let client = self.client.http_client();
        let url = format!("{}/session/tickets", self.client.config().base_url);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to create session ticket: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Re-export ticket type for convenience
pub use crate::models::TicketType;
