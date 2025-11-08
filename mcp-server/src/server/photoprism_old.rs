use std::sync::Arc;
use turbomcp::prelude::*;
use crate::client::PhotoPrismClient;
use crate::config::Config;
use crate::types::*;

/// Main PhotoPrism MCP Server
#[derive(Clone)]
pub struct PhotoPrismServer {
    /// PhotoPrism API client
    client: Arc<PhotoPrismClient>,
}

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
        })
    }

    /// Run server with stdio transport
    pub async fn run_stdio(self) -> Result<(), anyhow::Error> {
        tracing::info!("Starting PhotoPrism MCP server with STDIO transport");

        let server = turbomcp::Server::new(
            "photoprism",
            "0.1.0",
        );

        // Register all tools
        let server = self.register_tools(server);

        // Register all resources
        let server = self.register_resources(server);

        server.run_stdio().await?;
        Ok(())
    }

    /// Run server with HTTP transport
    pub async fn run_http(self, addr: &str) -> Result<(), anyhow::Error> {
        tracing::info!("Starting PhotoPrism MCP server with HTTP transport on {}", addr);

        let server = turbomcp::Server::new(
            "photoprism",
            "0.1.0",
        );

        // Register all tools
        let server = self.register_tools(server);

        // Register all resources
        let server = self.register_resources(server);

        server.run_http(addr).await?;
        Ok(())
    }

    /// Register all tools with the server
    fn register_tools(self, mut server: turbomcp::Server) -> turbomcp::Server {
        let client = self.client.clone();

        // Photo tools
        server.add_tool(
            "get_photo",
            "Get photo details by UID",
            {
                let client = client.clone();
                move |ctx, params: serde_json::Value| {
                    let client = client.clone();
                    Box::pin(async move {
                        let uid = params.get("uid")
                            .and_then(|v| v.as_str())
                            .ok_or_else(|| McpError::invalid_request("Missing uid parameter"))?
                            .to_string();

                        let photo = client.get_photo(&uid).await
                            .map_err(|e| McpError::internal(e.to_string()))?;

                        Ok(serde_json::to_string_pretty(&photo)?)
                    })
                }
            }
        );

        server.add_tool(
            "update_photo",
            "Update photo metadata",
            {
                let client = client.clone();
                move |ctx, params: serde_json::Value| {
                    let client = client.clone();
                    Box::pin(async move {
                        let uid = params.get("uid")
                            .and_then(|v| v.as_str())
                            .ok_or_else(|| McpError::invalid_request("Missing uid parameter"))?
                            .to_string();

                        let updates: PhotoUpdate = serde_json::from_value(params)?;

                        let photo = client.update_photo(&uid, &updates).await
                            .map_err(|e| McpError::internal(e.to_string()))?;

                        Ok(serde_json::to_string_pretty(&photo)?)
                    })
                }
            }
        );

        // Search tools
        server.add_tool(
            "search_photos",
            "Search photos by text query",
            {
                let client = client.clone();
                move |ctx, params: serde_json::Value| {
                    let client = client.clone();
                    Box::pin(async move {
                        let query = params.get("query")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();

                        let count = params.get("count")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(100) as u32;

                        let photos = client.search_photos(&query, count).await
                            .map_err(|e| McpError::internal(e.to_string()))?;

                        Ok(serde_json::to_string_pretty(&photos)?)
                    })
                }
            }
        );

        // Album tools
        server.add_tool(
            "list_albums",
            "List all albums",
            {
                let client = client.clone();
                move |ctx, params: serde_json::Value| {
                    let client = client.clone();
                    Box::pin(async move {
                        let albums = client.list_albums().await
                            .map_err(|e| McpError::internal(e.to_string()))?;

                        Ok(serde_json::to_string_pretty(&albums)?)
                    })
                }
            }
        );

        server.add_tool(
            "get_album",
            "Get album details by UID",
            {
                let client = client.clone();
                move |ctx, params: serde_json::Value| {
                    let client = client.clone();
                    Box::pin(async move {
                        let uid = params.get("uid")
                            .and_then(|v| v.as_str())
                            .ok_or_else(|| McpError::invalid_request("Missing uid parameter"))?
                            .to_string();

                        let album = client.get_album(&uid).await
                            .map_err(|e| McpError::internal(e.to_string()))?;

                        Ok(serde_json::to_string_pretty(&album)?)
                    })
                }
            }
        );

        server.add_tool(
            "create_album",
            "Create a new album",
            {
                let client = client.clone();
                move |ctx, params: serde_json::Value| {
                    let client = client.clone();
                    Box::pin(async move {
                        let create: AlbumCreate = serde_json::from_value(params)?;

                        let album = client.create_album(&create).await
                            .map_err(|e| McpError::internal(e.to_string()))?;

                        Ok(serde_json::to_string_pretty(&album)?)
                    })
                }
            }
        );

        server.add_tool(
            "add_photos_to_album",
            "Add photos to an album",
            {
                let client = client.clone();
                move |ctx, params: serde_json::Value| {
                    let client = client.clone();
                    Box::pin(async move {
                        let album_uid = params.get("album_uid")
                            .and_then(|v| v.as_str())
                            .ok_or_else(|| McpError::invalid_request("Missing album_uid parameter"))?
                            .to_string();

                        let photo_uids: Vec<String> = params.get("photo_uids")
                            .and_then(|v| v.as_array())
                            .ok_or_else(|| McpError::invalid_request("Missing photo_uids parameter"))?
                            .iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect();

                        client.add_photos_to_album(&album_uid, &photo_uids).await
                            .map_err(|e| McpError::internal(e.to_string()))?;

                        Ok(format!("Added {} photos to album {}", photo_uids.len(), album_uid))
                    })
                }
            }
        );

        // Label tools
        server.add_tool(
            "list_labels",
            "List all labels/tags",
            {
                let client = client.clone();
                move |ctx, params: serde_json::Value| {
                    let client = client.clone();
                    Box::pin(async move {
                        let labels = client.list_labels().await
                            .map_err(|e| McpError::internal(e.to_string()))?;

                        Ok(serde_json::to_string_pretty(&labels)?)
                    })
                }
            }
        );

        server.add_tool(
            "get_photos_by_label",
            "Get photos with a specific label",
            {
                let client = client.clone();
                move |ctx, params: serde_json::Value| {
                    let client = client.clone();
                    Box::pin(async move {
                        let label = params.get("label")
                            .and_then(|v| v.as_str())
                            .ok_or_else(|| McpError::invalid_request("Missing label parameter"))?
                            .to_string();

                        let count = params.get("count")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(100) as u32;

                        let photos = client.get_photos_by_label(&label, count).await
                            .map_err(|e| McpError::internal(e.to_string()))?;

                        Ok(serde_json::to_string_pretty(&photos)?)
                    })
                }
            }
        );

        // Subject tools
        server.add_tool(
            "list_subjects",
            "List all subjects/people",
            {
                let client = client.clone();
                move |ctx, params: serde_json::Value| {
                    let client = client.clone();
                    Box::pin(async move {
                        let subjects = client.list_subjects().await
                            .map_err(|e| McpError::internal(e.to_string()))?;

                        Ok(serde_json::to_string_pretty(&subjects)?)
                    })
                }
            }
        );

        server.add_tool(
            "get_subject_photos",
            "Get photos of a specific subject/person",
            {
                let client = client.clone();
                move |ctx, params: serde_json::Value| {
                    let client = client.clone();
                    Box::pin(async move {
                        let subject_uid = params.get("subject_uid")
                            .and_then(|v| v.as_str())
                            .ok_or_else(|| McpError::invalid_request("Missing subject_uid parameter"))?
                            .to_string();

                        let count = params.get("count")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(100) as u32;

                        let photos = client.get_photos_by_subject(&subject_uid, count).await
                            .map_err(|e| McpError::internal(e.to_string()))?;

                        Ok(serde_json::to_string_pretty(&photos)?)
                    })
                }
            }
        );

        // Library tools
        server.add_tool(
            "get_status",
            "Get PhotoPrism library status",
            {
                let client = client.clone();
                move |ctx, params: serde_json::Value| {
                    let client = client.clone();
                    Box::pin(async move {
                        let status = client.get_status().await
                            .map_err(|e| McpError::internal(e.to_string()))?;

                        Ok(serde_json::to_string_pretty(&status)?)
                    })
                }
            }
        );

        server
    }

    /// Register all resources with the server
    fn register_resources(self, mut server: turbomcp::Server) -> turbomcp::Server {
        let client = self.client.clone();

        // Photo resources
        server.add_resource(
            "photoprism://photos/recent",
            "Recent photos",
            {
                let client = client.clone();
                move || {
                    let client = client.clone();
                    Box::pin(async move {
                        let photos = client.search_photos("", 20).await
                            .map_err(|e| McpError::internal(e.to_string()))?;

                        Ok(serde_json::to_string_pretty(&photos)?)
                    })
                }
            }
        );

        server.add_resource(
            "photoprism://photos/favorites",
            "Favorite photos",
            {
                let client = client.clone();
                move || {
                    let client = client.clone();
                    Box::pin(async move {
                        let photos = client.search_photos("favorite:true", 100).await
                            .map_err(|e| McpError::internal(e.to_string()))?;

                        Ok(serde_json::to_string_pretty(&photos)?)
                    })
                }
            }
        );

        // Album resources
        server.add_resource(
            "photoprism://albums/list",
            "All albums",
            {
                let client = client.clone();
                move || {
                    let client = client.clone();
                    Box::pin(async move {
                        let albums = client.list_albums().await
                            .map_err(|e| McpError::internal(e.to_string()))?;

                        Ok(serde_json::to_string_pretty(&albums)?)
                    })
                }
            }
        );

        // System resources
        server.add_resource(
            "photoprism://system/status",
            "System status",
            {
                let client = client.clone();
                move || {
                    let client = client.clone();
                    Box::pin(async move {
                        let status = client.get_status().await
                            .map_err(|e| McpError::internal(e.to_string()))?;

                        Ok(serde_json::to_string_pretty(&status)?)
                    })
                }
            }
        );

        server
    }
}
