//! Data models for storage layer
//!
//! This module defines the data structures used for storing and retrieving
//! scraped content and metadata.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Content data structure for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredContent {
    /// Unique identifier for the content
    pub id: String,
    /// URL where the content was scraped from
    pub url: String,
    /// Domain of the source URL
    pub domain: String,
    /// Platform/scraper that extracted this content
    pub platform: String,
    /// Title of the content
    pub title: Option<String>,
    /// Main text content
    pub text: Option<String>,
    /// HTML content (if available)
    pub html: Option<String>,
    /// Extracted metadata
    pub metadata: HashMap<String, String>,
    /// Extracted links
    pub links: Vec<String>,
    /// Extracted images
    pub images: Vec<String>,
    /// Timestamp when content was scraped
    pub scraped_at: chrono::DateTime<chrono::Utc>,
    /// Timestamp when content was stored
    pub stored_at: chrono::DateTime<chrono::Utc>,
    /// Content hash for deduplication
    pub content_hash: String,
    /// Size of content in bytes
    pub size_bytes: u64,
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl StoredContent {
    pub fn new(
        url: String,
        domain: String,
        platform: String,
        title: Option<String>,
        text: Option<String>,
        html: Option<String>,
        metadata: HashMap<String, String>,
    ) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let scraped_at = chrono::Utc::now();
        let stored_at = chrono::Utc::now();
        
        // Calculate content hash for deduplication
        let content_for_hash = format!("{}{}{}", 
            title.as_deref().unwrap_or(""),
            text.as_deref().unwrap_or(""),
            url
        );
        let content_hash = format!("{:x}", md5::compute(content_for_hash.as_bytes()));
        
        // Calculate approximate size
        let size_bytes = (title.as_deref().unwrap_or("").len() +
                         text.as_deref().unwrap_or("").len() +
                         html.as_deref().unwrap_or("").len() +
                         url.len()) as u64;
        
        Self {
            id,
            url,
            domain,
            platform,
            title,
            text,
            html,
            metadata,
            links: Vec::new(),
            images: Vec::new(),
            scraped_at,
            stored_at,
            content_hash,
            size_bytes,
            tags: Vec::new(),
        }
    }
    
    /// Set extracted links
    pub fn with_links(mut self, links: Vec<String>) -> Self {
        self.links = links;
        self
    }
    
    /// Set extracted images
    pub fn with_images(mut self, images: Vec<String>) -> Self {
        self.images = images;
        self
    }
    
    /// Set tags for categorization
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
    
    /// Update the stored timestamp
    pub fn mark_stored(&mut self) {
        self.stored_at = chrono::Utc::now();
    }
}

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    /// Total number of documents stored
    pub total_documents: u64,
    /// Total size of stored content in bytes
    pub total_size_bytes: u64,
    /// Number of archived documents (S3)
    pub archived_documents: u64,
    /// Size of archived content in bytes
    pub archived_size_bytes: u64,
    /// Number of unique domains
    pub unique_domains: u64,
    /// Number of unique platforms
    pub unique_platforms: u64,
    /// Average document size in bytes
    pub avg_document_size: u64,
    /// Storage efficiency ratio (compressed/uncompressed)
    pub compression_ratio: f64,
}

impl Default for StorageStats {
    fn default() -> Self {
        Self {
            total_documents: 0,
            total_size_bytes: 0,
            archived_documents: 0,
            archived_size_bytes: 0,
            unique_domains: 0,
            unique_platforms: 0,
            avg_document_size: 0,
            compression_ratio: 1.0,
        }
    }
}

impl StorageStats {
    /// Calculate derived statistics
    pub fn calculate_derived(&mut self) {
        if self.total_documents > 0 {
            self.avg_document_size = self.total_size_bytes / self.total_documents;
        }
        
        if self.archived_size_bytes > 0 && self.total_size_bytes > 0 {
            self.compression_ratio = self.archived_size_bytes as f64 / self.total_size_bytes as f64;
        }
    }
}

/// Query parameters for content search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentQuery {
    /// Filter by URL pattern
    pub url_pattern: Option<String>,
    /// Filter by domain
    pub domain: Option<String>,
    /// Filter by platform
    pub platform: Option<String>,
    /// Filter by date range (start)
    pub scraped_after: Option<chrono::DateTime<chrono::Utc>>,
    /// Filter by date range (end)
    pub scraped_before: Option<chrono::DateTime<chrono::Utc>>,
    /// Filter by tags
    pub tags: Vec<String>,
    /// Maximum number of results
    pub limit: Option<u32>,
    /// Offset for pagination
    pub offset: Option<u32>,
    /// Sort order (newest_first, oldest_first, size_desc, size_asc)
    pub sort_by: Option<String>,
}

impl Default for ContentQuery {
    fn default() -> Self {
        Self {
            url_pattern: None,
            domain: None,
            platform: None,
            scraped_after: None,
            scraped_before: None,
            tags: Vec::new(),
            limit: Some(100),
            offset: Some(0),
            sort_by: Some("newest_first".to_string()),
        }
    }
}

/// Batch operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    /// Number of successful operations
    pub success_count: u32,
    /// Number of failed operations
    pub error_count: u32,
    /// List of error messages
    pub errors: Vec<String>,
    /// Total processing time in milliseconds
    pub processing_time_ms: u64,
}

impl BatchResult {
    pub fn new() -> Self {
        Self {
            success_count: 0,
            error_count: 0,
            errors: Vec::new(),
            processing_time_ms: 0,
        }
    }
    
    pub fn add_success(&mut self) {
        self.success_count += 1;
    }
    
    pub fn add_error(&mut self, error: String) {
        self.error_count += 1;
        self.errors.push(error);
    }
    
    pub fn is_success(&self) -> bool {
        self.error_count == 0
    }
    
    pub fn total_operations(&self) -> u32 {
        self.success_count + self.error_count
    }
}

impl Default for BatchResult {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stored_content_creation() {
        let content = StoredContent::new(
            "https://example.com".to_string(),
            "example.com".to_string(),
            "generic".to_string(),
            Some("Test Title".to_string()),
            Some("Test content".to_string()),
            None,
            HashMap::new(),
        );
        
        assert_eq!(content.url, "https://example.com");
        assert_eq!(content.domain, "example.com");
        assert_eq!(content.platform, "generic");
        assert!(!content.id.is_empty());
        assert!(!content.content_hash.is_empty());
        assert!(content.size_bytes > 0);
    }

    #[test]
    fn test_storage_stats_calculation() {
        let mut stats = StorageStats {
            total_documents: 100,
            total_size_bytes: 1000000,
            archived_size_bytes: 800000,
            ..Default::default()
        };
        
        stats.calculate_derived();
        
        assert_eq!(stats.avg_document_size, 10000);
        assert_eq!(stats.compression_ratio, 0.8);
    }

    #[test]
    fn test_batch_result() {
        let mut result = BatchResult::new();
        
        result.add_success();
        result.add_success();
        result.add_error("Test error".to_string());
        
        assert_eq!(result.success_count, 2);
        assert_eq!(result.error_count, 1);
        assert_eq!(result.total_operations(), 3);
        assert!(!result.is_success());
    }

    #[test]
    fn test_content_query_defaults() {
        let query = ContentQuery::default();
        
        assert_eq!(query.limit, Some(100));
        assert_eq!(query.offset, Some(0));
        assert_eq!(query.sort_by, Some("newest_first".to_string()));
    }
}