/*!
 * Models module for Crawl4AI
 * 
 * This module defines the core data structures used throughout the Crawl4AI system,
 * including documents, links, metadata, and extraction results.
 */

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// A document represents a crawled web page or resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Unique identifier for the document
    pub id: String,
    /// URL of the document
    pub url: String,
    /// Title of the document (if available)
    pub title: String,
    /// Content of the document (may be HTML, JSON, etc.)
    pub content: String,
    /// HTML content of the document (if applicable)
    pub html: String,
    /// Plain text content of the document
    pub text: String,
    /// Metadata about the document
    pub metadata: Metadata,
    /// Links found in the document
    pub links: Vec<Link>,
    /// Extracted structured content
    pub extracted: HashMap<String, ExtractedContent>,
}

/// Metadata about a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// URL of the document
    pub url: String,
    /// Content type of the document
    pub content_type: String,
    /// Time when the document was fetched
    pub fetch_time: DateTime<Utc>,
    /// HTTP status code
    pub status_code: u16,
    /// HTTP headers
    pub headers: HashMap<String, String>,
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
    /// Documents in the batch
    pub documents: Vec<Document>,
    /// Time when the batch was created
    pub created_at: DateTime<Utc>,
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
