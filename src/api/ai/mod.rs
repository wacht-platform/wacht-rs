//! AI-related API modules.

pub mod actor_mcp_servers;
pub mod actor_project_threads;
pub mod actor_projects;
pub mod actors;
pub mod agents;
pub mod composio;
pub mod knowledge_bases;
pub mod mcp_servers;
pub mod tools;

use crate::client::WachtClient;

#[derive(Debug, Clone)]
pub struct AiApi {
    client: WachtClient,
}

impl AiApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn agents(&self) -> agents::AgentsApi {
        agents::AgentsApi::new(self.client.clone())
    }

    pub fn tools(&self) -> tools::ToolsApi {
        tools::ToolsApi::new(self.client.clone())
    }

    pub fn knowledge_bases(&self) -> knowledge_bases::KnowledgeBasesApi {
        knowledge_bases::KnowledgeBasesApi::new(self.client.clone())
    }

    pub fn mcp_servers(&self) -> mcp_servers::McpServersApi {
        mcp_servers::McpServersApi::new(self.client.clone())
    }

    pub fn actor_projects(&self) -> actor_projects::ActorProjectsApi {
        actor_projects::ActorProjectsApi::new(self.client.clone())
    }

    pub fn actor_project_threads(&self) -> actor_project_threads::ActorProjectThreadsApi {
        actor_project_threads::ActorProjectThreadsApi::new(self.client.clone())
    }

    pub fn actor_mcp_servers(&self) -> actor_mcp_servers::ActorMcpServersApi {
        actor_mcp_servers::ActorMcpServersApi::new(self.client.clone())
    }

    pub fn actors(&self) -> actors::ActorsApi {
        actors::ActorsApi::new(self.client.clone())
    }

    pub fn composio(&self) -> composio::ComposioApi {
        composio::ComposioApi::new(self.client.clone())
    }
}

pub use actor_mcp_servers::{
    ConnectActorMcpServerBuilder, DisconnectActorMcpServerBuilder, ListActorMcpServersBuilder,
};
pub use actor_project_threads::{
    ArchiveActorProjectThreadBuilder, FetchActorProjectThreadAssignmentsBuilder,
    FetchActorProjectThreadBuilder, FetchActorProjectThreadFilesystemBuilder,
    FetchActorProjectThreadFilesystemFileBuilder, FetchActorProjectThreadMessagesBuilder,
    FetchActorProjectThreadTaskGraphsBuilder, RunActorProjectThreadBuilder,
    SearchActorProjectThreadsBuilder, UnarchiveActorProjectThreadBuilder,
    UpdateActorProjectThreadBuilder,
};
pub use actor_projects::{
    AnswerActorProjectBoardItemQuestionBuilder, ApproveActorProjectBoardItemToolBuilder,
    ArchiveActorProjectBoardItemBuilder, ArchiveActorProjectBuilder,
    CancelActorProjectBoardItemBuilder, CreateActorProjectBoardItemBuilder,
    CreateActorProjectBoardItemCommentBuilder, CreateActorProjectBuilder,
    CreateActorProjectThreadBuilder, FetchActorProjectBoardBuilder,
    FetchActorProjectBoardItemAssignmentsBuilder, FetchActorProjectBoardItemBuilder,
    FetchActorProjectBoardItemCommentsBuilder, FetchActorProjectBoardItemFilesystemBuilder,
    FetchActorProjectBoardItemFilesystemFileBuilder, FetchActorProjectBuilder,
    FetchActorProjectThreadsBuilder, ListActorProjectsBuilder, SearchActorProjectsBuilder,
    UnarchiveActorProjectBoardItemBuilder, UnarchiveActorProjectBuilder,
    UpdateActorProjectBoardItemBuilder, UpdateActorProjectBuilder,
};
pub use actors::CreateActorBuilder;
pub use agents::{
    AttachSubAgentBuilder, CreateAgentBuilder, DeleteAgentBuilder, DeleteAgentSkillBuilder,
    DetachSubAgentBuilder, FetchAgentBuilder, FetchAgentDetailsBuilder,
    ImportAgentSkillBundleBuilder, ListAgentSkillTreeBuilder, ListAgentSubAgentsBuilder,
    ListAgentsBuilder, ListAgentsOptions, ReadAgentSkillFileBuilder, UpdateAgentBuilder,
};
pub use composio::{
    DisableComposioAppBuilder, EnableComposioAppBuilder, FetchComposioConfigBuilder,
    FetchComposioToolkitAuthDetailsBuilder, ListComposioToolkitAuthConfigsBuilder,
    ListComposioToolkitsBuilder, UpdateComposioConfigBuilder,
};
pub use knowledge_bases::{
    AttachKnowledgeBaseBuilder, CreateKnowledgeBaseBuilder, DeleteDocumentBuilder,
    DeleteKnowledgeBaseBuilder, DetachKnowledgeBaseBuilder, FetchAgentKnowledgeBasesBuilder,
    FetchDocumentsBuilder, FetchKnowledgeBaseBuilder, FetchKnowledgeBasesBuilder,
    ListKnowledgeBasesOptions, UpdateKnowledgeBaseBuilder, UploadDocumentBuilder,
};
pub use mcp_servers::{
    CreateMcpServerBuilder, DeleteMcpServerBuilder, DiscoverMcpServerBuilder,
    FetchMcpServerBuilder, FetchMcpServersBuilder, ListMcpServersOptions, UpdateMcpServerBuilder,
};
pub use tools::{
    AttachToolBuilder, CreateToolBuilder, DeleteToolBuilder, DetachToolBuilder,
    FetchAgentToolsBuilder, FetchToolBuilder, ListToolsBuilder, ListToolsOptions,
    UpdateToolBuilder,
};
