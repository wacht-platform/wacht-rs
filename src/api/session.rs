//! Session Management Module
//!
//! Handles session tickets for authentication using builder pattern.

use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{CreateSessionTicketRequest, SessionTicketResponse, TicketType},
};

/// Builder for creating a session ticket
pub struct CreateTicketBuilder {
    request: CreateSessionTicketRequest,
}

impl CreateTicketBuilder {
    pub fn new(request: CreateSessionTicketRequest) -> Self {
        Self { request }
    }

    pub async fn send(self) -> Result<SessionTicketResponse> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/session/tickets", config.base_url);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to create session ticket: {}", error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Create a session ticket for a user using builder pattern
pub fn create(request: CreateSessionTicketRequest) -> CreateTicketBuilder {
    CreateTicketBuilder::new(request)
}

/// Re-export ticket type for convenience
pub use crate::models::TicketType;
