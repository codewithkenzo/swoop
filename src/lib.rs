//! crawl4ai_core crate
//!
//! This crate provides functionality for the Fin-Age project.

/*!
 * Crawl4AI Core - High-performance web crawling and data extraction engine
 * 
 * This library provides a Rust implementation of a high-performance web crawler
 * with advanced content extraction capabilities, designed for integration with
 * the QuantumScribe ecosystem.
 */

use mimalloc::MiMalloc;

// Use mimalloc for better memory performance
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

// Module declarations
pub mod crawler;
pub mod parser;
pub mod storage;
pub mod models;
pub mod config;
pub mod error;
pub mod utils;
pub mod rate_limiter;
pub mod server;
pub mod monitoring;

// Re-exports for convenient usage
pub use crawler::{Crawler, CrawlerBuilder, CrawlJob, CrawlStats};
pub use parser::{Parser, ParserBuilder, ExtractorRule, ContentType};
pub use storage::{Storage, StorageBuilder, VectorStorage, DocumentStorage};
pub use models::{CrawlResult, Document, Metadata, Link, ExtractedContent};
pub use config::{Config, CrawlConfig, ParserConfig, StorageConfig};
pub use error::{Error, Result};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library information
pub fn info() -> String {
    format!(
        "Crawl4AI Core v{}\nHigh-performance web crawling and data extraction engine",
        VERSION
    )
}

/// Initialize the library with default configuration
pub fn init() -> Result<()> {
    // Initialize logging
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    
    log::info!("Initializing Crawl4AI Core v{}", VERSION);
    
    // Check for GPU support
    #[cfg(feature = "gpu")]
    {
        log::info!("GPU acceleration enabled");
    }
    
    Ok(())
}

/// Create a new crawler with default configuration
pub fn new_crawler() -> CrawlerBuilder {
    CrawlerBuilder::new()
}

/// Create a new parser with default configuration
pub fn new_parser() -> ParserBuilder {
    ParserBuilder::new()
}

/// Create a new storage with default configuration
pub fn new_storage() -> StorageBuilder {
    StorageBuilder::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
    
    #[test]
    fn test_info() {
        let info = info();
        assert!(info.contains("Crawl4AI Core"));
        assert!(info.contains(VERSION));
    }
    
    #[test]
    fn test_init() {
        let result = init();
        assert!(result.is_ok());
    }
}
