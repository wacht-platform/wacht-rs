//! Deployment Credentials Module
//!
//! Mints a fresh set of deployment credentials (publishable key, hosts, and
//! a one-shot API key). Typically used by bootstrap scripts and the `wacht`
//! CLI; not something an end-user backend should call per request.

use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::DeploymentCredentialsResponse,
};

#[derive(Debug, Clone)]
pub struct CredentialsApi {
    client: WachtClient,
}

impl CredentialsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn create(&self) -> CreateDeploymentCredentialsBuilder {
        CreateDeploymentCredentialsBuilder::new(self.client.clone())
    }
}

pub struct CreateDeploymentCredentialsBuilder {
    client: WachtClient,
}

impl CreateDeploymentCredentialsBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub async fn send(self) -> Result<DeploymentCredentialsResponse> {
        let client = self.client.http_client();
        let url = format!("{}/credentials", self.client.config().base_url);
        let response = client.post(&url).send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to mint deployment credentials",
                &error_body,
            ))
        }
    }
}
