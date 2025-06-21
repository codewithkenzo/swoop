/*!
 * Configuration module for Crawl4AI
 * 
 * This module defines the configuration structures for the crawler,
 * parser, and storage components.
 */

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Main configuration for Crawl4AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Crawler configuration
    pub crawler: CrawlConfig,
    /// Parser configuration
    pub parser: ParserConfig,
    /// Storage configuration
    pub storage: StorageConfig,
    /// GPU configuration
    pub gpu: GpuConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            crawler: CrawlConfig::default(),
            parser: ParserConfig::default(),
            storage: StorageConfig::default(),
            gpu: GpuConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

/// Configuration for the crawler component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlConfig {
    /// User agent string
    pub user_agent: String,
    /// Maximum number of concurrent requests
    pub max_concurrent_requests: usize,
    /// Maximum number of URLs to crawl (0 = unlimited)
    pub max_urls: usize,
    /// Maximum depth to crawl (0 = unlimited)
    pub max_depth: usize,
    /// Whether to respect robots.txt
    pub respect_robots_txt: bool,
    /// Delay between requests to the same domain (in milliseconds)
    pub politeness_delay: u64,
    /// Request timeout (in seconds)
    pub request_timeout: u64,
    /// Connection timeout (in seconds)
    pub connect_timeout: u64,
    /// Maximum number of redirects to follow
    pub max_redirects: usize,
    /// Proxy URL (if any)
    pub proxy: Option<String>,
    /// Cookies to include with requests (domain -> cookie string)
    pub cookies: HashMap<String, String>,
    /// URL patterns to include (regex)
    pub url_patterns: Vec<String>,
    /// URL patterns to exclude (regex)
    pub exclude_patterns: Vec<String>,
    /// Allowed domains
    pub allowed_domains: Vec<String>,
    /// Whether to follow links
    pub follow_links: bool,
    /// Whether to extract links from JavaScript
    pub extract_js_links: bool,
    /// Whether to use headless browser for JavaScript rendering
    pub use_headless_browser: bool,
    /// Headless browser executable path
    pub headless_browser_path: Option<String>,
    /// Headless browser arguments
    pub headless_browser_args: Vec<String>,
    /// Whether to capture screenshots
    pub capture_screenshots: bool,
    /// Screenshot format (png, jpeg)
    pub screenshot_format: String,
    /// Whether to retry failed requests
    pub retry_failed: bool,
    /// Maximum number of retries
    pub max_retries: usize,
    /// Retry delay (in milliseconds)
    pub retry_delay: u64,
}

impl Default for CrawlConfig {
    fn default() -> Self {
        Self {
            user_agent: "Crawl4AI/1.0 (+https://quantumscribe.ai/bot)".to_string(),
            max_concurrent_requests: 10,
            max_urls: 0,
            max_depth: 0,
            respect_robots_txt: true,
            politeness_delay: 1000,
            request_timeout: 30,
            connect_timeout: 10,
            max_redirects: 5,
            proxy: None,
            cookies: HashMap::new(),
            url_patterns: Vec::new(),
            exclude_patterns: Vec::new(),
            allowed_domains: Vec::new(),
            follow_links: true,
            extract_js_links: false,
            use_headless_browser: false,
            headless_browser_path: None,
            headless_browser_args: vec!["--headless".to_string(), "--disable-gpu".to_string()],
            capture_screenshots: false,
            screenshot_format: "png".to_string(),
            retry_failed: true,
            max_retries: 3,
            retry_delay: 5000,
        }
    }
}

/// Configuration for the parser component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    /// Default character encoding
    pub default_encoding: String,
    /// Whether to extract metadata
    pub extract_metadata: bool,
    /// Whether to extract title
    pub extract_title: bool,
    /// Whether to extract text
    pub extract_text: bool,
    /// Whether to extract links
    pub extract_links: bool,
    /// Whether to extract images
    pub extract_images: bool,
    /// Whether to extract structured data
    pub extract_structured_data: bool,
    /// Whether to extract microdata
    pub extract_microdata: bool,
    /// Whether to extract JSON-LD
    pub extract_jsonld: bool,
    /// Whether to extract OpenGraph metadata
    pub extract_opengraph: bool,
    /// Whether to extract Twitter Card metadata
    pub extract_twitter_card: bool,
    /// Whether to extract schema.org metadata
    pub extract_schema_org: bool,
    /// Whether to clean HTML
    pub clean_html: bool,
    /// Whether to remove scripts
    pub remove_scripts: bool,
    /// Whether to remove styles
    pub remove_styles: bool,
    /// Whether to remove comments
    pub remove_comments: bool,
    /// Whether to normalize whitespace
    pub normalize_whitespace: bool,
    /// Custom extraction rules
    pub extraction_rules: Vec<ExtractionRule>,
    /// Content types to parse
    pub content_types: Vec<String>,
    /// Maximum content size to parse (in bytes, 0 = unlimited)
    pub max_content_size: usize,
    /// LLM-enhanced parsing
    pub use_llm: bool,
    /// LLM model to use
    pub llm_model: String,
    /// LLM API key
    pub llm_api_key: Option<String>,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            default_encoding: "utf-8".to_string(),
            extract_metadata: true,
            extract_title: true,
            extract_text: true,
            extract_links: true,
            extract_images: true,
            extract_structured_data: true,
            extract_microdata: true,
            extract_jsonld: true,
            extract_opengraph: true,
            extract_twitter_card: true,
            extract_schema_org: true,
            clean_html: true,
            remove_scripts: true,
            remove_styles: true,
            remove_comments: true,
            normalize_whitespace: true,
            extraction_rules: Vec::new(),
            content_types: vec![
                "text/html".to_string(),
                "application/json".to_string(),
                "application/xml".to_string(),
                "text/plain".to_string(),
            ],
            max_content_size: 10 * 1024 * 1024, // 10 MB
            use_llm: false,
            llm_model: "mistralai/mistral-7b-instruct-v0.2".to_string(),
            llm_api_key: None,
        }
    }
}

/// Configuration for the storage component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Storage type
    pub storage_type: StorageType,
    /// Database connection string
    pub connection_string: String,
    /// Whether to use vector storage
    pub use_vector_storage: bool,
    /// Vector storage type
    pub vector_storage_type: VectorStorageType,
    /// Vector storage connection string
    pub vector_connection_string: String,
    /// Vector dimension
    pub vector_dimension: usize,
    /// Whether to store raw HTML
    pub store_html: bool,
    /// Whether to store text
    pub store_text: bool,
    /// Whether to store metadata
    pub store_metadata: bool,
    /// Whether to store links
    pub store_links: bool,
    /// Whether to store screenshots
    pub store_screenshots: bool,
    /// Whether to compress data
    pub compress_data: bool,
    /// Compression level (1-9)
    pub compression_level: u32,
    /// Whether to use encryption
    pub use_encryption: bool,
    /// Encryption key
    pub encryption_key: Option<String>,
    /// Whether to use cache
    pub use_cache: bool,
    /// Cache size (in items)
    pub cache_size: usize,
    /// Cache TTL (in seconds)
    pub cache_ttl: u64,
    /// Whether to use Redis for caching
    pub use_redis: bool,
    /// Redis connection string
    pub redis_connection_string: Option<String>,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            storage_type: StorageType::SQLite,
            connection_string: "crawl4ai.db".to_string(),
            use_vector_storage: false,
            vector_storage_type: VectorStorageType::Qdrant,
            vector_connection_string: "http://localhost:6333".to_string(),
            vector_dimension: 384,
            store_html: true,
            store_text: true,
            store_metadata: true,
            store_links: true,
            store_screenshots: false,
            compress_data: true,
            compression_level: 6,
            use_encryption: false,
            encryption_key: None,
            use_cache: true,
            cache_size: 1000,
            cache_ttl: 3600,
            use_redis: false,
            redis_connection_string: None,
        }
    }
}

/// Configuration for GPU acceleration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuConfig {
    /// Whether to use GPU acceleration
    pub use_gpu: bool,
    /// GPU device ID
    pub device_id: usize,
    /// Maximum GPU memory usage (in MB, 0 = unlimited)
    pub max_memory: usize,
    /// Whether to use mixed precision
    pub use_mixed_precision: bool,
    /// Whether to use tensor cores
    pub use_tensor_cores: bool,
    /// Batch size for GPU operations
    pub batch_size: usize,
}

impl Default for GpuConfig {
    fn default() -> Self {
        Self {
            use_gpu: false,
            device_id: 0,
            max_memory: 0,
            use_mixed_precision: true,
            use_tensor_cores: true,
            batch_size: 16,
        }
    }
}

/// Configuration for logging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: LogLevel,
    /// Whether to log to file
    pub log_to_file: bool,
    /// Log file path
    pub log_file: Option<String>,
    /// Whether to log to console
    pub log_to_console: bool,
    /// Whether to use JSON format
    pub json_format: bool,
    /// Whether to include timestamps
    pub include_timestamps: bool,
    /// Whether to include source location
    pub include_source_location: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            log_to_file: false,
            log_file: None,
            log_to_console: true,
            json_format: false,
            include_timestamps: true,
            include_source_location: false,
        }
    }
}

/// Storage type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageType {
    /// SQLite database
    SQLite,
    /// PostgreSQL database
    PostgreSQL,
    /// MySQL database
    MySQL,
    /// File system
    FileSystem,
    /// Memory
    Memory,
}

/// Vector storage type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VectorStorageType {
    /// Qdrant vector database
    Qdrant,
    /// Milvus vector database
    Milvus,
    /// FAISS vector database
    FAISS,
    /// Pinecone vector database
    Pinecone,
    /// Weaviate vector database
    Weaviate,
}

/// Log level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogLevel {
    /// Trace level
    Trace,
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warning level
    Warn,
    /// Error level
    Error,
}

/// Extraction rule for the parser
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionRule {
    /// Name of the rule
    pub name: String,
    /// Selector type
    pub selector_type: SelectorType,
    /// Selector
    pub selector: String,
    /// Attribute to extract (if applicable)
    pub attribute: Option<String>,
    /// Whether to extract multiple matches
    pub multiple: bool,
    /// Whether the rule is required
    pub required: bool,
    /// Default value if not found
    pub default_value: Option<String>,
}

/// Selector type for extraction rules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SelectorType {
    /// CSS selector
    CSS,
    /// XPath selector
    XPath,
    /// JSON path
    JSONPath,
    /// Regular expression
    Regex,
}
