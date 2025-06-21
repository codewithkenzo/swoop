/*!
 * In-memory storage implementation for Crawl4AI
 * 
 * This provides a fast, concurrent in-memory storage backend
 * suitable for development, testing, and small-scale deployments.
 */

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use async_trait::async_trait;
use dashmap::DashMap;
use tokio::sync::RwLock;

use crate::error::{Error, Result};
use crate::models::{Document, DocumentBatch};
use super::{Storage, StorageStats};

/// In-memory storage implementation
pub struct MemoryStorage {
    /// Document storage
    documents: Arc<DashMap<String, Document>>,
    /// Storage statistics
    stats: Arc<RwLock<StorageStats>>,
    /// Creation time for stats
    created_at: Instant,
}

impl MemoryStorage {
    /// Create a new MemoryStorage instance
    pub fn new() -> Self {
        Self {
            documents: Arc::new(DashMap::new()),
            stats: Arc::new(RwLock::new(StorageStats {
                total_documents: 0,
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
        stats.total_documents = self.documents.len() as u64;
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
        
        match self.documents.insert(document.id.clone(), document.clone()) {
            _ => {
                let operation_time = start_time.elapsed().as_millis() as f64;
                self.update_stats(true, operation_time).await;
                Ok(())
            }
        }
    }
    
    async fn get_document(&self, id: &str) -> Result<Option<Document>> {
        let start_time = Instant::now();
        
        let result = self.documents.get(id).map(|entry| entry.value().clone());
        let operation_time = start_time.elapsed().as_millis() as f64;
        self.update_stats(true, operation_time).await;
        
        Ok(result)
    }
    
    async fn update_document(&self, document: &Document) -> Result<()> {
        let start_time = Instant::now();
        
        if self.documents.contains_key(&document.id) {
            self.documents.insert(document.id.clone(), document.clone());
            let operation_time = start_time.elapsed().as_millis() as f64;
            self.update_stats(true, operation_time).await;
            Ok(())
        } else {
            let operation_time = start_time.elapsed().as_millis() as f64;
            self.update_stats(false, operation_time).await;
            Err(Error::NotFound(format!("Document with id '{}' not found", document.id)))
        }
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
                Err(Error::NotFound(format!("Document with id '{}' not found", id)))
            }
        }
    }
    
    async fn search_documents(&self, query: &str, limit: Option<usize>) -> Result<Vec<Document>> {
        let start_time = Instant::now();
        
        let query_lower = query.to_lowercase();
        let mut results: Vec<Document> = self.documents
            .iter()
            .filter_map(|entry| {
                let document = entry.value();
                // Simple text search in title and content
                if document.title.to_lowercase().contains(&query_lower) ||
                   document.text.to_lowercase().contains(&query_lower) {
                    Some(document.clone())
                } else {
                    None
                }
            })
            .collect();
        
        // Apply limit if specified
        if let Some(limit) = limit {
            results.truncate(limit);
        }
        
        let operation_time = start_time.elapsed().as_millis() as f64;
        self.update_stats(true, operation_time).await;
        
        Ok(results)
    }
    
    async fn store_batch(&self, batch: &DocumentBatch) -> Result<()> {
        let start_time = Instant::now();
        
        for document in &batch.documents {
            self.documents.insert(document.id.clone(), document.clone());
        }
        
        let operation_time = start_time.elapsed().as_millis() as f64;
        self.update_stats(true, operation_time).await;
        
        Ok(())
    }
    
    async fn get_stats(&self) -> Result<StorageStats> {
        let start_time = Instant::now();
        
        let stats = self.stats.read().await.clone();
        let _operation_time = start_time.elapsed().as_millis() as f64;
        
        // Don't update stats for stats query to avoid infinite recursion
        // operation_time is intentionally unused here to prevent recursion
        Ok(stats)
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// Estimate the size of a document in bytes
fn estimate_document_size(document: &Document) -> u64 {
    let base_size = std::mem::size_of::<Document>() as u64;
    let string_sizes = 
        document.id.len() as u64 +
        document.url.len() as u64 +
        document.title.len() as u64 +
        document.content.len() as u64 +
        document.html.len() as u64 +
        document.text.len() as u64;
    
    let links_size = document.links.iter()
        .map(|link| link.url.len() + link.text.len() + link.source_url.len())
        .sum::<usize>() as u64;
    
    let extracted_size = document.extracted.iter()
        .map(|(k, v)| k.len() + v.content.len() + v.name.len())
        .sum::<usize>() as u64;
    
    base_size + string_sizes + links_size + extracted_size
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Document, Metadata};
    use chrono::Utc;
    
    fn create_test_document(id: &str) -> Document {
        Document {
            id: id.to_string(),
            url: format!("https://example.com/{}", id),
            title: format!("Test Document {}", id),
            content: format!("Content for document {}", id),
            html: format!("<html><body>Content for document {}</body></html>", id),
            text: format!("Content for document {}", id),
            metadata: Metadata {
                url: format!("https://example.com/{}", id),
                content_type: "text/html".to_string(),
                fetch_time: Utc::now(),
                status_code: 200,
                headers: HashMap::new(),
            },
            links: Vec::new(),
            extracted: HashMap::new(),
        }
    }
    
    #[tokio::test]
    async fn test_memory_storage_crud() {
        let storage = MemoryStorage::new();
        let doc = create_test_document("test1");
        
        // Test store
        assert!(storage.store_document(&doc).await.is_ok());
        
        // Test get
        let retrieved = storage.get_document("test1").await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, "test1");
        
        // Test update
        let mut updated_doc = doc.clone();
        updated_doc.title = "Updated Title".to_string();
        assert!(storage.update_document(&updated_doc).await.is_ok());
        
        let retrieved = storage.get_document("test1").await.unwrap().unwrap();
        assert_eq!(retrieved.title, "Updated Title");
        
        // Test delete
        assert!(storage.delete_document("test1").await.is_ok());
        let retrieved = storage.get_document("test1").await.unwrap();
        assert!(retrieved.is_none());
    }
    
    #[tokio::test]
    async fn test_memory_storage_search() {
        let storage = MemoryStorage::new();
        
        // Store test documents
        for i in 1..=5 {
            let doc = create_test_document(&format!("doc{}", i));
            storage.store_document(&doc).await.unwrap();
        }
        
        // Test search
        let results = storage.search_documents("document", Some(3)).await.unwrap();
        assert!(results.len() <= 3);
        assert!(!results.is_empty());
    }
    
    #[tokio::test]
    async fn test_memory_storage_stats() {
        let storage = MemoryStorage::new();
        let doc = create_test_document("test1");
        
        storage.store_document(&doc).await.unwrap();
        
        let stats = storage.get_stats().await.unwrap();
        assert_eq!(stats.total_documents, 1);
        assert!(stats.total_size_bytes > 0);
        assert_eq!(stats.successful_operations, 1);
    }
} 