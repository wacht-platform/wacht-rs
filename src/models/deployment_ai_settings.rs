use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentStorageProvider {
    S3,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentLlmProvider {
    Gemini,
    Openai,
    Openrouter,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentEmbeddingProvider {
    Gemini,
    Openai,
    Openrouter,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentStorageSettingsResponse {
    pub provider: DeploymentStorageProvider,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub strong_llm_provider: DeploymentLlmProvider,
    pub weak_llm_provider: DeploymentLlmProvider,
    pub gemini_api_key_set: bool,
    pub openrouter_api_key_set: bool,
    pub openrouter_require_parameters: bool,
    pub openai_api_key_set: bool,
    pub anthropic_api_key_set: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strong_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_model: Option<String>,
    pub embedding_provider: DeploymentEmbeddingProvider,
    pub embedding_model: String,
    pub embedding_dimension: i32,
    pub storage: DeploymentStorageSettingsResponse,
}

/// A named, deployment-scoped LLM provider profile (keys masked).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentAiProviderProfileResponse {
    pub id: String,
    pub deployment_id: String,
    pub provider: DeploymentLlmProvider,
    pub name: String,
    pub slug: String,
    pub api_key_set: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_model: Option<String>,
    pub enabled: bool,
    pub disable_prompt_caching: bool,
    pub disable_reasoning_effort: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateDeploymentAiProviderProfileRequest {
    pub provider: DeploymentLlmProvider,
    pub name: String,
    pub slug: String,
    pub api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_prompt_caching: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_reasoning_effort: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UpdateDeploymentAiProviderProfileRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_prompt_caching: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_reasoning_effort: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UpdateDeploymentAiSettingsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strong_llm_provider: Option<DeploymentLlmProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_llm_provider: Option<DeploymentLlmProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gemini_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openrouter_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openrouter_require_parameters: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openai_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anthropic_api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strong_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weak_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_provider: Option<DeploymentEmbeddingProvider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_dimension: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<UpdateDeploymentStorageSettingsRequest>,
}
