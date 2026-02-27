use crate::{
    client::WachtClient,
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

#[derive(Debug, Clone)]
pub struct ToolsApi {
    client: WachtClient,
}

impl ToolsApi {
    pub(crate) fn new(client: WachtClient) -> Self {
        Self { client }
    }

    pub fn list_tools(&self) -> ListToolsBuilder {
        ListToolsBuilder::new(self.client.clone())
    }

    pub fn create_tool(&self) -> CreateToolBuilder {
        CreateToolBuilder::new(self.client.clone())
    }

    pub fn fetch_tool(&self) -> FetchToolBuilder {
        FetchToolBuilder::new(self.client.clone())
    }

    pub fn update_tool(&self) -> UpdateToolBuilder {
        UpdateToolBuilder::new(self.client.clone())
    }

    pub fn delete_tool(&self) -> DeleteToolBuilder {
        DeleteToolBuilder::new(self.client.clone())
    }
}

pub struct ListToolsBuilder {
    client: WachtClient,
    options: Option<ListToolsOptions>,
}

impl ListToolsBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
            options: None,
        }
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
        let client = self.client.http_client();
        let url = format!("{}/ai/tools", self.client.config().base_url);

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

pub struct CreateToolBuilder {
    client: WachtClient,
    request: CreateAiToolRequest,
}

impl CreateToolBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
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
        let client = self.client.http_client();
        let url = format!("{}/ai/tools", self.client.config().base_url);

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

pub struct FetchToolBuilder {
    client: WachtClient,
    tool_id: String,
}

impl FetchToolBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
            tool_id: String::new(),
        }
    }

    pub fn tool_id(mut self, tool_id: &str) -> Self {
        self.tool_id = tool_id.to_string();
        self
    }

    pub async fn send(self) -> Result<AiTool> {
        let client = self.client.http_client();
        let url = format!("{}/ai/tools/{}", self.client.config().base_url, self.tool_id);

        let response = client.get(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to get tool {}: {error_body}", self.tool_id),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

pub struct UpdateToolBuilder {
    client: WachtClient,
    tool_id: String,
    request: UpdateAiToolRequest,
}

impl UpdateToolBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
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
        let client = self.client.http_client();
        let url = format!("{}/ai/tools/{}", self.client.config().base_url, self.tool_id);

        let response = client.patch(&url).json(&self.request).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to update tool {}: {error_body}", self.tool_id),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}

pub struct DeleteToolBuilder {
    client: WachtClient,
    tool_id: String,
}

impl DeleteToolBuilder {
    pub fn new(client: WachtClient) -> Self {
        Self {
            client,
            tool_id: String::new(),
        }
    }

    pub fn tool_id(mut self, tool_id: &str) -> Self {
        self.tool_id = tool_id.to_string();
        self
    }

    pub async fn send(self) -> Result<()> {
        let client = self.client.http_client();
        let url = format!("{}/ai/tools/{}", self.client.config().base_url, self.tool_id);

        let response = client.delete(&url).send().await?;
        let status = response.status();

        if status.is_success() {
            Ok(())
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status,
                message: format!("Failed to delete tool {}: {error_body}", self.tool_id),
                details: serde_json::from_str(&error_body).ok(),
            })
        }
    }
}
