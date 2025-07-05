/*!
 * PostgreSQL storage backend integration tests
 * 
 * These tests verify that the PostgreSQL storage implementation correctly
 * stores and retrieves documents, batches, and other data structures.
 */

#[cfg(feature = "postgres")]
mod postgres_tests {
    use swoop::storage::postgres::PostgresStorage;
    use swoop::storage::Storage;
    use swoop::models::{Document, DocumentBatch, CrawlPage, VectorRecord};
    use chrono::Utc;
    use std::collections::HashMap;

    /// Get test database URL from environment or use default
    fn get_test_database_url() -> String {
        std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://swoop_user:swoop_pass@localhost:5432/swoop_db".to_string())
    }

    /// Create a test document
    fn create_test_document() -> Document {
        Document {
            id: "test_doc_1".to_string(),
            title: "Test Document".to_string(),
            content: "This is test content for the PostgreSQL storage test.".to_string(),
            summary: Some("Test summary".to_string()),
            quality_score: Some(0.95),
            content_hash: Some("abc123".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            source_url: Some("https://example.com/test.pdf".to_string()),
            document_type: Some("pdf".to_string()),
            language: Some("en".to_string()),
            word_count: Some(10),
            size_bytes: Some(1024),
            content_type: Some("application/pdf".to_string()),
            file_size: Some(2048),
            extracted_at: Utc::now(),
            metadata: swoop::models::Metadata {
                source_url: Some("https://example.com/test.pdf".to_string()),
                content_type: Some("application/pdf".to_string()),
                processed_at: Utc::now(),
                processor: Some("test_processor".to_string()),
                custom: {
                    let mut map = HashMap::new();
                    map.insert("test_key".to_string(), "test_value".to_string());
                    map
                },
                file_extension: Some("pdf".to_string()),
                original_filename: Some("test.pdf".to_string()),
            },
        }
    }

    #[tokio::test]
    async fn test_postgres_connection() {
        let database_url = get_test_database_url();
        
        // Skip test if no PostgreSQL available
        let storage = match PostgresStorage::new(&database_url).await {
            Ok(storage) => storage,
            Err(_) => {
                println!("Skipping PostgreSQL tests - database not available");
                return;
            }
        };

        // Initialize tables
        assert!(storage.initialize().await.is_ok());

        // Test health check
        let health = storage.health_check().await.unwrap();
        assert!(health, "PostgreSQL health check should pass");
    }

    #[tokio::test]
    async fn test_document_round_trip() {
        let database_url = get_test_database_url();
        
        let storage = match PostgresStorage::new(&database_url).await {
            Ok(storage) => storage,
            Err(_) => {
                println!("Skipping PostgreSQL document test - database not available");
                return;
            }
        };

        // Initialize tables
        storage.initialize().await.unwrap();

        let document = create_test_document();
        let doc_id = document.id.clone();

        // Store document
        assert!(storage.store_document(&document).await.is_ok());

        // Retrieve document
        let retrieved = storage.retrieve_document(&doc_id).await.unwrap();
        assert!(retrieved.is_some());

        let retrieved_doc = retrieved.unwrap();
        assert_eq!(retrieved_doc.id, document.id);
        assert_eq!(retrieved_doc.title, document.title);
        assert_eq!(retrieved_doc.content, document.content);
        assert_eq!(retrieved_doc.summary, document.summary);
        assert_eq!(retrieved_doc.quality_score, document.quality_score);
        assert_eq!(retrieved_doc.document_type, document.document_type);

        // List documents
        let doc_ids = storage.list_documents().await.unwrap();
        assert!(doc_ids.contains(&doc_id));

        // Delete document
        assert!(storage.delete_document(&doc_id).await.is_ok());

        // Verify deletion
        let deleted = storage.retrieve_document(&doc_id).await.unwrap();
        assert!(deleted.is_none());
    }

    #[tokio::test]
    async fn test_document_batch_operations() {
        let database_url = get_test_database_url();
        
        let storage = match PostgresStorage::new(&database_url).await {
            Ok(storage) => storage,
            Err(_) => {
                println!("Skipping PostgreSQL batch test - database not available");
                return;
            }
        };

        storage.initialize().await.unwrap();

        let batch = DocumentBatch {
            id: "test_batch_1".to_string(),
            document_ids: vec!["doc1".to_string(), "doc2".to_string(), "doc3".to_string()],
            total_documents: 3,
            status: "processing".to_string(),
            created_at: Utc::now(),
        };

        // Store batch
        assert!(storage.store_batch(&batch).await.is_ok());

        // Retrieve batch
        let retrieved = storage.retrieve_batch(&batch.id).await.unwrap();
        assert!(retrieved.is_some());

        let retrieved_batch = retrieved.unwrap();
        assert_eq!(retrieved_batch.id, batch.id);
        assert_eq!(retrieved_batch.document_ids, batch.document_ids);
        assert_eq!(retrieved_batch.total_documents, batch.total_documents);
        assert_eq!(retrieved_batch.status, batch.status);
    }

    #[tokio::test]
    async fn test_crawl_page_operations() {
        let database_url = get_test_database_url();
        
        let storage = match PostgresStorage::new(&database_url).await {
            Ok(storage) => storage,
            Err(_) => {
                println!("Skipping PostgreSQL crawl page test - database not available");
                return;
            }
        };

        storage.initialize().await.unwrap();

        let crawl_page = CrawlPage {
            id: "page_1".to_string(),
            job_id: "job_1".to_string(),
            url: "https://example.com/page1".to_string(),
            status_code: 200,
            text_length: 1500,
            fetched_at: Utc::now(),
        };

        // Store crawl page
        assert!(storage.store_crawl_page(&crawl_page).await.is_ok());

        // List crawl pages for job
        let pages = storage.list_crawl_pages("job_1", 0, 10).await.unwrap();
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].id, crawl_page.id);
        assert_eq!(pages[0].url, crawl_page.url);
        assert_eq!(pages[0].status_code, crawl_page.status_code);
    }

    #[tokio::test]
    async fn test_vector_record_operations() {
        let database_url = get_test_database_url();
        
        let storage = match PostgresStorage::new(&database_url).await {
            Ok(storage) => storage,
            Err(_) => {
                println!("Skipping PostgreSQL vector test - database not available");
                return;
            }
        };

        storage.initialize().await.unwrap();

        // First create a document to reference
        let document = create_test_document();
        storage.store_document(&document).await.unwrap();

        let vector_record = VectorRecord {
            id: "vector_1".to_string(),
            document_id: document.id.clone(),
            vector: vec![0.1, 0.2, 0.3, 0.4, 0.5],
            metadata: {
                let mut map = HashMap::new();
                map.insert("model".to_string(), "text-embedding-3-small".to_string());
                map
            },
            created_at: Utc::now(),
        };

        // Store vector record
        assert!(storage.store_document_vector(&vector_record).await.is_ok());

        // Clean up
        storage.delete_document(&document.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_upsert_operations() {
        let database_url = get_test_database_url();
        
        let storage = match PostgresStorage::new(&database_url).await {
            Ok(storage) => storage,
            Err(_) => {
                println!("Skipping PostgreSQL upsert test - database not available");
                return;
            }
        };

        storage.initialize().await.unwrap();

        let mut document = create_test_document();
        document.id = "upsert_test_doc".to_string();

        // Store document first time
        storage.store_document(&document).await.unwrap();

        // Modify and store again (should update)
        document.title = "Updated Test Document".to_string();
        document.content = "Updated content".to_string();
        
        storage.store_document(&document).await.unwrap();

        // Retrieve and verify update
        let retrieved = storage.retrieve_document(&document.id).await.unwrap();
        assert!(retrieved.is_some());
        
        let retrieved_doc = retrieved.unwrap();
        assert_eq!(retrieved_doc.title, "Updated Test Document");
        assert_eq!(retrieved_doc.content, "Updated content");

        // Clean up
        storage.delete_document(&document.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_pagination() {
        let database_url = get_test_database_url();
        
        let storage = match PostgresStorage::new(&database_url).await {
            Ok(storage) => storage,
            Err(_) => {
                println!("Skipping PostgreSQL pagination test - database not available");
                return;
            }
        };

        storage.initialize().await.unwrap();

        // Create multiple crawl pages
        let job_id = "pagination_test_job";
        for i in 0..15 {
            let page = CrawlPage {
                id: format!("page_{}", i),
                job_id: job_id.to_string(),
                url: format!("https://example.com/page{}", i),
                status_code: 200,
                text_length: 1000 + i,
                fetched_at: Utc::now(),
            };
            storage.store_crawl_page(&page).await.unwrap();
        }

        // Test pagination
        let first_page = storage.list_crawl_pages(job_id, 0, 5).await.unwrap();
        assert_eq!(first_page.len(), 5);

        let second_page = storage.list_crawl_pages(job_id, 5, 5).await.unwrap();
        assert_eq!(second_page.len(), 5);

        let third_page = storage.list_crawl_pages(job_id, 10, 5).await.unwrap();
        assert_eq!(third_page.len(), 5);

        let fourth_page = storage.list_crawl_pages(job_id, 15, 5).await.unwrap();
        assert_eq!(fourth_page.len(), 0);
    }
}

// Export the tests so they can be run
#[cfg(not(feature = "postgres"))]
mod postgres_tests {
    #[test]
    fn postgres_feature_disabled() {
        println!("PostgreSQL tests skipped - 'postgres' feature not enabled");
    }
}