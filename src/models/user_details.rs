use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaVersion {
    V1,
    V2,
}

/// Second-factor policy. Wire values are `"none"`, `"optional"`, `"enforced"`
/// (matches `platform-api::models::SecondFactorPolicy`). Earlier versions of
/// this SDK shipped `disabled / optional / required`, which caused user-detail
/// responses to fail to decode entirely.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SecondFactorPolicy {
    None,
    Optional,
    Enforced,
}

impl std::fmt::Display for SecondFactorPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SecondFactorPolicy::None => "none",
            SecondFactorPolicy::Optional => "optional",
            SecondFactorPolicy::Enforced => "enforced",
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStrategy {
    Otp,
    OauthGoogle,
    OauthGithub,
    OauthMicrosoft,
    OauthFacebook,
    OauthLinkedin,
    OauthDiscord,
    OauthApple,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEmailAddress {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub deployment_id: String,
    pub user_id: String,
    pub email: String,
    pub is_primary: bool,
    pub verified: bool,
    pub verified_at: String,
    pub verification_strategy: VerificationStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPhoneNumber {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub user_id: String,
    pub phone_number: String,
    pub country_code: String,
    pub verified: bool,
    pub verified_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialConnection {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub user_id: String,
    pub user_email_address_id: String,
    pub provider: String,
    pub email_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Segment {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    pub deployment_id: String,
    pub name: String,
    pub r#type: String, // "organization", "workspace", or "user"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDetails {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub first_name: String,
    pub last_name: String,
    pub username: Option<String>,
    pub profile_picture_url: String,
    pub schema_version: SchemaVersion,
    pub disabled: bool,
    pub second_factor_policy: SecondFactorPolicy,
    pub availability: String,
    pub last_password_reset_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_organization_membership_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_workspace_membership_id: Option<String>,
    pub deployment_id: String,
    pub public_metadata: serde_json::Value,
    pub private_metadata: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_email_address_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_phone_number_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_phone_number: Option<String>,
    pub email_addresses: Vec<UserEmailAddress>,
    pub phone_numbers: Vec<UserPhoneNumber>,
    pub social_connections: Vec<SocialConnection>,
    pub segments: Vec<Segment>,
    pub has_password: bool,
    pub has_backup_codes: bool,
}
