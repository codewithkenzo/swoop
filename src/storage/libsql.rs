/*!
 * libSQL storage implementation for Swoop Document Processing
 * Optimized for serverless/edge deployments (Vercel, Cloudflare, etc.)
 */

use async_trait::async_trait;
use libsql_client::{Client, Config, Statement};
use serde_json;
use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::models::{Document, DocumentBatch, Metadata, Link, ExtractedContent};
use super::{Storage, StorageStats};

/// libSQL storage implementation optimized for serverless environments
pub struct LibSqlStorage {
    client: Client,
}

impl LibSqlStorage {
    /// Create a new libSQL storage instance
    /// For local development: "file:local.db"
    /// For Turso: "libsql://your-database-url.turso.io"
    pub async fn new(database_url: &str, auth_token: Option<&str>) -> Result<Self> {
        let mut config = Config::new(database_url)
            .map_err(|e| Error::Storage(format!("Invalid libSQL URL: {}", e)))?;
        
        if let Some(token) = auth_token {
            config = config.with_auth_token(token);
        }
        
        let client = Client::from_config(config)
            .await
            .map_err(|e| Error::Storage(format!("Failed to connect to libSQL: {}", e)))?;
        
        let storage = Self { client };
        storage.init_tables().await?;
        
        Ok(storage)
    }
    
    /// Initialize database tables
    async fn init_tables(&self) -> Result<()> {
        // Documents table
        let documents_sql = r#"
            CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                format TEXT NOT NULL,
                source_url TEXT,
                metadata TEXT,
                processed_at TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
        "#;
        
        // Document batches table
        let batches_sql = r#"
            CREATE TABLE IF NOT EXISTS document_batches (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                document_ids TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
        "#;
        
        // Analytics/stats table
        let analytics_sql = r#"
            CREATE TABLE IF NOT EXISTS analytics (
                id TEXT PRIMARY KEY,
                event_type TEXT NOT NULL,
                data TEXT NOT NULL,
                timestamp TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
        "#;
        
        // Create indexes for performance
        let indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_documents_format ON documents(format)",
            "CREATE INDEX IF NOT EXISTS idx_documents_created_at ON documents(created_at)",
            "CREATE INDEX IF NOT EXISTS idx_batches_created_at ON document_batches(created_at)",
            "CREATE INDEX IF NOT EXISTS idx_analytics_event_type ON analytics(event_type)",
            "CREATE INDEX IF NOT EXISTS idx_analytics_timestamp ON analytics(timestamp)",
        ];
        
        // Execute table creation
        self.client.execute(documents_sql).await
            .map_err(|e| Error::Storage(format!("Failed to create documents table: {}", e)))?;
        
        self.client.execute(batches_sql).await
            .map_err(|e| Error::Storage(format!("Failed to create batches table: {}", e)))?;
        
        self.client.execute(analytics_sql).await
            .map_err(|e| Error::Storage(format!("Failed to create analytics table: {}", e)))?;
        
        // Create indexes
        for index_sql in indexes {
            self.client.execute(index_sql).await
                .map_err(|e| Error::Storage(format!("Failed to create index: {}", e)))?;
        }
        
        Ok(())
    }
}

#[async_trait]
impl Storage for LibSqlStorage {
    async fn store_document(&self, document: &Document) -> Result<()> {
        let metadata_json = serde_json::to_string(&document.metadata)
            .map_err(|e| Error::Storage(format!("Failed to serialize metadata: {}", e)))?;
        
        let stmt = Statement::with_args(
            "INSERT OR REPLACE INTO documents (id, title, content, format, source_url, metadata, processed_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            libsql_client::args![
                document.id.clone(),
                document.title.clone(), 
                document.content.clone(),
                document.format.clone(),
                document.source_url.clone(),
                metadata_json,
                document.processed_at.to_rfc3339()
            ]
        );
        
        self.client.execute(stmt).await
            .map_err(|e| Error::Storage(format!("Failed to store document: {}", e)))?;
        
        Ok(())
    }
    
    async fn retrieve_document(&self, id: &str) -> Result<Document> {
        let stmt = Statement::with_args(
            "SELECT id, title, content, format, source_url, metadata, processed_at FROM documents WHERE id = ?",
            libsql_client::args![id]
        );
        
        let mut rows = self.client.execute(stmt).await
            .map_err(|e| Error::Storage(format!("Failed to query document: {}", e)))?;
        
        if let Some(row) = rows.next().await
            .map_err(|e| Error::Storage(format!("Failed to read row: {}", e)))? {
            
            let metadata_json: String = row.get(5)
                .map_err(|e| Error::Storage(format!("Failed to get metadata: {}", e)))?;
            
            let metadata: Metadata = serde_json::from_str(&metadata_json)
                .map_err(|e| Error::Storage(format!("Failed to deserialize metadata: {}", e)))?;
            
            let processed_at_str: String = row.get(6)
                .map_err(|e| Error::Storage(format!("Failed to get processed_at: {}", e)))?;
            
            let processed_at = chrono::DateTime::parse_from_rfc3339(&processed_at_str)
                .map_err(|e| Error::Storage(format!("Failed to parse processed_at: {}", e)))?
                .with_timezone(&chrono::Utc);
            
            Ok(Document {
                id: row.get(0).map_err(|e| Error::Storage(format!("Failed to get id: {}", e)))?,
                title: row.get(1).map_err(|e| Error::Storage(format!("Failed to get title: {}", e)))?,
                content: row.get(2).map_err(|e| Error::Storage(format!("Failed to get content: {}", e)))?,
                format: row.get(3).map_err(|e| Error::Storage(format!("Failed to get format: {}", e)))?,
                source_url: row.get(4).map_err(|e| Error::Storage(format!("Failed to get source_url: {}", e)))?,
                metadata,
                processed_at,
            })
        } else {
            Err(Error::Storage(format!("Document not found: {}", id)))
        }
    }
    
    async fn list_documents(&self, limit: Option<usize>) -> Result<Vec<Document>> {
        let sql = match limit {
            Some(l) => format!("SELECT id, title, content, format, source_url, metadata, processed_at FROM documents ORDER BY created_at DESC LIMIT {}", l),
            None => "SELECT id, title, content, format, source_url, metadata, processed_at FROM documents ORDER BY created_at DESC".to_string(),
        };
        
        let mut rows = self.client.execute(&sql).await
            .map_err(|e| Error::Storage(format!("Failed to list documents: {}", e)))?;
        
        let mut documents = Vec::new();
        
        while let Some(row) = rows.next().await
            .map_err(|e| Error::Storage(format!("Failed to read row: {}", e)))? {
            
            let metadata_json: String = row.get(5)
                .map_err(|e| Error::Storage(format!("Failed to get metadata: {}", e)))?;
            
            let metadata: Metadata = serde_json::from_str(&metadata_json)
                .map_err(|e| Error::Storage(format!("Failed to deserialize metadata: {}", e)))?;
            
            let processed_at_str: String = row.get(6)
                .map_err(|e| Error::Storage(format!("Failed to get processed_at: {}", e)))?;
            
            let processed_at = chrono::DateTime::parse_from_rfc3339(&processed_at_str)
                .map_err(|e| Error::Storage(format!("Failed to parse processed_at: {}", e)))?
                .with_timezone(&chrono::Utc);
            
            documents.push(Document {
                id: row.get(0).map_err(|e| Error::Storage(format!("Failed to get id: {}", e)))?,
                title: row.get(1).map_err(|e| Error::Storage(format!("Failed to get title: {}", e)))?,
                content: row.get(2).map_err(|e| Error::Storage(format!("Failed to get content: {}", e)))?,
                format: row.get(3).map_err(|e| Error::Storage(format!("Failed to get format: {}", e)))?,
                source_url: row.get(4).map_err(|e| Error::Storage(format!("Failed to get source_url: {}", e)))?,
                metadata,
                processed_at,
            });
        }
        
        Ok(documents)
    }
    
    async fn delete_document(&self, id: &str) -> Result<()> {
        let stmt = Statement::with_args(
            "DELETE FROM documents WHERE id = ?",
            libsql_client::args![id]
        );
        
        self.client.execute(stmt).await
            .map_err(|e| Error::Storage(format!("Failed to delete document: {}", e)))?;
        
        Ok(())
    }
    
    async fn store_batch(&self, batch: &DocumentBatch) -> Result<()> {
        let document_ids_json = serde_json::to_string(&batch.document_ids)
            .map_err(|e| Error::Storage(format!("Failed to serialize document_ids: {}", e)))?;
        
        let stmt = Statement::with_args(
            "INSERT OR REPLACE INTO document_batches (id, name, description, document_ids) VALUES (?, ?, ?, ?)",
            libsql_client::args![
                batch.id.clone(),
                batch.name.clone(),
                batch.description.clone(),
                document_ids_json
            ]
        );
        
        self.client.execute(stmt).await
            .map_err(|e| Error::Storage(format!("Failed to store batch: {}", e)))?;
        
        Ok(())
    }
    
    async fn retrieve_batch(&self, id: &str) -> Result<DocumentBatch> {
        let stmt = Statement::with_args(
            "SELECT id, name, description, document_ids FROM document_batches WHERE id = ?",
            libsql_client::args![id]
        );
        
        let mut rows = self.client.execute(stmt).await
            .map_err(|e| Error::Storage(format!("Failed to query batch: {}", e)))?;
        
        if let Some(row) = rows.next().await
            .map_err(|e| Error::Storage(format!("Failed to read row: {}", e)))? {
            
            let document_ids_json: String = row.get(3)
                .map_err(|e| Error::Storage(format!("Failed to get document_ids: {}", e)))?;
            
            let document_ids: Vec<String> = serde_json::from_str(&document_ids_json)
                .map_err(|e| Error::Storage(format!("Failed to deserialize document_ids: {}", e)))?;
            
            Ok(DocumentBatch {
                id: row.get(0).map_err(|e| Error::Storage(format!("Failed to get id: {}", e)))?,
                name: row.get(1).map_err(|e| Error::Storage(format!("Failed to get name: {}", e)))?,
                description: row.get(2).map_err(|e| Error::Storage(format!("Failed to get description: {}", e)))?,
                document_ids,
            })
        } else {
            Err(Error::Storage(format!("Batch not found: {}", id)))
        }
    }
    
    async fn get_stats(&self) -> Result<StorageStats> {
        // Get document count
        let mut doc_rows = self.client.execute("SELECT COUNT(*) FROM documents").await
            .map_err(|e| Error::Storage(format!("Failed to count documents: {}", e)))?;
        
        let document_count = if let Some(row) = doc_rows.next().await
            .map_err(|e| Error::Storage(format!("Failed to read count: {}", e)))? {
            row.get::<i64>(0).map_err(|e| Error::Storage(format!("Failed to get count: {}", e)))? as usize
        } else {
            0
        };
        
        // Get batch count
        let mut batch_rows = self.client.execute("SELECT COUNT(*) FROM document_batches").await
            .map_err(|e| Error::Storage(format!("Failed to count batches: {}", e)))?;
        
        let batch_count = if let Some(row) = batch_rows.next().await
            .map_err(|e| Error::Storage(format!("Failed to read count: {}", e)))? {
            row.get::<i64>(0).map_err(|e| Error::Storage(format!("Failed to get count: {}", e)))? as usize
        } else {
            0
        };
        
        Ok(StorageStats {
            total_documents: document_count,
            total_batches: batch_count,
            storage_backend: "libSQL".to_string(),
        })
    }
} 