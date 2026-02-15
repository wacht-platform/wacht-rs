use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{AiTool, CreateAiToolRequest, PaginatedResponse, UpdateAiToolRequest},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListToolsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

/// List all AI tools (async function)
pub async fn list_tools(options: Option<ListToolsOptions>) -> Result<PaginatedResponse<AiTool>> {
    ListToolsBuilder::new()
        .with_options(options.unwrap_or_default())
        .send()
        .await
}

/// Builder for listing AI tools
pub struct ListToolsBuilder {
    options: Option<ListToolsOptions>,
}

impl ListToolsBuilder {
    pub fn new() -> Self {
        Self { options: None }
    }

    pub fn with_options(mut self, options: ListToolsOptions) -> Self {
        self.options = Some(options);
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        let mut opts = self.options.unwrap_or_default();
        opts.limit = Some(limit);
        self.options = Some(opts);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        let mut opts = self.options.unwrap_or_default();
        opts.offset = Some(offset);
        self.options = Some(opts);
        self
    }

    pub fn search(mut self, search: &str) -> Self {
        let mut opts = self.options.unwrap_or_default();
        opts.search = Some(search.to_string());
        self.options = Some(opts);
        self
    }

    pub fn is_active(mut self, is_active: bool) -> Self {
        let mut opts = self.options.unwrap_or_default();
        opts.is_active = Some(is_active);
        self.options = Some(opts);
        self
    }

    pub async fn send(self) -> Result<PaginatedResponse<AiTool>> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/tools", config.base_url);

        let mut request = client.get(&url);

        if let Some(opts) = self.options {
            request = request.query(&opts);
        }

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to list tools: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Create a new AI tool (async function)
pub async fn create_tool(request: CreateAiToolRequest) -> Result<AiTool> {
    CreateToolBuilder::new().request(request).send().await
}

/// Builder for creating AI tools
pub struct CreateToolBuilder {
    request: CreateAiToolRequest,
}

impl CreateToolBuilder {
    pub fn new() -> Self {
        Self {
            request: CreateAiToolRequest::new(
                "".to_string(),
                "".to_string(),
                serde_json::Value::Object(serde_json::Map::new()),
            ),
        }
    }

    pub fn request(mut self, request: CreateAiToolRequest) -> Self {
        self.request = request;
        self
    }

    pub async fn send(self) -> Result<AiTool> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/tools", config.base_url);

        let response = client.post(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to create tool: {error_body}"),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Get a specific AI tool by ID (async function)
pub async fn fetch_tool(tool_id: &str) -> Result<AiTool> {
    FetchToolBuilder::new().tool_id(tool_id).send().await
}

/// Builder for fetching AI tools
pub struct FetchToolBuilder {
    tool_id: String,
}

impl FetchToolBuilder {
    pub fn new() -> Self {
        Self {
            tool_id: String::new(),
        }
    }

    pub fn tool_id(mut self, tool_id: &str) -> Self {
        self.tool_id = tool_id.to_string();
        self
    }

    pub async fn send(self) -> Result<AiTool> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/tools/{}", config.base_url, self.tool_id);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to get tool {}: {}", self.tool_id, error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Update an AI tool (async function)
pub async fn update_tool(tool_id: &str, request: UpdateAiToolRequest) -> Result<AiTool> {
    UpdateToolBuilder::new()
        .tool_id(tool_id)
        .request(request)
        .send()
        .await
}

/// Builder for updating AI tools
pub struct UpdateToolBuilder {
    tool_id: String,
    request: UpdateAiToolRequest,
}

impl UpdateToolBuilder {
    pub fn new() -> Self {
        Self {
            tool_id: String::new(),
            request: UpdateAiToolRequest::new(),
        }
    }

    pub fn tool_id(mut self, tool_id: &str) -> Self {
        self.tool_id = tool_id.to_string();
        self
    }

    pub fn request(mut self, request: UpdateAiToolRequest) -> Self {
        self.request = request;
        self
    }

    pub async fn send(self) -> Result<AiTool> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/tools/{}", config.base_url, self.tool_id);

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update tool {}: {}", self.tool_id, error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

/// Delete an AI tool (async function)
pub async fn delete_tool(tool_id: &str) -> Result<()> {
    DeleteToolBuilder::new().tool_id(tool_id).send().await
}

/// Builder for deleting AI tools
pub struct DeleteToolBuilder {
    tool_id: String,
}

impl DeleteToolBuilder {
    pub fn new() -> Self {
        Self {
            tool_id: String::new(),
        }
    }

    pub fn tool_id(mut self, tool_id: &str) -> Self {
        self.tool_id = tool_id.to_string();
        self
    }

    pub async fn send(self) -> Result<()> {
        let config = get_config();
        let client = get_client();
        let url = format!("{}/ai/tools/{}", config.base_url, self.tool_id);

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete tool {}: {}", self.tool_id, error_body),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}
