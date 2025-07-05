/*!
 * PostgreSQL storage backend for Swoop
 * 
 * This module implements the Storage trait using PostgreSQL with sqlx.
 * It provides persistent storage with SQL queries and proper error handling.
 */

use async_trait::async_trait;
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::error::{Result, SwoopError};
use crate::models::{Document, DocumentBatch, CrawlPage, VectorRecord};
use crate::storage::Storage;

/// PostgreSQL storage backend
#[derive(Debug, Clone)]
pub struct PostgresStorage {
    pool: PgPool,
}

impl PostgresStorage {
    /// Create a new PostgreSQL storage instance
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPool::connect(database_url)
            .await
            .map_err(|e| SwoopError::DatabaseConnection(format!("Failed to connect to PostgreSQL: {}", e)))?;
        
        Ok(Self { pool })
    }

    /// Initialize database tables (called during startup)
    pub async fn initialize(&self) -> Result<()> {
        // Create documents table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS documents (
                id VARCHAR PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                summary TEXT,
                quality_score REAL,
                content_hash VARCHAR,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                source_url TEXT,
                document_type VARCHAR,
                language VARCHAR,
                word_count INTEGER,
                size_bytes BIGINT,
                content_type VARCHAR,
                file_size BIGINT,
                extracted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                metadata JSONB DEFAULT '{}'::jsonb
            )
        "#)
        .execute(&self.pool)
        .await
        .map_err(|e| SwoopError::DatabaseOperation(format!("Failed to create documents table: {}", e)))?;

        // Create document_batches table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS document_batches (
                id VARCHAR PRIMARY KEY,
                document_ids JSONB NOT NULL,
                total_documents INTEGER NOT NULL,
                status VARCHAR NOT NULL DEFAULT 'pending',
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
        "#)
        .execute(&self.pool)
        .await
        .map_err(|e| SwoopError::DatabaseOperation(format!("Failed to create document_batches table: {}", e)))?;

        // Create crawl_pages table
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS crawl_pages (
                id VARCHAR PRIMARY KEY,
                job_id VARCHAR NOT NULL,
                url TEXT NOT NULL,
                status_code SMALLINT NOT NULL,
                text_length INTEGER NOT NULL,
                fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
        "#)
        .execute(&self.pool)
        .await
        .map_err(|e| SwoopError::DatabaseOperation(format!("Failed to create crawl_pages table: {}", e)))?;

        // Create vector_records table (for AI embeddings)
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS vector_records (
                id VARCHAR PRIMARY KEY,
                document_id VARCHAR NOT NULL,
                vector REAL[] NOT NULL,
                metadata JSONB DEFAULT '{}'::jsonb,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
            )
        "#)
        .execute(&self.pool)
        .await
        .map_err(|e| SwoopError::DatabaseOperation(format!("Failed to create vector_records table: {}", e)))?;

        // Create indexes for better performance
        let indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_documents_created_at ON documents(created_at)",
            "CREATE INDEX IF NOT EXISTS idx_documents_document_type ON documents(document_type)",
            "CREATE INDEX IF NOT EXISTS idx_documents_source_url ON documents(source_url)",
            "CREATE INDEX IF NOT EXISTS idx_crawl_pages_job_id ON crawl_pages(job_id)",
            "CREATE INDEX IF NOT EXISTS idx_crawl_pages_fetched_at ON crawl_pages(fetched_at)",
            "CREATE INDEX IF NOT EXISTS idx_vector_records_document_id ON vector_records(document_id)",
        ];

        for index_sql in indexes {
            sqlx::query(index_sql)
                .execute(&self.pool)
                .await
                .map_err(|e| SwoopError::DatabaseOperation(format!("Failed to create index: {}", e)))?;
        }

        Ok(())
    }

    /// Convert database row to Document model
    fn row_to_document(row: &sqlx::postgres::PgRow) -> Result<Document> {
        let metadata_json: serde_json::Value = row.try_get("metadata")
            .unwrap_or_else(|_| serde_json::json!({}));
        
        let custom_metadata: HashMap<String, String> = if let serde_json::Value::Object(map) = metadata_json {
            map.into_iter()
                .filter_map(|(k, v)| {
                    if let serde_json::Value::String(s) = v {
                        Some((k, s))
                    } else {
                        Some((k, v.to_string()))
                    }
                })
                .collect()
        } else {
            HashMap::new()
        };

        Ok(Document {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            content: row.try_get("content")?,
            summary: row.try_get("summary")?,
            quality_score: row.try_get("quality_score")?,
            content_hash: row.try_get("content_hash")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            source_url: row.try_get("source_url")?,
            document_type: row.try_get("document_type")?,
            language: row.try_get("language")?,
            word_count: row.try_get::<Option<i32>, _>("word_count")?.map(|w| w as usize),
            size_bytes: row.try_get::<Option<i64>, _>("size_bytes")?.map(|s| s as u64),
            content_type: row.try_get("content_type")?,
            file_size: row.try_get::<Option<i64>, _>("file_size")?.map(|s| s as u64),
            extracted_at: row.try_get("extracted_at")?,
            metadata: crate::models::Metadata {
                source_url: row.try_get("source_url")?,
                content_type: row.try_get("content_type")?,
                processed_at: row.try_get("extracted_at")?,
                processor: None,
                custom: custom_metadata,
                file_extension: None,
                original_filename: None,
            },
        })
    }
}

#[async_trait]
impl Storage for PostgresStorage {
    async fn store_document(&self, document: &Document) -> Result<()> {
        let metadata_json = serde_json::to_value(&document.metadata.custom)
            .map_err(|e| SwoopError::SerializationError(format!("Failed to serialize metadata: {}", e)))?;

        sqlx::query(r#"
            INSERT INTO documents (
                id, title, content, summary, quality_score, content_hash,
                created_at, updated_at, source_url, document_type, language,
                word_count, size_bytes, content_type, file_size, extracted_at, metadata
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            ON CONFLICT (id) DO UPDATE SET
                title = EXCLUDED.title,
                content = EXCLUDED.content,
                summary = EXCLUDED.summary,
                quality_score = EXCLUDED.quality_score,
                content_hash = EXCLUDED.content_hash,
                updated_at = EXCLUDED.updated_at,
                source_url = EXCLUDED.source_url,
                document_type = EXCLUDED.document_type,
                language = EXCLUDED.language,
                word_count = EXCLUDED.word_count,
                size_bytes = EXCLUDED.size_bytes,
                content_type = EXCLUDED.content_type,
                file_size = EXCLUDED.file_size,
                extracted_at = EXCLUDED.extracted_at,
                metadata = EXCLUDED.metadata
        "#)
        .bind(&document.id)
        .bind(&document.title)
        .bind(&document.content)
        .bind(&document.summary)
        .bind(document.quality_score)
        .bind(&document.content_hash)
        .bind(document.created_at)
        .bind(document.updated_at)
        .bind(&document.source_url)
        .bind(&document.document_type)
        .bind(&document.language)
        .bind(document.word_count.map(|w| w as i32))
        .bind(document.size_bytes.map(|s| s as i64))
        .bind(&document.content_type)
        .bind(document.file_size.map(|s| s as i64))
        .bind(document.extracted_at)
        .bind(metadata_json)
        .execute(&self.pool)
        .await
        .map_err(|e| SwoopError::DatabaseOperation(format!("Failed to store document: {}", e)))?;

        Ok(())
    }

    async fn retrieve_document(&self, id: &str) -> Result<Option<Document>> {
        let row = sqlx::query(r#"
            SELECT id, title, content, summary, quality_score, content_hash,
                   created_at, updated_at, source_url, document_type, language,
                   word_count, size_bytes, content_type, file_size, extracted_at, metadata
            FROM documents 
            WHERE id = $1
        "#)
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| SwoopError::DatabaseOperation(format!("Failed to retrieve document: {}", e)))?;

        match row {
            Some(row) => Ok(Some(Self::row_to_document(&row)?)),
            None => Ok(None),
        }
    }

    async fn list_documents(&self) -> Result<Vec<String>> {
        let rows = sqlx::query("SELECT id FROM documents ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| SwoopError::DatabaseOperation(format!("Failed to list documents: {}", e)))?;

        Ok(rows.into_iter().map(|row| row.get("id")).collect())
    }

    async fn delete_document(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM documents WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| SwoopError::DatabaseOperation(format!("Failed to delete document: {}", e)))?;

        Ok(())
    }

    async fn store_batch(&self, batch: &DocumentBatch) -> Result<()> {
        let document_ids_json = serde_json::to_value(&batch.document_ids)
            .map_err(|e| SwoopError::SerializationError(format!("Failed to serialize document IDs: {}", e)))?;

        sqlx::query(r#"
            INSERT INTO document_batches (id, document_ids, total_documents, status, created_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                document_ids = EXCLUDED.document_ids,
                total_documents = EXCLUDED.total_documents,
                status = EXCLUDED.status
        "#)
        .bind(&batch.id)
        .bind(document_ids_json)
        .bind(batch.total_documents as i32)
        .bind(&batch.status)
        .bind(batch.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| SwoopError::DatabaseOperation(format!("Failed to store batch: {}", e)))?;

        Ok(())
    }

    async fn retrieve_batch(&self, id: &str) -> Result<Option<DocumentBatch>> {
        let row = sqlx::query(r#"
            SELECT id, document_ids, total_documents, status, created_at
            FROM document_batches 
            WHERE id = $1
        "#)
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| SwoopError::DatabaseOperation(format!("Failed to retrieve batch: {}", e)))?;

        match row {
            Some(row) => {
                let document_ids_json: serde_json::Value = row.try_get("document_ids")?;
                let document_ids: Vec<String> = serde_json::from_value(document_ids_json)
                    .map_err(|e| SwoopError::SerializationError(format!("Failed to deserialize document IDs: {}", e)))?;

                Ok(Some(DocumentBatch {
                    id: row.try_get("id")?,
                    document_ids,
                    total_documents: row.try_get::<i32, _>("total_documents")? as usize,
                    status: row.try_get("status")?,
                    created_at: row.try_get("created_at")?,
                }))
            }
            None => Ok(None),
        }
    }

    async fn health_check(&self) -> Result<bool> {
        match sqlx::query("SELECT 1 as health_check")
            .fetch_one(&self.pool)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                log::warn!("PostgreSQL health check failed: {}", e);
                Ok(false)
            }
        }
    }

    async fn store_crawl_page(&self, page: &CrawlPage) -> Result<()> {
        sqlx::query(r#"
            INSERT INTO crawl_pages (id, job_id, url, status_code, text_length, fetched_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE SET
                job_id = EXCLUDED.job_id,
                url = EXCLUDED.url,
                status_code = EXCLUDED.status_code,
                text_length = EXCLUDED.text_length,
                fetched_at = EXCLUDED.fetched_at
        "#)
        .bind(&page.id)
        .bind(&page.job_id)
        .bind(&page.url)
        .bind(page.status_code as i16)
        .bind(page.text_length as i32)
        .bind(page.fetched_at)
        .execute(&self.pool)
        .await
        .map_err(|e| SwoopError::DatabaseOperation(format!("Failed to store crawl page: {}", e)))?;

        Ok(())
    }

    async fn store_document_vector(&self, vector_record: &VectorRecord) -> Result<()> {
        sqlx::query(r#"
            INSERT INTO vector_records (id, document_id, vector, metadata, created_at)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                document_id = EXCLUDED.document_id,
                vector = EXCLUDED.vector,
                metadata = EXCLUDED.metadata,
                created_at = EXCLUDED.created_at
        "#)
        .bind(&vector_record.id)
        .bind(&vector_record.document_id)
        .bind(&vector_record.vector)
        .bind(serde_json::to_value(&vector_record.metadata)
            .map_err(|e| SwoopError::SerializationError(format!("Failed to serialize vector metadata: {}", e)))?)
        .bind(vector_record.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| SwoopError::DatabaseOperation(format!("Failed to store vector record: {}", e)))?;

        Ok(())
    }

    async fn list_crawl_pages(&self, job_id: &str, offset: usize, limit: usize) -> Result<Vec<CrawlPage>> {
        let rows = sqlx::query(r#"
            SELECT id, job_id, url, status_code, text_length, fetched_at
            FROM crawl_pages 
            WHERE job_id = $1
            ORDER BY fetched_at DESC
            OFFSET $2 LIMIT $3
        "#)
        .bind(job_id)
        .bind(offset as i64)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| SwoopError::DatabaseOperation(format!("Failed to list crawl pages: {}", e)))?;

        let mut pages = Vec::new();
        for row in rows {
            pages.push(CrawlPage {
                id: row.try_get("id")?,
                job_id: row.try_get("job_id")?,
                url: row.try_get("url")?,
                status_code: row.try_get::<i16, _>("status_code")? as u16,
                text_length: row.try_get::<i32, _>("text_length")? as usize,
                fetched_at: row.try_get("fetched_at")?,
            });
        }

        Ok(pages)
    }
}