use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaVersion {
    V1,
    V2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecondFactorPolicy {
    None,
    Optional,
    Enforced,
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
    pub connection_type: String,
    pub provider_user_id: String,
    pub provider_email: Option<String>,
    pub provider_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Segment {
    pub id: String,
    pub created_at: String,
    pub updated_at: String,
    pub name: String,
    pub description: Option<String>,
    pub rules: Vec<serde_json::Value>,
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
