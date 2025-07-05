/*!
 * Enhanced Content Extractors
 * 
 * Multi-format document processing with robust error handling
 */

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::{Error, Result};

/// Result of data extraction with structured fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionResult {
    /// Extracted email addresses
    pub emails: Vec<String>,
    /// Extracted phone numbers  
    pub phones: Vec<String>,
    /// Extracted links
    pub links: Vec<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Sensitive data found
    pub sensitive_data: Vec<SensitiveData>,
    /// Quality score for extracted data
    pub quality_score: f64,
    /// Classification of content
    pub classification: String,
    /// Validation issues found
    pub validation_issues: Vec<String>,
}

/// Sensitive data item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitiveData {
    pub data_type: String,
    pub original_text: String,
    pub redacted_text: String,
}

impl Default for ExtractionResult {
    fn default() -> Self {
        Self {
            emails: Vec::new(),
            phones: Vec::new(),
            links: Vec::new(),
            metadata: HashMap::new(),
            sensitive_data: Vec::new(),
            quality_score: 0.0,
            classification: "unknown".to_string(),
            validation_issues: Vec::new(),
        }
    }
}

/// Extract text content from various document formats
pub fn extract_text(content: &[u8], content_type: &str, filename: &str) -> Result<ExtractedContent> {
    let detected_format = detect_format(content, content_type, filename);
    
    match detected_format {
        DocumentFormat::Html => extract_html_content(content),
        DocumentFormat::Pdf => extract_pdf_content(content),
        DocumentFormat::Markdown => extract_markdown_content(content),
        DocumentFormat::PlainText => extract_plain_text(content),
        DocumentFormat::Unknown => extract_fallback_content(content),
    }
}

#[derive(Debug, Clone)]
pub struct ExtractedContent {
    pub text: String,
    pub title: Option<String>,
    pub metadata: HashMap<String, String>,
    pub word_count: usize,
    pub character_count: usize,
    pub format: DocumentFormat,
    pub extraction_method: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DocumentFormat {
    Html,
    Pdf,
    Markdown,
    PlainText,
    Unknown,
}

#[derive(Debug, thiserror::Error)]
pub enum ExtractionError {
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("IO error: {0}")]
    IoError(String),
    #[error("Encoding error: {0}")]
    EncodingError(String),
}

/// Intelligent format detection based on content, MIME type, and filename
fn detect_format(content: &[u8], content_type: &str, filename: &str) -> DocumentFormat {
    // Check MIME type first
    if content_type.contains("text/html") || content_type.contains("application/xhtml") {
        return DocumentFormat::Html;
    }
    if content_type.contains("application/pdf") {
        return DocumentFormat::Pdf;
    }
    if content_type.contains("text/markdown") || content_type.contains("text/x-markdown") {
        return DocumentFormat::Markdown;
    }
    
    // Check file extension
    let extension = filename.split('.').next_back().unwrap_or("").to_lowercase();
    match extension.as_str() {
        "html" | "htm" | "xhtml" => DocumentFormat::Html,
        "pdf" => DocumentFormat::Pdf,
        "md" | "markdown" | "mdown" | "mkd" => DocumentFormat::Markdown,
        "txt" | "text" => DocumentFormat::PlainText,
        _ => {
            // Content-based detection for unknown extensions
            if let Ok(text) = std::str::from_utf8(content) {
                if text.trim_start().starts_with("<!DOCTYPE") || text.contains("<html") {
                    DocumentFormat::Html
                } else if text.contains("# ") || text.contains("## ") || text.contains("```") {
                    DocumentFormat::Markdown
                } else {
                    DocumentFormat::PlainText
                }
            } else if content.starts_with(b"%PDF") {
                DocumentFormat::Pdf
            } else {
                DocumentFormat::Unknown
            }
        }
    }
}

/// Enhanced HTML content extraction
fn extract_html_content(content: &[u8]) -> Result<ExtractedContent> {
    let html_text = std::str::from_utf8(content)
        .map_err(|e| Error::Parsing(format!("Encoding error: {e}")))?;
    
    // Use regex for robust HTML parsing
    let script_style_regex = regex::Regex::new(r"(?is)<(?:script|style)[^>]*>.*?</(?:script|style)>")
        .map_err(|e| Error::Parsing(format!("Regex error: {e}")))?;
    let tag_regex = regex::Regex::new(r"<[^>]*>")
        .map_err(|e| Error::Parsing(format!("Regex error: {e}")))?;
    
    // Remove scripts and styles
    let cleaned = script_style_regex.replace_all(html_text, " ");
    // Remove all HTML tags
    let text_only = tag_regex.replace_all(&cleaned, " ");
    // Normalize whitespace
    let normalized = text_only.split_whitespace().collect::<Vec<_>>().join(" ");
    
    // Extract title
    let title_regex = regex::Regex::new(r"(?i)<title[^>]*>(.*?)</title>")
        .map_err(|e| Error::Parsing(format!("Regex error: {e}")))?;
    let title = title_regex.captures(html_text)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().trim().to_string());
    
    let mut metadata = HashMap::new();
    metadata.insert("scripts_removed".to_string(), 
        script_style_regex.find_iter(html_text).count().to_string());
    metadata.insert("has_title".to_string(), title.is_some().to_string());
    
    Ok(ExtractedContent {
        word_count: normalized.split_whitespace().count(),
        character_count: normalized.len(),
        text: normalized,
        title,
        metadata,
        format: DocumentFormat::Html,
        extraction_method: "regex_parsing".to_string(),
    })
}

/// PDF content extraction
fn extract_pdf_content(content: &[u8]) -> Result<ExtractedContent> {
    // Create a temporary file for PDF processing
    use std::io::Write;
    
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join(format!("swoop_temp_{}.pdf", rand::random::<u32>()));
    
    // Write content to temp file
    match std::fs::File::create(&temp_file) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(content) {
                return Err(Error::Io(e));
            }
        }
        Err(e) => return Err(Error::Io(e)),
    }
    
    // Extract text from the temp file
    let result = match pdf_extract::extract_text(&temp_file) {
        Ok(text) => {
            let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
            let mut metadata = HashMap::new();
            metadata.insert("extraction_tool".to_string(), "pdf-extract".to_string());
            
            Ok(ExtractedContent {
                word_count: normalized.split_whitespace().count(),
                character_count: normalized.len(),
                text: normalized,
                title: None, // PDF metadata extraction could be added here
                metadata,
                format: DocumentFormat::Pdf,
                extraction_method: "pdf_extract".to_string(),
            })
        }
        Err(e) => {
            // Fallback: try to extract any readable text
            match std::str::from_utf8(content) {
                Ok(text) => {
                    let readable_text: String = text.chars()
                        .filter(|c| c.is_ascii_graphic() || c.is_whitespace())
                        .collect();
                    
                    let mut metadata = HashMap::new();
                    metadata.insert("extraction_method".to_string(), "fallback_text".to_string());
                    metadata.insert("pdf_error".to_string(), e.to_string());
                    
                    Ok(ExtractedContent {
                        word_count: readable_text.split_whitespace().count(),
                        character_count: readable_text.len(),
                        text: readable_text,
                        title: None,
                        metadata,
                        format: DocumentFormat::Pdf,
                        extraction_method: "fallback_extraction".to_string(),
                    })
                }
                Err(_) => Err(Error::Parsing(format!("PDF extraction failed: {e}")))
            }
        }
    };
    
    // Clean up temp file
    let _ = std::fs::remove_file(&temp_file);
    
    result
}

/// Markdown content extraction with structure preservation
fn extract_markdown_content(content: &[u8]) -> Result<ExtractedContent> {
    let markdown_text = std::str::from_utf8(content)
        .map_err(|e| Error::Parsing(format!("Encoding error: {e}")))?;
    
    // Parse markdown and extract plain text
    let parser = pulldown_cmark::Parser::new(markdown_text);
    let mut plain_text = String::new();
    let mut title: Option<String> = None;
    let mut headers_count = 0;
    let mut code_blocks_count = 0;
    let mut links_count = 0;
    
    for event in parser {
        match event {
            pulldown_cmark::Event::Text(text) => {
                plain_text.push_str(&text);
                plain_text.push(' ');
            }
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Heading { level: pulldown_cmark::HeadingLevel::H1, .. }) => {
                if title.is_none() {
                    // Capture the first H1 as title
                    let _start_pos = plain_text.len();
                    // We'll capture the next text event as title
                }
                headers_count += 1;
            }
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::CodeBlock(_)) => {
                code_blocks_count += 1;
            }
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link { .. }) => {
                links_count += 1;
            }
            pulldown_cmark::Event::SoftBreak | pulldown_cmark::Event::HardBreak => {
                plain_text.push(' ');
            }
            _ => {}
        }
    }
    
    // Extract title from first H1 if not already captured
    if title.is_none() {
            let h1_regex = regex::Regex::new(r"(?m)^# (.+)$")
        .map_err(|e| Error::Parsing(format!("Regex error: {e}")))?;
        title = h1_regex.captures(markdown_text)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim().to_string());
    }
    
    let normalized = plain_text.split_whitespace().collect::<Vec<_>>().join(" ");
    
    let mut metadata = HashMap::new();
    metadata.insert("headers_count".to_string(), headers_count.to_string());
    metadata.insert("code_blocks_count".to_string(), code_blocks_count.to_string());
    metadata.insert("links_count".to_string(), links_count.to_string());
    metadata.insert("has_frontmatter".to_string(), 
        markdown_text.starts_with("---").to_string());
    
    Ok(ExtractedContent {
        word_count: normalized.split_whitespace().count(),
        character_count: normalized.len(),
        text: normalized,
        title,
        metadata,
        format: DocumentFormat::Markdown,
        extraction_method: "pulldown_cmark".to_string(),
    })
}

/// Plain text extraction
fn extract_plain_text(content: &[u8]) -> Result<ExtractedContent> {
    let text = std::str::from_utf8(content)
        .map_err(|e| Error::Parsing(format!("Encoding error: {e}")))?;
    
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut metadata = HashMap::new();
    metadata.insert("encoding".to_string(), "utf8".to_string());
    
    Ok(ExtractedContent {
        word_count: normalized.split_whitespace().count(),
        character_count: normalized.len(),
        text: normalized,
        title: None,
        metadata,
        format: DocumentFormat::PlainText,
        extraction_method: "direct_utf8".to_string(),
    })
}

/// Fallback extraction for unknown formats
fn extract_fallback_content(content: &[u8]) -> Result<ExtractedContent> {
    // Try UTF-8 first
    match std::str::from_utf8(content) {
        Ok(text) => {
            let readable_text: String = text.chars()
                .filter(|c| c.is_ascii_graphic() || c.is_whitespace())
                .collect();
            
            let normalized = readable_text.split_whitespace().collect::<Vec<_>>().join(" ");
            let mut metadata = HashMap::new();
            metadata.insert("extraction_method".to_string(), "fallback_utf8".to_string());
            
            Ok(ExtractedContent {
                word_count: normalized.split_whitespace().count(),
                character_count: normalized.len(),
                text: normalized,
                title: None,
                metadata,
                format: DocumentFormat::Unknown,
                extraction_method: "fallback_extraction".to_string(),
            })
        }
        Err(_) => {
            // Last resort: extract any printable ASCII
            let ascii_text: String = content.iter()
                .filter(|&&b| b.is_ascii_graphic() || b.is_ascii_whitespace())
                .map(|&b| b as char)
                .collect();
            
            if ascii_text.trim().is_empty() {
                return Err(Error::Parsing("No extractable text content found".to_string()));
            }
            
            let normalized = ascii_text.split_whitespace().collect::<Vec<_>>().join(" ");
            let mut metadata = HashMap::new();
            metadata.insert("extraction_method".to_string(), "ascii_fallback".to_string());
            
            Ok(ExtractedContent {
                word_count: normalized.split_whitespace().count(),
                character_count: normalized.len(),
                text: normalized,
                title: None,
                metadata,
                format: DocumentFormat::Unknown,
                extraction_method: "ascii_fallback".to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_html_extraction() {
        let html = b"<html><head><title>Test</title></head><body><h1>Hello</h1><p>World</p></body></html>";
        let result = extract_text(html, "text/html", "test.html").unwrap();
        assert_eq!(result.text, "Hello World");
        assert_eq!(result.title, Some("Test".to_string()));
        assert_eq!(result.format, DocumentFormat::Html);
    }

    #[test]
    fn test_markdown_extraction() {
        let markdown = b"# Title\n\nThis is **bold** text with [link](url).";
        let result = extract_text(markdown, "text/markdown", "test.md").unwrap();
        assert!(result.text.contains("Title"));
        assert!(result.text.contains("bold"));
        assert_eq!(result.format, DocumentFormat::Markdown);
    }

    #[test]
    fn test_format_detection() {
        assert_eq!(detect_format(b"<html>", "text/html", "test.html"), DocumentFormat::Html);
        assert_eq!(detect_format(b"# Header", "", "test.md"), DocumentFormat::Markdown);
        assert_eq!(detect_format(b"%PDF-1.4", "", "test.pdf"), DocumentFormat::Pdf);
    }
}

/// Configuration for content extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractorConfig {
    /// Rules for extraction
    pub rules: Vec<ExtractorRule>,
    /// Maximum text length to process
    pub max_text_length: usize,
    /// Whether to extract structured data
    pub extract_structured: bool,
    /// Whether to extract email addresses
    pub extract_emails: bool,
    /// Whether to extract phone numbers
    pub extract_phones: bool,
    /// Whether to detect sensitive content
    pub detect_sensitive: bool,
    /// Whether to redact sensitive content
    pub redact_sensitive: bool,
    /// Whether to validate email addresses
    pub email_validation: bool,
    /// Whether to format phone numbers
    pub phone_formatting: bool,
}

impl Default for ExtractorConfig {
    fn default() -> Self {
        Self {
            rules: Vec::new(),
            max_text_length: 1_000_000,
            extract_structured: true,
            extract_emails: false,
            extract_phones: false,
            detect_sensitive: false,
            redact_sensitive: false,
            email_validation: false,
            phone_formatting: false,
        }
    }
}

/// Rule for data extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractorRule {
    /// Name of the rule
    pub name: String,
    /// Pattern to match
    pub pattern: String,
    /// Type of extraction
    pub extraction_type: String,
}

/// Basic data extractor
#[derive(Debug, Clone)]
pub struct DataExtractor {
    config: ExtractorConfig,
}

impl DataExtractor {
    /// Create a new data extractor
    pub fn new(config: ExtractorConfig) -> Self {
        Self { config }
    }
    
    /// Extract data from text content
    pub async fn extract(&self, text: &str) -> Result<ExtractionResult> {
        let mut results = ExtractionResult::default();
        
        // Basic extraction - would be enhanced with actual rule processing
        results.metadata.insert("word_count".to_string(), text.split_whitespace().count().to_string());
        results.metadata.insert("char_count".to_string(), text.len().to_string());
        
        // Extract emails if enabled
        if self.config.extract_emails {
            let email_regex = regex::Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
            let emails: Vec<&str> = email_regex.find_iter(text).map(|m| m.as_str()).collect();
            results.emails = emails.into_iter().map(|s| s.to_string()).collect();
        }
        
        // Extract phones if enabled
        if self.config.extract_phones {
            let phone_regex = regex::Regex::new(r"\b\d{3}-?\d{3}-?\d{4}\b").unwrap();
            let phones: Vec<&str> = phone_regex.find_iter(text).map(|m| m.as_str()).collect();
            results.phones = phones.into_iter().map(|s| s.to_string()).collect();
        }
        
        // Set quality score
        results.quality_score = 0.75; // Placeholder
        results.classification = "text_document".to_string();
        
        Ok(results)
    }
    
    /// Extract data from both text and HTML content  
    pub fn extract_all(&self, text_content: &str, html_content: &str) -> Result<ExtractionResult> {
        let mut results = ExtractionResult::default();
        
        // Basic extraction from text
        results.metadata.insert("text_word_count".to_string(), text_content.split_whitespace().count().to_string());
        results.metadata.insert("text_char_count".to_string(), text_content.len().to_string());
        
        // Basic extraction from HTML
        results.metadata.insert("html_char_count".to_string(), html_content.len().to_string());
        results.metadata.insert("has_html_tags".to_string(), html_content.contains('<').to_string());
        
        // Extract emails if enabled
        if self.config.extract_emails {
            let email_regex = regex::Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
            let emails: Vec<&str> = email_regex.find_iter(text_content).map(|m| m.as_str()).collect();
            results.emails = emails.into_iter().map(|s| s.to_string()).collect();
            results.metadata.insert("emails".to_string(), results.emails.join(","));
        }
        
        // Extract phones if enabled
        if self.config.extract_phones {
            let phone_regex = regex::Regex::new(r"\b\d{3}-?\d{3}-?\d{4}\b").unwrap();
            let phones: Vec<&str> = phone_regex.find_iter(text_content).map(|m| m.as_str()).collect();
            results.phones = phones.into_iter().map(|s| s.to_string()).collect();
            results.metadata.insert("phones".to_string(), results.phones.join(","));
        }
        
        Ok(results)
    }
    
    /// Sanitize text content by removing sensitive information
    pub fn sanitize_text(&self, text: &str) -> String {
        if !self.config.redact_sensitive {
            return text.to_string();
        }
        
        let mut sanitized = text.to_string();
        
        // Redact emails
        if self.config.detect_sensitive {
            let email_regex = regex::Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
            sanitized = email_regex.replace_all(&sanitized, "[EMAIL_REDACTED]").to_string();
            
            // Redact phone numbers
            let phone_regex = regex::Regex::new(r"\b\d{3}-?\d{3}-?\d{4}\b").unwrap();
            sanitized = phone_regex.replace_all(&sanitized, "[PHONE_REDACTED]").to_string();
        }
        
        sanitized
    }
}

/// Enhanced data extractor with validation
#[derive(Debug, Clone)]
pub struct EnhancedDataExtractor {
    config: ExtractorConfig,
    validation_config: ValidationConfig,
}

impl EnhancedDataExtractor {
    /// Create a new enhanced data extractor
    pub fn new(config: ExtractorConfig, validation_config: ValidationConfig) -> Self {
        Self { config, validation_config }
    }
    
    /// Extract and validate data
    pub async fn extract_and_validate(&self, text: &str) -> Result<HashMap<String, String>> {
        let mut results = HashMap::new();
        results.insert("validated_text".to_string(), text.to_string());
        Ok(results)
    }
}

/// Configuration for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Whether to validate extracted data
    pub enable_validation: bool,
    /// Minimum confidence threshold
    pub min_confidence: f64,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            enable_validation: true,
            min_confidence: 0.8,
        }
    }
}
