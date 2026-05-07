use crate::{
    client::WachtClient,
    error::{Error, Result},
    models::{ActorMcpServerConnectResponse, ActorMcpServerSummary, PaginatedResponse},
};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct ActorMcpServersApi {
    client: WachtClient,
}
impl ActorMcpServersApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }
    pub fn list_actor_mcp_servers(
        &self,
        actor_id: impl Into<String>,
    ) -> ListActorMcpServersBuilder {
        ListActorMcpServersBuilder::new(self.client.clone(), actor_id)
    }
    pub fn connect_actor_mcp_server(
        &self,
        actor_id: impl Into<String>,
        mcp_server_id: impl Into<String>,
    ) -> ConnectActorMcpServerBuilder {
        ConnectActorMcpServerBuilder::new(self.client.clone(), actor_id, mcp_server_id)
    }
    pub fn disconnect_actor_mcp_server(
        &self,
        actor_id: impl Into<String>,
        mcp_server_id: impl Into<String>,
    ) -> DisconnectActorMcpServerBuilder {
        DisconnectActorMcpServerBuilder::new(self.client.clone(), actor_id, mcp_server_id)
    }
}

fn api_error(status: reqwest::StatusCode, prefix: &str, body: String) -> Error {
    Error::api_from_text(status, prefix, &body)
}

#[derive(Debug, Serialize)]
struct ActorQuery {
    actor_id: String,
}

pub struct ListActorMcpServersBuilder {
    client: WachtClient,
    actor_id: String,
}
impl ListActorMcpServersBuilder {
    pub fn new(client: WachtClient, actor_id: impl Into<String>) -> Self {
        Self {
            client,
            actor_id: actor_id.into(),
        }
    }
    pub async fn send(self) -> Result<PaginatedResponse<ActorMcpServerSummary>> {
        let response = self
            .client
            .http_client()
            .get(format!(
                "{}/ai/actor-mcp-servers",
                self.client.config().base_url
            ))
            .query(&ActorQuery {
                actor_id: self.actor_id,
            })
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to list actor MCP servers",
                response.text().await?,
            ))
        }
    }
}

pub struct ConnectActorMcpServerBuilder {
    client: WachtClient,
    actor_id: String,
    mcp_server_id: String,
}
impl ConnectActorMcpServerBuilder {
    pub fn new(
        client: WachtClient,
        actor_id: impl Into<String>,
        mcp_server_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            actor_id: actor_id.into(),
            mcp_server_id: mcp_server_id.into(),
        }
    }
    pub async fn send(self) -> Result<ActorMcpServerConnectResponse> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-mcp-servers/{}/connect",
                self.client.config().base_url,
                self.mcp_server_id
            ))
            .query(&ActorQuery {
                actor_id: self.actor_id,
            })
            .json(&serde_json::json!({}))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(response.json().await?)
        } else {
            Err(api_error(
                status,
                "Failed to connect actor MCP server",
                response.text().await?,
            ))
        }
    }
}

pub struct DisconnectActorMcpServerBuilder {
    client: WachtClient,
    actor_id: String,
    mcp_server_id: String,
}
impl DisconnectActorMcpServerBuilder {
    pub fn new(
        client: WachtClient,
        actor_id: impl Into<String>,
        mcp_server_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            actor_id: actor_id.into(),
            mcp_server_id: mcp_server_id.into(),
        }
    }
    pub async fn send(self) -> Result<()> {
        let response = self
            .client
            .http_client()
            .post(format!(
                "{}/ai/actor-mcp-servers/{}/disconnect",
                self.client.config().base_url,
                self.mcp_server_id
            ))
            .query(&ActorQuery {
                actor_id: self.actor_id,
            })
            .json(&serde_json::json!({}))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(api_error(
                status,
                "Failed to disconnect actor MCP server",
                response.text().await?,
            ))
        }
    }
}
