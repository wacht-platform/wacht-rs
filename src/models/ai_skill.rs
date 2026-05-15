use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillScope {
    System,
    Agent,
}

impl SkillScope {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Agent => "agent",
        }
    }
}

/// One row of the agent-skills summary. `mount_path` is the path the skill is
/// surfaced at inside the agent's filesystem; `source` is `"system"` or
/// `"agent"`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkillSummaryEntry {
    pub slug: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub mount_path: String,
    pub source: String,
}

/// Combined system + agent skills available to an agent. System skills are
/// built into Wacht; agent skills are uploaded per-agent via the import
/// bundle endpoint.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentSkillsSummary {
    pub system: Vec<SkillSummaryEntry>,
    pub agent: Vec<SkillSummaryEntry>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkillTreeEntry {
    pub name: String,
    pub path: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkillTreeResponse {
    pub scope: SkillScope,
    pub path: String,
    pub entries: Vec<SkillTreeEntry>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkillFileResponse {
    pub scope: SkillScope,
    pub path: String,
    pub is_text: bool,
    pub size_bytes: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_base64: Option<String>,
}
