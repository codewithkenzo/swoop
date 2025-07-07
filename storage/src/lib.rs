//! Storage crate - High-performance data persistence layer
//!
//! This crate provides data storage and persistence capabilities using ScyllaDB
//! for time-series data and S3-compatible storage for data archival.

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod config;
pub mod scylla_store;
pub mod s3_store;
pub mod models;

/// Configuration for storage systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// ScyllaDB configuration
    pub scylla: ScyllaConfig,
    /// S3-compatible storage configuration
    pub s3: S3Config,
}

/// ScyllaDB connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScyllaConfig {
    /// Cluster nodes (e.g., ["127.0.0.1:9042"])
    pub nodes: Vec<String>,
    /// Keyspace name
    pub keyspace: String,
    /// Connection timeout in seconds
    pub timeout_secs: u64,
    /// Compression algorithm
    pub compression: Option<String>,
}

impl Default for ScyllaConfig {
    fn default() -> Self {
        Self {
            nodes: vec!["127.0.0.1:9042".to_string()],
            keyspace: "swoop".to_string(),
            timeout_secs: 30,
            compression: Some("lz4".to_string()),
        }
    }
}

/// S3-compatible storage configuration (DEPRECATED - use config::SecureS3Config)
#[deprecated(note = "Use config::SecureS3Config for secure credential management")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3Config {
    /// S3 endpoint URL
    pub endpoint: String,
    /// Access key ID
    pub access_key_id: String,
    /// Secret access key
    pub secret_access_key: String,
    /// S3 bucket name
    pub bucket: String,
    /// AWS region
    pub region: String,
}

impl Default for S3Config {
    fn default() -> Self {
        Self {
            endpoint: "https://s3.amazonaws.com".to_string(),
            access_key_id: "".to_string(),
            secret_access_key: "".to_string(),
            bucket: "swoop-data".to_string(),
            region: "us-east-1".to_string(),
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            scylla: ScyllaConfig::default(),
            s3: S3Config::default(),
        }
    }
}

/// Trait for storage backends
pub trait StorageBackend {
    /// Store extracted content
    async fn store_content(&self, content: &models::StoredContent) -> Result<String>;
    
    /// Retrieve content by ID
    async fn get_content(&self, id: &str) -> Result<Option<models::StoredContent>>;
    
    /// Query content by URL
    async fn get_content_by_url(&self, url: &str) -> Result<Vec<models::StoredContent>>;
    
    /// Delete content by ID
    async fn delete_content(&self, id: &str) -> Result<bool>;
    
    /// Get storage statistics
    async fn get_stats(&self) -> Result<models::StorageStats>;
}

/// Storage manager that coordinates multiple storage backends
pub struct StorageManager {
    scylla_store: Option<scylla_store::ScyllaStore>,
    s3_store: Option<s3_store::S3Store>,
}

impl StorageManager {
    pub fn new() -> Self {
        Self {
            scylla_store: None,
            s3_store: None,
        }
    }
    
    pub async fn with_scylla(mut self, config: ScyllaConfig) -> Result<Self> {
        self.scylla_store = Some(scylla_store::ScyllaStore::new(config).await?);
        Ok(self)
    }
    
    pub async fn with_s3(mut self, config: S3Config) -> Result<Self> {
        self.s3_store = Some(s3_store::S3Store::new(config).await?);
        Ok(self)
    }
    
    /// Store content in primary storage (ScyllaDB) and optionally archive to S3
    pub async fn store_content(&self, content: &models::StoredContent) -> Result<String> {
        let mut content_id = None;
        
        // Store in ScyllaDB (primary storage)
        if let Some(scylla) = &self.scylla_store {
            content_id = Some(scylla.store_content(content).await?);
        }
        
        // Archive to S3 (secondary storage)
        if let Some(s3) = &self.s3_store {
            s3.store_content(content).await?;
        }
        
        content_id.ok_or_else(|| anyhow::anyhow!("No primary storage configured"))
    }
    
    /// Retrieve content by ID from primary storage
    pub async fn get_content(&self, id: &str) -> Result<Option<models::StoredContent>> {
        if let Some(scylla) = &self.scylla_store {
            return scylla.get_content(id).await;
        }
        
        if let Some(s3) = &self.s3_store {
            return s3.get_content(id).await;
        }
        
        Err(anyhow::anyhow!("No storage backend configured"))
    }
    
    /// Get combined storage statistics
    pub async fn get_stats(&self) -> Result<models::StorageStats> {
        let mut stats = models::StorageStats::default();
        
        if let Some(scylla) = &self.scylla_store {
            let scylla_stats = scylla.get_stats().await?;
            stats.total_documents += scylla_stats.total_documents;
            stats.total_size_bytes += scylla_stats.total_size_bytes;
        }
        
        if let Some(s3) = &self.s3_store {
            let s3_stats = s3.get_stats().await?;
            stats.archived_documents = s3_stats.total_documents;
            stats.archived_size_bytes = s3_stats.total_size_bytes;
        }
        
        Ok(stats)
    }
}

impl Default for StorageManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_config_defaults() {
        let config = StorageConfig::default();
        assert_eq!(config.scylla.keyspace, "swoop");
        assert_eq!(config.s3.bucket, "swoop-data");
    }

    #[test]
    fn test_scylla_config_defaults() {
        let config = ScyllaConfig::default();
        assert_eq!(config.nodes, vec!["127.0.0.1:9042".to_string()]);
        assert_eq!(config.timeout_secs, 30);
        assert_eq!(config.compression, Some("lz4".to_string()));
    }

    #[test]
    fn test_storage_manager_creation() {
        let manager = StorageManager::new();
        assert!(manager.scylla_store.is_none());
        assert!(manager.s3_store.is_none());
    }
}
