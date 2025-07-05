/*!
 * SQLite storage implementation for Swoop Document Processing
 */

use async_trait::async_trait;
use sqlx::{sqlite::SqlitePool, Row};
use serde_json;

use crate::error::Result;
use crate::models::{Document, DocumentBatch};
use super::{Storage, StorageStats};

/// SQLite storage implementation
#[derive(Debug, Clone)]
pub struct SqliteStorage {
    pool: SqlitePool,
}

impl SqliteStorage {
    /// Create a new SQLite storage
    pub async fn new(connection_string: &str) -> Result<Self> {
        let pool = SqlitePool::connect(connection_string).await?;
        let storage = Self { pool };
        storage.run_migrations().await?;
        Ok(storage)
    }

    /// Run database migrations
    async fn run_migrations(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                summary TEXT,
                metadata TEXT NOT NULL,
                quality_score REAL,
                content_hash TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                source_url TEXT,
                document_type TEXT,
                language TEXT,
                word_count INTEGER,
                size_bytes INTEGER,
                content_type TEXT,
                file_size INTEGER,
                extracted_at INTEGER NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS document_batches (
                id TEXT PRIMARY KEY,
                document_ids TEXT NOT NULL,
                total_documents INTEGER NOT NULL,
                status TEXT NOT NULL,
                created_at INTEGER NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Convert Document to database values
    fn document_to_values(doc: &Document) -> Result<(String, String, String, Option<String>, String, Option<f64>, Option<String>, i64, i64, Option<String>, Option<String>, Option<String>, Option<i64>, Option<i64>, Option<String>, Option<i64>, i64)> {
        let metadata_json = serde_json::to_string(&doc.metadata)?;
        
        Ok((
            doc.id.clone(),
            doc.title.clone(),
            doc.content.clone(),
            doc.summary.clone(),
            metadata_json,
            doc.quality_score,
            doc.content_hash.clone(),
            doc.created_at.timestamp(),
            doc.updated_at.timestamp(),
            doc.source_url.clone(),
            doc.document_type.clone(),
            doc.language.clone(),
            doc.word_count.map(|c| c as i64),
            doc.size_bytes.map(|s| s as i64),
            doc.content_type.clone(),
            doc.file_size.map(|s| s as i64),
            doc.extracted_at.timestamp(),
        ))
    }

    /// Convert database row to Document
    fn row_to_document(row: &sqlx::sqlite::SqliteRow) -> Result<Document> {
        let metadata_json: String = row.try_get("metadata")?;
        let metadata = serde_json::from_str(&metadata_json)?;
        
        Ok(Document {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            content: row.try_get("content")?,
            summary: row.try_get("summary")?,
            metadata,
            quality_score: row.try_get("quality_score")?,
            content_hash: row.try_get("content_hash")?,
            created_at: chrono::DateTime::from_timestamp(row.try_get("created_at")?, 0)
                .unwrap_or_else(chrono::Utc::now),
            updated_at: chrono::DateTime::from_timestamp(row.try_get("updated_at")?, 0)
                .unwrap_or_else(chrono::Utc::now),
            source_url: row.try_get("source_url")?,
            document_type: row.try_get("document_type")?,
            language: row.try_get("language")?,
            word_count: row.try_get::<Option<i64>, _>("word_count")?.map(|c| c as usize),
            size_bytes: row.try_get::<Option<i64>, _>("size_bytes")?.map(|s| s as u64),
            content_type: row.try_get("content_type")?,
            file_size: row.try_get::<Option<i64>, _>("file_size")?.map(|s| s as u64),
            extracted_at: chrono::DateTime::from_timestamp(row.try_get("extracted_at")?, 0)
                .unwrap_or_else(chrono::Utc::now),
        })
    }
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn store_document(&self, document: &Document) -> Result<()> {
        let values = Self::document_to_values(document)?;
        
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO documents (
                id, title, content, summary, metadata, quality_score, content_hash,
                created_at, updated_at, source_url, document_type, language, 
                word_count, size_bytes, content_type, file_size, extracted_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&values.0)  // id
        .bind(&values.1)  // title
        .bind(&values.2)  // content
        .bind(&values.3)  // summary
        .bind(&values.4)  // metadata
        .bind(values.5)  // quality_score
        .bind(&values.6)  // content_hash
        .bind(values.7)  // created_at
        .bind(values.8)  // updated_at
        .bind(&values.9)  // source_url
        .bind(&values.10) // document_type
        .bind(&values.11) // language
        .bind(values.12) // word_count
        .bind(values.13) // size_bytes
        .bind(&values.14) // content_type
        .bind(values.15) // file_size
        .bind(values.16) // extracted_at
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn retrieve_document(&self, id: &str) -> Result<Option<Document>> {
        let row = sqlx::query("SELECT * FROM documents WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_document(&row)?)),
            None => Ok(None),
        }
    }

    async fn list_documents(&self) -> Result<Vec<String>> {
        let rows = sqlx::query("SELECT id FROM documents ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        let mut document_ids = Vec::new();
        for row in rows {
            document_ids.push(row.try_get("id")?);
        }

        Ok(document_ids)
    }
    
    async fn delete_document(&self, id: &str) -> Result<()> {
        let result = sqlx::query("DELETE FROM documents WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(crate::error::Error::Storage(format!("Document with id {id} not found")));
        }

        Ok(())
    }
    
    async fn store_batch(&self, batch: &DocumentBatch) -> Result<()> {
        let document_ids_json = serde_json::to_string(&batch.document_ids)?;
        
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO document_batches (
                id, document_ids, total_documents, status, created_at
            ) VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(&batch.id)
        .bind(&document_ids_json)
        .bind(batch.total_documents as i64)
        .bind(&batch.status)
        .bind(batch.created_at.timestamp())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn retrieve_batch(&self, id: &str) -> Result<Option<DocumentBatch>> {
        let row = sqlx::query("SELECT * FROM document_batches WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => {
                let document_ids_json: String = row.try_get("document_ids")?;
                let document_ids: Vec<String> = serde_json::from_str(&document_ids_json)?;

                Ok(Some(DocumentBatch {
                    id: row.try_get("id")?,
                    document_ids,
                    total_documents: row.try_get::<i64, _>("total_documents")? as usize,
                    status: row.try_get("status")?,
                    created_at: chrono::DateTime::from_timestamp(row.try_get("created_at")?, 0)
                        .unwrap_or_else(chrono::Utc::now),
                }))
            }
            None => Ok(None),
        }
    }
    
    async fn health_check(&self) -> Result<bool> {
        // Try a simple query to check if the database is accessible
        let result = sqlx::query("SELECT 1")
            .fetch_optional(&self.pool)
            .await;
        
        Ok(result.is_ok())
    }
}

// Additional convenience methods not part of the Storage trait
impl SqliteStorage {
    /// Get document (direct access, not part of trait)
    pub async fn get_document(&self, id: &str) -> Result<Option<Document>> {
        self.retrieve_document(id).await
    }
    
    /// Update document (not part of trait)
    pub async fn update_document(&self, document: &Document) -> Result<()> {
        // Same as store_document for SQLite (INSERT OR REPLACE)
        self.store_document(document).await
    }

    /// List documents with limit (not part of trait)
    pub async fn list_documents_with_limit(&self, limit: Option<usize>) -> Result<Vec<Document>> {
        let limit = limit.unwrap_or(50) as i64;
        
        let rows = sqlx::query("SELECT * FROM documents ORDER BY created_at DESC LIMIT ?")
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;

        let mut documents = Vec::new();
        for row in rows {
            documents.push(Self::row_to_document(&row)?);
        }

        Ok(documents)
    }
    
    /// Search documents (not part of trait)
    pub async fn search_documents(&self, query: &str, limit: Option<usize>) -> Result<Vec<Document>> {
        let limit = limit.unwrap_or(50) as i64;
        
        // Simple text search in title and content
        let rows = sqlx::query(
            r#"
            SELECT * FROM documents 
            WHERE title LIKE ? OR content LIKE ?
            ORDER BY 
                CASE 
                    WHEN title LIKE ? THEN 1 
                    ELSE 2 
                END,
                created_at DESC
            LIMIT ?
            "#
        )
        .bind(format!("%{query}%"))
        .bind(format!("%{query}%"))
        .bind(format!("%{query}%"))
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut documents = Vec::new();
        for row in rows {
            documents.push(Self::row_to_document(&row)?);
        }

        Ok(documents)
    }
    
    /// Get storage statistics (not part of trait)
    pub async fn get_stats(&self) -> Result<StorageStats> {
        let stats_row = sqlx::query(
            r#"
            SELECT 
                COUNT(*) as total_documents,
                SUM(LENGTH(content)) as total_size_bytes
            FROM documents
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        let batches_row = sqlx::query("SELECT COUNT(*) as total_batches FROM document_batches")
            .fetch_one(&self.pool)
            .await?;

        Ok(StorageStats {
            total_documents: stats_row.try_get::<i64, _>("total_documents")? as usize,
            total_batches: batches_row.try_get::<i64, _>("total_batches")? as usize,
            storage_backend: "sqlite".to_string(),
            total_size_bytes: stats_row.try_get::<Option<i64>, _>("total_size_bytes")?.unwrap_or(0) as u64,
            successful_operations: 0, // Would need separate tracking
            failed_operations: 0,     // Would need separate tracking
            avg_operation_time_ms: 0.0, // Would need separate tracking
        })
    }
} 