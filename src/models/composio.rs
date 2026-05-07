use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComposioEnabledApp {
    pub slug: String,
    pub auth_config_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_scheme: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComposioConfigResponse {
    pub enabled: bool,
    pub use_platform_key: bool,
    pub api_key_set: bool,
    pub enabled_apps: Vec<ComposioEnabledApp>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateComposioConfigRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_platform_key: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_apps: Option<Vec<ComposioEnabledApp>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComposioToolkit {
    pub slug: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    pub categories: Vec<String>,
    pub auth_schemes: Vec<String>,
    pub tool_count: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComposioToolkitListResponse {
    pub toolkits: Vec<ComposioToolkit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComposioAuthConfigSummary {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_scheme: Option<String>,
    pub is_composio_managed: bool,
    pub toolkit_slug: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComposioAuthConfigListResponse {
    pub auth_configs: Vec<ComposioAuthConfigSummary>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ComposioEnableAppAuth {
    Managed {
        #[serde(skip_serializing_if = "Option::is_none")]
        auth_scheme: Option<String>,
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        credentials: serde_json::Map<String, Value>,
    },
    Custom {
        auth_scheme: String,
        #[serde(default)]
        credentials: serde_json::Map<String, Value>,
    },
    UseExisting {
        auth_config_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        auth_scheme: Option<String>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnableComposioAppRequest {
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    pub auth: ComposioEnableAppAuth,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComposioToolkitAuthField {
    pub name: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub description: String,
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComposioToolkitAuthFields {
    pub required: Vec<ComposioToolkitAuthField>,
    pub optional: Vec<ComposioToolkitAuthField>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComposioToolkitAuthMode {
    pub mode: String,
    pub name: String,
    pub auth_config_creation: ComposioToolkitAuthFields,
    pub connected_account_initiation: ComposioToolkitAuthFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_hint_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComposioToolkitDetailsResponse {
    pub slug: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    pub composio_managed_auth_schemes: Vec<String>,
    pub auth_modes: Vec<ComposioToolkitAuthMode>,
}
