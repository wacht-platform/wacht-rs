use crate::{
    client::{get_client, get_config},
    error::Result,
    models::{GenerateTokenRequest, GenerateTokenResponse},
};

/// Generate JWT Token
/// 
/// Generate a JWT token for a given session with optional custom claims using handlebars templates. 
/// The token includes standard claims (iss, sub, iat, exp, session_id) plus organization and 
/// workspace permissions if applicable. Custom claims can be added using JWT templates with 
/// handlebars syntax that has access to user, session, organization, and workspace data.
pub async fn generate_token(
    deployment_id: i64,
    generate_token_request: GenerateTokenRequest,
) -> Result<GenerateTokenResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/deployments/{}/token", config.base_url, deployment_id);
    
    let response = client
        .post(&url)
        .json(&generate_token_request)
        .send()
        .await?;
    
    if response.status().is_success() {
        let token_response: GenerateTokenResponse = response.json().await?;
        Ok(token_response)
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        Err(crate::error::Error::Api {
            status,
            message: error_text,
            details: None,
        })
    }
}