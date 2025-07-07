//! Utility functions for scraping operations
//!
//! This module provides common utilities for URL processing, rate limiting,
//! and other scraping-related operations.

use anyhow::Result;
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Rate limiter for controlling request frequency
pub struct RateLimiter {
    requests_per_second: f64,
    last_request: Option<Instant>,
}

impl RateLimiter {
    pub fn new(requests_per_second: f64) -> Self {
        Self {
            requests_per_second,
            last_request: None,
        }
    }
    
    pub async fn wait_if_needed(&mut self) {
        if let Some(last) = self.last_request {
            let min_interval = Duration::from_secs_f64(1.0 / self.requests_per_second);
            let elapsed = last.elapsed();
            
            if elapsed < min_interval {
                let wait_time = min_interval - elapsed;
                sleep(wait_time).await;
            }
        }
        
        self.last_request = Some(Instant::now());
    }
}

/// Normalize URLs by removing fragments and common tracking parameters
pub fn normalize_url(url: &str) -> String {
    let mut normalized = url.to_string();
    
    // Remove fragment identifier
    if let Some(fragment_pos) = normalized.find('#') {
        normalized.truncate(fragment_pos);
    }
    
    // TODO: Add more normalization rules as needed
    // - Remove common tracking parameters
    // - Normalize case
    // - Remove trailing slashes
    
    normalized
}

/// Check if a URL is valid and accessible
pub fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}

/// Extract domain from URL
pub fn extract_domain(url: &str) -> Result<String> {
    let url = url::Url::parse(url)?;
    Ok(url.host_str().unwrap_or("").to_string())
}

/// Generate a realistic user agent string
pub fn generate_user_agent() -> String {
    // In a real implementation, this would rotate between different user agents
    "Mozilla/5.0 (X11; Linux x86_64; rv:91.0) Gecko/20100101 Firefox/91.0".to_string()
}

/// Sleep for a random duration to avoid detection
pub async fn random_delay(min_ms: u64, max_ms: u64) {
    use rand::Rng;
    
    let mut rng = rand::thread_rng();
    let delay_ms = rng.gen_range(min_ms..=max_ms);
    sleep(Duration::from_millis(delay_ms)).await;
}

/// Detect if content is likely to be bot-protected
pub fn is_bot_protected(html: &str) -> bool {
    let html_lower = html.to_lowercase();
    
    // Common indicators of bot protection
    html_lower.contains("captcha") ||
    html_lower.contains("cloudflare") ||
    html_lower.contains("access denied") ||
    html_lower.contains("blocked") ||
    html_lower.contains("robot") && html_lower.contains("detected")
}

/// Parse robots.txt content
pub fn parse_robots_txt(content: &str) -> RobotsTxt {
    let mut robots = RobotsTxt::new();
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((directive, value)) = line.split_once(':') {
            let directive = directive.trim().to_lowercase();
            let value = value.trim();
            
            match directive.as_str() {
                "user-agent" => robots.user_agent = Some(value.to_string()),
                "disallow" => robots.disallow.push(value.to_string()),
                "allow" => robots.allow.push(value.to_string()),
                "crawl-delay" => {
                    if let Ok(delay) = value.parse::<u64>() {
                        robots.crawl_delay = Some(delay);
                    }
                }
                _ => {}
            }
        }
    }
    
    robots
}

/// Robots.txt parser result
#[derive(Debug, Clone)]
pub struct RobotsTxt {
    pub user_agent: Option<String>,
    pub disallow: Vec<String>,
    pub allow: Vec<String>,
    pub crawl_delay: Option<u64>,
}

impl RobotsTxt {
    pub fn new() -> Self {
        Self {
            user_agent: None,
            disallow: Vec::new(),
            allow: Vec::new(),
            crawl_delay: None,
        }
    }
    
    pub fn is_allowed(&self, path: &str) -> bool {
        // Check if path is explicitly disallowed
        for disallow_pattern in &self.disallow {
            if path.starts_with(disallow_pattern) {
                return false;
            }
        }
        
        // Check if path is explicitly allowed
        for allow_pattern in &self.allow {
            if path.starts_with(allow_pattern) {
                return true;
            }
        }
        
        // Default to allowed if no specific rules match
        true
    }
}

impl Default for RobotsTxt {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_url() {
        assert_eq!(normalize_url("https://example.com/path#fragment"), "https://example.com/path");
        assert_eq!(normalize_url("https://example.com/path"), "https://example.com/path");
    }

    #[test]
    fn test_is_valid_url() {
        assert!(is_valid_url("https://example.com"));
        assert!(is_valid_url("http://example.com"));
        assert!(!is_valid_url("ftp://example.com"));
        assert!(!is_valid_url("not-a-url"));
    }

    #[test]
    fn test_extract_domain() {
        assert_eq!(extract_domain("https://example.com/path").unwrap(), "example.com");
        assert_eq!(extract_domain("https://sub.example.com/path").unwrap(), "sub.example.com");
    }

    #[test]
    fn test_is_bot_protected() {
        assert!(is_bot_protected("This page contains a CAPTCHA"));
        assert!(is_bot_protected("Cloudflare protection enabled"));
        assert!(!is_bot_protected("Normal page content"));
    }

    #[test]
    fn test_robots_txt_parsing() {
        let robots_content = r#"
            User-agent: *
            Disallow: /private/
            Allow: /public/
            Crawl-delay: 1
        "#;
        
        let robots = parse_robots_txt(robots_content);
        assert_eq!(robots.user_agent, Some("*".to_string()));
        assert!(robots.disallow.contains(&"/private/".to_string()));
        assert!(robots.allow.contains(&"/public/".to_string()));
        assert_eq!(robots.crawl_delay, Some(1));
    }

    #[test]
    fn test_robots_txt_is_allowed() {
        let robots_content = r#"
            User-agent: *
            Disallow: /private/
            Allow: /public/
        "#;
        
        let robots = parse_robots_txt(robots_content);
        assert!(!robots.is_allowed("/private/secret"));
        assert!(robots.is_allowed("/public/info"));
        assert!(robots.is_allowed("/other/path"));
    }

    #[tokio::test]
    async fn test_rate_limiter() {
        let mut rate_limiter = RateLimiter::new(10.0); // 10 requests per second
        
        let start = Instant::now();
        rate_limiter.wait_if_needed().await;
        rate_limiter.wait_if_needed().await;
        let elapsed = start.elapsed();
        
        // Should have waited at least 100ms between requests
        assert!(elapsed >= Duration::from_millis(100));
    }
}