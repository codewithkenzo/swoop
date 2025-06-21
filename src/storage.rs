/*!
 * Storage module for Crawl4AI
 * 
 * This module provides storage abstractions and implementations for different backends
 * including document storage, vector storage, and caching layers.
 */

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::error::{Error, Result};
use crate::models::{Document, DocumentVector, DocumentBatch};

/// Core storage trait for document operations
#[async_trait]
pub trait Storage: Send + Sync {
    /// Store a document
    async fn store_document(&self, document: &Document) -> Result<()>;
    
    /// Retrieve a document by ID
    async fn get_document(&self, id: &str) -> Result<Option<Document>>;
    
    /// Update an existing document
    async fn update_document(&self, document: &Document) -> Result<()>;
    
    /// Delete a document by ID
    async fn delete_document(&self, id: &str) -> Result<()>;
    
    /// Search documents by query
    async fn search_documents(&self, query: &str, limit: Option<usize>) -> Result<Vec<Document>>;
    
    /// Store a batch of documents
    async fn store_batch(&self, batch: &DocumentBatch) -> Result<()>;
    
    /// Get storage statistics
    async fn get_stats(&self) -> Result<StorageStats>;
}

/// Vector storage trait for semantic search operations
#[async_trait]
pub trait VectorStorage: Send + Sync {
    /// Store a document vector
    async fn store_vector(&self, vector: &DocumentVector) -> Result<()>;
    
    /// Perform similarity search
    async fn similarity_search(&self, query_vector: &[f32], limit: usize) -> Result<Vec<DocumentVector>>;
    
    /// Store multiple vectors
    async fn store_vectors(&self, vectors: &[DocumentVector]) -> Result<()>;
    
    /// Delete a vector by document ID
    async fn delete_vector(&self, document_id: &str) -> Result<()>;
    
    /// Get vector by document ID
    async fn get_vector(&self, document_id: &str) -> Result<Option<DocumentVector>>;
}

/// Document storage trait for specialized document operations
#[async_trait]
pub trait DocumentStorage: Send + Sync {
    /// Store HTML content
    async fn store_html(&self, document_id: &str, html: &str) -> Result<()>;
    
    /// Store plain text content
    async fn store_text(&self, document_id: &str, text: &str) -> Result<()>;
    
    /// Store metadata
    async fn store_metadata(&self, document_id: &str, metadata: &HashMap<String, String>) -> Result<()>;
    
    /// Store screenshot
    async fn store_screenshot(&self, document_id: &str, screenshot: &[u8]) -> Result<()>;
    
    /// Retrieve stored content
    async fn get_content(&self, document_id: &str, content_type: ContentType) -> Result<Option<Vec<u8>>>;
}

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    /// Total number of documents
    pub total_documents: u64,
    /// Total storage size in bytes
    pub total_size_bytes: u64,
    /// Number of successful operations
    pub successful_operations: u64,
    /// Number of failed operations
    pub failed_operations: u64,
    /// Average operation time in milliseconds
    pub avg_operation_time_ms: f64,
}

/// Content type for document storage
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentType {
    Html,
    Text,
    Metadata,
    Screenshot,
}

/// Builder for creating storage instances
pub struct StorageBuilder {
    config: crate::config::StorageConfig,
}

impl StorageBuilder {
    /// Create a new StorageBuilder
    pub fn new() -> Self {
        Self {
            config: crate::config::StorageConfig::default(),
        }
    }
    
    /// Set the storage configuration
    pub fn with_config(mut self, config: crate::config::StorageConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Build the storage instance
    pub async fn build(self) -> Result<Arc<dyn Storage>> {
        match self.config.storage_type {
            crate::config::StorageType::Memory => {
                Ok(Arc::new(MemoryStorage::new()))
            },
            crate::config::StorageType::FileSystem => {
                let fs_storage = FileSystemStorage::new(&self.config.connection_string).await?;
                Ok(Arc::new(fs_storage))
            },
            crate::config::StorageType::SQLite => {
                let sqlite_storage = SqliteStorage::new(&self.config.connection_string).await?;
                Ok(Arc::new(sqlite_storage))
            },
            _ => Err(Error::Storage("Unsupported storage type".to_string())),
        }
    }
}

impl Default for StorageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Storage implementations
pub mod memory;
pub mod filesystem;
mod sqlite;

pub use memory::MemoryStorage;
pub use filesystem::FileSystemStorage;
pub use sqlite::SqliteStorage; 