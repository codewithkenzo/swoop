use swoop::{config::{Config, StorageConfig}, storage::{create_storage, Storage}, models::Document};

/// Integration test: store a document in SQLite (in-memory) and retrieve it back.
#[tokio::test]
async fn sqlite_store_and_retrieve_document() {
    // Build a config that instructs storage factory to use SQLite with an in-memory DB
    let mut config = Config::default();
    config.storage = StorageConfig {
        backend: "sqlite".to_string(),
        sqlite_path: Some(":memory:".to_string()),
        ..StorageConfig::default()
    };

    // Create the storage backend
    let storage = create_storage(&config).await.expect("create storage");

    // Create a new document
    let doc = Document::new("Test Doc", "Hello world! This is a test document.");

    // Persist
    storage.store_document(&doc).await.expect("store document");

    // Retrieve
    let fetched = storage
        .retrieve_document(&doc.id)
        .await
        .expect("retrieve document")
        .expect("document should exist");

    assert_eq!(fetched.id, doc.id);
    assert_eq!(fetched.title, doc.title);
    assert_eq!(fetched.content, doc.content);
} 