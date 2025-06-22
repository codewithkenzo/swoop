//! # Swoop - AI-Powered Document Intelligence Platform
//!
//! Swoop is a comprehensive document analysis and web crawling platform built in Rust.
//! It provides intelligent document processing, automated content extraction, and
//! advanced analytics capabilities.
//!
//! ## Features
//!
//! - **Web Crawling**: High-performance web crawling with rate limiting and respect for robots.txt
//! - **Document Analysis**: AI-powered document categorization, entity extraction, and semantic analysis
//! - **Storage Backends**: Multiple storage options including memory, filesystem, SQLite, and Redis
//! - **Monitoring**: Built-in Prometheus metrics and health checks
//! - **API Server**: RESTful API with real-time status updates
//!
//! ## Usage
//!
//! ```rust,no_run
//! use swoop::{Crawler, CrawlConfig, storage::MemoryStorage};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let storage = MemoryStorage::new();
//!     let crawler = Crawler::new(storage);
//!     
//!     let config = CrawlConfig::default();
//!     let results = crawler.crawl_url("https://example.com", &config).await?;
//!     
//!     println!("Crawled {} pages", results.len());
//!     Ok(())
//! }
//! ```

// Core modules
pub mod config;
pub mod crawler;
pub mod error;
pub mod extractors;
pub mod loaders;
pub mod models;
pub mod parser;
pub mod rate_limiter;
pub mod server;
pub mod storage;
pub mod utils;
pub mod monitoring;

// AI module (conditionally compiled)
#[cfg(feature = "ai")]
pub mod ai;

// Re-export commonly used types
pub use config::{Config, CrawlConfig};
pub use crawler::Crawler;
pub use error::Error as SwoopError;
pub use models::{Document, CrawlResult, DocumentSummary};
pub use rate_limiter::RateLimiter;
pub use server::CrawlServer;
pub use storage::Storage;
pub use monitoring::{MonitoringSystem, HealthStatus};

// Re-export AI types when feature is enabled
#[cfg(feature = "ai")]
pub use ai::{
    DocumentAnalyzer, AnalysisConfig, AnalysisResults,
    DocumentCategory, ExtractedEntity,
    detect_language,
};

#[cfg(feature = "ai")]
pub use ai::ner::EntityType;

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, SwoopError>;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Build information
pub const BUILD_INFO: &str = concat!(
    "Swoop v",
    env!("CARGO_PKG_VERSION"),
    " - AI-Powered Document Intelligence Platform"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info() {
        assert!(!VERSION.is_empty());
        println!("Swoop version: {}", VERSION);
    }

    #[cfg(feature = "ai")]
    #[test]
    fn test_ai_feature_enabled() {
        // Test that AI types are available when feature is enabled
        let config = AnalysisConfig::default();
        assert!(config.categorization);
        assert!(config.entity_extraction);
    }

    #[cfg(not(feature = "ai"))]
    #[test]
    fn test_ai_feature_disabled() {
        // Test that core functionality works without AI features
        let _config = Config::default();
        // This test just ensures compilation works without AI features
    }
} 