//! Scrapers crate - Platform-specific web scraping modules
//!
//! This crate provides specialized scrapers for different social media platforms
//! and websites, implementing anti-bot evasion and rate limiting strategies.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod browser;
pub mod extractors;
pub mod platforms;
pub mod rate_limiter;
pub mod utils;

/// Configuration for scraping operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScraperConfig {
    /// Maximum number of concurrent requests
    pub max_concurrent: usize,
    /// Request timeout in seconds
    pub timeout_secs: u64,
    /// Rate limiting: requests per second
    pub rate_limit: f64,
    /// User agent string to use
    pub user_agent: String,
    /// Headers to include in requests
    pub headers: HashMap<String, String>,
}

impl Default for ScraperConfig {
    fn default() -> Self {
        let mut headers = HashMap::new();
        headers.insert(
            "Accept".to_string(),
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".to_string(),
        );
        headers.insert("Accept-Language".to_string(), "en-US,en;q=0.5".to_string());
        headers.insert("Accept-Encoding".to_string(), "gzip, deflate".to_string());
        headers.insert("Cache-Control".to_string(), "no-cache".to_string());

        Self {
            max_concurrent: 10,
            timeout_secs: 30,
            rate_limit: 1.0, // 1 request per second by default
            user_agent: "Mozilla/5.0 (X11; Linux x86_64; rv:91.0) Gecko/20100101 Firefox/91.0"
                .to_string(),
            headers,
        }
    }
}

/// Common data structure for extracted content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedContent {
    /// URL where the content was found
    pub url: String,
    /// Title of the content
    pub title: Option<String>,
    /// Main text content
    pub text: Option<String>,
    /// Metadata about the content
    pub metadata: HashMap<String, String>,
    /// Timestamp when content was extracted
    pub extracted_at: chrono::DateTime<chrono::Utc>,
}

/// Trait for platform-specific scrapers
pub trait PlatformScraper {
    /// Extract content from a URL
    fn extract(
        &self,
        url: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ExtractedContent>> + Send + '_>>;

    /// Check if the scraper can handle this URL
    fn can_handle(&self, url: &str) -> bool;

    /// Get the platform name
    fn platform_name(&self) -> &'static str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_scraper_config() {
        let config = ScraperConfig::default();
        assert_eq!(config.max_concurrent, 10);
        assert_eq!(config.timeout_secs, 30);
        assert_eq!(config.rate_limit, 1.0);
        assert!(!config.user_agent.is_empty());
        assert!(!config.headers.is_empty());
    }

    #[test]
    fn test_extracted_content_creation() {
        let content = ExtractedContent {
            url: "https://example.com".to_string(),
            title: Some("Test Title".to_string()),
            text: Some("Test content".to_string()),
            metadata: HashMap::new(),
            extracted_at: chrono::Utc::now(),
        };

        assert_eq!(content.url, "https://example.com");
        assert_eq!(content.title, Some("Test Title".to_string()));
    }
}
