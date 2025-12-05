
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSegmentRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl UpdateSegmentRequest {
    pub fn new() -> UpdateSegmentRequest {
        UpdateSegmentRequest {
            name: None,
            r#type: None,
        }
    }
}
