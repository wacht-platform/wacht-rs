use serde::{Deserialize, Serialize};

/// Ticket type for session tickets
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TicketType {
    /// Allow an admin to impersonate another user
    Impersonation,
    /// Grant access to specific AI agents
    AgentAccess,
    /// Grant access to a webhook app
    WebhookAppAccess,
    /// Grant access to an API auth app
    ApiAuthAccess,
}

/// Agent session identifier mode for agent access tickets
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentSessionIdentifier {
    Static,
    Signin,
}

/// Request to create a session ticket
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSessionTicketRequest {
    /// Type of ticket to create
    pub ticket_type: TicketType,
    /// User ID to impersonate (required for impersonation tickets)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// List of agent IDs to grant access to (required for agent_access tickets)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
    /// Agent session identifier mode (optional, agent_access only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_session_identifier: Option<AgentSessionIdentifier>,
    /// Context group for grouping agent sessions (required for agent_access tickets)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_group: Option<String>,
    /// Slug of the webhook app to access (required for webhook_app_access tickets)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_app_slug: Option<String>,
    /// Slug of the API auth app to access (required for api_auth_access tickets)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_auth_app_slug: Option<String>,
    /// Unix timestamp when the ticket expires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
}

impl CreateSessionTicketRequest {
    /// Create a new impersonation ticket request
    pub fn impersonation(user_id: impl Into<String>) -> Self {
        Self {
            ticket_type: TicketType::Impersonation,
            user_id: Some(user_id.into()),
            agent_ids: None,
            agent_session_identifier: None,
            context_group: None,
            webhook_app_slug: None,
            api_auth_app_slug: None,
            expires_at: None,
        }
    }

    /// Create a new agent access ticket request
    pub fn agent_access(agent_ids: Vec<String>, context_group: impl Into<String>) -> Self {
        Self {
            ticket_type: TicketType::AgentAccess,
            user_id: None,
            agent_ids: Some(agent_ids),
            agent_session_identifier: Some(AgentSessionIdentifier::Static),
            context_group: Some(context_group.into()),
            webhook_app_slug: None,
            api_auth_app_slug: None,
            expires_at: None,
        }
    }

    /// Create a new webhook app access ticket request
    pub fn webhook_app_access(webhook_app_slug: impl Into<String>) -> Self {
        Self {
            ticket_type: TicketType::WebhookAppAccess,
            user_id: None,
            agent_ids: None,
            agent_session_identifier: None,
            context_group: None,
            webhook_app_slug: Some(webhook_app_slug.into()),
            api_auth_app_slug: None,
            expires_at: None,
        }
    }

    /// Create a new API auth app access ticket request
    pub fn api_auth_access(api_auth_app_slug: impl Into<String>) -> Self {
        Self {
            ticket_type: TicketType::ApiAuthAccess,
            user_id: None,
            agent_ids: None,
            agent_session_identifier: None,
            context_group: None,
            webhook_app_slug: None,
            api_auth_app_slug: Some(api_auth_app_slug.into()),
            expires_at: None,
        }
    }

    /// Create a new agent access ticket request using signin-scoped mode
    pub fn agent_access_signin(agent_ids: Vec<String>) -> Self {
        Self {
            ticket_type: TicketType::AgentAccess,
            user_id: None,
            agent_ids: Some(agent_ids),
            agent_session_identifier: Some(AgentSessionIdentifier::Signin),
            context_group: None,
            webhook_app_slug: None,
            api_auth_app_slug: None,
            expires_at: None,
        }
    }

    /// Set the expiration timestamp
    pub fn expires_at(mut self, expires_at: i64) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    /// Set agent session identifier mode for agent_access tickets
    pub fn agent_session_identifier(mut self, mode: AgentSessionIdentifier) -> Self {
        self.agent_session_identifier = Some(mode);
        self
    }
}

/// Response containing the session ticket
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionTicketResponse {
    /// The ticket string to be exchanged via GET /session/ticket/exchange
    pub ticket: String,
    /// Unix timestamp when the ticket expires
    pub expires_at: i64,
}
