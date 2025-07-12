# Wacht Rust SDK

The official Rust SDK for the Wacht platform, providing a type-safe client for all Wacht APIs and authentication middleware for Axum web applications.

## Features

- **Lazy Initialization**: Global HTTP client with authentication headers baked in
- **Required Authentication**: SDK enforces authentication at initialization with API key
- **Environment Configuration**: Automatic configuration from environment variables
- **Type-Safe API**: Strongly typed request/response models
- **Idiomatic Rust**: Clean, ergonomic API design following Rust best practices
- **Async/Await**: Full async support using Tokio
- **Error Handling**: Comprehensive error types with detailed messages

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
wacht = "2.0.0"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use wacht::{init_from_env, WachtConfig, AgentsApi, HealthApi};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize with environment variables (recommended)
    // Requires: WACHT_API_KEY
    init_from_env()?;

    // Or initialize with API key
    wacht::init(WachtConfig::new(
        "https://api.wacht.dev",
        "your-api-key"
    ))?;

    // Check API health
    let health = HealthApi::check().await?;
    println!("API Status: {}", health.status);

    // List AI agents
    let agents = AgentsApi::list(None).await?;
    for agent in agents.data {
        println!("Agent: {}", agent.name);
    }

    Ok(())
}
```

## Configuration

### Environment Variables

- `WACHT_API_URL`: Base URL for the API (default: https://api.wacht.dev)
- `WACHT_API_KEY`: API key for authentication (passed as Bearer token)
- `WACHT_PUBLIC_KEY`: Optional PEM-encoded public key for JWT verification

**Note**: `WACHT_API_KEY` must be set for authentication.

### Programmatic Configuration

```rust
use wacht::{init, init_with_auto_key, WachtConfig};

// Initialize with API key
init(WachtConfig::new(
    "https://api.wacht.dev",
    "your-api-key"
))?;

// Initialize with API key and manually set public key
init(WachtConfig::new(
    "https://api.wacht.dev",
    "your-api-key"
).with_public_key("-----BEGIN PUBLIC KEY-----\n..."))?;

// Initialize with automatic public key fetching
init_with_auto_key(
    "https://api.wacht.dev",
    "your-api-key"
).await?;
```

## API Modules

### AI Agents (`AgentsApi`)

```rust
use wacht::{AgentsApi, models::CreateAiAgentRequest};
use wacht::api::agents::ListAgentsOptions;

// List agents with filtering
let options = ListAgentsOptions {
    page: Some(1),
    per_page: Some(20),
    is_active: Some(true),
    search: Some("assistant".to_string()),
};
let agents = AgentsApi::list(Some(options)).await?;

// Get specific agent
let agent = AgentsApi::get("agent-id").await?;

// Create new agent
let new_agent = CreateAiAgentRequest {
    name: "Support Bot".to_string(),
    description: Some("Customer support assistant".to_string()),
    configuration: Some(serde_json::json!({
        "model": "gpt-4"
    })),
};
let created = AgentsApi::create(new_agent).await?;

// Update agent
let update = UpdateAiAgentRequest {
    name: Some("Updated Name".to_string()),
    ..Default::default()
};
let updated = AgentsApi::update("agent-id", update).await?;

// Delete agent
AgentsApi::delete("agent-id").await?;
```

### Health Check (`HealthApi`)

```rust
use wacht::HealthApi;

// Full health check
let health = HealthApi::check().await?;
println!("Status: {}", health.status);

// Simple ping
let is_alive = HealthApi::ping().await?;
```

### Other APIs

- `AnalyticsApi`: Analytics and usage data
- `ExecutionContextApi`: AI execution contexts
- `KnowledgeBasesApi`: Knowledge base management
- `OrganizationsApi`: Organization management
- `SettingsApi`: Platform settings
- `ToolsApi`: AI tools management
- `UsersApi`: User management
- `WorkflowsApi`: AI workflow management
- `WorkspacesApi`: Workspace management

## Error Handling

```rust
use wacht::{AgentsApi, Error};

match AgentsApi::get("invalid-id").await {
    Ok(agent) => println!("Found: {}", agent.name),
    Err(Error::Api { status, message, details }) => {
        println!("API Error {}: {}", status, message);
        if let Some(details) = details {
            println!("Details: {}", details);
        }
    }
    Err(Error::Request(e)) => println!("Network error: {}", e),
    Err(e) => println!("Other error: {}", e),
}
```


## License

MIT
