# PhotoPrism MCP Server - Rust Implementation Guide

## Table of Contents

1. [Overview](#overview)
2. [Architecture & Design](#architecture--design)
3. [Project Structure](#project-structure)
4. [Dependencies & Setup](#dependencies--setup)
5. [Core Components](#core-components)
6. [Tool Definition Patterns](#tool-definition-patterns)
7. [Schema Generation](#schema-generation)
8. [API Client Implementation](#api-client-implementation)
9. [Error Handling](#error-handling)
10. [Authentication](#authentication)
11. [Resource Handlers](#resource-handlers)
12. [Testing Strategy](#testing-strategy)
13. [Deployment](#deployment)
14. [Implementation Roadmap](#implementation-roadmap)

---

## Overview

### Technology Stack

- **Language**: Rust (Edition 2021)
- **MCP Framework**: TurboMCP 2.2.1
- **HTTP Client**: reqwest with rustls-tls
- **Async Runtime**: Tokio
- **Serialization**: serde + serde_json
- **Schema**: schemars for JSON Schema generation

### Key Features

- Zero-boilerplate tool definitions with `#[tool]` macro
- Automatic JSON schema generation from Rust types
- Type-safe parameter validation at compile time
- Multiple transport protocols (stdio, HTTP/SSE, WebSocket, TCP)
- Built-in error handling and logging
- Session-based authentication

---

## Architecture & Design

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                  MCP Client (Claude)                     │
└───────────────────┬─────────────────────────────────────┘
                    │ MCP Protocol
                    │ (JSON-RPC 2.0)
┌───────────────────▼─────────────────────────────────────┐
│              PhotoPrism MCP Server (Rust)                │
│                                                           │
│  ┌────────────────────────────────────────────────────┐  │
│  │           TurboMCP Framework                       │  │
│  │  ┌──────────────┐  ┌──────────────┐               │  │
│  │  │ Tool Handler │  │   Resource   │               │  │
│  │  │   Registry   │  │   Registry   │               │  │
│  │  └──────────────┘  └──────────────┘               │  │
│  │  ┌──────────────────────────────────┐             │  │
│  │  │  Transport Layer (STDIO/HTTP)    │             │  │
│  │  └──────────────────────────────────┘             │  │
│  └────────────────────────────────────────────────────┘  │
│                                                           │
│  ┌────────────────────────────────────────────────────┐  │
│  │         PhotoPrism API Client                      │  │
│  │  ┌──────────────┐  ┌──────────────┐               │  │
│  │  │  Auth Layer  │  │  HTTP Client │               │  │
│  │  │  (Sessions)  │  │   (reqwest)  │               │  │
│  │  └──────────────┘  └──────────────┘               │  │
│  └────────────────────────────────────────────────────┘  │
└───────────────────┬─────────────────────────────────────┘
                    │ REST API
                    │ (Bearer Token)
┌───────────────────▼─────────────────────────────────────┐
│              PhotoPrism Backend (Go)                     │
│                 /api/v1/*                                │
└──────────────────────────────────────────────────────────┘
```

### Design Principles

1. **Separation of Concerns**
   - Tools: Business logic and parameter validation
   - API Client: HTTP communication with PhotoPrism
   - Resources: Read-only data access via URIs
   - Server: MCP protocol and transport handling

2. **Type Safety**
   - Leverage Rust's type system for compile-time validation
   - Use serde for automatic serialization/deserialization
   - Generate JSON schemas from Rust types with schemars

3. **Error Handling**
   - Use `McpResult<T>` for all tool/resource handlers
   - Map PhotoPrism API errors to meaningful MCP errors
   - Provide clear error messages to users

---

## Project Structure

```
photoprism-mcp/
├── Cargo.toml                 # Project dependencies
├── README.md                  # User documentation
├── IMPLEMENTATION_GUIDE.md    # This file
│
├── src/
│   ├── main.rs               # Entry point & server setup
│   ├── lib.rs                # Library exports
│   │
│   ├── server/               # MCP server implementation
│   │   ├── mod.rs
│   │   └── photoprism.rs     # Main server struct with #[server]
│   │
│   ├── tools/                # Tool implementations
│   │   ├── mod.rs            # Re-exports all tools
│   │   ├── photos.rs         # Photo management tools
│   │   ├── albums.rs         # Album management tools
│   │   ├── search.rs         # Search tools
│   │   ├── labels.rs         # Label tools
│   │   ├── subjects.rs       # Subject/people tools
│   │   └── library.rs        # Library/indexing tools
│   │
│   ├── resources/            # Resource handlers
│   │   ├── mod.rs
│   │   ├── photos.rs         # Photo resources
│   │   └── albums.rs         # Album resources
│   │
│   ├── client/               # PhotoPrism API client
│   │   ├── mod.rs
│   │   ├── auth.rs           # Authentication
│   │   ├── photos.rs         # Photo endpoints
│   │   ├── albums.rs         # Album endpoints
│   │   ├── search.rs         # Search endpoints
│   │   └── types.rs          # API types/models
│   │
│   ├── types/                # Shared types
│   │   ├── mod.rs
│   │   ├── photo.rs          # Photo models
│   │   ├── album.rs          # Album models
│   │   └── common.rs         # Common types
│   │
│   ├── error.rs              # Error types
│   └── config.rs             # Configuration
│
└── tests/                    # Integration tests
    ├── tools_test.rs
    └── client_test.rs
```

---

## Dependencies & Setup

### Cargo.toml

```toml
[package]
name = "photoprism-mcp"
version = "0.1.0"
edition = "2021"

[dependencies]
# MCP Framework
turbomcp = "2.2.1"
turbomcp-protocol = "2.2.1"

# Async Runtime
tokio = { version = "1.47", features = ["full"] }
async-trait = "0.1"

# HTTP Client
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
schemars = { version = "1.0", features = ["chrono04"] }

# Error Handling
thiserror = "2.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Date/Time
chrono = { version = "0.4", features = ["serde"] }

# Config
serde_yaml = "0.9"
dirs = "5.0"

# Utilities
once_cell = "1.19"
uuid = { version = "1.6", features = ["v4", "serde"] }

[dev-dependencies]
mockito = "1.2"
tokio-test = "0.4"
```

### Initial Setup

```bash
# Create new Cargo project
cargo new photoprism-mcp --name photoprism-mcp

# Change to project directory
cd photoprism-mcp

# Add dependencies
cargo add turbomcp@2.2.1
cargo add tokio@1.47 --features full
cargo add reqwest@0.12 --features json,rustls-tls
cargo add serde@1.0 --features derive
cargo add serde_json@1.0
cargo add schemars@1.0 --features chrono04
cargo add thiserror@2.0
cargo add anyhow@1.0
cargo add tracing@0.1
cargo add tracing-subscriber@0.3 --features env-filter
cargo add chrono@0.4 --features serde
cargo add async-trait@0.1

# Test compilation
cargo build
```

---

## Core Components

### 1. Server Implementation

**File: `src/server/photoprism.rs`**

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use turbomcp::prelude::*;
use crate::client::PhotoPrismClient;
use crate::config::Config;

/// Main PhotoPrism MCP Server
#[derive(Clone)]
pub struct PhotoPrismServer {
    /// PhotoPrism API client
    client: Arc<PhotoPrismClient>,

    /// Configuration
    config: Arc<RwLock<Config>>,
}

#[turbomcp::server(
    name = "photoprism",
    version = "0.1.0",
    description = "MCP server for PhotoPrism photo library management",
    transports = ["stdio", "http"]
)]
impl PhotoPrismServer {
    /// Create a new PhotoPrism MCP server
    pub fn new(config: Config) -> Result<Self, anyhow::Error> {
        let client = PhotoPrismClient::new(
            config.base_url.clone(),
            config.username.clone(),
            config.password.clone(),
        )?;

        Ok(Self {
            client: Arc::new(client),
            config: Arc::new(RwLock::new(config)),
        })
    }

    // Tools are defined here using #[tool] macro
    // (See Tool Definition Patterns section)
}
```

### 2. Configuration

**File: `src/config.rs`**

```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// PhotoPrism base URL (e.g., http://localhost:2342)
    pub base_url: String,

    /// PhotoPrism username
    pub username: String,

    /// PhotoPrism password
    #[serde(skip_serializing)]
    pub password: String,

    /// Session token (cached)
    #[serde(skip)]
    pub session_token: Option<String>,
}

impl Config {
    /// Load configuration from file or environment
    pub fn load() -> Result<Self, anyhow::Error> {
        // Try loading from ~/.photoprism-mcp/config.yaml
        let config_path = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Cannot find config directory"))?
            .join("photoprism-mcp")
            .join("config.yaml");

        if config_path.exists() {
            let contents = std::fs::read_to_string(&config_path)?;
            return Ok(serde_yaml::from_str(&contents)?);
        }

        // Fallback to environment variables
        Ok(Self {
            base_url: std::env::var("PHOTOPRISM_URL")
                .unwrap_or_else(|_| "http://localhost:2342".to_string()),
            username: std::env::var("PHOTOPRISM_USERNAME")
                .unwrap_or_else(|_| "admin".to_string()),
            password: std::env::var("PHOTOPRISM_PASSWORD")?,
            session_token: None,
        })
    }
}
```

### 3. Main Entry Point

**File: `src/main.rs`**

```rust
use anyhow::Result;
use photoprism_mcp::server::PhotoPrismServer;
use photoprism_mcp::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    // Load configuration
    let config = Config::load()?;

    // Create server
    let server = PhotoPrismServer::new(config)?;

    // Determine transport from environment or CLI args
    let transport = std::env::var("TRANSPORT")
        .unwrap_or_else(|_| "stdio".to_string());

    match transport.as_str() {
        "http" => {
            tracing::info!("Starting HTTP transport on 0.0.0.0:3000");
            server.run_http("0.0.0.0:3000").await?;
        }
        "stdio" | _ => {
            tracing::info!("Starting STDIO transport");
            server.run_stdio().await?;
        }
    }

    Ok(())
}
```

---

## Tool Definition Patterns

### Pattern 1: Simple Tool (No Complex Types)

```rust
/// Search photos with text query
#[tool("Search photos by text query")]
async fn search_photos(
    &self,
    #[description("Search query string")] query: String,
    #[description("Maximum number of results (default: 100)")] count: Option<u32>,
) -> McpResult<String> {
    let count = count.unwrap_or(100);

    let results = self.client
        .search_photos(&query, count)
        .await
        .map_err(|e| McpError::internal(format!("Search failed: {}", e)))?;

    Ok(serde_json::to_string_pretty(&results)?)
}
```

### Pattern 2: Structured Parameters

```rust
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Search filters for photos
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct PhotoSearchParams {
    /// Search query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,

    /// Quality filter (1-7)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(range(min = 1, max = 7))]
    pub quality: Option<u8>,

    /// Filter by album UID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,

    /// Maximum results (default: 100, max: 1000)
    #[serde(default = "default_count")]
    #[schemars(range(min = 1, max = 1000))]
    pub count: u32,

    /// Offset for pagination
    #[serde(default)]
    pub offset: u32,
}

fn default_count() -> u32 { 100 }

/// Advanced photo search with filters
#[tool("Search photos with advanced filters")]
async fn search_photos_advanced(
    &self,
    params: PhotoSearchParams,
) -> McpResult<String> {
    let results = self.client
        .search_photos_advanced(params)
        .await
        .map_err(|e| McpError::internal(format!("Search failed: {}", e)))?;

    Ok(serde_json::to_string_pretty(&results)?)
}
```

### Pattern 3: Context Usage (Logging)

```rust
/// Create a new album
#[tool("Create a new album")]
async fn create_album(
    &self,
    ctx: Context,
    #[description("Album title")] title: String,
    #[description("Album description")] description: Option<String>,
) -> McpResult<String> {
    ctx.info(&format!("Creating album: {}", title)).await?;

    let album = self.client
        .create_album(&title, description.as_deref())
        .await
        .map_err(|e| {
            ctx.error(&format!("Failed to create album: {}", e));
            McpError::internal(format!("Album creation failed: {}", e))
        })?;

    ctx.info(&format!("Album created with UID: {}", album.uid)).await?;

    Ok(serde_json::to_string_pretty(&album)?)
}
```

### Pattern 4: Validation & Error Handling

```rust
/// Delete a photo
#[tool("Delete a photo by UID")]
async fn delete_photo(
    &self,
    ctx: Context,
    #[description("Photo UID to delete")] uid: String,
) -> McpResult<String> {
    // Validate UID format
    if uid.is_empty() {
        return Err(McpError::invalid_request("Photo UID cannot be empty"));
    }

    if uid.len() != 16 {
        return Err(McpError::invalid_request(
            "Invalid UID format (must be 16 characters)"
        ));
    }

    ctx.warn(&format!("Deleting photo: {}", uid)).await?;

    self.client
        .delete_photo(&uid)
        .await
        .map_err(|e| McpError::internal(format!("Delete failed: {}", e)))?;

    Ok(format!("Photo {} deleted successfully", uid))
}
```

### Pattern 5: Enum Parameters

```rust
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum PhotoType {
    Image,
    Video,
    Live,
    Raw,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Newest,
    Oldest,
    Name,
    Size,
    Duration,
}

/// List photos with type filter
#[tool("List photos filtered by type")]
async fn list_photos_by_type(
    &self,
    #[description("Type of photos to list")] photo_type: PhotoType,
    #[description("Sort order")] sort: Option<SortOrder>,
    count: Option<u32>,
) -> McpResult<String> {
    let count = count.unwrap_or(100);
    let sort = sort.unwrap_or(SortOrder::Newest);

    let results = self.client
        .list_photos(photo_type, sort, count)
        .await
        .map_err(|e| McpError::internal(format!("Failed to list photos: {}", e)))?;

    Ok(serde_json::to_string_pretty(&results)?)
}
```

---

## Schema Generation

TurboMCP automatically generates JSON schemas from your Rust types using the `schemars` crate. Here's how it works:

### Automatic Schema Derivation

```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// This struct automatically gets a JSON schema
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct PhotoMetadata {
    /// Photo title
    pub title: String,

    /// Photo description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// ISO value (100-6400)
    #[schemars(range(min = 100, max = 6400))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso: Option<u32>,

    /// Exposure time in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposure: Option<f64>,

    /// GPS coordinates [latitude, longitude]
    #[schemars(length(min = 2, max = 2))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<Vec<f64>>,

    /// Tags/labels
    #[serde(default)]
    pub tags: Vec<String>,
}
```

### Generated JSON Schema

The above Rust struct generates this JSON schema:

```json
{
  "type": "object",
  "properties": {
    "title": {
      "type": "string",
      "description": "Photo title"
    },
    "description": {
      "type": "string",
      "description": "Photo description"
    },
    "iso": {
      "type": "integer",
      "minimum": 100,
      "maximum": 6400,
      "description": "ISO value (100-6400)"
    },
    "exposure": {
      "type": "number",
      "description": "Exposure time in seconds"
    },
    "coordinates": {
      "type": "array",
      "items": { "type": "number" },
      "minItems": 2,
      "maxItems": 2,
      "description": "GPS coordinates [latitude, longitude]"
    },
    "tags": {
      "type": "array",
      "items": { "type": "string" },
      "description": "Tags/labels",
      "default": []
    }
  },
  "required": ["title", "tags"]
}
```

### Schema Attributes

Common `schemars` attributes:

```rust
#[schemars(range(min = 1, max = 100))]        // Numeric range
#[schemars(length(min = 1, max = 255))]       // String/Array length
#[schemars(regex = "^[a-z0-9-]+$")]           // Pattern validation
#[schemars(default)]                          // Has default value
#[schemars(example = "example_fn")]           // Example value
```

---

## API Client Implementation

### Client Structure

**File: `src/client/mod.rs`**

```rust
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct PhotoPrismClient {
    http_client: Client,
    base_url: String,
    username: String,
    password: String,
    session_token: Arc<RwLock<Option<String>>>,
}

impl PhotoPrismClient {
    pub fn new(
        base_url: String,
        username: String,
        password: String,
    ) -> Result<Self, anyhow::Error> {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self {
            http_client,
            base_url: base_url.trim_end_matches('/').to_string(),
            username,
            password,
            session_token: Arc::new(RwLock::new(None)),
        })
    }

    /// Ensure we have a valid session token
    async fn ensure_authenticated(&self) -> Result<String, anyhow::Error> {
        // Check if we have a cached token
        {
            let token_guard = self.session_token.read().await;
            if let Some(token) = token_guard.as_ref() {
                return Ok(token.clone());
            }
        }

        // Login to get a new token
        #[derive(Serialize)]
        struct LoginRequest {
            username: String,
            password: String,
        }

        #[derive(Deserialize)]
        struct LoginResponse {
            id: String,
            // Other fields...
        }

        let login_url = format!("{}/api/v1/session", self.base_url);
        let response = self.http_client
            .post(&login_url)
            .json(&LoginRequest {
                username: self.username.clone(),
                password: self.password.clone(),
            })
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Authentication failed: {}", response.status()));
        }

        let login_response: LoginResponse = response.json().await?;
        let token = login_response.id;

        // Cache the token
        {
            let mut token_guard = self.session_token.write().await;
            *token_guard = Some(token.clone());
        }

        Ok(token)
    }

    /// Make an authenticated GET request
    async fn get<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
    ) -> Result<T, anyhow::Error> {
        let token = self.ensure_authenticated().await?;
        let url = format!("{}{}", self.base_url, path);

        let response = self.http_client
            .get(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Request failed: {}", response.status()));
        }

        Ok(response.json().await?)
    }

    /// Make an authenticated POST request
    async fn post<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<R, anyhow::Error> {
        let token = self.ensure_authenticated().await?;
        let url = format!("{}{}", self.base_url, path);

        let response = self.http_client
            .post(&url)
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .json(body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Request failed: {}", response.status()));
        }

        Ok(response.json().await?)
    }

    // Similar methods for PUT and DELETE...
}
```

### API Methods Example

**File: `src/client/photos.rs`**

```rust
use super::PhotoPrismClient;
use crate::types::photo::{Photo, PhotoList};
use serde::Serialize;

impl PhotoPrismClient {
    /// Search photos
    pub async fn search_photos(
        &self,
        query: &str,
        count: u32,
    ) -> Result<PhotoList, anyhow::Error> {
        let path = format!("/api/v1/photos?q={}&count={}", query, count);
        self.get(&path).await
    }

    /// Get photo by UID
    pub async fn get_photo(&self, uid: &str) -> Result<Photo, anyhow::Error> {
        let path = format!("/api/v1/photos/{}", uid);
        self.get(&path).await
    }

    /// Update photo metadata
    pub async fn update_photo(
        &self,
        uid: &str,
        updates: &PhotoUpdate,
    ) -> Result<Photo, anyhow::Error> {
        let path = format!("/api/v1/photos/{}", uid);
        self.put(&path, updates).await
    }

    /// Delete photo
    pub async fn delete_photo(&self, uid: &str) -> Result<(), anyhow::Error> {
        let path = format!("/api/v1/photos/{}", uid);
        self.delete(&path).await
    }
}

#[derive(Serialize)]
pub struct PhotoUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,
}
```

---

## Error Handling

### Error Types

**File: `src/error.rs`**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PhotoPrismError {
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("API request failed: {0}")]
    ApiError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// Convert PhotoPrismError to McpError
impl From<PhotoPrismError> for turbomcp::McpError {
    fn from(err: PhotoPrismError) -> Self {
        match err {
            PhotoPrismError::AuthenticationFailed(msg) => {
                turbomcp::McpError::unauthorized(msg)
            }
            PhotoPrismError::NotFound(msg) => {
                turbomcp::McpError::not_found(msg)
            }
            PhotoPrismError::InvalidParameter(msg) => {
                turbomcp::McpError::invalid_request(msg)
            }
            _ => turbomcp::McpError::internal(err.to_string()),
        }
    }
}
```

---

## Authentication

### Session Token Management

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AuthManager {
    session_token: Arc<RwLock<Option<String>>>,
    token_expiry: Arc<RwLock<Option<std::time::Instant>>>,
}

impl AuthManager {
    pub fn new() -> Self {
        Self {
            session_token: Arc::new(RwLock::new(None)),
            token_expiry: Arc::new(RwLock::new(None)),
        }
    }

    /// Check if token is valid (not expired)
    pub async fn is_token_valid(&self) -> bool {
        let expiry_guard = self.token_expiry.read().await;
        if let Some(expiry) = *expiry_guard {
            std::time::Instant::now() < expiry
        } else {
            false
        }
    }

    /// Set session token with expiry (e.g., 24 hours)
    pub async fn set_token(&self, token: String) {
        let expiry = std::time::Instant::now()
            + std::time::Duration::from_secs(24 * 3600);

        {
            let mut token_guard = self.session_token.write().await;
            *token_guard = Some(token);
        }

        {
            let mut expiry_guard = self.token_expiry.write().await;
            *expiry_guard = Some(expiry);
        }
    }

    /// Get current token
    pub async fn get_token(&self) -> Option<String> {
        self.session_token.read().await.clone()
    }

    /// Clear token
    pub async fn clear_token(&self) {
        {
            let mut token_guard = self.session_token.write().await;
            *token_guard = None;
        }
        {
            let mut expiry_guard = self.token_expiry.write().await;
            *expiry_guard = None;
        }
    }
}
```

---

## Resource Handlers

### Resource Pattern

```rust
/// List recent photos
#[resource("photoprism://photos/recent")]
async fn recent_photos(&self) -> McpResult<String> {
    let photos = self.client
        .search_photos("", 20)
        .await
        .map_err(|e| McpError::internal(format!("Failed to fetch recent photos: {}", e)))?;

    Ok(serde_json::to_string_pretty(&photos)?)
}

/// Get photo details by UID
#[resource("photoprism://photos/{uid}")]
async fn photo_details(&self, uid: String) -> McpResult<String> {
    let photo = self.client
        .get_photo(&uid)
        .await
        .map_err(|e| McpError::not_found(format!("Photo not found: {}", e)))?;

    Ok(serde_json::to_string_pretty(&photo)?)
}

/// List albums
#[resource("photoprism://albums/list")]
async fn list_albums(&self) -> McpResult<String> {
    let albums = self.client
        .list_albums()
        .await
        .map_err(|e| McpError::internal(format!("Failed to fetch albums: {}", e)))?;

    Ok(serde_json::to_string_pretty(&albums)?)
}

/// Get album by UID
#[resource("photoprism://albums/{uid}")]
async fn album_details(&self, uid: String) -> McpResult<String> {
    let album = self.client
        .get_album(&uid)
        .await
        .map_err(|e| McpError::not_found(format!("Album not found: {}", e)))?;

    Ok(serde_json::to_string_pretty(&album)?)
}
```

---

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_search_photos() {
        let mut server = Server::new_async().await;

        // Mock authentication endpoint
        let _auth_mock = server.mock("POST", "/api/v1/session")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"test-token"}"#)
            .create();

        // Mock search endpoint
        let _search_mock = server.mock("GET", "/api/v1/photos?q=sunset&count=10")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[]"#)
            .create();

        let client = PhotoPrismClient::new(
            server.url(),
            "test".to_string(),
            "test".to_string(),
        ).unwrap();

        let result = client.search_photos("sunset", 10).await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests

```rust
// tests/tools_test.rs
use photoprism_mcp::server::PhotoPrismServer;
use photoprism_mcp::config::Config;

#[tokio::test]
async fn test_search_photos_tool() {
    let config = Config {
        base_url: "http://localhost:2342".to_string(),
        username: "admin".to_string(),
        password: "password".to_string(),
        session_token: None,
    };

    let server = PhotoPrismServer::new(config).unwrap();

    // Test tool execution
    // (Implementation depends on TurboMCP testing utilities)
}
```

---

## Deployment

### Docker

**Dockerfile**

```dockerfile
FROM rust:1.75 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/photoprism-mcp /usr/local/bin/

ENV PHOTOPRISM_URL=http://localhost:2342
ENV PHOTOPRISM_USERNAME=admin
ENV TRANSPORT=stdio

ENTRYPOINT ["/usr/local/bin/photoprism-mcp"]
```

### Claude Desktop Configuration

**~/.config/claude/config.json** (Linux/macOS)

```json
{
  "mcpServers": {
    "photoprism": {
      "command": "/path/to/photoprism-mcp",
      "args": [],
      "env": {
        "PHOTOPRISM_URL": "http://localhost:2342",
        "PHOTOPRISM_USERNAME": "admin",
        "PHOTOPRISM_PASSWORD": "your-password",
        "TRANSPORT": "stdio"
      }
    }
  }
}
```

---

## Implementation Roadmap

### Phase 1: Core Foundation (Week 1)

**Tasks:**
1. ✅ Project setup with Cargo.toml
2. ✅ Configuration loading (env vars + config file)
3. ✅ PhotoPrism API client with authentication
4. ✅ Basic server structure with TurboMCP
5. ✅ Error handling types

**Deliverables:**
- Working authentication with PhotoPrism
- Basic MCP server skeleton
- Development environment ready

### Phase 2: Essential Tools (Week 2)

**Tasks:**
1. Photo search tools
   - `search_photos` - Basic text search
   - `search_photos_advanced` - Filters & sorting
   - `get_photo` - Get photo details

2. Album management
   - `list_albums` - List all albums
   - `get_album` - Get album details
   - `create_album` - Create new album
   - `add_photos_to_album` - Add photos

3. Basic library operations
   - `list_labels` - List available labels
   - `list_subjects` - List people/subjects

**Deliverables:**
- 8-10 working tools
- Comprehensive error handling
- Basic integration tests

### Phase 3: Advanced Features (Week 3)

**Tasks:**
1. Photo operations
   - `update_photo` - Update metadata
   - `delete_photo` - Delete photo
   - `download_photo` - Download file

2. Batch operations
   - `batch_archive` - Archive multiple photos
   - `batch_delete` - Delete multiple photos

3. Resources
   - `photoprism://photos/recent`
   - `photoprism://albums/list`
   - `photoprism://photos/{uid}`

**Deliverables:**
- 15-20 total tools
- Resource handlers
- Documentation

### Phase 4: Polish & Production (Week 4)

**Tasks:**
1. HTTP transport support
2. Comprehensive testing
3. Docker packaging
4. Documentation & examples
5. Performance optimization
6. CI/CD setup

**Deliverables:**
- Production-ready MCP server
- Docker images
- Complete documentation
- Published to crates.io (optional)

---

## Key Implementation Notes

### 1. Schema Generation is Automatic

When you define a struct with `#[derive(JsonSchema)]`, TurboMCP automatically generates the JSON schema. You don't need to manually write schema definitions.

### 2. Tool Registration is Automatic

The `#[tool]` macro automatically registers tools. You just implement methods in the server impl block.

### 3. Error Conversion

Always convert API errors to `McpError`:

```rust
.map_err(|e| McpError::internal(format!("Operation failed: {}", e)))?
```

### 4. Async All the Way

All tool handlers must be `async fn` and return `McpResult<T>`.

### 5. Context Usage

Use `Context` parameter for logging, request metadata:

```rust
async fn my_tool(&self, ctx: Context, param: String) -> McpResult<String> {
    ctx.info("Processing request").await?;
    // ... implementation
}
```

---

## Quick Reference

### Common Patterns

```rust
// Simple tool
#[tool("Description")]
async fn tool_name(&self, param: String) -> McpResult<String> {
    Ok(result)
}

// Tool with context
#[tool("Description")]
async fn tool_name(&self, ctx: Context, param: String) -> McpResult<String> {
    ctx.info("Log message").await?;
    Ok(result)
}

// Tool with structured params
#[tool("Description")]
async fn tool_name(&self, params: MyParams) -> McpResult<String> {
    Ok(result)
}

// Resource handler
#[resource("scheme://path/{param}")]
async fn resource_name(&self, param: String) -> McpResult<String> {
    Ok(result)
}

// API client call
let result = self.client
    .some_method(args)
    .await
    .map_err(|e| McpError::internal(e.to_string()))?;
```

### Error Types

```rust
McpError::invalid_request(msg)  // 400 - Bad request
McpError::unauthorized(msg)      // 401 - Auth failed
McpError::not_found(msg)         // 404 - Not found
McpError::internal(msg)          // 500 - Server error
McpError::protocol(msg)          // Protocol violation
```

### Schemars Attributes

```rust
#[schemars(range(min = 1, max = 100))]
#[schemars(length(min = 1, max = 255))]
#[schemars(regex = "pattern")]
#[schemars(default)]
```

---

## Conclusion

This guide provides a comprehensive blueprint for implementing the PhotoPrism MCP server in Rust using TurboMCP. The key advantages:

1. **Zero Boilerplate**: Macros handle all the MCP protocol details
2. **Type Safety**: Compile-time validation of parameters and schemas
3. **Performance**: Rust's efficiency + async runtime
4. **Maintainability**: Clear separation of concerns

Follow the roadmap sequentially, starting with Phase 1 to establish the foundation, then progressively adding features in subsequent phases.
