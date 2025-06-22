/*!
 * Content Extraction Module
 * 
 * Provides basic content extraction capabilities for HTML and text documents
 */

use std::collections::HashMap;
use crate::error::Result;

/// Extract text content from HTML
pub fn extract_html_content(html: &str) -> Result<String> {
    // Simple HTML tag removal - replace with proper HTML parsing in production
    let mut content = html.to_string();
    
    // Remove script and style tags with their content
    content = remove_tags_with_content(&content, "script");
    content = remove_tags_with_content(&content, "style");
    
    // Remove HTML tags
    content = remove_html_tags(&content);
    
    // Clean up whitespace
    content = clean_whitespace(&content);
    
    Ok(content)
}

/// Remove HTML tags with their content
fn remove_tags_with_content(html: &str, tag: &str) -> String {
    let start_pattern = format!("<{}", tag);
    let end_pattern = format!("</{}>", tag);
    
    let mut result = html.to_string();
    
    while let Some(start) = result.find(&start_pattern) {
        if let Some(tag_end) = result[start..].find('>') {
            let tag_close_start = start + tag_end + 1;
            if let Some(end) = result[tag_close_start..].find(&end_pattern) {
                let end_pos = tag_close_start + end + end_pattern.len();
                result.replace_range(start..end_pos, "");
            } else {
                break;
            }
        } else {
            break;
        }
    }
    
    result
}

/// Remove HTML tags
fn remove_html_tags(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => {
                in_tag = false;
                result.push(' '); // Replace tag with space
            }
            _ if !in_tag => result.push(ch),
            _ => {} // Skip characters inside tags
        }
    }
    
    result
}

/// Clean up whitespace
fn clean_whitespace(text: &str) -> String {
    // Replace multiple whitespace with single space
    let mut result = String::new();
    let mut prev_was_space = false;
    
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !prev_was_space {
                result.push(' ');
                prev_was_space = true;
            }
        } else {
            result.push(ch);
            prev_was_space = false;
        }
    }
    
    result.trim().to_string()
}

/// Extract basic metadata from HTML
pub fn extract_html_metadata(html: &str) -> HashMap<String, String> {
    let mut metadata = HashMap::new();
    
    // Extract title
    if let Some(title) = extract_tag_content(html, "title") {
        metadata.insert("title".to_string(), title);
    }
    
    // Extract meta tags
    extract_meta_tags(html, &mut metadata);
    
    metadata
}

/// Extract content from a specific HTML tag
fn extract_tag_content(html: &str, tag: &str) -> Option<String> {
    let start_pattern = format!("<{}", tag);
    let end_pattern = format!("</{}>", tag);
    
    if let Some(start) = html.find(&start_pattern) {
        if let Some(tag_end) = html[start..].find('>') {
            let content_start = start + tag_end + 1;
            if let Some(end) = html[content_start..].find(&end_pattern) {
                let content = &html[content_start..content_start + end];
                return Some(clean_whitespace(content));
            }
        }
    }
    
    None
}

/// Extract meta tags from HTML
fn extract_meta_tags(html: &str, metadata: &mut HashMap<String, String>) {
    let mut pos = 0;
    
    while let Some(meta_start) = html[pos..].find("<meta") {
        let absolute_start = pos + meta_start;
        if let Some(meta_end) = html[absolute_start..].find('>') {
            let meta_tag = &html[absolute_start..absolute_start + meta_end + 1];
            
            // Extract name and content attributes
            if let (Some(name), Some(content)) = (extract_attribute(meta_tag, "name"), extract_attribute(meta_tag, "content")) {
                metadata.insert(name, content);
            }
            
            // Extract property and content attributes (for Open Graph)
            if let (Some(property), Some(content)) = (extract_attribute(meta_tag, "property"), extract_attribute(meta_tag, "content")) {
                metadata.insert(property, content);
            }
            
            pos = absolute_start + meta_end + 1;
        } else {
            break;
        }
    }
}

/// Extract attribute value from HTML tag
fn extract_attribute(tag: &str, attr_name: &str) -> Option<String> {
    let pattern = format!("{}=", attr_name);
    
    if let Some(start) = tag.find(&pattern) {
        let value_start = start + pattern.len();
        let chars: Vec<char> = tag[value_start..].chars().collect();
        
        if chars.is_empty() {
            return None;
        }
        
        let quote_char = chars[0];
        if quote_char == '"' || quote_char == '\'' {
            // Find closing quote
            if let Some(end) = chars[1..].iter().position(|&c| c == quote_char) {
                let value: String = chars[1..end + 1].iter().collect();
                return Some(value);
            }
        }
    }
    
    None
}
