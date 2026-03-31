use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentStorageProvider {
    S3,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentStorageSettings {
    pub provider: DeploymentStorageProvider,
    pub bucket: Option<String>,
    pub region: Option<String>,
    pub endpoint: Option<String>,
    pub root_prefix: Option<String>,
    pub force_path_style: bool,
    pub access_key_id_set: bool,
    pub secret_access_key_set: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UpdateDeploymentStorageSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<DeploymentStorageProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_path_style: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentAiSettings {
    pub gemini_api_key_set: bool,
    pub openai_api_key_set: bool,
    pub anthropic_api_key_set: bool,
    pub storage: DeploymentStorageSettings,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UpdateDeploymentAiSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gemini_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openai_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anthropic_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<UpdateDeploymentStorageSettingsRequest>,
}
