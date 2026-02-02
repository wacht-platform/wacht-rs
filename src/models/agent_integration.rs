use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IntegrationType {
    Teams,
    ClickUp,
    WhatsApp,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TeamsConfig {
    Bot {
        #[serde(skip_serializing_if = "Option::is_none")]
        app_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        webhook_url: Option<String>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClickUpConfig {
    ApiKey {
        api_key: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WhatsAppConfig {
    Business {
        phone_number_id: String,
        access_token: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IntegrationConfig {
    Teams(TeamsConfig),
    ClickUp(ClickUpConfig),
    WhatsApp(WhatsAppConfig),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentIntegration {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub deployment_id: String,
    pub agent_id: String,
    pub integration_type: IntegrationType,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<IntegrationConfig>,
}
