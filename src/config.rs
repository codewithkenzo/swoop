/*!
 * Configuration module for Swoop
 */

use serde::{Deserialize, Serialize};

/// Parser configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserConfig {
    pub max_content_length: usize,
    pub enable_html_parsing: bool,
    pub enable_pdf_parsing: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            max_content_length: 10_000_000, // 10MB
            enable_html_parsing: true,
            enable_pdf_parsing: true,
        }
    }
}

/// Extraction rule for content parsing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionRule {
    pub name: String,
    pub selector: String,
    /// Type of content to extract
    pub content_type: String,
    pub selector_type: SelectorType,
    /// Whether to extract multiple values
    pub multiple: bool,
    /// HTML attribute to extract (for CSS selectors)
    pub attribute: Option<String>,
    /// Default value if extraction fails
    pub default_value: Option<String>,
    /// Whether this field is required
    pub required: bool,
}

/// Type of selector used in extraction rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelectorType {
    CSS,
    XPath,
    Regex,
    JSONPath,
}

/// Storage configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StorageConfig {
    /// Storage backend type
    pub backend: String, // "memory", "sqlite", "postgres", "libsql"
    /// Connection string for the storage backend
    pub connection_string: Option<String>,
    /// Auth token for remote storage (e.g., Turso)
    pub auth_token: Option<String>,
    /// SQLite database file path
    pub sqlite_path: Option<String>,
    /// PostgreSQL database URL
    pub database_url: Option<String>,
    /// Enable WAL mode for SQLite
    pub wal_mode: bool,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            backend: "memory".to_string(),
            connection_string: None,
            auth_token: None,
            sqlite_path: None,
            database_url: None,
            wal_mode: false,
        }
    }
}

/// Type of storage backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageType {
    Memory,
    FileSystem,
    SQLite,
}

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct Config {
    pub parser: ParserConfig,
    pub storage: StorageConfig,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Config::default();
        
        // Check for DATABASE_URL environment variable
        if let Ok(database_url) = std::env::var("DATABASE_URL") {
            config.storage.database_url = Some(database_url);
            
            // Auto-detect backend type from URL
            if config.storage.database_url.as_ref().unwrap().starts_with("postgres://") 
                || config.storage.database_url.as_ref().unwrap().starts_with("postgresql://") {
                config.storage.backend = "postgres".to_string();
            }
        }
        
        // Check for other storage-related environment variables
        if let Ok(sqlite_path) = std::env::var("SQLITE_PATH") {
            config.storage.sqlite_path = Some(sqlite_path);
            if config.storage.backend == "memory" {
                config.storage.backend = "sqlite".to_string();
            }
        }
        
        if let Ok(connection_string) = std::env::var("STORAGE_CONNECTION_STRING") {
            config.storage.connection_string = Some(connection_string);
        }
        
        if let Ok(auth_token) = std::env::var("STORAGE_AUTH_TOKEN") {
            config.storage.auth_token = Some(auth_token);
        }
        
        config
    }
}

