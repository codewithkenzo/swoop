// Test utilities - imports are conditionally needed
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use axum::Router;
#[allow(unused_imports)]
use tokio::sync::RwLock;

// Unused imports removed - add back when needed for tests
// use crate::api_server::{create_router, SystemStats};
use crate::crawler::CrawlerBuilder;
use crate::document_processor::DocumentProcessor;
use crate::llm::LLMService;
use crate::models::DocumentWorkspace;
#[cfg(feature = "libsql")]
use crate::storage::libsql::LibSqlStorage;
#[cfg(not(feature = "libsql"))]
use crate::storage::sqlite::SqliteStorage;
use crate::storage::Storage;
use crate::common::ApiResponse; // keep unused for now but maybe needed

/// Build an Axum router wired with an in-memory state suitable for integration tests.
/// This avoids external network calls by using the stub LLM service and an in-memory
/// libSQL database (file:memory).
#[cfg(test)]
pub async fn build_test_router() -> Router {
    // Prepare workspace and storage
    let workspace = DocumentWorkspace::default();

    // In-memory database - use libsql if available, otherwise sqlite
    #[cfg(feature = "libsql")]
    let storage = {
    let db_url = "file:test.db?mode=memory&cache=shared";
        LibSqlStorage::new(db_url, None)
        .await
            .expect("failed to init in-memory libsql storage")
    };
    
    #[cfg(not(feature = "libsql"))]
    let storage = {
        SqliteStorage::new(":memory:")
            .await
            .expect("failed to init in-memory sqlite storage")
    };

    // Build crawler with memory storage wrapper
    let storage_arc: Arc<dyn Storage> = Arc::new(storage.clone());
    let crawler = CrawlerBuilder::new()
        .with_storage(storage_arc)
        .build()
        .expect("failed to build crawler");

    // Stub LLM service (no network)
    let llm_service = LLMService::new_stub().await;

    let app_state = crate::api_server::AppState {
        workspace: Arc::new(workspace),
        documents: Arc::new(RwLock::new(HashMap::new())),
        conversations: Arc::new(RwLock::new(HashMap::new())),
        system_stats: Arc::new(RwLock::new(SystemStats {
            documents_processed: 0,
            documents_pending: 0,
            average_quality_score: 0.0,
            processing_rate_per_hour: 0.0,
            error_rate: 0.0,
            uptime_seconds: 0,
            memory_usage_mb: 0.0,
        })),
        llm_service: Arc::new(llm_service),
        crawler: Arc::new(crawler),
        processor: Arc::new(DocumentProcessor::new(None)),
        storage: Arc::new(storage),
    };

    // Insert a sample pending document so /api/documents/{id}/stream yields events in tests
    {
        use crate::api_server::{DocumentProcessingStatus, ProcessingStatus};
        use chrono::Utc;
        let mut docs = app_state.documents.write().await;
        docs.insert("test-doc-1".to_string(), DocumentProcessingStatus {
            id: "test-doc-1".to_string(),
            filename: "sample.txt".to_string(),
            status: ProcessingStatus::Pending,
            progress: 0.0,
            quality_score: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            processing_time_ms: None,
            error_message: None,
        });
    }

    create_router(app_state)
} 