# wacht-rs

Official Rust SDK for Wacht.

## Install

```toml
[dependencies]
wacht = "0.1.0-beta.5"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

For Axum middleware support:

```toml
[dependencies]
wacht = { version = "0.1.0-beta.5", features = ["axum"] }
```

## Configuration

Environment-based setup expects:
- `WACHT_API_KEY`
- `WACHT_PUBLISHABLE_KEY` (preferred) or `WACHT_FRONTEND_HOST`
- optional: `WACHT_PUBLIC_SIGNING_KEY`

## Client Usage (instance-based)

```rust
use wacht::WachtClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WachtClient::from_env().await?;

    let apps = client.webhooks().list_webhook_apps().send().await?;
    println!("apps: {}", apps.len());

    Ok(())
}
```

You can also construct explicitly:

```rust
use wacht::{WachtClient, WachtConfig};

let config = WachtConfig::new("wk_live_xxx", "https://your-deployment.fapi.trywacht.xyz");
let client = WachtClient::new(config)?;
```

## Gateway Authz (API key + OAuth access token)

```rust
use wacht::gateway::GatewayPrincipalType;

let authz = client
    .gateway()
    .verify_request_with_principal_type(
        GatewayPrincipalType::ApiKey,
        "wk_live_xxx",
        "GET",
        "/v1/data",
    )
    .await?;

let principal = authz.resolve_principal_context();
println!("app_slug = {}", principal.identity.app_slug);
```

OAuth access token:

```rust
let authz = client
    .gateway()
    .verify_oauth_access_token_request("oauth_access_token", "POST", "/v1/data")
    .await?;
```

## Axum Middleware (`axum` feature)

Session JWT middleware:

```rust
use axum::{routing::get, Router};
use wacht::middleware::AuthLayer;

async fn protected() -> &'static str {
    "ok"
}

let app = Router::new()
    .route("/protected", get(protected))
    .layer(AuthLayer::new());
```

Extract auth context:

```rust
use axum::response::IntoResponse;
use wacht::middleware::RequireAuth;

async fn whoami(auth: RequireAuth) -> impl IntoResponse {
    format!("user_id={}", auth.user_id)
}
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
