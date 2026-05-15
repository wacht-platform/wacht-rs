use serde::{Deserialize, Serialize};

/// Request to delegate a task from one thread to a target lane thread within
/// the same project.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DelegateProjectTaskRequest {
    /// Lane thread that will own the new board item.
    pub target_lane_thread_id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Routing tags. Empty / omitted matches any agent on the target lane.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability_tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegateProjectTaskResponse {
    /// Stable task key (e.g. `T-7`).
    pub task_key: String,
    pub board_item_id: String,
    pub target_lane_thread_id: String,
    /// Agent assigned to execute the delegated task.
    pub assigned_agent_id: String,
}
