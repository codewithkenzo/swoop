/*!
 * File system storage implementation
 * 
 * This module provides file system-based storage for documents.
 */

use std::path::PathBuf;
use std::fs;
use tokio::fs as async_fs;
use serde_json;

use crate::{Error, Result};
use crate::models::{Document, DocumentBatch};
use crate::storage::Storage;

/// File system storage implementation
#[derive(Debug, Clone)]
pub struct FileSystemStorage {
    /// Base directory for storage
    pub base_path: PathBuf,
}

impl FileSystemStorage {
    /// Create a new file system storage
    pub fn new(base_path: PathBuf) -> Result<Self> {
        // Create base directory if it doesn't exist
        if !base_path.exists() {
            fs::create_dir_all(&base_path)
                .map_err(|e| Error::Storage(format!("Failed to create base directory: {e}")))?;
        }
        
        Ok(Self { base_path })
    }
    
    /// Get the path for a document file
    fn document_path(&self, id: &str) -> PathBuf {
        self.base_path.join("documents").join(format!("{id}.json"))
    }
    
    /// Get the path for a batch file
    fn batch_path(&self, id: &str) -> PathBuf {
        self.base_path.join("batches").join(format!("{id}.json"))
    }
}

#[async_trait::async_trait]
impl Storage for FileSystemStorage {
    async fn store_document(&self, document: &Document) -> Result<()> {
        let doc_dir = self.base_path.join("documents");
        if !doc_dir.exists() {
            async_fs::create_dir_all(&doc_dir).await
                .map_err(|e| Error::Storage(format!("Failed to create documents directory: {e}")))?;
        }
        
        let doc_path = self.document_path(&document.id);
        let json = serde_json::to_string_pretty(document)
            .map_err(|e| Error::Storage(format!("Failed to serialize document: {e}")))?;
        
        async_fs::write(&doc_path, json).await
            .map_err(|e| Error::Storage(format!("Failed to write document: {e}")))?;
        
        Ok(())
    }
    
    async fn retrieve_document(&self, id: &str) -> Result<Option<Document>> {
        let doc_path = self.document_path(id);
        
        if !doc_path.exists() {
            return Ok(None);
        }
        
        let content = async_fs::read_to_string(&doc_path).await
            .map_err(|e| Error::Storage(format!("Failed to read document: {e}")))?;
        
        let document: Document = serde_json::from_str(&content)
            .map_err(|e| Error::Storage(format!("Failed to deserialize document: {e}")))?;
        
        Ok(Some(document))
    }
    
    async fn list_documents(&self) -> Result<Vec<String>> {
        let doc_dir = self.base_path.join("documents");
        
        if !doc_dir.exists() {
            return Ok(Vec::new());
        }
        
        let mut entries = async_fs::read_dir(&doc_dir).await
            .map_err(|e| Error::Storage(format!("Failed to read documents directory: {e}")))?;
        
        let mut document_ids = Vec::new();
        
        while let Some(entry) = entries.next_entry().await
            .map_err(|e| Error::Storage(format!("Failed to read directory entry: {e}")))? {
            
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".json") {
                    let id = file_name.trim_end_matches(".json");
                    document_ids.push(id.to_string());
                }
            }
        }
        
        Ok(document_ids)
    }
    
    async fn delete_document(&self, id: &str) -> Result<()> {
        let doc_path = self.document_path(id);
        
        if doc_path.exists() {
            async_fs::remove_file(&doc_path).await
                .map_err(|e| Error::Storage(format!("Failed to delete document: {e}")))?;
        }
        
        Ok(())
    }
    
    async fn store_batch(&self, batch: &DocumentBatch) -> Result<()> {
        let batch_dir = self.base_path.join("batches");
        if !batch_dir.exists() {
            async_fs::create_dir_all(&batch_dir).await
                .map_err(|e| Error::Storage(format!("Failed to create batches directory: {e}")))?;
        }
        
        let batch_path = self.batch_path(&batch.id);
        let json = serde_json::to_string_pretty(batch)
            .map_err(|e| Error::Storage(format!("Failed to serialize batch: {e}")))?;
        
        async_fs::write(&batch_path, json).await
            .map_err(|e| Error::Storage(format!("Failed to write batch: {e}")))?;
        
        Ok(())
    }
    
    async fn retrieve_batch(&self, id: &str) -> Result<Option<DocumentBatch>> {
        let batch_path = self.batch_path(id);
        
        if !batch_path.exists() {
            return Ok(None);
        }
        
        let content = async_fs::read_to_string(&batch_path).await
            .map_err(|e| Error::Storage(format!("Failed to read batch: {e}")))?;
        
        let batch: DocumentBatch = serde_json::from_str(&content)
            .map_err(|e| Error::Storage(format!("Failed to deserialize batch: {e}")))?;
        
        Ok(Some(batch))
    }
    
    async fn health_check(&self) -> Result<bool> {
        // Check if base directory is accessible
        if !self.base_path.exists() {
            return Ok(false);
        }
        
        // Try to create a test file to ensure write permissions
        let test_file = self.base_path.join("health_check.tmp");
        
        match async_fs::write(&test_file, "test").await {
            Ok(_) => {
                // Clean up test file
                if let Err(e) = async_fs::remove_file(&test_file).await {
                    eprintln!("Warning: Failed to clean up health check file: {e}");
                }
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }
} 