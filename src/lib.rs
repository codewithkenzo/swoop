/*!
 * Swoop - Advanced Document Processing & AI Intelligence Platform
 * 
 * A production-ready document processing and AI analysis system with multi-model LLM integration.
 */

// Core modules
pub mod error;
pub mod models;
pub mod config;
pub mod extractors;
pub mod llm;
pub mod document_processor;
pub mod storage;
pub mod parser;
pub mod intelligence;
pub mod chat;
pub mod loaders;
pub mod rate_limiter;
// pub mod crawler;   // Disabled for now - has legacy dependencies
// pub mod ai;            // Re-enable gradually
// pub mod monitoring;    // Re-enable gradually
// pub mod server;        // Keep disabled for now
// pub mod api_server;    // Keep disabled for now

// Re-export main types
pub use error::{Result, Error};
pub use models::*;
pub use config::Config;
pub use document_processor::DocumentProcessor;
pub use storage::Storage;
pub use parser::{Parser, ExtractorRule, ParseResult};

/// System information and version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Initialize the Swoop system
pub fn init() -> Result<()> {
    env_logger::init();
    Ok(())
}

/// Get system information
pub fn system_info() -> std::collections::HashMap<String, String> {
    let mut info = std::collections::HashMap::new();
    info.insert("name".to_string(), NAME.to_string());
    info.insert("version".to_string(), VERSION.to_string());
    info.insert("rust_version".to_string(), "1.88.0-nightly".to_string());
    info.insert("features".to_string(), "document_processing,extraction,storage,llm_integration".to_string());
    info
} 