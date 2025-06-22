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
    pub selector_type: SelectorType,
}

/// Type of selector used in extraction rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelectorType {
    CSS,
    XPath,
    Regex,
}

/// Storage configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StorageConfig {
    /// Storage backend type
    pub backend: String, // "memory", "sqlite", "libsql"
    /// Connection string for the storage backend
    pub connection_string: Option<String>,
    /// Auth token for remote storage (e.g., Turso)
    pub auth_token: Option<String>,
    /// SQLite database file path
    pub sqlite_path: Option<String>,
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
pub struct Config {
    pub parser: ParserConfig,
    pub storage: StorageConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            parser: ParserConfig::default(),
            storage: StorageConfig::default(),
        }
    }
}
