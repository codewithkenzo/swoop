/*!
 * Parser module for Crawl4AI
 * 
 * This module provides functionality for parsing different types of content,
 * including HTML, JSON, XML, and plain text.
 */

use std::collections::HashMap;
use std::sync::Arc;

use regex::Regex;
use scraper::{Html, Selector};
use serde_json::Value;
use tokio::sync::RwLock;
use url::Url;
use serde::{Serialize, Deserialize};

use crate::config::{ParserConfig, SelectorType};
use crate::error::{Error, Result};
use crate::models::{ExtractedContent, Link, Metadata};

/// Content type enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentType {
    /// HTML content
    Html,
    /// JSON content
    Json,
    /// XML content
    Xml,
    /// Plain text content
    Text,
    /// PDF content
    Pdf,
    /// Unknown content type
    Unknown,
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        let content_type = content_type.to_lowercase();
        if content_type.contains("html") {
            ContentType::Html
        } else if content_type.contains("json") {
            ContentType::Json
        } else if content_type.contains("xml") {
            ContentType::Xml
        } else if content_type.contains("text") {
            ContentType::Text
        } else if content_type.contains("pdf") {
            ContentType::Pdf
        } else {
            ContentType::Unknown
        }
    }
}

/// Extractor rule for parsing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractorRule {
    /// Name of the rule
    pub name: String,
    /// CSS selector or pattern
    pub selector: String, 
    /// Type of content to extract
    pub content_type: String,
    /// Type of selector
    pub selector_type: SelectorType,
    /// Whether to extract multiple values
    pub multiple: bool,
    /// HTML attribute to extract (for CSS selectors)
    pub attribute: Option<String>,
    /// Default value if extraction fails
    pub default_value: Option<String>,
    /// Whether this field is required
    pub required: bool,
}

/// Result of parsing operation
#[derive(Debug, Clone)]
pub struct ParseResult {
    /// Document title
    pub title: String,
    /// Raw HTML content
    pub html: String,
    /// Extracted text content
    pub text: String,
    /// Processed content
    pub content: String,
    /// Extracted structured content
    pub extracted: HashMap<String, ExtractedContent>,
    /// Links found in the document
    pub links: Vec<Link>,
    /// Success status
    pub success: bool,
    /// Error message if any
    pub error: Option<String>,
}

impl Default for ParseResult {
    fn default() -> Self {
        Self {
            title: String::new(),
            html: String::new(),
            text: String::new(),
            content: String::new(),
            extracted: HashMap::new(),
            links: Vec::new(),
            success: true,
            error: None,
        }
    }
}

/// Builder for creating a Parser
#[derive(Debug)]
pub struct ParserBuilder {
    /// Configuration for the parser
    config: ParserConfig,
    /// Custom extraction rules
    rules: Vec<ExtractorRule>,
}

impl Default for ParserBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ParserBuilder {
    /// Create a new ParserBuilder with default configuration
    pub fn new() -> Self {
        Self {
            config: ParserConfig::default(),
            rules: Vec::new(),
        }
    }
    
    /// Set the configuration
    pub fn with_config(mut self, config: ParserConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Add an extraction rule
    pub fn with_rule(mut self, rule: ExtractorRule) -> Self {
        self.rules.push(rule);
        self
    }
    
    /// Add multiple extraction rules
    pub fn with_rules(mut self, rules: Vec<ExtractorRule>) -> Self {
        self.rules.extend(rules);
        self
    }
    
    /// Build the Parser
    pub fn build(self) -> Parser {
        Parser {
            config: self.config,
            rules: Arc::new(RwLock::new(self.rules)),
        }
    }
}

/// Parser for extracting content from documents
#[derive(Debug)]
pub struct Parser {
    /// Configuration for the parser
    config: ParserConfig,
    /// Extraction rules
    rules: Arc<RwLock<Vec<ExtractorRule>>>,
}

impl Parser {
    /// Create a new Parser with default configuration
    pub fn new() -> Self {
        Self {
            config: ParserConfig::default(),
            rules: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Parse content asynchronously
    pub async fn parse(&self, content: &[u8], content_type_str: &str, metadata: &Metadata) -> Result<ParseResult> {
        // Determine content type
        let content_type = ContentType::from(content_type_str);
        
        // Check content size
        if self.config.max_content_length > 0 && content.len() > self.config.max_content_length {
            return Err(Error::Parser(format!(
                "Content size ({} bytes) exceeds maximum allowed size ({} bytes)",
                content.len(),
                self.config.max_content_length
            )));
        }

        // Get extraction rules once
        let rules = self.rules.read().await.clone();

        // Parse content synchronously based on type
        match content_type {
            ContentType::Html => self.parse_html_sync(content, metadata, &rules),
            ContentType::Json => self.parse_json_sync(content, metadata, &rules),
            ContentType::Xml => self.parse_xml_sync(content, metadata, &rules),
            ContentType::Text => self.parse_text_sync(content, metadata, &rules),
            ContentType::Pdf => self.parse_pdf_sync(content, metadata, &rules),
            ContentType::Unknown => {
                // Try to detect content type by examining content
                let content_str = String::from_utf8_lossy(content);
                if content_str.trim_start().starts_with('<') {
                    self.parse_html_sync(content, metadata, &rules)
                } else if content_str.trim_start().starts_with('{') || content_str.trim_start().starts_with('[') {
                    self.parse_json_sync(content, metadata, &rules)
                } else {
                    self.parse_text_sync(content, metadata, &rules)
                }
            }
        }
    }

    /// Parse HTML content synchronously
    fn parse_html_sync(&self, content: &[u8], metadata: &Metadata, rules: &[ExtractorRule]) -> Result<ParseResult> {
        let html_str = String::from_utf8_lossy(content);
        let document = Html::parse_document(&html_str);

        // Extract content synchronously
        let title = self.extract_title_sync(&document).unwrap_or_default();
        let text = self.extract_text_sync(&document).unwrap_or_default();
        let links = self.extract_links_sync(&document, metadata.source_url.as_deref().unwrap_or("")).unwrap_or_default();
        let mut extracted = self.apply_extraction_rules_sync(&document, rules)?;
        
        // Add basic content extraction
        if !title.is_empty() {
            extracted.insert("title".to_string(), ExtractedContent {
                content_type: "text".to_string(),
                name: "title".to_string(),
                content: title.clone(),
                attributes: HashMap::new(),
            });
        }
        if !text.is_empty() {
            extracted.insert("text".to_string(), ExtractedContent {
                content_type: "text".to_string(),
                name: "text".to_string(),
                content: text.clone(),
                attributes: HashMap::new(),
            });
        }
        extracted.insert("html".to_string(), ExtractedContent {
            content_type: "html".to_string(),
            name: "html".to_string(),
            content: html_str.to_string(),
            attributes: HashMap::new(),
        });

        Ok(ParseResult {
            title,
            html: html_str.to_string(),
            text,
            content: html_str.to_string(),
            extracted,
            links,
            success: true,
            error: None,
        })
    }

    /// Parse JSON content synchronously
    fn parse_json_sync(&self, content: &[u8], metadata: &Metadata, rules: &[ExtractorRule]) -> Result<ParseResult> {
        let json_str = String::from_utf8_lossy(content);
        let json: Value = serde_json::from_str(&json_str)
            .map_err(|e| Error::Parser(format!("Invalid JSON: {e}")))?;

        // Extract content synchronously
        let title = json
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let text = json
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| {
                // Fallback to stringified JSON
                &json_str
            })
            .to_string();
        let links = self.extract_links_from_json_sync(&json, metadata.source_url.as_deref().unwrap_or("")).unwrap_or_default();
        let extracted = self.apply_json_extraction_rules_sync(&json, metadata, rules)?;

        Ok(ParseResult {
            title,
            html: String::new(),
            text,
            content: json_str.to_string(),
            extracted,
            links,
            success: true,
            error: None,
        })
    }

    /// Parse XML content synchronously
    fn parse_xml_sync(&self, content: &[u8], _metadata: &Metadata, _rules: &[ExtractorRule]) -> Result<ParseResult> {
        let xml_str = String::from_utf8_lossy(content);
        
        // Simple XML title extraction without kuchiki
        let title = if let Some(start) = xml_str.find("<title>") {
            if let Some(end) = xml_str[start + 7..].find("</title>") {
                xml_str[start + 7..start + 7 + end].to_string()
            } else {
                "XML Document".to_string()
            }
        } else {
            "XML Document".to_string()
        };
        
        Ok(ParseResult {
            title,
            html: String::new(),
            text: xml_str.to_string(),
            content: xml_str.to_string(),
            links: Vec::new(),
            extracted: HashMap::new(),
            success: true,
            error: None,
        })
    }

    /// Parse text content synchronously
    fn parse_text_sync(&self, content: &[u8], metadata: &Metadata, rules: &[ExtractorRule]) -> Result<ParseResult> {
        let text = String::from_utf8_lossy(content);
        let links = self.extract_links_from_text_sync(&text, metadata.source_url.as_deref().unwrap_or("")).unwrap_or_default();
        let extracted = self.apply_text_extraction_rules_sync(&text, metadata, rules)?;

        Ok(ParseResult {
            title: metadata.source_url.clone().unwrap_or_default(),
            html: String::new(),
            text: text.to_string(),
            content: text.to_string(),
            links,
            extracted,
            success: true,
            error: None,
        })
    }

    /// Parse PDF content synchronously
    fn parse_pdf_sync(&self, _content: &[u8], metadata: &Metadata, _rules: &[ExtractorRule]) -> Result<ParseResult> {
        // TODO: Implement PDF parsing
        // For now, return a placeholder result
        let result = ParseResult {
            title: metadata.source_url.clone().unwrap_or_default(),
            html: String::new(),
            text: "PDF content not yet supported".to_string(),
            content: String::new(),
            links: Vec::new(),
            extracted: HashMap::new(),
            success: true,
            error: None,
        };
        
        Ok(result)
    }

    /// Extract title from HTML document synchronously
    fn extract_title_sync(&self, document: &Html) -> Option<String> {
        // Try different title selectors
        let selectors = [
            "title",
            "h1",
            "[property='og:title']",
            "[name='twitter:title']",
            ".title",
            "#title",
        ];

        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    let title = if selector_str.starts_with('[') {
                        element.value().attr("content").unwrap_or("").to_string()
                    } else {
                        element.text().collect::<Vec<_>>().join(" ").trim().to_string()
                    };
                    
                    if !title.is_empty() {
                        return Some(title);
                    }
                }
            }
        }

        None
    }

    /// Extract text from HTML document synchronously
    fn extract_text_sync(&self, document: &Html) -> Option<String> {
        // Extract text content
        let text = document.root_element().text().collect::<Vec<_>>().join(" ");
        let cleaned = text
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");

        if cleaned.is_empty() {
            None
        } else {
            Some(cleaned)
        }
    }

    /// Extract links from HTML document synchronously
    fn extract_links_sync(&self, document: &Html, base_url: &str) -> Option<Vec<Link>> {
        let selector = Selector::parse("a[href]").ok()?;
        let mut links = Vec::new();

        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                let link_text = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
                let rel = element.value().attr("rel").map(|s| s.to_string());

                // Resolve relative URLs
                let absolute_url = if href.starts_with("http") {
                    href.to_string()
                } else {
                    match Url::parse(base_url) {
                        Ok(base) => match base.join(href) {
                            Ok(resolved) => resolved.to_string(),
                            Err(_) => continue,
                        },
                        Err(_) => continue,
                    }
                };

                links.push(Link {
                    url: absolute_url,
                    text: link_text,
                    source_url: base_url.to_string(),
                    rel,
                });
            }
        }

        Some(links)
    }

    /// Extract links from text content synchronously
    fn extract_links_from_text_sync(&self, text: &str, base_url: &str) -> Option<Vec<Link>> {
        let url_regex = Regex::new(r"https?://[^\s<>\[\]{}|\\^`]+").ok()?;
        let mut links = Vec::new();

        for match_result in url_regex.find_iter(text) {
            let url = match_result.as_str().to_string();
            links.push(Link {
                url: url.clone(),
                text: url.clone(),
                source_url: base_url.to_string(),
                rel: None,
            });
        }

        if links.is_empty() {
            None
        } else {
            Some(links)
        }
    }

    /// Apply extraction rules synchronously
    fn apply_extraction_rules_sync(&self, document: &Html, rules: &[ExtractorRule]) -> Result<HashMap<String, ExtractedContent>> {
        let mut extracted = HashMap::new();

        for rule in rules {
            match rule.selector_type {
                SelectorType::CSS => {
                    if let Ok(selector) = Selector::parse(&rule.selector) {
                        let elements: Vec<_> = document.select(&selector).collect();
                        
                        if rule.multiple {
                            let values: Vec<String> = elements
                                .iter()
                                .map(|el| {
                                    if let Some(attr) = &rule.attribute {
                                        el.value().attr(attr).unwrap_or("").to_string()
                                    } else {
                                        el.text().collect::<Vec<_>>().join(" ").trim().to_string()
                                    }
                                })
                                .filter(|s| !s.is_empty())
                                .collect();
                            
                            extracted.insert(rule.name.clone(), ExtractedContent {
                                content_type: "html".to_string(),
                                name: rule.name.clone(),
                                content: values.join(", "),
                                attributes: HashMap::new(),
                            });
                        } else if let Some(element) = elements.first() {
                            let value = if let Some(attr) = &rule.attribute {
                                element.value().attr(attr).unwrap_or("").to_string()
                            } else {
                                element.text().collect::<Vec<_>>().join(" ").trim().to_string()
                            };
                            
                            if !value.is_empty() {
                                extracted.insert(rule.name.clone(), ExtractedContent {
                                    content_type: "html".to_string(),
                                    name: rule.name.clone(),
                                    content: value,
                                    attributes: HashMap::new(),
                                });
                            } else if let Some(default) = &rule.default_value {
                                extracted.insert(rule.name.clone(), ExtractedContent {
                                    content_type: "html".to_string(),
                                    name: rule.name.clone(),
                                    content: default.clone(),
                                    attributes: HashMap::new(),
                                });
                            }
                        } else if rule.required {
                            return Err(Error::Parser(format!("Required rule '{}' did not match any elements", rule.name)));
                        } else if let Some(default) = &rule.default_value {
                            extracted.insert(rule.name.clone(), ExtractedContent {
                                content_type: "html".to_string(),
                                name: rule.name.clone(),
                                content: default.clone(),
                                attributes: HashMap::new(),
                            });
                        }
                    }
                }
                SelectorType::XPath => {
                    // XPath not implemented for now
                    if rule.required {
                        return Err(Error::Parser(format!("XPath selector not implemented for required rule '{}'", rule.name)));
                    }
                }
                SelectorType::JSONPath => {
                    // JSONPath not implemented for now
                    if rule.required {
                        return Err(Error::Parser(format!("JSONPath selector not implemented for required rule '{}'", rule.name)));
                    }
                }
                SelectorType::Regex => {
                    // Regex not implemented for now
                    if rule.required {
                        return Err(Error::Parser(format!("Regex selector not implemented for required rule '{}'", rule.name)));
                    }
                }
            }
        }

        Ok(extracted)
    }

    /// Apply extraction rules to JSON document
    fn apply_json_extraction_rules_sync(&self, _json: &Value, _metadata: &Metadata, _rules: &[ExtractorRule]) -> Result<HashMap<String, ExtractedContent>> {
        // TODO: Implement JSON extraction rules
        Ok(HashMap::new())
    }

    /// Apply extraction rules to XML document
    #[allow(dead_code)]
    fn apply_xml_extraction_rules_sync(&self, _xml_content: &str, _metadata: &Metadata, _rules: &[ExtractorRule]) -> Result<HashMap<String, ExtractedContent>> {
        // TODO: Implement XML extraction rules
        Ok(HashMap::new())
    }

    /// Apply extraction rules to text document
    fn apply_text_extraction_rules_sync(&self, _text: &str, _metadata: &Metadata, _rules: &[ExtractorRule]) -> Result<HashMap<String, ExtractedContent>> {
        // TODO: Implement text extraction rules
        Ok(HashMap::new())
    }

    /// Extract links from JSON document synchronously
    fn extract_links_from_json_sync(&self, _json: &Value, _base_url: &str) -> Option<Vec<Link>> {
        // TODO: Implement link extraction from JSON
        Some(Vec::new())
    }

    /// Extract links from XML document synchronously
    #[allow(dead_code)]
    fn extract_links_from_xml_sync(&self, _xml_content: &str, _base_url: &str) -> Option<Vec<Link>> {
        // TODO: Implement link extraction from XML
        Some(Vec::new())
    }

    /// Add an extraction rule
    pub async fn add_rule(&self, rule: ExtractorRule) {
        let mut rules = self.rules.write().await;
        rules.push(rule);
    }

    /// Remove an extraction rule by name
    pub async fn remove_rule(&self, name: &str) -> bool {
        let mut rules = self.rules.write().await;
        let initial_len = rules.len();
        rules.retain(|rule| rule.name != name);
        rules.len() < initial_len
    }

    /// Get all extraction rules
    pub async fn get_rules(&self) -> Vec<ExtractorRule> {
        self.rules.read().await.clone()
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}




