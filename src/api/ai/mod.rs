//! AI-related API modules
//!
//! This module contains all AI functionality including agents, tools, knowledge bases, and execution contexts.

pub mod agents;
pub mod execution_context;
pub mod knowledge_bases;
pub mod tools;

// Re-export all public items for convenience
pub use agents::{
    list_agents, fetch_agent, create_agent, update_agent, delete_agent,
    fetch_agent_details, ListAgentsOptions, ListAgentsBuilder, CreateAgentBuilder,
    FetchAgentBuilder, UpdateAgentBuilder, DeleteAgentBuilder, FetchAgentDetailsBuilder,
};
pub use execution_context::{
    fetch_execution_contexts, create_execution_context, update_execution_context,
    execute_agent, ExecutionContextListResponse, ListExecutionContextsOptions,
    UpdateExecutionContextRequest,
};
pub use knowledge_bases::{
    fetch_knowledge_bases, fetch_knowledge_base, create_knowledge_base,
    update_knowledge_base, delete_knowledge_base, fetch_documents,
    upload_document, delete_document, ListKnowledgeBasesOptions,
};
pub use tools::{
    list_tools, fetch_tool, create_tool, update_tool, delete_tool,
    ListToolsOptions, ListToolsBuilder, CreateToolBuilder,
    FetchToolBuilder, UpdateToolBuilder, DeleteToolBuilder,
};
