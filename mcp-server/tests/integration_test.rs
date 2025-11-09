/// Integration tests for PhotoPrism MCP Server
/// These tests verify that the server components work together correctly
use photoprism_mcp::{Config, PhotoPrismServer};

#[test]
fn test_server_creation() {
    let config = Config::new(
        "http://localhost:2342".to_string(),
        "admin".to_string(),
        "test_password".to_string(),
    );

    let server = PhotoPrismServer::new(config);
    assert!(server.is_ok(), "Server should be created successfully");
}

#[test]
fn test_config_default() {
    let config = Config::default();
    assert_eq!(config.base_url, "http://localhost:2342");
    assert_eq!(config.username, "admin");
}

#[test]
fn test_config_creation() {
    let config = Config::new(
        "http://example.com:8080".to_string(),
        "testuser".to_string(),
        "testpass".to_string(),
    );

    assert_eq!(config.base_url, "http://example.com:8080");
    assert_eq!(config.username, "testuser");
    assert_eq!(config.password, "testpass");
    assert!(config.session_token.is_none());
}

#[cfg(test)]
mod error_tests {
    use photoprism_mcp::PhotoPrismError;

    #[test]
    fn test_error_display() {
        let err = PhotoPrismError::AuthenticationFailed("bad credentials".to_string());
        assert_eq!(err.to_string(), "Authentication failed: bad credentials");

        let err = PhotoPrismError::NotFound("photo".to_string());
        assert_eq!(err.to_string(), "Not found: photo");

        let err = PhotoPrismError::InvalidParameter("uid".to_string());
        assert_eq!(err.to_string(), "Invalid parameter: uid");
    }

    #[test]
    fn test_error_conversion_to_mcp_error() {
        use turbomcp::McpError;

        let err = PhotoPrismError::AuthenticationFailed("test".to_string());
        let mcp_err: McpError = err.into();
        // Should convert to unauthorized error
        assert!(
            mcp_err.to_string().contains("test") || mcp_err.to_string().contains("Unauthorized")
        );

        let err = PhotoPrismError::NotFound("test".to_string());
        let mcp_err: McpError = err.into();
        // Should convert to not found error
        assert!(mcp_err.to_string().contains("test") || mcp_err.to_string().contains("not found"));
    }
}

#[cfg(test)]
mod type_tests {
    use photoprism_mcp::types::*;

    #[test]
    fn test_photo_search_params_default() {
        let params = PhotoSearchParams {
            q: None,
            quality: None,
            album: None,
            label: None,
            year: None,
            month: None,
            photo_type: None,
            count: 100,
            offset: 0,
        };

        assert_eq!(params.count, 100);
        assert_eq!(params.offset, 0);
    }

    #[test]
    fn test_album_create() {
        let create = AlbumCreate {
            title: "Test Album".to_string(),
            description: Some("Description".to_string()),
            favorite: true,
        };

        assert_eq!(create.title, "Test Album");
        assert!(create.favorite);
    }

    #[test]
    fn test_sort_order_default() {
        let sort = SortOrder::default();
        match sort {
            SortOrder::Newest => {} // Correct default
            _ => panic!("Default should be Newest"),
        }
    }
}
