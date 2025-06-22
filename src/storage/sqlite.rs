/*!
 * SQLite storage implementation for Swoop Document Processing
 */

use async_trait::async_trait;
use sqlx::{sqlite::SqlitePool, Row, Sqlite, Transaction};
use serde_json;
use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::models::{Document, DocumentBatch, Metadata, Link, ExtractedContent};
use super::{Storage, StorageStats};

/// SQLite storage implementation
pub struct SqliteStorage {
    pool: SqlitePool,
}

impl SqliteStorage {
    /// Create a new SQLite storage instance
    pub async fn new(connection_string: &str) -> Result<Self> {
        let pool = SqlitePool::connect(connection_string)
            .await
            .map_err(|e| Error::Storage(format!("Failed to connect to SQLite: {}", e)))?;
        
        let storage = Self { pool };
        storage.run_migrations().await?;
        Ok(storage)
    }

    /// Run database migrations
    async fn run_migrations(&self) -> Result<()> {
        // Create documents table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                url TEXT NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                html TEXT NOT NULL,
                text TEXT NOT NULL,
                content_type TEXT,
                file_size INTEGER,
                extracted_at DATETIME NOT NULL,
                
                -- Metadata fields (flattened for better querying)
                metadata_url TEXT NOT NULL,
                metadata_content_type TEXT NOT NULL,
                metadata_fetch_time DATETIME NOT NULL,
                metadata_status_code INTEGER NOT NULL,
                metadata_headers TEXT NOT NULL, -- JSON
                
                -- Serialized complex fields
                links TEXT NOT NULL, -- JSON array
                extracted TEXT NOT NULL, -- JSON object
                
                -- Search optimization
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::Storage(format!("Failed to create documents table: {}", e)))?;

        // Create indexes for better query performance
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_documents_url ON documents(url)")
            .execute(&self.pool)
            .await
            .map_err(|e| Error::Storage(format!("Failed to create URL index: {}", e)))?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_documents_title ON documents(title)")
            .execute(&self.pool)
            .await
            .map_err(|e| Error::Storage(format!("Failed to create title index: {}", e)))?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_documents_extracted_at ON documents(extracted_at)")
            .execute(&self.pool)
            .await
            .map_err(|e| Error::Storage(format!("Failed to create extracted_at index: {}", e)))?;

        // Create FTS table for full-text search
        sqlx::query(
            r#"
            CREATE VIRTUAL TABLE IF NOT EXISTS documents_fts USING fts5(
                id UNINDEXED,
                title,
                content,
                text,
                content='documents',
                content_rowid='rowid'
            )
            "#
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::Storage(format!("Failed to create FTS table: {}", e)))?;

        // Create triggers to keep FTS table in sync
        sqlx::query(
            r#"
            CREATE TRIGGER IF NOT EXISTS documents_fts_insert AFTER INSERT ON documents
            BEGIN
                INSERT INTO documents_fts(id, title, content, text) 
                VALUES (NEW.id, NEW.title, NEW.content, NEW.text);
            END
            "#
        )
        .execute(&self.pool)
        .await
        .ok(); // Ignore if trigger already exists

        sqlx::query(
            r#"
            CREATE TRIGGER IF NOT EXISTS documents_fts_update AFTER UPDATE ON documents
            BEGIN
                UPDATE documents_fts SET title=NEW.title, content=NEW.content, text=NEW.text
                WHERE id=NEW.id;
            END
            "#
        )
        .execute(&self.pool)
        .await
        .ok(); // Ignore if trigger already exists

        sqlx::query(
            r#"
            CREATE TRIGGER IF NOT EXISTS documents_fts_delete AFTER DELETE ON documents
            BEGIN
                DELETE FROM documents_fts WHERE id=OLD.id;
            END
            "#
        )
        .execute(&self.pool)
        .await
        .ok(); // Ignore if trigger already exists

        Ok(())
    }

    /// Convert database row to Document
    fn row_to_document(&self, row: &sqlx::sqlite::SqliteRow) -> Result<Document> {
        let links_json: String = row.try_get("links")?;
        let links: Vec<Link> = serde_json::from_str(&links_json)
            .map_err(|e| Error::Storage(format!("Failed to deserialize links: {}", e)))?;

        let extracted_json: String = row.try_get("extracted")?;
        let extracted: HashMap<String, ExtractedContent> = serde_json::from_str(&extracted_json)
            .map_err(|e| Error::Storage(format!("Failed to deserialize extracted content: {}", e)))?;

        let headers_json: String = row.try_get("metadata_headers")?;
        let headers: HashMap<String, String> = serde_json::from_str(&headers_json)
            .map_err(|e| Error::Storage(format!("Failed to deserialize headers: {}", e)))?;

        let metadata = Metadata {
            url: row.try_get("metadata_url")?,
            content_type: row.try_get("metadata_content_type")?,
            fetch_time: row.try_get("metadata_fetch_time")?,
            status_code: row.try_get::<i64, _>("metadata_status_code")? as u16,
            headers,
        };

        Ok(Document {
            id: row.try_get("id")?,
            url: row.try_get("url")?,
            title: row.try_get("title")?,
            content: row.try_get("content")?,
            html: row.try_get("html")?,
            text: row.try_get("text")?,
            metadata,
            links,
            extracted,
            content_type: row.try_get("content_type")?,
            file_size: row.try_get::<Option<i64>, _>("file_size")?.map(|v| v as u64),
            extracted_at: row.try_get("extracted_at")?,
        })
    }
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn store_document(&self, document: &Document) -> Result<()> {
        let links_json = serde_json::to_string(&document.links)
            .map_err(|e| Error::Storage(format!("Failed to serialize links: {}", e)))?;
        
        let extracted_json = serde_json::to_string(&document.extracted)
            .map_err(|e| Error::Storage(format!("Failed to serialize extracted content: {}", e)))?;
        
        let headers_json = serde_json::to_string(&document.metadata.headers)
            .map_err(|e| Error::Storage(format!("Failed to serialize headers: {}", e)))?;

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO documents (
                id, url, title, content, html, text, content_type, file_size, extracted_at,
                metadata_url, metadata_content_type, metadata_fetch_time, metadata_status_code, metadata_headers,
                links, extracted, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
            "#
        )
        .bind(&document.id)
        .bind(&document.url)
        .bind(&document.title)
        .bind(&document.content)
        .bind(&document.html)
        .bind(&document.text)
        .bind(&document.content_type)
        .bind(document.file_size.map(|s| s as i64))
        .bind(&document.extracted_at)
        .bind(&document.metadata.url)
        .bind(&document.metadata.content_type)
        .bind(&document.metadata.fetch_time)
        .bind(document.metadata.status_code as i64)
        .bind(&headers_json)
        .bind(&links_json)
        .bind(&extracted_json)
        .execute(&self.pool)
        .await
        .map_err(|e| Error::Storage(format!("Failed to store document: {}", e)))?;

        Ok(())
    }
    
    async fn get_document(&self, id: &str) -> Result<Option<Document>> {
        let row = sqlx::query("SELECT * FROM documents WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::Storage(format!("Failed to get document: {}", e)))?;

        match row {
            Some(row) => Ok(Some(self.row_to_document(&row)?)),
            None => Ok(None),
        }
    }
    
    async fn update_document(&self, document: &Document) -> Result<()> {
        // Same as store_document since we use INSERT OR REPLACE
        self.store_document(document).await
    }
    
    async fn delete_document(&self, id: &str) -> Result<()> {
        let result = sqlx::query("DELETE FROM documents WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| Error::Storage(format!("Failed to delete document: {}", e)))?;

        if result.rows_affected() == 0 {
            return Err(Error::Storage(format!("Document with id {} not found", id)));
        }

        Ok(())
    }
    
    async fn search_documents(&self, query: &str, limit: Option<usize>) -> Result<Vec<Document>> {
        let limit = limit.unwrap_or(50);
        
        // Use FTS for full-text search
        let rows = sqlx::query(
            r#"
            SELECT d.* FROM documents d
            INNER JOIN documents_fts fts ON d.id = fts.id
            WHERE documents_fts MATCH ?
            ORDER BY rank
            LIMIT ?
            "#
        )
        .bind(query)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::Storage(format!("Failed to search documents: {}", e)))?;

        let mut documents = Vec::new();
        for row in rows {
            documents.push(self.row_to_document(&row)?);
        }

        Ok(documents)
    }
    
    async fn store_batch(&self, batch: &DocumentBatch) -> Result<()> {
        let mut tx = self.pool.begin().await
            .map_err(|e| Error::Storage(format!("Failed to start transaction: {}", e)))?;

        for document in &batch.documents {
            let links_json = serde_json::to_string(&document.links)
                .map_err(|e| Error::Storage(format!("Failed to serialize links: {}", e)))?;
            
            let extracted_json = serde_json::to_string(&document.extracted)
                .map_err(|e| Error::Storage(format!("Failed to serialize extracted content: {}", e)))?;
            
            let headers_json = serde_json::to_string(&document.metadata.headers)
                .map_err(|e| Error::Storage(format!("Failed to serialize headers: {}", e)))?;

            sqlx::query(
                r#"
                INSERT OR REPLACE INTO documents (
                    id, url, title, content, html, text, content_type, file_size, extracted_at,
                    metadata_url, metadata_content_type, metadata_fetch_time, metadata_status_code, metadata_headers,
                    links, extracted, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
                "#
            )
            .bind(&document.id)
            .bind(&document.url)
            .bind(&document.title)
            .bind(&document.content)
            .bind(&document.html)
            .bind(&document.text)
            .bind(&document.content_type)
            .bind(document.file_size.map(|s| s as i64))
            .bind(&document.extracted_at)
            .bind(&document.metadata.url)
            .bind(&document.metadata.content_type)
            .bind(&document.metadata.fetch_time)
            .bind(document.metadata.status_code as i64)
            .bind(&headers_json)
            .bind(&links_json)
            .bind(&extracted_json)
            .execute(&mut *tx)
            .await
            .map_err(|e| Error::Storage(format!("Failed to store document in batch: {}", e)))?;
        }

        tx.commit().await
            .map_err(|e| Error::Storage(format!("Failed to commit batch transaction: {}", e)))?;

        Ok(())
    }
    
    async fn get_stats(&self) -> Result<StorageStats> {
        let stats_row = sqlx::query(
            r#"
            SELECT 
                COUNT(*) as total_documents,
                SUM(LENGTH(content) + LENGTH(html) + LENGTH(text)) as total_size_bytes
            FROM documents
            "#
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| Error::Storage(format!("Failed to get stats: {}", e)))?;

        Ok(StorageStats {
            total_documents: stats_row.try_get::<i64, _>("total_documents")? as u64,
            total_size_bytes: stats_row.try_get::<Option<i64>, _>("total_size_bytes")?.unwrap_or(0) as u64,
            successful_operations: 0, // Would need separate tracking
            failed_operations: 0,     // Would need separate tracking
            avg_operation_time_ms: 0.0, // Would need separate tracking
        })
    }
} 