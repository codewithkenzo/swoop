/*!
 * Intelligent Data Processing Module
 * 
 * Advanced content analysis, quality assessment, and preventive intelligence
 * for production-ready data processing workflows.
 */

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Intelligence processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceConfig {
    /// Enable content quality analysis
    pub enable_quality_analysis: bool,
    /// Enable duplicate detection
    pub enable_deduplication: bool,
    /// Enable content classification
    pub enable_classification: bool,
    /// Minimum quality score threshold (0.0-1.0)
    pub min_quality_score: f64,
    /// Maximum content age in days
    pub max_content_age_days: u32,
    /// Enable language detection
    pub enable_language_detection: bool,
    /// Supported languages
    pub supported_languages: Vec<String>,
}

impl Default for IntelligenceConfig {
    fn default() -> Self {
        Self {
            enable_quality_analysis: true,
            enable_deduplication: true,
            enable_classification: true,
            min_quality_score: 0.7,
            max_content_age_days: 30,
            enable_language_detection: true,
            supported_languages: vec![
                "en".to_string(), "es".to_string(), "fr".to_string(),
                "de".to_string(), "it".to_string(), "pt".to_string(),
                "ru".to_string(), "zh".to_string(), "ja".to_string(),
                "ko".to_string(), "ar".to_string()
            ],
        }
    }
}

/// Content intelligence metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentIntelligence {
    /// Quality score (0.0-1.0)
    pub quality_score: f64,
    /// Content classification
    pub classification: ContentClassification,
    /// Language detection results
    pub language_info: LanguageInfo,
    /// Duplicate detection results
    pub duplicate_info: DuplicateInfo,
    /// Content validation results
    pub validation: ValidationResults,
    /// Processing metadata
    pub metadata: ProcessingMetadata,
}

/// Content classification results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentClassification {
    /// Primary content type
    pub primary_type: ContentType,
    /// Content categories with confidence scores
    pub categories: HashMap<String, f64>,
    /// Content topics extracted
    pub topics: Vec<String>,
    /// Estimated reading time in minutes
    pub reading_time_minutes: u32,
}

/// Language detection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageInfo {
    /// Detected primary language
    pub primary_language: String,
    /// Language confidence score
    pub confidence: f64,
    /// Other detected languages
    pub other_languages: Vec<(String, f64)>,
    /// Text encoding detected
    pub encoding: String,
}

/// Duplicate detection results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateInfo {
    /// Is content duplicate
    pub is_duplicate: bool,
    /// Similarity score with existing content
    pub similarity_score: f64,
    /// Similar content references
    pub similar_content_ids: Vec<String>,
    /// Content hash for deduplication
    pub content_hash: String,
}

/// Content validation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResults {
    /// Overall validation status
    pub is_valid: bool,
    /// Validation errors
    pub errors: Vec<ValidationError>,
    /// Validation warnings
    pub warnings: Vec<String>,
    /// Content completeness score
    pub completeness_score: f64,
}

/// Processing metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingMetadata {
    /// Processing timestamp
    pub processed_at: chrono::DateTime<chrono::Utc>,
    /// Processing duration in milliseconds
    pub processing_time_ms: u64,
    /// Content size in bytes
    pub content_size_bytes: usize,
    /// Processing version
    pub processing_version: String,
    /// Quality flags
    pub quality_flags: Vec<String>,
}

/// Content type classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Article,
    BlogPost,
    NewsArticle,
    ProductPage,
    Documentation,
    ForumPost,
    SocialMediaPost,
    EmailContent,
    LegalDocument,
    TechnicalSpec,
    ContactInfo,
    Other(String),
}

/// Validation error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub error_type: String,
    pub message: String,
    pub severity: ErrorSeverity,
    pub field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Main intelligence processor
pub struct IntelligenceProcessor {
    config: IntelligenceConfig,
}

impl IntelligenceProcessor {
    /// Create new intelligence processor
    pub fn new(config: IntelligenceConfig) -> Self {
        Self { config }
    }

    /// Process content with full intelligence pipeline
    pub async fn process_content(
        &self,
        content: &str,
        existing_content_hashes: &[String],
    ) -> Result<ContentIntelligence> {
        let start_time = std::time::Instant::now();
        
        // Generate content hash for deduplication
        let content_hash = self.generate_content_hash(content);
        
        // Check for duplicates
        let is_duplicate = existing_content_hashes.contains(&content_hash);
        let similarity_score = if is_duplicate { 1.0 } else { 0.0 };
        
        // Basic quality scoring
        let quality_score = self.calculate_quality_score(content);
        
        // Language detection (simplified)
        let language_info = self.detect_language(content).await?;
        
        // Content classification
        let classification = self.classify_content(content);
        
        // Validation
        let validation = self.validate_content(content);
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        Ok(ContentIntelligence {
            quality_score,
            classification,
            language_info,
            duplicate_info: DuplicateInfo {
                is_duplicate,
                similarity_score,
                similar_content_ids: if is_duplicate { vec![content_hash.clone()] } else { vec![] },
                content_hash,
            },
            validation,
            metadata: ProcessingMetadata {
                processed_at: chrono::Utc::now(),
                processing_time_ms: processing_time,
                content_size_bytes: content.len(),
                processing_version: "1.0.0".to_string(),
                quality_flags: self.generate_quality_flags(&validation, quality_score),
            },
        })
    }

    /// Check if content meets quality threshold
    pub fn meets_quality_threshold(&self, intelligence: &ContentIntelligence) -> bool {
        intelligence.quality_score >= self.config.min_quality_score
    }

    /// Generate recommendations based on intelligence analysis
    pub fn generate_recommendations(&self, intelligence: &ContentIntelligence) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if intelligence.quality_score < 0.8 {
            recommendations.push("Consider improving content quality".to_string());
        }
        
        if intelligence.duplicate_info.is_duplicate {
            recommendations.push("Duplicate content detected - consider deduplication".to_string());
        }
        
        if !intelligence.validation.is_valid {
            recommendations.push("Content validation failed - review errors".to_string());
        }
        
        if intelligence.language_info.confidence < 0.9 {
            recommendations.push("Language detection uncertain - verify content language".to_string());
        }
        
        recommendations
    }

    // Private helper methods
    fn calculate_quality_score(&self, content: &str) -> f64 {
        let mut score = 1.0;
        
        // Basic quality metrics
        let word_count = content.split_whitespace().count();
        let char_count = content.chars().count();
        
        // Penalize very short content
        if word_count < 10 {
            score *= 0.5;
        }
        
        // Penalize content with poor character-to-word ratio
        if char_count > 0 && word_count > 0 {
            let avg_word_length = char_count as f64 / word_count as f64;
            if avg_word_length < 3.0 || avg_word_length > 15.0 {
                score *= 0.8;
            }
        }
        
        // Ensure score is between 0.0 and 1.0
        score.max(0.0).min(1.0)
    }

    fn generate_content_hash(&self, content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    async fn detect_language(&self, content: &str) -> Result<LanguageInfo> {
        // Simplified language detection
        let primary_language = if content.chars().any(|c| c.is_ascii()) {
            "en".to_string()
        } else {
            "unknown".to_string()
        };
        
        Ok(LanguageInfo {
            primary_language: primary_language.clone(),
            confidence: 0.9,
            other_languages: vec![],
            encoding: "UTF-8".to_string(),
        })
    }

    fn classify_content(&self, content: &str) -> ContentClassification {
        let word_count = content.split_whitespace().count();
        let reading_time = (word_count / 200).max(1) as u32; // Assume 200 words per minute
        
        // Simple classification based on content characteristics
        let primary_type = if content.contains("@") && content.contains(".com") {
            ContentType::EmailContent
        } else if content.len() > 1000 {
            ContentType::Article
        } else if content.len() > 500 {
            ContentType::BlogPost
        } else {
            ContentType::Other("short_content".to_string())
        };
        
        let mut categories = HashMap::new();
        categories.insert("general".to_string(), 0.8);
        
        ContentClassification {
            primary_type,
            categories,
            topics: vec!["general".to_string()],
            reading_time_minutes: reading_time,
        }
    }

    fn validate_content(&self, content: &str) -> ValidationResults {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        // Basic validation rules
        if content.trim().is_empty() {
            errors.push(ValidationError {
                error_type: "empty_content".to_string(),
                message: "Content is empty".to_string(),
                severity: ErrorSeverity::High,
                field: Some("content".to_string()),
            });
        }
        
        if content.len() < 10 {
            warnings.push("Content is very short".to_string());
        }
        
        let is_valid = errors.is_empty();
        let completeness_score = if content.len() > 100 { 1.0 } else { content.len() as f64 / 100.0 };
        
        ValidationResults {
            is_valid,
            errors,
            warnings,
            completeness_score,
        }
    }

    fn generate_quality_flags(&self, validation: &ValidationResults, quality_score: f64) -> Vec<String> {
        let mut flags = Vec::new();
        
        if quality_score < 0.5 {
            flags.push("low_quality".to_string());
        } else if quality_score > 0.9 {
            flags.push("high_quality".to_string());
        }
        
        if !validation.is_valid {
            flags.push("validation_failed".to_string());
        }
        
        if validation.completeness_score < 0.8 {
            flags.push("incomplete_content".to_string());
        }
        
        flags
    }
} 