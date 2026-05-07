use serde::{Deserialize, Serialize};

/// Generic paginated response type
/// Matches the backend PaginatedResponse<T> structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub offset: i64,
    pub limit: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}
