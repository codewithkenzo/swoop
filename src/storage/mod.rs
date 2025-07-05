use async_trait::async_trait;
use crate::error::Result;
use crate::models::{Document, DocumentBatch};

pub mod memory;
pub mod sqlite;
pub mod filesystem;

#[cfg(feature = "libsql")]
pub mod libsql;

/// Storage backend types
#[derive(Debug, Clone)]
pub enum StorageBackend {
    Memory,
    Sqlite(String),
    FileSystem(String),
    #[cfg(feature = "libsql")]
    LibSql(String),
}

/// Storage statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub total_documents: usize,
    pub total_batches: usize,
    pub storage_backend: String,
    pub total_size_bytes: u64,
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub avg_operation_time_ms: f64,
}

impl Default for StorageStats {
    fn default() -> Self {
        Self {
            total_documents: 0,
            total_batches: 0,
            storage_backend: "unknown".to_string(),
            total_size_bytes: 0,
            successful_operations: 0,
            failed_operations: 0,
            avg_operation_time_ms: 0.0,
        }
    }
}

/// Storage trait for document persistence
#[async_trait]
pub trait Storage: Send + Sync {
    /// Store a document
    async fn store_document(&self, document: &Document) -> Result<()>;
    
    /// Retrieve a document by ID (returns Option)
    async fn retrieve_document(&self, id: &str) -> Result<Option<Document>>;
    
    /// List document IDs
    async fn list_documents(&self) -> Result<Vec<String>>;
    
    /// Delete a document by ID
    async fn delete_document(&self, id: &str) -> Result<()>;
    
    /// Store a document batch
    async fn store_batch(&self, batch: &DocumentBatch) -> Result<()>;
    
    /// Retrieve a document batch by ID (returns Option)
    async fn retrieve_batch(&self, id: &str) -> Result<Option<DocumentBatch>>;
    
    /// Health check for storage backend
    async fn health_check(&self) -> Result<bool>;

    /// Optional: store a crawl page record. Backends that don't care can keep default no-op.
    async fn store_crawl_page(&self, _page: &crate::models::CrawlPage) -> Result<()> {
        Ok(())
    }

    /// Store a document vector (for embeddings/AI features)
    async fn store_document_vector(&self, _vector_record: &crate::models::VectorRecord) -> Result<()> {
        Ok(())
    }

    /// List crawl pages for a job with pagination
    async fn list_crawl_pages(&self, _job_id: &str, _offset: usize, _limit: usize) -> Result<Vec<crate::models::CrawlPage>> {
        Ok(vec![])
    }
} 