//! Content extractors for different types of data
//!
//! This module provides utilities for extracting specific types of content
//! from web pages, including text, metadata, and structured data.

use anyhow::Result;
use std::collections::HashMap;
use ammonia::{Builder, clean};
use once_cell::sync::Lazy;
use regex::Regex;

// Pre-compiled regexes for performance
static SCRIPT_STYLE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)<(script|style)[^>]*>.*?</\1>").unwrap()
});

static HTML_TAG_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"<[^>]*>").unwrap()
});

static WHITESPACE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\s+").unwrap()
});


/// Extract text content from HTML with comprehensive security measures
pub fn extract_text_secure(html: &str) -> Result<String> {
    // Step 1: Basic HTML sanitization using ammonia
    let mut builder = Builder::default();
    builder
        .tags(std::collections::HashSet::new()) // Remove all tags
        .clean_content_tags(std::collections::HashSet::new())
        .strip_comments(true);
    
    let sanitized = builder.clean(html).to_string();
    
    // Step 2: Remove any remaining script/style content
    let no_scripts = SCRIPT_STYLE_REGEX.replace_all(&sanitized, "");
    
    // Step 3: Remove HTML tags (defensive measure)
    let no_tags = HTML_TAG_REGEX.replace_all(&no_scripts, " ");
    
    // Step 4: Normalize whitespace
    let normalized = WHITESPACE_REGEX.replace_all(&no_tags, " ");
    
    Ok(normalized.trim().to_string())
}

/// Extract the page title from HTML
pub fn extract_title(html: &str) -> Result<Option<String>> {
    if let Some(captures) = regex::Regex::new(r"(?i)<title[^>]*>(.*?)</title>")
        .unwrap()
        .captures(html)
    {
        if let Some(title) = captures.get(1) {
            let title = title.as_str().trim();
            if !title.is_empty() {
                return Ok(Some(title.to_string()));
            }
        }
    }
    
    Ok(None)
}


/// Extract meta tags from HTML with proper sanitization
pub fn extract_metadata_secure(html: &str) -> Result<HashMap<String, String>> {
    let mut metadata = HashMap::new();
    
    // Use ammonia to pre-clean the HTML
    let clean_html = clean(html);
    
    // Safe regex patterns for meta tags
    let name_regex = Regex::new(r#"(?i)<meta[^>]*name\s*=\s*["']([^"']+)["'][^>]*content\s*=\s*["']([^"']+)["'][^>]*>"#)?;
    let property_regex = Regex::new(r#"(?i)<meta[^>]*property\s*=\s*["']([^"']+)["'][^>]*content\s*=\s*["']([^"']+)["'][^>]*>"#)?;
    
    for captures in name_regex.captures_iter(&clean_html) {
        if let (Some(name), Some(content)) = (captures.get(1), captures.get(2)) {
            let name_str = name.as_str().to_lowercase();
            let content_str = clean(content.as_str()); // Additional cleaning
            
            // Validate metadata keys (only allow safe characters)
            if is_safe_metadata_key(&name_str) {
                metadata.insert(name_str, content_str);
            }
        }
    }
    
    for captures in property_regex.captures_iter(&clean_html) {
        if let (Some(property), Some(content)) = (captures.get(1), captures.get(2)) {
            let property_str = property.as_str().to_lowercase();
            let content_str = clean(content.as_str()); // Additional cleaning
            
            // Validate metadata keys (only allow safe characters)
            if is_safe_metadata_key(&property_str) {
                metadata.insert(property_str, content_str);
            }
        }
    }
    
    Ok(metadata)
}

fn is_safe_metadata_key(key: &str) -> bool {
    // Only allow alphanumeric, dash, underscore, colon
    key.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == ':')
}

/// Extract links from HTML
pub fn extract_links(html: &str) -> Result<Vec<String>> {
    let mut links = Vec::new();
    
    let link_regex = regex::Regex::new(r#"(?i)<a[^>]*href=["']([^"']+)["'][^>]*>"#).unwrap();
    for captures in link_regex.captures_iter(html) {
        if let Some(href) = captures.get(1) {
            links.push(href.as_str().to_string());
        }
    }
    
    Ok(links)
}

/// Extract images from HTML
pub fn extract_images(html: &str) -> Result<Vec<String>> {
    let mut images = Vec::new();
    
    let img_regex = regex::Regex::new(r#"(?i)<img[^>]*src=["']([^"']+)["'][^>]*>"#).unwrap();
    for captures in img_regex.captures_iter(html) {
        if let Some(src) = captures.get(1) {
            images.push(src.as_str().to_string());
        }
    }
    
    Ok(images)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_title() {
        let html = r#"<html><head><title>Test Title</title></head><body></body></html>"#;
        let title = extract_title(html).unwrap();
        assert_eq!(title, Some("Test Title".to_string()));
    }

    #[test]
    fn test_extract_metadata() {
        let html = r#"<html><head><meta name="description" content="Test description"><meta property="og:title" content="OG Title"></head><body></body></html>"#;
        let metadata = extract_metadata_secure(html).unwrap();
        assert_eq!(metadata.get("description"), Some(&"Test description".to_string()));
        assert_eq!(metadata.get("og:title"), Some(&"OG Title".to_string()));
    }

    #[test]
    fn test_extract_links() {
        let html = r#"<html><body><a href="https://example.com">Link</a><a href="/relative">Relative</a></body></html>"#;
        let links = extract_links(html).unwrap();
        assert!(links.contains(&"https://example.com".to_string()));
        assert!(links.contains(&"/relative".to_string()));
    }
}
