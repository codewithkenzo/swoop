use async_trait::async_trait;
use crate::error::Result;
use crate::models::{Document, DocumentBatch};

pub mod memory;
pub mod sqlite;
pub mod filesystem;

#[cfg(feature = "libsql")]
pub mod libsql;

#[cfg(feature = "postgres")]
pub mod postgres;

/// Storage backend types
#[derive(Debug, Clone)]
pub enum StorageBackend {
    Memory,
    Sqlite(String),
    FileSystem(String),
    #[cfg(feature = "postgres")]
    Postgres(String),
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

/// Create a storage backend based on configuration
pub async fn create_storage(config: &crate::config::Config) -> Result<Box<dyn Storage>> {
    match config.storage.backend.as_str() {
        "memory" => {
            log::info!("Using memory storage backend");
            Ok(Box::new(memory::MemoryStorage::new()))
        }
        "sqlite" => {
            let default_path = "swoop.db".to_string();
            let path = config.storage.sqlite_path.as_ref()
                .unwrap_or(&default_path);
            log::info!("Using SQLite storage backend: {}", path);
            Ok(Box::new(sqlite::SqliteStorage::new(path).await?))
        }
        #[cfg(feature = "postgres")]
        "postgres" => {
            if let Some(database_url) = &config.storage.database_url {
                log::info!("Using PostgreSQL storage backend");
                let storage = postgres::PostgresStorage::new(database_url).await?;
                storage.initialize().await?;
                Ok(Box::new(storage))
            } else {
                log::warn!("PostgreSQL backend selected but no DATABASE_URL provided, falling back to memory storage");
                Ok(Box::new(memory::MemoryStorage::new()))
            }
        }
        #[cfg(feature = "libsql")]
        "libsql" => {
            if let Some(connection_string) = &config.storage.connection_string {
                log::info!("Using LibSQL storage backend");
                Ok(Box::new(libsql::LibSqlStorage::new(connection_string, config.storage.auth_token.as_deref()).await?))
            } else {
                log::warn!("LibSQL backend selected but no connection string provided, falling back to memory storage");
                Ok(Box::new(memory::MemoryStorage::new()))
            }
        }
        "filesystem" => {
            let default_path = "./swoop_data".to_string();
            let path = config.storage.connection_string.as_ref()
                .unwrap_or(&default_path);
            log::info!("Using filesystem storage backend: {}", path);
            Ok(Box::new(filesystem::FileSystemStorage::new(std::path::PathBuf::from(path))?))
        }
        _ => {
            log::warn!("Unknown storage backend '{}', falling back to memory storage", config.storage.backend);
            Ok(Box::new(memory::MemoryStorage::new()))
        }
    }
} 