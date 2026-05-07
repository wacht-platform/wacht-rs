use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{Actor, CreateActorRequest},
};

#[derive(Debug, Clone)]
pub struct ActorsApi {
    client: WachtClient,
}

impl ActorsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn create_actor(&self, request: CreateActorRequest) -> CreateActorBuilder {
        CreateActorBuilder::new(self.client.clone(), request)
    }
}

#[derive(Debug, Clone)]
pub struct CreateActorBuilder {
    client: WachtClient,
    request: CreateActorRequest,
}

impl CreateActorBuilder {
    pub fn new(client: WachtClient, request: CreateActorRequest) -> Self {
        Self { client, request }
    }

    pub async fn send(self) -> Result<Actor> {
        let client = self.client.http_client();
        let url = format!("{}/ai/actors", self.client.config().base_url);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to create actor",
                &error_body,
            ))
        }
    }
}
