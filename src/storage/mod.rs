use async_trait::async_trait;
use crate::error::Result;
use crate::models::{Document, DocumentBatch};

pub mod memory;
pub mod sqlite;

#[cfg(feature = "libsql")]
pub mod libsql;

/// Storage backend types
#[derive(Debug, Clone)]
pub enum StorageBackend {
    Memory,
    Sqlite(String),
    #[cfg(feature = "libsql")]
    LibSql(String),
}

/// Storage statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub total_documents: usize,
    pub total_batches: usize,
    pub storage_backend: String,
}

/// Storage trait for document persistence
#[async_trait]
pub trait Storage: Send + Sync {
    /// Store a document
    async fn store_document(&self, document: &Document) -> Result<()>;
    
    /// Retrieve a document by ID
    async fn retrieve_document(&self, id: &str) -> Result<Document>;
    
    /// List documents with optional limit
    async fn list_documents(&self, limit: Option<usize>) -> Result<Vec<Document>>;
    
    /// Delete a document by ID
    async fn delete_document(&self, id: &str) -> Result<()>;
    
    /// Store a document batch
    async fn store_batch(&self, batch: &DocumentBatch) -> Result<()>;
    
    /// Retrieve a document batch by ID
    async fn retrieve_batch(&self, id: &str) -> Result<DocumentBatch>;
    
    /// Get storage statistics
    async fn get_stats(&self) -> Result<StorageStats>;
} 