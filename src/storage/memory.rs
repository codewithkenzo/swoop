/*!
 * In-memory storage implementation for Swoop Document Processing
 * 
 * This provides a fast, concurrent in-memory storage backend
 * suitable for development, testing, and small-scale deployments.
 */


use std::sync::Arc;
use std::time::Instant;

use async_trait::async_trait;
use dashmap::DashMap;
use tokio::sync::RwLock;

use crate::error::{Error, Result};
use crate::models::{Document, DocumentBatch};
use super::{Storage, StorageStats};

/// In-memory storage implementation
#[derive(Debug)]
pub struct MemoryStorage {
    /// Document storage
    documents: Arc<DashMap<String, Document>>,
    /// Document batch storage
    batches: Arc<DashMap<String, DocumentBatch>>,
    /// Storage statistics
    stats: Arc<RwLock<StorageStats>>,
    /// Creation time for stats
    #[allow(dead_code)]
    created_at: Instant,
}

impl MemoryStorage {
    /// Create a new MemoryStorage instance
    pub fn new() -> Self {
        Self {
            documents: Arc::new(DashMap::new()),
            batches: Arc::new(DashMap::new()),
            stats: Arc::new(RwLock::new(StorageStats {
                total_documents: 0,
                total_batches: 0,
                storage_backend: "Memory".to_string(),
                total_size_bytes: 0,
                successful_operations: 0,
                failed_operations: 0,
                avg_operation_time_ms: 0.0,
            })),
            created_at: Instant::now(),
        }
    }
    
    /// Update statistics
    async fn update_stats(&self, success: bool, operation_time: f64) {
        let mut stats = self.stats.write().await;
        
        if success {
            stats.successful_operations += 1;
        } else {
            stats.failed_operations += 1;
        }
        
        // Update average operation time
        let total_ops = stats.successful_operations + stats.failed_operations;
        if total_ops > 0 {
            stats.avg_operation_time_ms = 
                (stats.avg_operation_time_ms * (total_ops - 1) as f64 + operation_time) / total_ops as f64;
        }
        
        // Update document count and size
        stats.total_documents = self.documents.len();
        stats.total_batches = self.batches.len();
        stats.total_size_bytes = self.documents
            .iter()
            .map(|entry| estimate_document_size(entry.value()))
            .sum();
    }
}

#[async_trait]
impl Storage for MemoryStorage {
    async fn store_document(&self, document: &Document) -> Result<()> {
        let start_time = Instant::now();
        
        self.documents.insert(document.id.clone(), document.clone());
        let operation_time = start_time.elapsed().as_millis() as f64;
        self.update_stats(true, operation_time).await;
        
        Ok(())
    }
    
    async fn retrieve_document(&self, id: &str) -> Result<Option<Document>> {
        let start_time = Instant::now();
        
        let result = self.documents.get(id).map(|entry| entry.value().clone());
        let operation_time = start_time.elapsed().as_millis() as f64;
        self.update_stats(true, operation_time).await;
        
        Ok(result)
    }
    
    async fn list_documents(&self) -> Result<Vec<String>> {
        let start_time = Instant::now();
        
        let document_ids: Vec<String> = self.documents
            .iter()
            .map(|entry| entry.key().clone())
            .collect();
        
        let operation_time = start_time.elapsed().as_millis() as f64;
        self.update_stats(true, operation_time).await;
        
        Ok(document_ids)
    }
    
    async fn delete_document(&self, id: &str) -> Result<()> {
        let start_time = Instant::now();
        
        match self.documents.remove(id) {
            Some(_) => {
                let operation_time = start_time.elapsed().as_millis() as f64;
                self.update_stats(true, operation_time).await;
                Ok(())
            }
            None => {
                let operation_time = start_time.elapsed().as_millis() as f64;
                self.update_stats(false, operation_time).await;
                Err(Error::Storage(format!("Document with id '{}' not found", id)))
            }
        }
    }
    
    async fn store_batch(&self, batch: &DocumentBatch) -> Result<()> {
        let start_time = Instant::now();
        
        self.batches.insert(batch.id.clone(), batch.clone());
        
        let operation_time = start_time.elapsed().as_millis() as f64;
        self.update_stats(true, operation_time).await;
        
        Ok(())
    }
    
    async fn retrieve_batch(&self, id: &str) -> Result<Option<DocumentBatch>> {
        let start_time = Instant::now();
        
        let result = self.batches.get(id).map(|entry| entry.value().clone());
        let operation_time = start_time.elapsed().as_millis() as f64;
        self.update_stats(true, operation_time).await;
        
        Ok(result)
    }
    
    async fn health_check(&self) -> Result<bool> {
        // Memory storage is always healthy if we can access the data structures
        Ok(true)
    }
}

// Additional convenience methods not part of the Storage trait
impl MemoryStorage {
    /// Get document (direct access, not part of trait)
    pub async fn get_document(&self, id: &str) -> Result<Option<Document>> {
        self.retrieve_document(id).await
    }
    
    /// Update document (not part of trait)
    pub async fn update_document(&self, document: &Document) -> Result<()> {
        let start_time = Instant::now();
        
        if self.documents.contains_key(&document.id) {
            self.documents.insert(document.id.clone(), document.clone());
            let operation_time = start_time.elapsed().as_millis() as f64;
            self.update_stats(true, operation_time).await;
            Ok(())
        } else {
            let operation_time = start_time.elapsed().as_millis() as f64;
            self.update_stats(false, operation_time).await;
            Err(Error::Storage(format!("Document with id '{}' not found", document.id)))
        }
    }
    
    /// List documents with limit (not part of trait)
    pub async fn list_documents_with_limit(&self, limit: Option<usize>) -> Result<Vec<Document>> {
        let start_time = Instant::now();
        
        let mut documents: Vec<Document> = self.documents
            .iter()
            .map(|entry| entry.value().clone())
            .collect();
        
        // Sort by created_at (most recent first)
        documents.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        // Apply limit if specified
        if let Some(limit) = limit {
            documents.truncate(limit);
        }
        
        let operation_time = start_time.elapsed().as_millis() as f64;
        self.update_stats(true, operation_time).await;
        
        Ok(documents)
    }
    
    /// Search documents (not part of trait)
    pub async fn search_documents(&self, query: &str, limit: Option<usize>) -> Result<Vec<Document>> {
        let start_time = Instant::now();
        
        let query_lower = query.to_lowercase();
        let mut results: Vec<Document> = self.documents
            .iter()
            .filter_map(|entry| {
                let document = entry.value();
                // Simple text search in title and content
                if document.title.to_lowercase().contains(&query_lower) ||
                   document.content.to_lowercase().contains(&query_lower) {
                    Some(document.clone())
                } else {
                    None
                }
            })
            .collect();
        
        // Sort by relevance (more matches = higher relevance)
        results.sort_by(|a, b| {
            let a_matches = count_matches(&a.title, &query_lower) + count_matches(&a.content, &query_lower);
            let b_matches = count_matches(&b.title, &query_lower) + count_matches(&b.content, &query_lower);
            b_matches.cmp(&a_matches)
        });
        
        // Apply limit if specified
        if let Some(limit) = limit {
            results.truncate(limit);
        }
        
        let operation_time = start_time.elapsed().as_millis() as f64;
        self.update_stats(true, operation_time).await;
        
        Ok(results)
    }
    
    /// Get storage statistics (not part of trait)
    pub async fn get_stats(&self) -> Result<StorageStats> {
        let start_time = Instant::now();
        
        let mut stats = self.stats.read().await.clone();
        
        // Update real-time stats
        stats.total_documents = self.documents.len();
        stats.total_batches = self.batches.len();
        stats.total_size_bytes = self.documents
            .iter()
            .map(|entry| estimate_document_size(entry.value()))
            .sum();
        
        let _operation_time = start_time.elapsed().as_millis() as f64;
        // Don't update stats for stats query to avoid infinite recursion
        
        Ok(stats)
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// Count the number of matches of a query in text
fn count_matches(text: &str, query: &str) -> usize {
    text.to_lowercase().matches(query).count()
}

/// Estimate the size of a document in bytes
fn estimate_document_size(document: &Document) -> u64 {
    let base_size = std::mem::size_of::<Document>() as u64;
    let string_sizes = 
        document.id.len() as u64 +
        document.title.len() as u64 +
        document.content.len() as u64 +
        document.source_url.as_ref().map(|s| s.len() as u64).unwrap_or(0) +
        document.summary.as_ref().map(|s| s.len() as u64).unwrap_or(0) +
        document.document_type.as_ref().map(|s| s.len() as u64).unwrap_or(0) +
        document.language.as_ref().map(|s| s.len() as u64).unwrap_or(0);
    
    base_size + string_sizes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Metadata;
    use chrono::Utc;

    fn create_test_document(id: &str) -> Document {
        Document {
            id: id.to_string(),
            title: format!("Test Document {}", id),
            content: format!("This is test content for document {}", id),
            summary: Some(format!("Summary for {}", id)),
            metadata: Metadata::default(),
            quality_score: Some(0.8),
            content_hash: Some(format!("hash_{}", id)),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            source_url: Some(format!("https://example.com/{}", id)),
            document_type: Some("text".to_string()), 
            language: Some("en".to_string()),
            content_type: Some("text/plain".to_string()),
            file_size: Some(format!("This is test content for document {}", id).len() as u64),
            extracted_at: Utc::now(),
            word_count: Some(10),
            size_bytes: Some(100),
        }
    }

    #[tokio::test]
    async fn test_memory_storage_crud() {
        let storage = MemoryStorage::new();
        let doc = create_test_document("test1");

        // Test store
        storage.store_document(&doc).await.unwrap();

        // Test retrieve
        let retrieved = storage.retrieve_document("test1").await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, doc.id);

        // Test get (optional)
        let optional = storage.get_document("test1").await.unwrap();
        assert!(optional.is_some());

        // Test update
        let mut updated_doc = doc.clone();
        updated_doc.title = "Updated Title".to_string();
        storage.update_document(&updated_doc).await.unwrap();

        // Test delete
        storage.delete_document("test1").await.unwrap();
        let deleted = storage.get_document("test1").await.unwrap();
        assert!(deleted.is_none());
    }

    #[tokio::test]
    async fn test_memory_storage_search() {
        let storage = MemoryStorage::new();
        
        for i in 1..=5 {
            let doc = create_test_document(&format!("doc{}", i));
            storage.store_document(&doc).await.unwrap();
        }

        let results = storage.search_documents("Test", Some(3)).await.unwrap();
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_memory_storage_stats() {
        let storage = MemoryStorage::new();
        let doc = create_test_document("stats_test");
        
        storage.store_document(&doc).await.unwrap();
        let stats = storage.get_stats().await.unwrap();
        
        assert_eq!(stats.total_documents, 1);
        assert!(stats.total_size_bytes > 0);
    }
} 