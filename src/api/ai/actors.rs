use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{
        Actor, CreateActorRequest, LookupActorParams, LookupActorResponse, PaginatedResponse,
    },
};
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListActorsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_archived: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ActorsApi {
    client: WachtClient,
}

impl ActorsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    /// Paginated list of actors for the deployment.
    pub fn list_actors(&self) -> ListActorsBuilder {
        ListActorsBuilder::new(self.client.clone())
    }

    pub fn create_actor(&self, request: CreateActorRequest) -> CreateActorBuilder {
        CreateActorBuilder::new(self.client.clone(), request)
    }

    pub fn lookup_actor(&self, params: LookupActorParams) -> LookupActorBuilder {
        LookupActorBuilder::new(self.client.clone(), params)
    }
}

#[derive(Debug, Clone)]
pub struct ListActorsBuilder {
    client: WachtClient,
    options: ListActorsOptions,
}

impl ListActorsBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
            options: ListActorsOptions::default(),
        }
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.options.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: i64) -> Self {
        self.options.offset = Some(offset);
        self
    }

    pub fn search(mut self, search: impl Into<String>) -> Self {
        self.options.search = Some(search.into());
        self
    }

    pub fn include_archived(mut self, include_archived: bool) -> Self {
        self.options.include_archived = Some(include_archived);
        self
    }

    pub async fn send(self) -> Result<PaginatedResponse<Actor>> {
        let client = self.client.http_client();
        let url = format!("{}/ai/actors", self.client.config().base_url);

        let response = client.get(&url).query(&self.options).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::api_from_text(
                status,
                "Failed to list actors",
                &error_body,
            ))
        }
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
