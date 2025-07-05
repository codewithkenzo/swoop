/*!
 * Storage factory integration tests
 * 
 * These tests verify that the storage factory correctly creates
 * different storage backends based on configuration.
 */

use swoop::storage::create_storage;
use swoop::config::Config;
use swoop::models::Document;

#[tokio::test]
async fn test_memory_storage_factory() {
    let mut config = Config::default();
    config.storage.backend = "memory".to_string();
    
    let storage = create_storage(&config).await.unwrap();
    
    // Test that it's working
    assert!(storage.health_check().await.unwrap());
    
    // Test basic operations
    let doc = Document::new("Test Doc", "Test content");
    storage.store_document(&doc).await.unwrap();
    
    let retrieved = storage.retrieve_document(&doc.id).await.unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().title, "Test Doc");
}

#[tokio::test]
async fn test_config_from_env() {
    // Test without any env vars (should default to memory)
    let config = Config::from_env();
    assert_eq!(config.storage.backend, "memory");
    
    // Test with DATABASE_URL (should switch to postgres)
    std::env::set_var("DATABASE_URL", "postgresql://user:pass@localhost/test");
    let config = Config::from_env();
    assert_eq!(config.storage.backend, "postgres");
    assert!(config.storage.database_url.is_some());
    
    // Test with SQLITE_PATH (should switch to sqlite)
    std::env::remove_var("DATABASE_URL");
    std::env::set_var("SQLITE_PATH", "/tmp/test.db");
    let config = Config::from_env();
    assert_eq!(config.storage.backend, "sqlite");
    assert!(config.storage.sqlite_path.is_some());
    
    // Cleanup
    std::env::remove_var("SQLITE_PATH");
}

#[tokio::test]
async fn test_fallback_behavior() {
    let mut config = Config::default();
    
    // Test postgres without DATABASE_URL (should fallback to memory)
    config.storage.backend = "postgres".to_string();
    config.storage.database_url = None;
    
    let storage = create_storage(&config).await.unwrap();
    assert!(storage.health_check().await.unwrap());
    
    // Test unknown backend (should fallback to memory)
    config.storage.backend = "unknown_backend".to_string();
    let storage = create_storage(&config).await.unwrap();
    assert!(storage.health_check().await.unwrap());
}

#[cfg(feature = "postgres")]
#[tokio::test]
async fn test_postgres_storage_factory() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://swoop_user:swoop_pass@localhost:5432/swoop_db".to_string());
    
    let mut config = Config::default();
    config.storage.backend = "postgres".to_string();
    config.storage.database_url = Some(database_url);
    
    // This will only work if PostgreSQL is available
    match create_storage(&config).await {
        Ok(storage) => {
            // Test that PostgreSQL storage is working
            let health = storage.health_check().await.unwrap();
            if health {
                println!("PostgreSQL storage factory test: SUCCESS");
                
                // Test basic operations
                let doc = Document::new("Postgres Test Doc", "Postgres test content");
                storage.store_document(&doc).await.unwrap();
                
                let retrieved = storage.retrieve_document(&doc.id).await.unwrap();
                assert!(retrieved.is_some());
                assert_eq!(retrieved.unwrap().title, "Postgres Test Doc");
                
                // Cleanup
                storage.delete_document(&doc.id).await.unwrap();
            } else {
                println!("PostgreSQL storage factory test: Database not healthy, skipping");
            }
        }
        Err(_) => {
            println!("PostgreSQL storage factory test: Database not available, skipping");
        }
    }
}