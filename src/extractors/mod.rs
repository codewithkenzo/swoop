/*!
 * Enhanced Data Extraction Module
 * 
 * Production-ready data extraction with intelligence, validation,
 * and preventive error handling for document processing.
 */

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error::Result;
use crate::intelligence::{IntelligenceProcessor, ContentIntelligence, IntelligenceConfig};

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

/// Enhanced extraction results with intelligence metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedExtractionResults {
    /// Raw extraction results
    pub raw_data: ExtractionResults,
    /// Intelligence analysis
    pub intelligence: ContentIntelligence,
    /// Validation status
    pub validation_status: ValidationStatus,
    /// Processing recommendations
    pub recommendations: Vec<String>,
    /// Confidence scores for each extraction type
    pub confidence_scores: HashMap<String, f64>,
}

/// Validation status for extracted data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationStatus {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
    pub quality_score: f64,
}

/// Validation error details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub error_type: String,
    pub message: String,
    pub severity: ErrorSeverity,
}

/// Error severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Basic extraction results structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionResults {
    pub emails: Vec<EmailExtraction>,
    pub phones: Vec<PhoneExtraction>,
    pub links: Vec<LinkExtraction>,
    pub sensitive_data: Vec<SensitiveDataDetection>,
    pub metadata: ExtractionMetadata,
}

/// Email extraction with validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailExtraction {
    pub email: String,
    pub confidence: f64,
    pub context: String,
    pub validation_result: EmailValidation,
    pub position: (usize, usize),
}

/// Email validation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailValidation {
    pub is_valid: bool,
    pub domain_exists: bool,
    pub is_disposable: bool,
    pub risk_level: RiskLevel,
}

/// Phone extraction with validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneExtraction {
    pub phone: String,
    pub formatted: String,
    pub country_code: Option<String>,
    pub confidence: f64,
    pub context: String,
    pub validation_result: PhoneValidation,
    pub position: (usize, usize),
}

/// Phone validation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneValidation {
    pub is_valid: bool,
    pub is_mobile: Option<bool>,
    pub carrier: Option<String>,
    pub region: Option<String>,
    pub risk_level: RiskLevel,
}

/// Link extraction with analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkExtraction {
    pub url: String,
    pub text: String,
    pub link_type: LinkType,
    pub confidence: f64,
    pub security_assessment: SecurityAssessment,
    pub position: (usize, usize),
}

/// Link type classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkType {
    Internal,
    External,
    Email,
    Phone,
    Social,
    Download,
    Other,
}

/// Security assessment for links
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAssessment {
    pub is_safe: bool,
    pub risk_factors: Vec<String>,
    pub reputation_score: f64,
    pub threat_level: RiskLevel,
}

/// Sensitive data detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitiveDataDetection {
    pub data_type: SensitiveDataType,
    pub original_value: String,
    pub redacted_value: String,
    pub confidence: f64,
    pub context: String,
    pub position: (usize, usize),
}

/// Types of sensitive data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensitiveDataType {
    SSN,
    CreditCard,
    BankAccount,
    IPAddress,
    APIKey,
    Password,
    PersonalName,
    Address,
    Other(String),
}

/// Risk level classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Extraction metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionMetadata {
    pub processed_at: chrono::DateTime<chrono::Utc>,
    pub processing_time_ms: u64,
    pub content_length: usize,
    pub extraction_version: String,
    pub total_extractions: usize,
    pub success_rate: f64,
}

/// Enhanced data extractor with intelligence
pub struct EnhancedDataExtractor {
    intelligence_processor: IntelligenceProcessor,
    email_regex: Regex,
    phone_regex: Regex,
    ssn_regex: Regex,
    credit_card_regex: Regex,
    ip_address_regex: Regex,
    validation_config: ValidationConfig,
}

/// Validation configuration
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    pub min_confidence_threshold: f64,
    pub enable_domain_validation: bool,
    pub enable_phone_validation: bool,
    pub enable_security_checks: bool,
    pub max_extraction_per_type: usize,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            min_confidence_threshold: 0.7,
            enable_domain_validation: true,
            enable_phone_validation: true,
            enable_security_checks: true,
            max_extraction_per_type: 100,
        }
    }
}

impl EnhancedDataExtractor {
    /// Create new enhanced data extractor
    pub fn new(intelligence_config: IntelligenceConfig, validation_config: ValidationConfig) -> Result<Self> {
        let intelligence_processor = IntelligenceProcessor::new(intelligence_config);
        
        // Compile all regex patterns with error handling
        let email_regex = Regex::new(
            r"(?i)\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b"
        ).map_err(|e| crate::error::Error::Other(format!("Failed to compile email regex: {}", e)))?;
        
        let phone_regex = Regex::new(
            r"(?:\+?1[-.\s]?)?\(?([0-9]{3})\)?[-.\s]?([0-9]{3})[-.\s]?([0-9]{4})"
        ).map_err(|e| crate::error::Error::Other(format!("Failed to compile phone regex: {}", e)))?;
        
        let ssn_regex = Regex::new(
            r"\b\d{3}-?\d{2}-?\d{4}\b"
        ).map_err(|e| crate::error::Error::Other(format!("Failed to compile SSN regex: {}", e)))?;
        
        let credit_card_regex = Regex::new(
            r"\b(?:\d{4}[-\s]?){3}\d{4}\b"
        ).map_err(|e| crate::error::Error::Other(format!("Failed to compile credit card regex: {}", e)))?;
        
        let ip_address_regex = Regex::new(
            r"\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b"
        ).map_err(|e| crate::error::Error::Other(format!("Failed to compile IP address regex: {}", e)))?;
        
        Ok(Self {
            intelligence_processor,
            email_regex,
            phone_regex,
            ssn_regex,
            credit_card_regex,
            ip_address_regex,
            validation_config,
        })
    }
    
    /// Extract data with full intelligence analysis
    pub async fn extract_with_intelligence(
        &self,
        content: &str,
        url: &str,
        existing_content_hashes: &[String],
    ) -> Result<EnhancedExtractionResults> {
        let start_time = std::time::Instant::now();
        
        // Step 1: Run intelligence analysis
        let intelligence = self.intelligence_processor
            .process_content(content, url, existing_content_hashes).await?;
        
        // Step 2: Extract basic data
        let raw_data = self.extract_basic_data(content).await?;
        
        // Step 3: Validate extracted data
        let validation_status = self.validate_extractions(&raw_data).await?;
        
        // Step 4: Generate confidence scores
        let confidence_scores = self.calculate_confidence_scores(&raw_data, &intelligence);
        
        // Step 5: Generate recommendations
        let recommendations = self.generate_recommendations(&raw_data, &intelligence, &validation_status);
        
        Ok(EnhancedExtractionResults {
            raw_data,
            intelligence,
            validation_status,
            recommendations,
            confidence_scores,
        })
    }
    
    /// Extract basic data without full intelligence (faster)
    pub async fn extract_basic_data(&self, content: &str) -> Result<ExtractionResults> {
        let start_time = std::time::Instant::now();
        
        // Extract emails with validation
        let emails = self.extract_emails(content).await?;
        
        // Extract phones with validation
        let phones = self.extract_phones(content).await?;
        
        // Extract links
        let links = self.extract_links(content).await?;
        
        // Detect sensitive data
        let sensitive_data = self.detect_sensitive_data(content).await?;
        
        let processing_time = start_time.elapsed();
        let metadata = ExtractionMetadata {
            processed_at: chrono::Utc::now(),
            processing_time_ms: processing_time.as_millis() as u64,
            content_length: content.len(),
            extraction_version: env!("CARGO_PKG_VERSION").to_string(),
            total_extractions: emails.len() + phones.len() + links.len() + sensitive_data.len(),
            success_rate: 1.0, // Calculate based on validation results
        };
        
        Ok(ExtractionResults {
            emails,
            phones,
            links,
            sensitive_data,
            metadata,
        })
    }
    
    // Individual extraction methods
    async fn extract_emails(&self, content: &str) -> Result<Vec<EmailExtraction>> {
        let mut emails = Vec::new();
        
        for mat in self.email_regex.find_iter(content) {
            let email = mat.as_str().to_lowercase();
            let position = (mat.start(), mat.end());
            
            // Get context around the email
            let context = self.get_context(content, position, 50);
            
            // Validate email
            let validation_result = self.validate_email(&email).await?;
            
            // Calculate confidence based on various factors
            let confidence = self.calculate_email_confidence(&email, &context, &validation_result);
            
            emails.push(EmailExtraction {
                email,
                confidence,
                context,
                validation_result,
                position,
            });
            
            if emails.len() >= self.validation_config.max_extraction_per_type {
                break;
            }
        }
        
        Ok(emails)
    }
    
    async fn extract_phones(&self, content: &str) -> Result<Vec<PhoneExtraction>> {
        let mut phones = Vec::new();
        
        for mat in self.phone_regex.find_iter(content) {
            let phone = mat.as_str().to_string();
            let position = (mat.start(), mat.end());
            
            // Format phone number
            let formatted = self.format_phone(&phone);
            
            // Get context
            let context = self.get_context(content, position, 50);
            
            // Validate phone
            let validation_result = self.validate_phone(&phone).await?;
            
            // Calculate confidence
            let confidence = self.calculate_phone_confidence(&phone, &context, &validation_result);
            
            phones.push(PhoneExtraction {
                phone: phone.clone(),
                formatted,
                country_code: Some("US".to_string()), // Simple implementation
                confidence,
                context,
                validation_result,
                position,
            });
            
            if phones.len() >= self.validation_config.max_extraction_per_type {
                break;
            }
        }
        
        Ok(phones)
    }
    
    async fn extract_links(&self, content: &str) -> Result<Vec<LinkExtraction>> {
        // Simple link extraction - in production, use proper HTML parsing
        let link_regex = Regex::new(r#"https?://[^\s<>"'{}|\\^`[\]]+"#).unwrap();
        let mut links = Vec::new();
        
        for mat in link_regex.find_iter(content) {
            let url = mat.as_str().to_string();
            let position = (mat.start(), mat.end());
            
            let link_type = self.classify_link_type(&url);
            let security_assessment = self.assess_link_security(&url).await?;
            let confidence = self.calculate_link_confidence(&url, &security_assessment);
            
            links.push(LinkExtraction {
                url: url.clone(),
                text: url, // Simplified
                link_type,
                confidence,
                security_assessment,
                position,
            });
            
            if links.len() >= self.validation_config.max_extraction_per_type {
                break;
            }
        }
        
        Ok(links)
    }
    
    async fn detect_sensitive_data(&self, content: &str) -> Result<Vec<SensitiveDataDetection>> {
        let mut detections = Vec::new();
        
        // Detect SSNs
        for mat in self.ssn_regex.find_iter(content) {
            let original_value = mat.as_str().to_string();
            let redacted_value = self.redact_ssn(&original_value);
            let position = (mat.start(), mat.end());
            let context = self.get_context(content, position, 30);
            
            detections.push(SensitiveDataDetection {
                data_type: SensitiveDataType::SSN,
                original_value,
                redacted_value,
                confidence: 0.9,
                context,
                position,
            });
        }
        
        // Detect credit cards
        for mat in self.credit_card_regex.find_iter(content) {
            let original_value = mat.as_str().to_string();
            let redacted_value = self.redact_credit_card(&original_value);
            let position = (mat.start(), mat.end());
            let context = self.get_context(content, position, 30);
            
            detections.push(SensitiveDataDetection {
                data_type: SensitiveDataType::CreditCard,
                original_value,
                redacted_value,
                confidence: 0.8,
                context,
                position,
            });
        }
        
        Ok(detections)
    }
    
    // Helper methods for validation and processing
    async fn validate_email(&self, email: &str) -> Result<EmailValidation> {
        // Basic email validation
        let is_valid = email.contains('@') && email.contains('.');
        let domain = email.split('@').nth(1).unwrap_or("");
        let domain_exists = !domain.is_empty(); // Simplified
        let is_disposable = self.check_disposable_domain(domain);
        
        let risk_level = if is_disposable {
            RiskLevel::High
        } else if is_valid {
            RiskLevel::Low
        } else {
            RiskLevel::Medium
        };
        
        Ok(EmailValidation {
            is_valid,
            domain_exists,
            is_disposable,
            risk_level,
        })
    }
    
    async fn validate_phone(&self, phone: &str) -> Result<PhoneValidation> {
        // Basic phone validation
        let digits_only: String = phone.chars().filter(|c| c.is_numeric()).collect();
        let is_valid = digits_only.len() >= 10;
        
        Ok(PhoneValidation {
            is_valid,
            is_mobile: None, // Would need phone number validation service
            carrier: None,
            region: Some("US".to_string()),
            risk_level: if is_valid { RiskLevel::Low } else { RiskLevel::Medium },
        })
    }
    
    async fn assess_link_security(&self, url: &str) -> Result<SecurityAssessment> {
        let mut risk_factors = Vec::new();
        let mut reputation_score = 1.0;
        
        // Check for HTTPS
        if !url.starts_with("https://") {
            risk_factors.push("Not using HTTPS".to_string());
            reputation_score -= 0.3;
        }
        
        // Check for suspicious domains
        if url.contains("bit.ly") || url.contains("tinyurl") {
            risk_factors.push("URL shortener detected".to_string());
            reputation_score -= 0.2;
        }
        
        let threat_level = if reputation_score >= 0.8 {
            RiskLevel::Low
        } else if reputation_score >= 0.5 {
            RiskLevel::Medium
        } else {
            RiskLevel::High
        };
        
        Ok(SecurityAssessment {
            is_safe: reputation_score >= 0.5,
            risk_factors,
            reputation_score,
            threat_level,
        })
    }
    
    // Utility methods
    fn get_context(&self, content: &str, position: (usize, usize), window: usize) -> String {
        let start = position.0.saturating_sub(window);
        let end = (position.1 + window).min(content.len());
        content[start..end].to_string()
    }
    
    fn format_phone(&self, phone: &str) -> String {
        let digits: String = phone.chars().filter(|c| c.is_numeric()).collect();
        if digits.len() == 10 {
            format!("({}) {}-{}", &digits[0..3], &digits[3..6], &digits[6..10])
        } else {
            phone.to_string()
        }
    }
    
    fn redact_ssn(&self, ssn: &str) -> String {
        "XXX-XX-XXXX".to_string()
    }
    
    fn redact_credit_card(&self, cc: &str) -> String {
        "**** **** **** XXXX".to_string()
    }
    
    fn check_disposable_domain(&self, domain: &str) -> bool {
        // Simple check - in production, use a comprehensive list
        ["10minutemail.com", "tempmail.org", "guerrillamail.com"]
            .contains(&domain)
    }
    
    fn classify_link_type(&self, url: &str) -> LinkType {
        if url.contains("mailto:") {
            LinkType::Email
        } else if url.contains("tel:") {
            LinkType::Phone
        } else if url.contains("facebook.com") || url.contains("twitter.com") || url.contains("linkedin.com") {
            LinkType::Social
        } else {
            LinkType::External
        }
    }
    
    // Confidence calculation methods
    fn calculate_email_confidence(&self, email: &str, context: &str, validation: &EmailValidation) -> f64 {
        let mut confidence = 0.5;
        
        if validation.is_valid { confidence += 0.3; }
        if validation.domain_exists { confidence += 0.2; }
        if !validation.is_disposable { confidence += 0.1; }
        
        // Context analysis
        if context.to_lowercase().contains("email") || context.to_lowercase().contains("contact") {
            confidence += 0.1;
        }
        
        confidence.min(1.0)
    }
    
    fn calculate_phone_confidence(&self, phone: &str, context: &str, validation: &PhoneValidation) -> f64 {
        let mut confidence = 0.5;
        
        if validation.is_valid { confidence += 0.4; }
        
        // Context analysis
        if context.to_lowercase().contains("phone") || context.to_lowercase().contains("call") {
            confidence += 0.1;
        }
        
        confidence.min(1.0)
    }
    
    fn calculate_link_confidence(&self, url: &str, security: &SecurityAssessment) -> f64 {
        let mut confidence = 0.6;
        
        if security.is_safe { confidence += 0.3; }
        if security.reputation_score > 0.8 { confidence += 0.1; }
        
        confidence.min(1.0)
    }
    
    fn calculate_confidence_scores(&self, data: &ExtractionResults, intelligence: &ContentIntelligence) -> HashMap<String, f64> {
        let mut scores = HashMap::new();
        
        scores.insert("emails".to_string(), 
            data.emails.iter().map(|e| e.confidence).sum::<f64>() / data.emails.len().max(1) as f64);
        scores.insert("phones".to_string(), 
            data.phones.iter().map(|p| p.confidence).sum::<f64>() / data.phones.len().max(1) as f64);
        scores.insert("links".to_string(), 
            data.links.iter().map(|l| l.confidence).sum::<f64>() / data.links.len().max(1) as f64);
        scores.insert("overall_quality".to_string(), intelligence.quality_score);
        
        scores
    }
    
    async fn validate_extractions(&self, data: &ExtractionResults) -> Result<ValidationStatus> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut quality_score = 1.0;
        
        // Validate email extractions
        for email in &data.emails {
            if !email.validation_result.is_valid {
                errors.push(ValidationError {
                    field: "email".to_string(),
                    error_type: "invalid_format".to_string(),
                    message: format!("Invalid email format: {}", email.email),
                    severity: ErrorSeverity::Medium,
                });
                quality_score -= 0.1;
            }
            
            if email.validation_result.is_disposable {
                warnings.push(format!("Disposable email detected: {}", email.email));
            }
        }
        
        // Validate phone extractions
        for phone in &data.phones {
            if !phone.validation_result.is_valid {
                errors.push(ValidationError {
                    field: "phone".to_string(),
                    error_type: "invalid_format".to_string(),
                    message: format!("Invalid phone format: {}", phone.phone),
                    severity: ErrorSeverity::Medium,
                });
                quality_score -= 0.1;
            }
        }
        
        Ok(ValidationStatus {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            quality_score: quality_score.max(0.0),
        })
    }
    
    fn generate_recommendations(&self, data: &ExtractionResults, intelligence: &ContentIntelligence, validation: &ValidationStatus) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if !validation.is_valid {
            recommendations.push("Review and correct validation errors".to_string());
        }
        
        if intelligence.quality_score < 0.7 {
            recommendations.push("Content quality is below threshold - consider additional processing".to_string());
        }
        
        if data.emails.is_empty() && data.phones.is_empty() {
            recommendations.push("No contact information found - verify content source".to_string());
        }
        
        if !data.sensitive_data.is_empty() {
            recommendations.push("Sensitive data detected - ensure proper handling and compliance".to_string());
        }
        
        recommendations
    }
} 