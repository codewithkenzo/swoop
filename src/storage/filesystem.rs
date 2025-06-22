/*!
 * Filesystem storage implementation for Crawl4AI
 * 
 * This provides persistent file-based storage with optional compression
 * suitable for small to medium-scale deployments.
 */



use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use async_trait::async_trait;
use serde_json;
use tokio::fs;
// use tokio::io::{AsyncReadExt, AsyncWriteExt}; // For future file I/O operations
use tokio::sync::RwLock;

use crate::error::{Error, Result};
use crate::models::{Document, DocumentBatch};
use super::{Storage, StorageStats};

/// Filesystem storage implementation
pub struct FileSystemStorage {
    /// Base directory for storage
    base_path: PathBuf,
    /// Storage statistics
    stats: Arc<RwLock<StorageStats>>,
    /// Whether compression is enabled
    use_compression: bool,
}

impl FileSystemStorage {
    /// Create a new FileSystemStorage instance
    pub async fn new(base_path: &str) -> Result<Self> {
        let path = PathBuf::from(base_path);
        
        // Create base directory if it doesn't exist
        if !path.exists() {
            fs::create_dir_all(&path).await
                .map_err(|e| Error::Storage(format!("Failed to create storage directory: {}", e)))?;
        }
        
        // Create subdirectories
        let documents_dir = path.join("documents");
        let metadata_dir = path.join("metadata");
        
        fs::create_dir_all(&documents_dir).await
            .map_err(|e| Error::Storage(format!("Failed to create documents directory: {}", e)))?;
        fs::create_dir_all(&metadata_dir).await
            .map_err(|e| Error::Storage(format!("Failed to create metadata directory: {}", e)))?;
        
        Ok(Self {
            base_path: path,
            stats: Arc::new(RwLock::new(StorageStats {
                total_documents: 0,
                total_size_bytes: 0,
                successful_operations: 0,
                failed_operations: 0,
                avg_operation_time_ms: 0.0,
            })),
            use_compression: true,
        })
    }
    
    /// Create a new FileSystemStorage instance without compression
    pub async fn new_uncompressed(base_path: &str) -> Result<Self> {
        let mut storage = Self::new(base_path).await?;
        storage.use_compression = false;
        Ok(storage)
    }
    
    /// Get the path for a document file
    fn get_document_path(&self, id: &str) -> PathBuf {
        // Use first two characters for subdirectory to avoid too many files in one directory
        let subdir = if id.len() >= 2 {
            &id[0..2]
        } else {
            "00"
        };
        
        let extension = if self.use_compression { "json.gz" } else { "json" };
        self.base_path
            .join("documents")
            .join(subdir)
            .join(format!("{}.{}", id, extension))
    }
    
    /// Get the path for metadata file
    #[allow(dead_code)]
    fn get_metadata_path(&self, id: &str) -> PathBuf {
        let subdir = if id.len() >= 2 {
            &id[0..2]
        } else {
            "00"
        };
        
        self.base_path
            .join("metadata")
            .join(subdir)
            .join(format!("{}.json", id))
    }
    
    /// Write data to file with optional compression
    async fn write_file(&self, path: &Path, data: &[u8]) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await
                .map_err(|e| Error::Storage(format!("Failed to create directory: {}", e)))?;
        }
        
        let final_data = if self.use_compression {
            flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default())
                .finish()
                .map_err(|e| Error::Storage(format!("Compression failed: {}", e)))?;
            
            use std::io::Write;
            let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
            encoder.write_all(data)
                .map_err(|e| Error::Storage(format!("Compression write failed: {}", e)))?;
            encoder.finish()
                .map_err(|e| Error::Storage(format!("Compression finish failed: {}", e)))?
        } else {
            data.to_vec()
        };
        
        fs::write(path, final_data).await
            .map_err(|e| Error::Storage(format!("Failed to write file: {}", e)))
    }
    
    /// Read data from file with optional decompression
    async fn read_file(&self, path: &Path) -> Result<Vec<u8>> {
        let data = fs::read(path).await
            .map_err(|e| Error::Storage(format!("Failed to read file: {}", e)))?;
        
        if self.use_compression && path.extension().and_then(|s| s.to_str()) == Some("gz") {
            use std::io::Read;
            let mut decoder = flate2::read::GzDecoder::new(&data[..]);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed)
                .map_err(|e| Error::Storage(format!("Decompression failed: {}", e)))?;
            Ok(decompressed)
        } else {
            Ok(data)
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
        
        // Update document count and total size
        self.refresh_stats().await.ok();
    }
    
    /// Refresh storage statistics by scanning the filesystem
    async fn refresh_stats(&self) -> Result<()> {
        let mut total_documents = 0u64;
        let mut total_size = 0u64;
        
        let documents_dir = self.base_path.join("documents");
        if documents_dir.exists() {
            let mut entries = fs::read_dir(documents_dir).await
                .map_err(|e| Error::Storage(format!("Failed to read documents directory: {}", e)))?;
            
            while let Some(entry) = entries.next_entry().await
                .map_err(|e| Error::Storage(format!("Failed to read directory entry: {}", e)))? {
                
                if entry.file_type().await
                    .map_err(|e| Error::Storage(format!("Failed to get file type: {}", e)))?
                    .is_dir() {
                    
                    let mut subdir_entries = fs::read_dir(entry.path()).await
                        .map_err(|e| Error::Storage(format!("Failed to read subdirectory: {}", e)))?;
                    
                    while let Some(subentry) = subdir_entries.next_entry().await
                        .map_err(|e| Error::Storage(format!("Failed to read subdirectory entry: {}", e)))? {
                        
                        if subentry.file_type().await
                            .map_err(|e| Error::Storage(format!("Failed to get file type: {}", e)))?
                            .is_file() {
                            
                            total_documents += 1;
                            if let Ok(metadata) = subentry.metadata().await {
                                total_size += metadata.len();
                            }
                        }
                    }
                }
            }
        }
        
        let mut stats = self.stats.write().await;
        stats.total_documents = total_documents;
        stats.total_size_bytes = total_size;
        
        Ok(())
    }
}

#[async_trait]
impl Storage for FileSystemStorage {
    async fn store_document(&self, document: &Document) -> Result<()> {
        let start_time = Instant::now();
        
        let document_path = self.get_document_path(&document.id);
        let serialized = serde_json::to_vec(document)
            .map_err(|e| Error::Storage(format!("Failed to serialize document: {}", e)))?;
        
        match self.write_file(&document_path, &serialized).await {
            Ok(_) => {
                let operation_time = start_time.elapsed().as_millis() as f64;
                self.update_stats(true, operation_time).await;
                Ok(())
            }
            Err(e) => {
                let operation_time = start_time.elapsed().as_millis() as f64;
                self.update_stats(false, operation_time).await;
                Err(e)
            }
        }
    }
    
    async fn get_document(&self, id: &str) -> Result<Option<Document>> {
        let start_time = Instant::now();
        
        let document_path = self.get_document_path(id);
        
        if !document_path.exists() {
            let operation_time = start_time.elapsed().as_millis() as f64;
            self.update_stats(true, operation_time).await;
            return Ok(None);
        }
        
        match self.read_file(&document_path).await {
            Ok(data) => {
                match serde_json::from_slice::<Document>(&data) {
                    Ok(document) => {
                        let operation_time = start_time.elapsed().as_millis() as f64;
                        self.update_stats(true, operation_time).await;
                        Ok(Some(document))
                    }
                    Err(e) => {
                        let operation_time = start_time.elapsed().as_millis() as f64;
                        self.update_stats(false, operation_time).await;
                        Err(Error::Storage(format!("Failed to deserialize document: {}", e)))
                    }
                }
            }
            Err(e) => {
                let operation_time = start_time.elapsed().as_millis() as f64;
                self.update_stats(false, operation_time).await;
                Err(e)
            }
        }
    }
    
    async fn update_document(&self, document: &Document) -> Result<()> {
        let start_time = Instant::now();
        
        let document_path = self.get_document_path(&document.id);
        
        if !document_path.exists() {
            let operation_time = start_time.elapsed().as_millis() as f64;
            self.update_stats(false, operation_time).await;
            return Err(Error::NotFound(format!("Document with id '{}' not found", document.id)));
        }
        
        // Update is the same as store for filesystem
        self.store_document(document).await
    }
    
    async fn delete_document(&self, id: &str) -> Result<()> {
        let start_time = Instant::now();
        
        let document_path = self.get_document_path(id);
        
        if !document_path.exists() {
            let operation_time = start_time.elapsed().as_millis() as f64;
            self.update_stats(false, operation_time).await;
            return Err(Error::NotFound(format!("Document with id '{}' not found", id)));
        }
        
        match fs::remove_file(document_path).await {
            Ok(_) => {
                let operation_time = start_time.elapsed().as_millis() as f64;
                self.update_stats(true, operation_time).await;
                Ok(())
            }
            Err(e) => {
                let operation_time = start_time.elapsed().as_millis() as f64;
                self.update_stats(false, operation_time).await;
                Err(Error::Storage(format!("Failed to delete document: {}", e)))
            }
        }
    }
    
    async fn search_documents(&self, query: &str, limit: Option<usize>) -> Result<Vec<Document>> {
        let start_time = Instant::now();
        
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();
        
        let documents_dir = self.base_path.join("documents");
        if !documents_dir.exists() {
            let operation_time = start_time.elapsed().as_millis() as f64;
            self.update_stats(true, operation_time).await;
            return Ok(results);
        }
        
        let mut entries = fs::read_dir(documents_dir).await
            .map_err(|e| Error::Storage(format!("Failed to read documents directory: {}", e)))?;
        
        while let Some(entry) = entries.next_entry().await
            .map_err(|e| Error::Storage(format!("Failed to read directory entry: {}", e)))? {
            
            if entry.file_type().await
                .map_err(|e| Error::Storage(format!("Failed to get file type: {}", e)))?
                .is_dir() {
                
                let mut subdir_entries = fs::read_dir(entry.path()).await
                    .map_err(|e| Error::Storage(format!("Failed to read subdirectory: {}", e)))?;
                
                while let Some(subentry) = subdir_entries.next_entry().await
                    .map_err(|e| Error::Storage(format!("Failed to read subdirectory entry: {}", e)))? {
                    
                    if subentry.file_type().await
                        .map_err(|e| Error::Storage(format!("Failed to get file type: {}", e)))?
                        .is_file() {
                        
                        if let Ok(data) = self.read_file(&subentry.path()).await {
                            if let Ok(document) = serde_json::from_slice::<Document>(&data) {
                                // Simple text search in title and content
                                if document.title.to_lowercase().contains(&query_lower) ||
                                   document.text.to_lowercase().contains(&query_lower) {
                                    results.push(document);
                                    
                                    // Apply limit if specified
                                    if let Some(limit) = limit {
                                        if results.len() >= limit {
                                            let operation_time = start_time.elapsed().as_millis() as f64;
                                            self.update_stats(true, operation_time).await;
                                            return Ok(results);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        let operation_time = start_time.elapsed().as_millis() as f64;
        self.update_stats(true, operation_time).await;
        
        Ok(results)
    }
    
    async fn store_batch(&self, batch: &DocumentBatch) -> Result<()> {
        let start_time = Instant::now();
        
        for document in &batch.documents {
            if let Err(e) = self.store_document(document).await {
                let operation_time = start_time.elapsed().as_millis() as f64;
                self.update_stats(false, operation_time).await;
                return Err(e);
            }
        }
        
        let operation_time = start_time.elapsed().as_millis() as f64;
        self.update_stats(true, operation_time).await;
        
        Ok(())
    }
    
    async fn get_stats(&self) -> Result<StorageStats> {
        let start_time = Instant::now();
        
        // Refresh stats before returning
        self.refresh_stats().await?;
        
        let stats = self.stats.read().await.clone();
        let _operation_time = start_time.elapsed().as_millis() as f64;
        
        // operation_time not used in stats tracking to avoid affecting the statistics being queried
        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Document, Metadata};
    use chrono::Utc;
    use tempfile::TempDir;
    
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
    async fn test_filesystem_storage_crud() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileSystemStorage::new(temp_dir.path().to_str().unwrap()).await.unwrap();
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
    async fn test_filesystem_storage_compression() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileSystemStorage::new(temp_dir.path().to_str().unwrap()).await.unwrap();
        let doc = create_test_document("test1");
        
        storage.store_document(&doc).await.unwrap();
        
        // Check that compressed file exists
        let doc_path = storage.get_document_path("test1");
        assert!(doc_path.exists());
        assert!(doc_path.extension().unwrap() == "gz");
        
        // Verify we can still read it
        let retrieved = storage.get_document("test1").await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, "test1");
    }
    
    #[tokio::test]
    async fn test_filesystem_storage_search() {
        let temp_dir = TempDir::new().unwrap();
        let storage = FileSystemStorage::new(temp_dir.path().to_str().unwrap()).await.unwrap();
        
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
} 