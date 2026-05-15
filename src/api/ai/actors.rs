use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{Actor, CreateActorRequest, LookupActorParams, LookupActorResponse},
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

    pub fn lookup_actor(&self, params: LookupActorParams) -> LookupActorBuilder {
        LookupActorBuilder::new(self.client.clone(), params)
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

#[derive(Debug, Clone)]
pub struct LookupActorBuilder {
    client: WachtClient,
    params: LookupActorParams,
}

impl LookupActorBuilder {
    pub fn new(client: WachtClient, params: LookupActorParams) -> Self {
        Self { client, params }
    }

    pub async fn send(self) -> Result<LookupActorResponse> {
        let client = self.client.http_client();
        let url = format!("{}/ai/actors/lookup", self.client.config().base_url);

        let response = client
            .get(&url)
            .query(&[
                ("subject_type", self.params.subject_type.as_str()),
                ("external_key", self.params.external_key.as_str()),
            ])
            .send()
            .await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to lookup actor",
                &error_body,
            ))
        }
    }
}
