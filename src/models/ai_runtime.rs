use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CursorPage<T> {
    pub data: Vec<T>,
    pub limit: i64,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateActorRequest {
    pub subject_type: String,
    pub external_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Actor {
    pub id: String,
    pub deployment_id: String,
    pub subject_type: String,
    pub external_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActorProject {
    pub id: String,
    pub deployment_id: String,
    pub actor_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinator_thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_thread_id: Option<String>,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateActorProjectRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateActorProjectRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentThread {
    pub id: String,
    pub deployment_id: String,
    pub actor_id: String,
    pub project_id: String,
    pub title: String,
    pub thread_kind: String,
    pub thread_visibility: String,
    pub thread_purpose: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsibility: Option<String>,
    pub reusable: bool,
    pub accepts_assignments: bool,
    pub capability_tags: Vec<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instructions: Option<String>,
    pub last_activity_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_state: Option<Value>,
    pub next_event_sequence: i64,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAgentThreadRequest {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_purpose: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reusable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepts_assignments: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateAgentThreadRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instructions: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectTaskBoard {
    pub id: String,
    pub deployment_id: String,
    pub actor_id: String,
    pub project_id: String,
    pub title: String,
    pub status: String,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectTaskBoardItem {
    pub id: String,
    pub board_id: String,
    pub task_key: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_thread_id: Option<String>,
    pub metadata: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub state_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_for: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fired_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_question: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_approval: Option<Value>,
    pub mounts: Value,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleMount {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateProjectTaskBoardItemRequest {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_run_at: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<ScheduleMount>>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateProjectTaskBoardItemRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_run_at: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_schedule: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<ScheduleMount>>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectTaskBoardItemComment {
    pub id: String,
    pub deployment_id: String,
    pub board_item_id: String,
    pub actor_id: String,
    pub body: String,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_by_thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_summary: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateProjectTaskBoardItemCommentRequest {
    pub body: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AnswerValue {
    FreeText { value: String },
    SingleChoice { value: String },
    MultiChoice { values: Vec<String> },
    YesNo { value: bool },
    Number { value: f64 },
    Date { value: String },
    Confirm { accepted: bool },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuestionAnswer {
    pub question_id: String,
    pub value: AnswerValue,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnswerSubmission {
    pub answers: Vec<QuestionAnswer>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolApprovalMode {
    AllowOnce,
    AllowAlways,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApprovalSubmissionItem {
    pub tool_name: String,
    pub mode: ToolApprovalMode,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApprovalSubmission {
    pub request_message_id: String,
    pub approvals: Vec<ApprovalSubmissionItem>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppendProjectTaskBoardItemJournalRequest {
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_markdown: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Value>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectTaskBoardAssignmentTarget {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsibility: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capability_tags: Vec<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectTaskBoardAssignmentMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_target: Option<ProjectTaskBoardAssignmentTarget>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectTaskBoardItemEvent {
    pub id: String,
    pub board_item_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_run_id: Option<String>,
    pub event_type: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_markdown: Option<String>,
    pub details: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectTaskBoardItemAssignment {
    pub id: String,
    pub board_item_id: String,
    pub thread_id: String,
    pub assignment_role: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handoff_file_path: Option<String>,
    pub metadata: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_payload: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreadEvent {
    pub id: String,
    pub deployment_id: String,
    pub thread_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub board_item_id: Option<String>,
    pub event_type: String,
    pub status: String,
    pub priority: i32,
    pub payload: Value,
    pub available_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caused_by_conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caused_by_run_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caused_by_thread_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConversationRecord {
    pub id: String,
    pub thread_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_run_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub content: Value,
    pub message_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreadMessagesResponse {
    pub data: Vec<ConversationRecord>,
    pub has_more: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreadTaskGraph {
    pub id: String,
    pub deployment_id: String,
    pub thread_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub board_item_id: Option<String>,
    pub version: i32,
    pub status: String,
    pub metadata: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreadTaskNode {
    pub id: String,
    pub graph_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub board_item_id: Option<String>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub status: String,
    pub priority: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_thread_id: Option<String>,
    pub retry_count: i32,
    pub max_retries: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_until: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreadTaskEdge {
    pub graph_id: String,
    pub from_node_id: String,
    pub to_node_id: String,
    pub dependency_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreadTaskGraphSummary {
    pub graph_id: String,
    pub graph_status: String,
    pub total_nodes: i64,
    pub pending_nodes: i64,
    pub ready_nodes: i64,
    pub in_progress_nodes: i64,
    pub completed_nodes: i64,
    pub failed_nodes: i64,
    pub cancelled_nodes: i64,
    pub progress_percent: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskWorkspaceFileEntry {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskWorkspaceListing {
    pub exists: bool,
    pub files: Vec<TaskWorkspaceFileEntry>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskWorkspaceFileContent {
    pub path: String,
    pub name: String,
    pub mime_type: String,
    pub is_text: bool,
    pub size_bytes: u64,
    pub truncated: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub content: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub content_base64: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActorMcpServerSummary {
    pub id: String,
    pub name: String,
    pub endpoint: String,
    pub auth_type: String,
    pub requires_user_connection: bool,
    pub connection_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActorMcpServerConnectResponse {
    pub auth_url: String,
}
