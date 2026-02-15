use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSegmentRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub segment_type: String,
}

impl CreateSegmentRequest {
    pub fn new(name: String, segment_type: String) -> CreateSegmentRequest {
        CreateSegmentRequest { name, segment_type }
    }
}
