/*!
 * Swoop - Advanced Document Intelligence Platform
 * 
 * A production-ready document processing and AI chat system with
 * intelligent content analysis, fuzzy search, and custom personalities.
 */

// Core modules
pub mod error;
pub mod models;
// pub mod extractors;
// pub mod intelligence;
// pub mod chat;
// pub mod ai;
// pub mod monitoring;
// pub mod server;
// pub mod api_server;

// Re-export main types
pub use error::{Result, Error};
// pub use models::{Document, DocumentWorkspace};

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
    info.insert("status".to_string(), "minimal".to_string());
    info
} 