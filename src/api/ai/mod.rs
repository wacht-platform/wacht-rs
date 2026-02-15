//! AI-related API modules
//!
//! This module contains all AI functionality including agents, tools, knowledge bases, and execution contexts.

pub mod agents;
pub mod execution_context;
pub mod knowledge_bases;
pub mod tools;

// Re-export all public items for convenience
pub use agents::{
    CreateAgentBuilder, DeleteAgentBuilder, FetchAgentBuilder, FetchAgentDetailsBuilder,
    ListAgentsBuilder, ListAgentsOptions, UpdateAgentBuilder, create_agent, delete_agent,
    fetch_agent, fetch_agent_details, list_agents, update_agent,
};
pub use execution_context::{
    ExecutionContextListResponse, ListExecutionContextsOptions, UpdateExecutionContextRequest,
    create_execution_context, execute_agent, fetch_execution_contexts, update_execution_context,
};
pub use knowledge_bases::{
    ListKnowledgeBasesOptions, create_knowledge_base, delete_document, delete_knowledge_base,
    fetch_documents, fetch_knowledge_base, fetch_knowledge_bases, update_knowledge_base,
    upload_document,
};
pub use tools::{
    CreateToolBuilder, DeleteToolBuilder, FetchToolBuilder, ListToolsBuilder, ListToolsOptions,
    UpdateToolBuilder, create_tool, delete_tool, fetch_tool, list_tools, update_tool,
};
