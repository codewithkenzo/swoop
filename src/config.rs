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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub storage_type: StorageType,
    pub connection_string: Option<String>,
    pub max_documents: usize,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            storage_type: StorageType::Memory,
            connection_string: None,
            max_documents: 10_000,
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
