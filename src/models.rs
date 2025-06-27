/*!
 * Models module for Crawl4AI
 * 
 * This module defines the core data structures used throughout the Crawl4AI system,
 * including documents, links, metadata, and extraction results.
 */

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// A document represents a processed document with extracted content and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Unique identifier for the document
    pub id: String,
    /// Title of the document
    pub title: String,
    /// Main content of the document
    pub content: String,
    /// Optional summary of the document
    pub summary: Option<String>,
    /// Document metadata
    pub metadata: Metadata,
    /// Quality score of the document (0.0 to 1.0)
    pub quality_score: Option<f64>,
    /// Hash of the content for deduplication
    pub content_hash: Option<String>,
    /// When the document was created
    pub created_at: DateTime<Utc>,
    /// When the document was last updated
    pub updated_at: DateTime<Utc>,
    /// Source URL if available
    pub source_url: Option<String>,
    /// Type of document (pdf, markdown, html, etc.)
    pub document_type: Option<String>,
    /// Language of the document
    pub language: Option<String>,
    /// Word count
    pub word_count: Option<usize>,
    /// Size in bytes
    pub size_bytes: Option<u64>,
    /// Content type detected
    pub content_type: Option<String>,
    /// File size in bytes
    pub file_size: Option<u64>,
    /// When the document was extracted/processed
    pub extracted_at: DateTime<Utc>,
}

impl Document {
    /// Create a new document
    pub fn new(title: &str, content: &str) -> Self {
        let id = format!("doc_{}", uuid::Uuid::new_v4().to_string().replace('-', "")[..8].to_string());
        
        Self {
            id,
            title: title.to_string(),
            content: content.to_string(),
            summary: None,
            metadata: Metadata::default(),
            quality_score: None,
            content_hash: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            source_url: None,
            document_type: None,
            language: None,
            word_count: Some(content.split_whitespace().count()),
            size_bytes: Some(content.len() as u64),
            content_type: None,
            file_size: None,
            extracted_at: Utc::now(),
        }
    }
}

/// Metadata about a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// Original source URL
    pub source_url: Option<String>,
    /// Content type detected
    pub content_type: Option<String>,
    /// Processing timestamp
    pub processed_at: DateTime<Utc>,
    /// Processing model or method used
    pub processor: Option<String>,
    /// Additional custom metadata
    pub custom: HashMap<String, String>,
    /// File extension if applicable
    pub file_extension: Option<String>,
    /// Original filename if available
    pub original_filename: Option<String>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            source_url: None,
            content_type: None,
            processed_at: Utc::now(),
            processor: None,
            custom: HashMap::new(),
            file_extension: None,
            original_filename: None,
        }
    }
}

/// A link found in a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    /// URL of the link
    pub url: String,
    /// Text of the link
    pub text: String,
    /// Source URL where the link was found
    pub source_url: String,
    /// Relationship attribute (if available)
    pub rel: Option<String>,
}

/// Extracted structured content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedContent {
    /// Type of the extracted content
    pub content_type: String,
    /// Name or identifier for the extracted content
    pub name: String,
    /// The extracted content as a string
    pub content: String,
    /// Additional attributes or metadata
    pub attributes: HashMap<String, String>,
}

/// Result of a crawl operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlResult {
    /// Unique identifier for the crawl result
    pub id: String,
    /// URL that was crawled
    pub url: String,
    /// Success or failure
    pub success: bool,
    /// Error message (if any)
    pub error: Option<String>,
    /// Document (if successful)
    pub document: Option<Document>,
    /// Time when the crawl was performed
    pub crawl_time: DateTime<Utc>,
}

/// Vector representation of a document for semantic search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVector {
    /// Unique identifier for the document
    pub id: String,
    /// URL of the document
    pub url: String,
    /// Vector representation of the document
    pub vector: Vec<f32>,
    /// Metadata about the document
    pub metadata: HashMap<String, String>,
}

/// A batch of documents for efficient processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentBatch {
    /// Unique identifier for the batch
    pub id: String,
    /// Document IDs in the batch
    pub document_ids: Vec<String>,
    /// Total number of documents in the batch
    pub total_documents: usize,
    /// Processing status
    pub status: String,
    /// Time when the batch was created
    pub created_at: DateTime<Utc>,
}

impl DocumentBatch {
    /// Create a new document batch
    pub fn new(document_ids: Vec<String>) -> Self {
        let total_documents = document_ids.len();
        let id = format!("batch_{}", uuid::Uuid::new_v4().to_string().replace('-', "")[..8].to_string());
        
        Self {
            id,
            document_ids,
            total_documents,
            status: "pending".to_string(),
            created_at: Utc::now(),
        }
    }
}

/// A summary of a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSummary {
    /// Unique identifier for the document
    pub id: String,
    /// URL of the document
    pub url: String,
    /// Title of the document
    pub title: String,
    /// Summary of the document
    pub summary: String,
    /// Keywords or tags
    pub keywords: Vec<String>,
    /// Time when the summary was created
    pub created_at: DateTime<Utc>,
}

/// An entity extracted from a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// Name of the entity
    pub name: String,
    /// Type of the entity (person, organization, location, etc.)
    pub entity_type: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
    /// Context where the entity was found
    pub context: String,
    /// Document ID where the entity was found
    pub document_id: String,
}

/// A sentiment analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sentiment {
    /// Overall sentiment score (-1.0 to 1.0)
    pub score: f32,
    /// Sentiment label (positive, negative, neutral)
    pub label: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
    /// Document ID where the sentiment was analyzed
    pub document_id: String,
}

/// A topic extracted from a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    /// Name of the topic
    pub name: String,
    /// Keywords related to the topic
    pub keywords: Vec<String>,
    /// Relevance score (0.0 to 1.0)
    pub relevance: f32,
    /// Document IDs where the topic was found
    pub document_ids: Vec<String>,
}

/// A question-answer pair extracted from a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionAnswer {
    /// The question
    pub question: String,
    /// The answer
    pub answer: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
    /// Document ID where the QA pair was extracted
    pub document_id: String,
    /// Context surrounding the answer
    pub context: String,
}

/// Document workspace for managing documents
#[derive(Debug, Clone)]
pub struct DocumentWorkspace {
    /// Documents stored in the workspace
    pub documents: HashMap<String, Document>,
}

impl DocumentWorkspace {
    /// Create a new document workspace
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
        }
    }
}

impl Default for DocumentWorkspace {
    fn default() -> Self {
        Self::new()
    }
}

/// LLM-enhanced content analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmAnalysis {
    /// Document ID that was analyzed
    pub document_id: String,
    /// Summary of the document
    pub summary: String,
    /// Key insights extracted
    pub insights: Vec<String>,
    /// Questions generated from the content
    pub questions: Vec<String>,
    /// Model used for analysis
    pub model: String,
    /// Time when the analysis was performed
    pub analysis_time: DateTime<Utc>,
}

/// A search query for documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Query text
    pub query: String,
    /// Filter criteria
    pub filters: HashMap<String, String>,
    /// Number of results to return
    pub limit: usize,
    /// Offset for pagination
    pub offset: usize,
}

/// A search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Document ID
    pub document_id: String,
    /// URL of the document
    pub url: String,
    /// Title of the document
    pub title: String,
    /// Snippet or excerpt
    pub snippet: String,
    /// Relevance score (0.0 to 1.0)
    pub score: f32,
    /// Highlighted terms
    pub highlights: HashMap<String, Vec<String>>,
}

/// A crawl job configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlJobConfig {
    /// Unique identifier for the job
    pub id: String,
    /// Name of the job
    pub name: String,
    /// Description of the job
    pub description: String,
    /// Seed URLs
    pub seeds: Vec<String>,
    /// Maximum number of URLs to crawl
    pub max_urls: usize,
    /// Maximum depth to crawl
    pub max_depth: usize,
    /// Whether to respect robots.txt
    pub respect_robots_txt: bool,
    /// User agent to use
    pub user_agent: String,
    /// Politeness delay in milliseconds
    pub politeness_delay: u64,
    /// Maximum concurrent requests
    pub max_concurrent_requests: usize,
    /// URL patterns to include
    pub url_patterns: Vec<String>,
    /// URL patterns to exclude
    pub exclude_patterns: Vec<String>,
    /// Allowed domains
    pub allowed_domains: Vec<String>,
    /// Whether to follow links
    pub follow_links: bool,
    /// Schedule (cron expression)
    pub schedule: Option<String>,
    /// Whether the job is enabled
    pub enabled: bool,
}

/// Implementation of default values for CrawlJobConfig
impl Default for CrawlJobConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            seeds: Vec::new(),
            max_urls: 1000,
            max_depth: 3,
            respect_robots_txt: true,
            user_agent: "Crawl4AI/1.0".to_string(),
            politeness_delay: 1000,
            max_concurrent_requests: 5,
            url_patterns: Vec::new(),
            exclude_patterns: Vec::new(),
            allowed_domains: Vec::new(),
            follow_links: true,
            schedule: None,
            enabled: true,
        }
    }
}

/// A single fetched page record for crawl results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlPage {
    /// Unique id for the record
    pub id: String,
    /// Associated crawl job id
    pub job_id: String,
    /// URL fetched
    pub url: String,
    /// HTTP status code
    pub status_code: u16,
    /// Length of response text (bytes)
    pub text_length: usize,
    /// Timestamp when fetched
    pub fetched_at: DateTime<Utc>,
}
