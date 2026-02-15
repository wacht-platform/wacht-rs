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
wacht = "0.0.1-alpha.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use wacht::{init_from_env, init, WachtConfig};
use wacht::api::{agents, health};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize with environment variables (recommended)
    // Requires: WACHT_API_KEY and WACHT_FRONTEND_HOST
    init_from_env().await?;

    // Or initialize with API key and frontend URL
    wacht::init(WachtConfig::new(
        "your-api-key",
        "https://app.wacht.dev"
    ))?;

    // Check API health
    let health = health::check_health().await?;
    println!("API Status: {}", health.status);

    // List AI agents
    let agents = agents::fetch_agents(None).await?;
    for agent in agents.data {
        println!("Agent: {}", agent.name);
    }

    Ok(())
}
```

## Configuration

### Environment Variables

- `WACHT_API_KEY`: API key for authentication (passed as Bearer token) - **REQUIRED**
- `WACHT_FRONTEND_HOST`: Frontend URL (e.g., https://app.wacht.dev) - **REQUIRED**
- `WACHT_PUBLIC_SIGNING_KEY`: Optional PEM-encoded public key for JWT verification

**Note**: Both `WACHT_API_KEY` and `WACHT_FRONTEND_HOST` must be set when using `init_from_env()`.

### Programmatic Configuration

```rust
use wacht::{init, WachtConfig};

// Initialize with API key and frontend URL
init(WachtConfig::new(
    "your-api-key",
    "https://app.wacht.dev"
))?;

// Initialize with API key and manually set public key
init(WachtConfig::new(
    "your-api-key",
    "https://app.wacht.dev"
).with_public_key("-----BEGIN PUBLIC KEY-----\n..."))?;

// Initialize with automatic public key fetching
let config = WachtConfig::new(
    "your-api-key",
    "https://app.wacht.dev"
).load_public_key().await?;
init(config)?;
```

## API Modules

### AI Agents (`agents`)

```rust
use wacht::api::agents::{fetch_agents, fetch_agent, create_agent, update_agent, delete_agent, ListAgentsOptions};
use wacht::models::{CreateAiAgentRequest, UpdateAiAgentRequest};

// List agents with filtering
let options = ListAgentsOptions {
    page: Some(1),
    per_page: Some(20),
    is_active: Some(true),
    search: Some("assistant".to_string()),
};
let agents = fetch_agents(Some(options)).await?;

// Get specific agent
let agent = fetch_agent("agent-id").await?;

// Create new agent
let new_agent = CreateAiAgentRequest {
    name: "Support Bot".to_string(),
    model: "gpt-4".to_string(),
    description: Some("Customer support assistant".to_string()),
    system_prompt: Some("You are a helpful customer support assistant.".to_string()),
    temperature: Some(0.7),
    max_tokens: Some(1000),
    tools: None,
    knowledge_bases: None,
};
let created = create_agent(new_agent).await?;

// Update agent
let update = UpdateAiAgentRequest {
    name: Some("Updated Name".to_string()),
    ..Default::default()
};
let updated = update_agent("agent-id", update).await?;

// Delete agent
delete_agent("agent-id").await?;
```

### Health Check (`health`)

```rust
use wacht::api::health::{check_health, ping};

// Full health check
let health = check_health().await?;
println!("Status: {}", health.status);

// Simple ping
let is_alive = ping().await?;
```

### Other APIs

- `analytics`: Analytics and usage data
- `execution_context`: AI execution contexts
- `knowledge_bases`: Knowledge base management
- `organizations`: Organization management
- `settings`: Platform settings
- `tools`: AI tools management
- `users`: User management
- `workspaces`: Workspace management
- `deployments`: Deployment management
- `webhooks`: Webhook management
- `segments`: User segments management
- `notifications`: Notification management
- `integrations`: Agent integrations management

## Error Handling

```rust
use wacht::{Error, api::agents::fetch_agent};

match fetch_agent("invalid-id").await {
    Ok(agent) => println!("Found: {}", agent.name),
    Err(Error::Api { status, message, details }) => {
        println!("API Error {}: {}", status, message);
        if let Some(details) = details {
            println!("Details: {:?}", details);
        }
    }
    Err(Error::Request(e)) => println!("Network error: {}", e),
    Err(e) => println!("Other error: {}", e),
}
```

## Axum Middleware (Optional Feature)

The SDK includes optional Axum middleware for JWT authentication and permission checking. To enable it, add the `axum` feature:

```toml
[dependencies]
wacht = { version = "0.0.1-alpha.1", features = ["axum"] }
```

### JWT Authentication Middleware

```rust
use wacht::middleware::{JwtAuthLayer, JwtClaims};
use axum::{Router, routing::get, Extension};

async fn protected_handler(Extension(claims): Extension<JwtClaims>) -> String {
    format!("Hello, user {}!", claims.sub)
}

let app = Router::new()
    .route("/protected", get(protected_handler))
    .layer(JwtAuthLayer::new());
```

### Permission Middleware

```rust
use wacht::middleware::{PermissionLayer, JwtClaims};

async fn admin_only(Extension(claims): Extension<JwtClaims>) -> &'static str {
    "Admin access granted!"
}

let app = Router::new()
    .route("/admin", get(admin_only))
    .layer(PermissionLayer::new("admin:read")) // Require specific permission
    .layer(JwtAuthLayer::new()); // JWT auth must come after permission layer
```

## License

MIT
