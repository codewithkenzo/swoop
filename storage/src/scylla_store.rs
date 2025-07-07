//! ScyllaDB storage backend implementation
//!
//! This module provides high-performance time-series data storage using ScyllaDB.

use crate::{models, ScyllaConfig, StorageBackend};
use anyhow::Result;
use scylla::{Session, SessionBuilder};

/// ScyllaDB storage backend
pub struct ScyllaStore {
    session: Session,
    keyspace: String,
}

impl ScyllaStore {
    /// Create a new ScyllaDB store instance
    pub async fn new(config: ScyllaConfig) -> Result<Self> {
        let session: Session = SessionBuilder::new()
            .known_nodes(&config.nodes)
            .build()
            .await?;
        
        // Create keyspace if it doesn't exist
        let create_keyspace_query = format!(
            "CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{
                'class': 'SimpleStrategy',
                'replication_factor': 1
            }}",
            config.keyspace
        );
        
        session.query_unpaged(create_keyspace_query, &[]).await?;
        session.use_keyspace(&config.keyspace, false).await?;
        
        // Create tables
        let store = Self {
            session,
            keyspace: config.keyspace,
        };
        
        store.create_tables().await?;
        
        Ok(store)
    }
    
    /// Create necessary tables for storing content
    async fn create_tables(&self) -> Result<()> {
        // Main content table partitioned by domain and time
        let create_content_table = "
            CREATE TABLE IF NOT EXISTS content (
                domain text,
                scraped_date date,
                id uuid,
                url text,
                platform text,
                title text,
                text text,
                html text,
                metadata map<text, text>,
                links list<text>,
                images list<text>,
                scraped_at timestamp,
                stored_at timestamp,
                content_hash text,
                size_bytes bigint,
                tags list<text>,
                PRIMARY KEY ((domain, scraped_date), scraped_at, id)
            ) WITH CLUSTERING ORDER BY (scraped_at DESC)
        ";
        
        self.session.query_unpaged(create_content_table, &[]).await?;
        
        // Index table for URL lookups
        let create_url_index = "
            CREATE TABLE IF NOT EXISTS content_by_url (
                url_hash text,
                url text,
                id uuid,
                domain text,
                scraped_date date,
                PRIMARY KEY (url_hash, scraped_at, id)
            ) WITH CLUSTERING ORDER BY (scraped_at DESC)
        ";
        
        self.session.query_unpaged(create_url_index, &[]).await?;
        
        // Statistics table
        let create_stats_table = "
            CREATE TABLE IF NOT EXISTS storage_stats (
                stat_type text,
                stat_date date,
                total_documents counter,
                total_size_bytes counter,
                PRIMARY KEY (stat_type, stat_date)
            )
        ";
        
        self.session.query_unpaged(create_stats_table, &[]).await?;
        
        Ok(())
    }
}

impl StorageBackend for ScyllaStore {
    async fn store_content(&self, content: &models::StoredContent) -> Result<String> {
        // Insert into main content table
        let insert_content = "
            INSERT INTO content (
                domain, scraped_date, id, url, platform, title, text, html,
                metadata, links, images, scraped_at, stored_at, content_hash,
                size_bytes, tags
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ";
        
        let scraped_date = content.scraped_at.date_naive();
        let id = uuid::Uuid::parse_str(&content.id)?;
        
        self.session.query_unpaged(
            insert_content,
            (
                &content.domain,
                scraped_date,
                id,
                &content.url,
                &content.platform,
                &content.title,
                &content.text,
                &content.html,
                &content.metadata,
                &content.links,
                &content.images,
                content.scraped_at,
                content.stored_at,
                &content.content_hash,
                content.size_bytes as i64,
                &content.tags,
            ),
        ).await?;
        
        // Insert into URL index
        let url_hash = format!("{:x}", md5::compute(content.url.as_bytes()));
        let insert_url_index = "
            INSERT INTO content_by_url (url_hash, url, id, domain, scraped_date)
            VALUES (?, ?, ?, ?, ?)
        ";
        
        self.session.query_unpaged(
            insert_url_index,
            (&url_hash, &content.url, id, &content.domain, scraped_date),
        ).await?;
        
        // Update statistics
        let update_stats = "
            UPDATE storage_stats 
            SET total_documents = total_documents + 1,
                total_size_bytes = total_size_bytes + ?
            WHERE stat_type = 'daily' AND stat_date = ?
        ";
        
        self.session.query_unpaged(
            update_stats,
            (content.size_bytes as i64, scraped_date),
        ).await?;
        
        Ok(content.id.clone())
    }
    
    async fn get_content(&self, _id: &str) -> Result<Option<models::StoredContent>> {
        // For now, return None as we'd need to implement a proper lookup
        // In a real implementation, we'd need additional indexes or search by partition
        Ok(None)
    }
    
    async fn get_content_by_url(&self, url: &str) -> Result<Vec<models::StoredContent>> {
        let url_hash = format!("{:x}", md5::compute(url.as_bytes()));
        
        let query = "SELECT url, id, domain, scraped_date FROM content_by_url WHERE url_hash = ?";
        let _rows = self.session.query_unpaged(query, (&url_hash,)).await?;
        
        // For now, return empty vector
        // In a real implementation, we'd fetch full content from the main table
        Ok(Vec::new())
    }
    
    async fn delete_content(&self, _id: &str) -> Result<bool> {
        // For now, return false as we'd need to implement proper deletion
        // This would involve removing from both content and index tables
        Ok(false)
    }
    
    async fn get_stats(&self) -> Result<models::StorageStats> {
        let today = chrono::Utc::now().date_naive();
        
        let query = "SELECT total_documents, total_size_bytes FROM storage_stats WHERE stat_type = 'daily' AND stat_date = ?";
        let rows = self.session.query_unpaged(query, (today,)).await?;
        
        if let Some(row) = rows.rows {
            if let Some(first_row) = row.first() {
                let total_documents: i64 = first_row.columns[0].as_ref().unwrap().as_bigint().unwrap();
                let total_size_bytes: i64 = first_row.columns[1].as_ref().unwrap().as_bigint().unwrap();
                
                return Ok(models::StorageStats {
                    total_documents: total_documents as u64,
                    total_size_bytes: total_size_bytes as u64,
                    ..Default::default()
                });
            }
        }
        
        Ok(models::StorageStats::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_scylla_store_creation() {
        // This test would require a running ScyllaDB instance
        // For now, we'll just test the configuration
        let config = ScyllaConfig::default();
        assert_eq!(config.keyspace, "swoop");
        assert!(!config.nodes.is_empty());
    }

    #[test]
    fn test_stored_content_conversion() {
        let content = models::StoredContent::new(
            "https://example.com".to_string(),
            "example.com".to_string(),
            "generic".to_string(),
            Some("Test".to_string()),
            Some("Content".to_string()),
            None,
            HashMap::new(),
        );
        
        assert!(!content.id.is_empty());
        assert_eq!(content.domain, "example.com");
    }
}