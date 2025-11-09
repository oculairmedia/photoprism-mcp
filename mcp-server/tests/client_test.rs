use mockito::Server;
use photoprism_mcp::PhotoPrismClient;

#[tokio::test]
async fn test_authentication_success() {
    let mut server = Server::new_async().await;

    // Mock successful authentication
    let _auth_mock = server
        .mock("POST", "/api/v1/session")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"test-session-token-12345"}"#)
        .create_async()
        .await;

    let client = PhotoPrismClient::new(
        server.url(),
        "test_user".to_string(),
        "test_password".to_string(),
    );

    assert!(client.is_ok());
}

#[tokio::test]
async fn test_authentication_failure() {
    let mut server = Server::new_async().await;

    // Mock failed authentication
    let _auth_mock = server
        .mock("POST", "/api/v1/session")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error":"invalid credentials"}"#)
        .create_async()
        .await;

    let client = PhotoPrismClient::new(
        server.url(),
        "test_user".to_string(),
        "wrong_password".to_string(),
    )
    .expect("Client creation should succeed");

    // Try to search photos (which will trigger authentication)
    let result = client.search_photos("test", 10).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_search_photos() {
    let mut server = Server::new_async().await;

    // Mock authentication
    let _auth_mock = server
        .mock("POST", "/api/v1/session")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"test-token"}"#)
        .create_async()
        .await;

    // Mock search endpoint
    let _search_mock = server
        .mock("GET", "/api/v1/photos?q=sunset&count=10")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"[
            {
                "UID": "abc123def4567890",
                "Title": "Beautiful Sunset",
                "Description": "A sunset photo",
                "OriginalName": "sunset.jpg",
                "Type": "image",
                "Favorite": false,
                "Private": false,
                "Hash": "hash123",
                "Width": 1920,
                "Height": 1080
            }
        ]"#,
        )
        .create_async()
        .await;

    let client =
        PhotoPrismClient::new(server.url(), "test".to_string(), "test".to_string()).unwrap();

    let result = client.search_photos("sunset", 10).await;
    assert!(result.is_ok());

    let photos = result.unwrap();
    assert_eq!(photos.len(), 1);
    assert_eq!(photos[0].title, "Beautiful Sunset");
}

#[tokio::test]
async fn test_get_photo() {
    let mut server = Server::new_async().await;

    // Mock authentication
    let _auth_mock = server
        .mock("POST", "/api/v1/session")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"test-token"}"#)
        .create_async()
        .await;

    // Mock get photo endpoint
    let _get_mock = server
        .mock("GET", "/api/v1/photos/abc123def4567890")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "UID": "abc123def4567890",
            "Title": "Test Photo",
            "Description": "A test photo",
            "OriginalName": "test.jpg",
            "Type": "image",
            "Favorite": true,
            "Private": false,
            "Hash": "hash456",
            "Width": 2048,
            "Height": 1536
        }"#,
        )
        .create_async()
        .await;

    let client =
        PhotoPrismClient::new(server.url(), "test".to_string(), "test".to_string()).unwrap();

    let result = client.get_photo("abc123def4567890").await;
    assert!(result.is_ok());

    let photo = result.unwrap();
    assert_eq!(photo.uid, "abc123def4567890");
    assert_eq!(photo.title, "Test Photo");
    assert!(photo.favorite);
}

#[tokio::test]
async fn test_get_photo_not_found() {
    let mut server = Server::new_async().await;

    // Mock authentication
    let _auth_mock = server
        .mock("POST", "/api/v1/session")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"test-token"}"#)
        .create_async()
        .await;

    // Mock 404 response
    let _get_mock = server
        .mock("GET", "/api/v1/photos/nonexistent123")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error":"photo not found"}"#)
        .create_async()
        .await;

    let client =
        PhotoPrismClient::new(server.url(), "test".to_string(), "test".to_string()).unwrap();

    let result = client.get_photo("nonexistent123").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_albums() {
    let mut server = Server::new_async().await;

    // Mock authentication
    let _auth_mock = server
        .mock("POST", "/api/v1/session")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"test-token"}"#)
        .create_async()
        .await;

    // Mock albums endpoint
    let _albums_mock = server
        .mock("GET", "/api/v1/albums?count=1000")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"[
            {
                "UID": "album123",
                "Title": "Vacation 2024",
                "Description": "Summer vacation photos",
                "Type": "album",
                "Favorite": true,
                "PhotoCount": 42
            },
            {
                "UID": "album456",
                "Title": "Family",
                "Type": "album",
                "Favorite": false,
                "PhotoCount": 128
            }
        ]"#,
        )
        .create_async()
        .await;

    let client =
        PhotoPrismClient::new(server.url(), "test".to_string(), "test".to_string()).unwrap();

    let result = client.list_albums().await;
    assert!(result.is_ok());

    let albums = result.unwrap();
    assert_eq!(albums.len(), 2);
    assert_eq!(albums[0].title, "Vacation 2024");
    assert!(albums[0].favorite);
    assert_eq!(albums[1].title, "Family");
}

#[tokio::test]
async fn test_create_album() {
    let mut server = Server::new_async().await;

    // Mock authentication
    let _auth_mock = server
        .mock("POST", "/api/v1/session")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"test-token"}"#)
        .create_async()
        .await;

    // Mock create album endpoint
    let _create_mock = server
        .mock("POST", "/api/v1/albums")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{
            "UID": "newalbum789",
            "Title": "New Album",
            "Description": "A brand new album",
            "Type": "album",
            "Favorite": false,
            "PhotoCount": 0
        }"#,
        )
        .create_async()
        .await;

    let client =
        PhotoPrismClient::new(server.url(), "test".to_string(), "test".to_string()).unwrap();

    let create = photoprism_mcp::types::AlbumCreate {
        title: "New Album".to_string(),
        description: Some("A brand new album".to_string()),
        favorite: false,
    };

    let result = client.create_album(&create).await;
    assert!(result.is_ok());

    let album = result.unwrap();
    assert_eq!(album.uid, "newalbum789");
    assert_eq!(album.title, "New Album");
}

#[tokio::test]
async fn test_list_labels() {
    let mut server = Server::new_async().await;

    // Mock authentication
    let _auth_mock = server
        .mock("POST", "/api/v1/session")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"test-token"}"#)
        .create_async()
        .await;

    // Mock labels endpoint
    let _labels_mock = server
        .mock("GET", "/api/v1/labels?count=1000")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"[
            {
                "uid": "label001",
                "name": "sunset",
                "photo_count": 25,
                "priority": 80
            },
            {
                "uid": "label002",
                "name": "beach",
                "photo_count": 18,
                "priority": 60
            }
        ]"#,
        )
        .create_async()
        .await;

    let client =
        PhotoPrismClient::new(server.url(), "test".to_string(), "test".to_string()).unwrap();

    let result = client.list_labels().await;
    assert!(result.is_ok());

    let labels = result.unwrap();
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0].name, "sunset");
    assert_eq!(labels[0].photo_count, 25);
}

#[tokio::test]
async fn test_list_subjects() {
    let mut server = Server::new_async().await;

    // Mock authentication
    let _auth_mock = server
        .mock("POST", "/api/v1/session")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"test-token"}"#)
        .create_async()
        .await;

    // Mock subjects endpoint
    let _subjects_mock = server
        .mock("GET", "/api/v1/subjects?count=1000")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"[
            {
                "uid": "subject001",
                "name": "John Doe",
                "photo_count": 52,
                "favorite": true
            },
            {
                "uid": "subject002",
                "name": "Jane Smith",
                "photo_count": 38,
                "favorite": false
            }
        ]"#,
        )
        .create_async()
        .await;

    let client =
        PhotoPrismClient::new(server.url(), "test".to_string(), "test".to_string()).unwrap();

    let result = client.list_subjects().await;
    assert!(result.is_ok());

    let subjects = result.unwrap();
    assert_eq!(subjects.len(), 2);
    assert_eq!(subjects[0].name, "John Doe");
    assert!(subjects[0].favorite);
}
