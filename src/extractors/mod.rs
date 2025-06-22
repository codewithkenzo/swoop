use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error::Result;

// Email, phone, and sensitive data detection is implemented directly in this module

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedData {
    pub emails: Vec<String>,
    pub phones: Vec<String>,
    pub sensitive_data: Vec<SensitiveDataMatch>,
    pub links: Vec<String>,
    pub text_content: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitiveDataMatch {
    pub data_type: String,
    pub original_text: String,
    pub redacted_text: String,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub struct ExtractorConfig {
    pub extract_emails: bool,
    pub extract_phones: bool,
    pub detect_sensitive: bool,
    pub redact_sensitive: bool,
    pub email_validation: bool,
    pub phone_formatting: bool,
}

impl Default for ExtractorConfig {
    fn default() -> Self {
        Self {
            extract_emails: true,
            extract_phones: true,
            detect_sensitive: true,
            redact_sensitive: true,
            email_validation: true,
            phone_formatting: true,
        }
    }
}

pub struct DataExtractor {
    config: ExtractorConfig,
    email_regex: Regex,
    phone_regex: Regex,
    ssn_regex: Regex,
    credit_card_regex: Regex,
}

impl DataExtractor {
    pub fn new(config: ExtractorConfig) -> Self {
        // Email regex - comprehensive pattern for various email formats
        let email_regex = Regex::new(
            r"(?i)\b[A-Za-z0-9]([A-Za-z0-9._%-]*[A-Za-z0-9])?@[A-Za-z0-9]([A-Za-z0-9.-]*[A-Za-z0-9])?\.[A-Za-z]{2,}\b"
        ).unwrap();

        // Phone regex - supports various international formats
        let phone_regex = Regex::new(
            r"(?:\+?[1-9]\d{0,3}[-.\s]?)?\(?([0-9]{3})\)?[-.\s]?([0-9]{3})[-.\s]?([0-9]{4})|(?:\+?[1-9]\d{0,3}[-.\s]?)?[0-9]{10,14}"
        ).unwrap();

        // SSN regex - US Social Security Number pattern
        let ssn_regex = Regex::new(r"\b\d{3}-?\d{2}-?\d{4}\b").unwrap();

        // Credit card regex - basic pattern for major card types
        let credit_card_regex = Regex::new(
            r"\b(?:4[0-9]{12}(?:[0-9]{3})?|5[1-5][0-9]{14}|3[47][0-9]{13}|3[0-9]{13}|6(?:011|5[0-9]{2})[0-9]{12})\b"
        ).unwrap();

        Self {
            config,
            email_regex,
            phone_regex,
            ssn_regex,
            credit_card_regex,
        }
    }

    /// Extract all data from text content
    pub fn extract_all(&self, text: &str, html: &str) -> Result<ExtractedData> {
        let mut extracted = ExtractedData {
            emails: Vec::new(),
            phones: Vec::new(),
            sensitive_data: Vec::new(),
            links: Vec::new(),
            text_content: text.to_string(),
            metadata: HashMap::new(),
        };

        if self.config.extract_emails {
            extracted.emails = self.extract_emails(text);
        }

        if self.config.extract_phones {
            extracted.phones = self.extract_phones(text);
        }

        if self.config.detect_sensitive {
            extracted.sensitive_data = self.detect_sensitive_data(text);
        }

        // Extract links from HTML
        extracted.links = self.extract_links(html);

        // Add metadata
        extracted.metadata.insert("extraction_timestamp".to_string(), 
                                 chrono::Utc::now().to_rfc3339());
        extracted.metadata.insert("total_emails".to_string(), 
                                 extracted.emails.len().to_string());
        extracted.metadata.insert("total_phones".to_string(), 
                                 extracted.phones.len().to_string());
        extracted.metadata.insert("sensitive_items".to_string(), 
                                 extracted.sensitive_data.len().to_string());

        Ok(extracted)
    }

    /// Extract email addresses from text
    pub fn extract_emails(&self, text: &str) -> Vec<String> {
        let mut emails = Vec::new();
        
        for mat in self.email_regex.find_iter(text) {
            let email = mat.as_str().to_lowercase();
            
            if self.config.email_validation {
                // Basic validation - check for common invalid patterns
                if !email.starts_with('.') && !email.ends_with('.') 
                   && !email.contains("..") && email.contains('@') {
                    emails.push(email);
                }
            } else {
                emails.push(email);
            }
        }

        // Remove duplicates
        emails.sort();
        emails.dedup();
        emails
    }

    /// Extract phone numbers from text
    pub fn extract_phones(&self, text: &str) -> Vec<String> {
        let mut phones = Vec::new();
        
        for mat in self.phone_regex.find_iter(text) {
            let phone = mat.as_str();
            
            if self.config.phone_formatting {
                // Normalize phone number format
                let normalized = self.normalize_phone(phone);
                if !normalized.is_empty() {
                    phones.push(normalized);
                }
            } else {
                phones.push(phone.to_string());
            }
        }

        // Remove duplicates
        phones.sort();
        phones.dedup();
        phones
    }

    /// Detect sensitive data (SSN, credit cards, etc.)
    pub fn detect_sensitive_data(&self, text: &str) -> Vec<SensitiveDataMatch> {
        let mut sensitive_data = Vec::new();

        // Detect SSNs
        for mat in self.ssn_regex.find_iter(text) {
            let ssn = mat.as_str();
            sensitive_data.push(SensitiveDataMatch {
                data_type: "SSN".to_string(),
                original_text: ssn.to_string(),
                redacted_text: if self.config.redact_sensitive {
                    "XXX-XX-XXXX".to_string()
                } else {
                    ssn.to_string()
                },
                confidence: 0.95,
            });
        }

        // Detect credit cards
        for mat in self.credit_card_regex.find_iter(text) {
            let cc = mat.as_str();
            sensitive_data.push(SensitiveDataMatch {
                data_type: "Credit Card".to_string(),
                original_text: cc.to_string(),
                redacted_text: if self.config.redact_sensitive {
                    format!("XXXX-XXXX-XXXX-{}", &cc[cc.len()-4..])
                } else {
                    cc.to_string()
                },
                confidence: 0.90,
            });
        }

        sensitive_data
    }

    /// Extract links from HTML
    pub fn extract_links(&self, html: &str) -> Vec<String> {
        let link_regex = Regex::new(r#"href\s*=\s*["']([^"']+)["']"#).unwrap();
        let mut links = Vec::new();

        for mat in link_regex.captures_iter(html) {
            if let Some(link) = mat.get(1) {
                links.push(link.as_str().to_string());
            }
        }

        // Remove duplicates and sort
        links.sort();
        links.dedup();
        links
    }

    /// Normalize phone number to standard format
    fn normalize_phone(&self, phone: &str) -> String {
        // Remove all non-digit characters except +
        let digits: String = phone.chars()
            .filter(|c| c.is_ascii_digit() || *c == '+')
            .collect();

        // Basic US number formatting
        if digits.len() == 10 {
            format!("({}) {}-{}", &digits[0..3], &digits[3..6], &digits[6..10])
        } else if digits.len() == 11 && digits.starts_with('1') {
            format!("+1 ({}) {}-{}", &digits[1..4], &digits[4..7], &digits[7..11])
        } else if digits.starts_with('+') && digits.len() > 10 {
            digits // Keep international format as-is
        } else if digits.len() >= 10 {
            // Try to format as standard US number
            let end = digits.len().min(10);
            let start = digits.len() - 10;
            let core = &digits[start..end];
            if core.len() == 10 {
                format!("({}) {}-{}", &core[0..3], &core[3..6], &core[6..10])
            } else {
                phone.to_string()
            }
        } else {
            String::new() // Invalid phone number
        }
    }

    /// Get a sanitized version of text with sensitive data redacted
    pub fn sanitize_text(&self, text: &str) -> String {
        let mut sanitized = text.to_string();

        if self.config.redact_sensitive {
            // Redact SSNs
            sanitized = self.ssn_regex.replace_all(&sanitized, "XXX-XX-XXXX").to_string();
            
            // Redact credit cards
            sanitized = self.credit_card_regex.replace_all(&sanitized, "XXXX-XXXX-XXXX-XXXX").to_string();
        }

        sanitized
    }
} 