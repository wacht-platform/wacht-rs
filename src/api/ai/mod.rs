//! AI-related API modules
//!
//! This module contains all AI functionality including agents, tools, knowledge bases, and execution contexts.

pub mod agents;
pub mod execution_context;
pub mod knowledge_bases;
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

    pub fn execution_contexts(&self) -> execution_context::ExecutionContextsApi {
        execution_context::ExecutionContextsApi::new(self.client.clone())
    }
}

pub use agents::{
    CreateAgentBuilder, DeleteAgentBuilder, FetchAgentBuilder, FetchAgentDetailsBuilder,
    ListAgentsBuilder, ListAgentsOptions, UpdateAgentBuilder,
};
pub use execution_context::{
    CreateExecutionContextBuilder, ExecuteAgentBuilder, ExecutionContextListResponse,
    ListExecutionContextsBuilder, ListExecutionContextsOptions, UpdateExecutionContextBuilder,
    UpdateExecutionContextRequest,
};
pub use knowledge_bases::{
    ListKnowledgeBasesOptions, builders, CreateKnowledgeBaseBuilder, DeleteDocumentBuilder,
    DeleteKnowledgeBaseBuilder, FetchDocumentsBuilder, FetchKnowledgeBaseBuilder,
    FetchKnowledgeBasesBuilder, UpdateKnowledgeBaseBuilder, UploadDocumentBuilder,
};
pub use tools::{
    CreateToolBuilder, DeleteToolBuilder, FetchToolBuilder, ListToolsBuilder, ListToolsOptions,
    UpdateToolBuilder,
};
