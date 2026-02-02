use crate::{
    client::{get_client, get_config},
    error::{Error, Result},
    models::{AiTool, CreateAiToolRequest, UpdateAiToolRequest},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolListResponse {
    pub data: Vec<AiTool>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListToolsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

/// List all AI tools
pub async fn fetch_tools(options: Option<ListToolsOptions>) -> Result<ToolListResponse> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai/tools", config.base_url);
    
    let mut request = client.get(&url);
    
    if let Some(opts) = options {
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

/// Create a new AI tool
pub async fn create_tool(request: CreateAiToolRequest) -> Result<AiTool> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai/tools", config.base_url);
    
    let response = client.post(&url).json(&request).send().await?;
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

/// Get a specific AI tool by ID
pub async fn fetch_tool(tool_id: &str) -> Result<AiTool> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai/tools/{}", config.base_url, tool_id);
    
    let response = client.get(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to get tool {tool_id}: {error_body}"),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Update an AI tool
pub async fn update_tool(tool_id: &str, request: UpdateAiToolRequest) -> Result<AiTool> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai/tools/{}", config.base_url, tool_id);
    
    let response = client.patch(&url).json(&request).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(response.json().await?)
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to update tool {tool_id}: {error_body}"),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}

/// Delete an AI tool
pub async fn delete_tool(tool_id: &str) -> Result<()> {
    let config = get_config();
    let client = get_client();
    let url = format!("{}/ai/tools/{}", config.base_url, tool_id);
    
    let response = client.delete(&url).send().await?;
    let status = response.status();
    
    if status.is_success() {
        Ok(())
    } else {
        let error_body = response.text().await?;
        Err(Error::Api {
            status,
            message: format!("Failed to delete tool {tool_id}: {error_body}"),
            details: serde_json::from_str(&error_body).ok(),
        })
    }
}