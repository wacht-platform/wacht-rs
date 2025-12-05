
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSegmentRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CreateSegmentRequest {
    pub fn new(name: String, r#type: String) -> CreateSegmentRequest {
        CreateSegmentRequest {
            name,
            r#type,
        }
    }
}
