//! # Auto-Tagging Module
//!
//! This module provides intelligent auto-tagging for documents using a combination of
//! rule-based patterns, ML-based classification, and analysis results from other AI modules.
//!
//! ## Features
//!
//! - **Rule-Based Tagging**: Pattern matching for high-confidence tags
//! - **ML-Based Tagging**: Classification-based tag suggestions
//! - **Context-Aware Tagging**: Uses results from categorization and NER
//! - **Hierarchical Tags**: Support for nested tag structures
//! - **Confidence Scoring**: Each tag comes with a confidence score
//!
//! ## Tag Categories
//!
//! - **Content Type**: document-type, format, structure
//! - **Domain**: legal, academic, technical, business
//! - **Entities**: person-mentioned, organization-mentioned, location-mentioned
//! - **Topics**: extracted topics and themes
//! - **Metadata**: language, length, complexity
//!
//! ## Architecture
//!
//! Combines multiple tagging strategies:
//! - Keyword-based rules for high-precision tags
//! - ML classification for semantic tags
//! - Entity-based tags from NER results
//! - Category-based tags from document classification
//!
//! ## Usage
//!
//! ```rust,ignore
//! let tagger = AutoTagger::new();
//! let config = TaggingConfig::default();
//! let (tags, confidence) = tagger.generate_tags(content, &analysis_results).await?;
//! ```

use crate::ai::{AnalysisResults, DocumentCategory};
use crate::ai::ner::{EntityType, ExtractedEntity};
use crate::SwoopError;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Configuration for auto-tagging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaggingConfig {
    /// Enable rule-based tagging
    pub rule_based: bool,
    /// Enable ML-based tagging
    pub ml_based: bool,
    /// Enable entity-based tagging
    pub entity_based: bool,
    /// Enable category-based tagging
    pub category_based: bool,
    /// Minimum confidence threshold for tags
    pub min_confidence: f32,
    /// Maximum number of tags to generate
    pub max_tags: usize,
    /// Use hierarchical tag structure
    pub hierarchical: bool,
}

impl Default for TaggingConfig {
    fn default() -> Self {
        Self {
            rule_based: true,
            ml_based: true,
            entity_based: true,
            category_based: true,
            min_confidence: 0.3,
            max_tags: 20,
            hierarchical: true,
        }
    }
}

/// A generated tag with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedTag {
    /// The tag name
    pub name: String,
    /// Tag category/namespace
    pub category: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
    /// Source of the tag (rule, ml, entity, category)
    pub source: TagSource,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Source of a generated tag
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TagSource {
    Rule,
    MachineLearning,
    Entity,
    Category,
    Hybrid,
}

impl GeneratedTag {
    /// Create a new generated tag
    pub fn new(name: String, category: String, confidence: f32, source: TagSource) -> Self {
        Self {
            name,
            category,
            confidence,
            source,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to the tag
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Get the full hierarchical tag name
    pub fn full_name(&self) -> String {
        format!("{}:{}", self.category, self.name)
    }
}

/// Auto-tagger for intelligent document tagging
pub struct AutoTagger {
    /// Configuration for tagging
    config: TaggingConfig,
    /// Rule-based patterns for tagging
    rule_patterns: HashMap<String, Vec<TagRule>>,
    /// ML model for semantic tagging (placeholder)
    ml_model: Option<TaggingModel>,
    /// Tag taxonomy for hierarchical tagging
    tag_taxonomy: TagTaxonomy,
}

/// A rule for pattern-based tagging
#[derive(Debug, Clone)]
struct TagRule {
    pattern: regex::Regex,
    tag_name: String,
    tag_category: String,
    confidence: f32,
}

impl AutoTagger {
    /// Create a new auto-tagger with default configuration
    pub fn new() -> Self {
        Self::with_config(TaggingConfig::default())
    }

    /// Create a new auto-tagger with custom configuration
    pub fn with_config(config: TaggingConfig) -> Self {
        Self {
            rule_patterns: Self::build_rule_patterns(),
            ml_model: None, // TODO: Load ML model
            tag_taxonomy: TagTaxonomy::default(),
            config,
        }
    }

    /// Generate tags for a document based on content and analysis results
    pub async fn generate_tags(
        &self,
        content: &str,
        analysis_results: &AnalysisResults,
    ) -> Result<(Vec<String>, f32), SwoopError> {
        let mut all_tags = Vec::new();

        // Rule-based tagging
        if self.config.rule_based {
            let rule_tags = self.generate_rule_based_tags(content)?;
            all_tags.extend(rule_tags);
        }

        // Entity-based tagging
        if self.config.entity_based {
            let entity_tags = self.generate_entity_based_tags(&analysis_results.entities)?;
            all_tags.extend(entity_tags);
        }

        // Category-based tagging
        if self.config.category_based {
            if let Some(category) = &analysis_results.category {
                let category_tags = self.generate_category_based_tags(category)?;
                all_tags.extend(category_tags);
            }
        }

        // Language-based tagging
        if let Some(language) = &analysis_results.language {
            let lang_tag = GeneratedTag::new(
                language.clone(),
                "language".to_string(),
                0.9,
                TagSource::Category,
            );
            all_tags.push(lang_tag);
        }

        // Content-based metadata tags
        let metadata_tags = self.generate_metadata_tags(content)?;
        all_tags.extend(metadata_tags);

        // ML-based tagging (placeholder)
        if self.config.ml_based {
            let ml_tags = self.generate_ml_based_tags(content).await?;
            all_tags.extend(ml_tags);
        }

        // Filter by confidence and deduplicate
        let filtered_tags = self.filter_and_deduplicate_tags(all_tags)?;

        // Convert to string format and calculate overall confidence
        let tag_names: Vec<String> = filtered_tags
            .iter()
            .map(|tag| {
                if self.config.hierarchical {
                    tag.full_name()
                } else {
                    tag.name.clone()
                }
            })
            .collect();

        let overall_confidence = if filtered_tags.is_empty() {
            0.0
        } else {
            filtered_tags.iter().map(|t| t.confidence).sum::<f32>() / filtered_tags.len() as f32
        };

        Ok((tag_names, overall_confidence))
    }

    /// Generate rule-based tags using pattern matching
    fn generate_rule_based_tags(&self, content: &str) -> Result<Vec<GeneratedTag>, SwoopError> {
        let mut tags = Vec::new();
        let content_lower = content.to_lowercase();

        for (category, rules) in &self.rule_patterns {
            for rule in rules {
                if rule.pattern.is_match(&content_lower) {
                    let tag = GeneratedTag::new(
                        rule.tag_name.clone(),
                        rule.tag_category.clone(),
                        rule.confidence,
                        TagSource::Rule,
                    );
                    tags.push(tag);
                }
            }
        }

        Ok(tags)
    }

    /// Generate tags based on extracted entities
    fn generate_entity_based_tags(
        &self,
        entities: &[ExtractedEntity],
    ) -> Result<Vec<GeneratedTag>, SwoopError> {
        let mut tags = Vec::new();
        let mut entity_counts: HashMap<EntityType, usize> = HashMap::new();

        // Count entities by type
        for entity in entities {
            *entity_counts.entry(entity.entity_type.clone()).or_insert(0) += 1;
        }

        // Generate tags based on entity presence and frequency
        for (entity_type, count) in entity_counts {
            let tag_name = match entity_type {
                EntityType::Person => "people-mentioned",
                EntityType::Organization => "organizations-mentioned",
                EntityType::Location => "locations-mentioned",
                EntityType::Date => "dates-mentioned",
                EntityType::Money => "financial-amounts",
                EntityType::Miscellaneous => "entities-mentioned",
            };

            let confidence = (count as f32 / 10.0).min(0.9); // Scale confidence by frequency
            let tag = GeneratedTag::new(
                tag_name.to_string(),
                "entities".to_string(),
                confidence,
                TagSource::Entity,
            ).with_metadata("count".to_string(), count.to_string());

            tags.push(tag);
        }

        Ok(tags)
    }

    /// Generate tags based on document category
    fn generate_category_based_tags(
        &self,
        category: &DocumentCategory,
    ) -> Result<Vec<GeneratedTag>, SwoopError> {
        let mut tags = Vec::new();

        // Primary category tag
        let category_name = match category {
            DocumentCategory::Legal => "legal-document",
            DocumentCategory::Academic => "academic-paper",
            DocumentCategory::Technical => "technical-documentation",
            DocumentCategory::Business => "business-document",
            DocumentCategory::News => "news-article",
            DocumentCategory::Personal => "personal-document",
            DocumentCategory::Unknown => "unclassified",
        };

        let primary_tag = GeneratedTag::new(
            category_name.to_string(),
            "document-type".to_string(),
            0.8,
            TagSource::Category,
        );
        tags.push(primary_tag);

        // Domain-specific tags
        let domain_tag = match category {
            DocumentCategory::Legal => Some("legal"),
            DocumentCategory::Academic => Some("academic"),
            DocumentCategory::Technical => Some("technical"),
            DocumentCategory::Business => Some("business"),
            DocumentCategory::News => Some("media"),
            DocumentCategory::Personal => Some("personal"),
            DocumentCategory::Unknown => None,
        };

        if let Some(domain) = domain_tag {
            let tag = GeneratedTag::new(
                domain.to_string(),
                "domain".to_string(),
                0.7,
                TagSource::Category,
            );
            tags.push(tag);
        }

        Ok(tags)
    }

    /// Generate metadata tags based on content characteristics
    fn generate_metadata_tags(&self, content: &str) -> Result<Vec<GeneratedTag>, SwoopError> {
        let mut tags = Vec::new();
        let word_count = content.split_whitespace().count();
        let char_count = content.chars().count();

        // Length-based tags
        let length_tag = if word_count < 100 {
            "short"
        } else if word_count < 1000 {
            "medium"
        } else if word_count < 5000 {
            "long"
        } else {
            "very-long"
        };

        tags.push(GeneratedTag::new(
            length_tag.to_string(),
            "length".to_string(),
            0.9,
            TagSource::Rule,
        ).with_metadata("word_count".to_string(), word_count.to_string()));

        // Complexity-based tags (simple heuristic)
        let avg_word_length = char_count as f32 / word_count as f32;
        let complexity = if avg_word_length < 4.0 {
            "simple"
        } else if avg_word_length < 6.0 {
            "moderate"
        } else {
            "complex"
        };

        tags.push(GeneratedTag::new(
            complexity.to_string(),
            "complexity".to_string(),
            0.6,
            TagSource::Rule,
        ).with_metadata("avg_word_length".to_string(), avg_word_length.to_string()));

        Ok(tags)
    }

    /// Generate ML-based tags (placeholder implementation)
    async fn generate_ml_based_tags(&self, content: &str) -> Result<Vec<GeneratedTag>, SwoopError> {
        // TODO: Implement actual ML-based tagging
        // This would involve:
        // 1. Topic modeling (LDA, BERT-based)
        // 2. Semantic similarity to known topics
        // 3. Classification-based tag prediction

        let mut tags = Vec::new();

        // Simple keyword-based topic detection as placeholder
        let topics = vec![
            ("machine-learning", vec!["machine learning", "ai", "neural", "model"]),
            ("programming", vec!["code", "function", "algorithm", "software"]),
            ("finance", vec!["money", "investment", "financial", "budget"]),
            ("research", vec!["study", "analysis", "methodology", "hypothesis"]),
        ];

        let content_lower = content.to_lowercase();
        for (topic, keywords) in topics {
            let matches = keywords.iter().filter(|&kw| content_lower.contains(kw)).count();
            if matches > 0 {
                let confidence = (matches as f32 / keywords.len() as f32).min(0.8);
                let tag = GeneratedTag::new(
                    topic.to_string(),
                    "topic".to_string(),
                    confidence,
                    TagSource::MachineLearning,
                ).with_metadata("keyword_matches".to_string(), matches.to_string());
                tags.push(tag);
            }
        }

        Ok(tags)
    }

    /// Filter tags by confidence and remove duplicates
    fn filter_and_deduplicate_tags(
        &self,
        mut tags: Vec<GeneratedTag>,
    ) -> Result<Vec<GeneratedTag>, SwoopError> {
        // Filter by minimum confidence
        tags.retain(|tag| tag.confidence >= self.config.min_confidence);

        // Sort by confidence (descending)
        tags.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

        // Remove duplicates (keep highest confidence)
        let mut seen = HashSet::new();
        let mut deduplicated = Vec::new();

        for tag in tags {
            let key = tag.full_name();
            if !seen.contains(&key) {
                seen.insert(key);
                deduplicated.push(tag);
            }
        }

        // Limit to max_tags
        deduplicated.truncate(self.config.max_tags);

        Ok(deduplicated)
    }

    /// Build rule patterns for tag generation
    fn build_rule_patterns() -> HashMap<String, Vec<TagRule>> {
        let mut patterns = HashMap::new();

        // Format-based rules
        let mut format_rules = Vec::new();
        format_rules.push(TagRule {
            pattern: regex::Regex::new(r"\.pdf$").unwrap(),
            tag_name: "pdf".to_string(),
            tag_category: "format".to_string(),
            confidence: 0.95,
        });

        // Content structure rules
        let mut structure_rules = Vec::new();
        structure_rules.push(TagRule {
            pattern: regex::Regex::new(r"table of contents|toc").unwrap(),
            tag_name: "structured".to_string(),
            tag_category: "structure".to_string(),
            confidence: 0.8,
        });

        structure_rules.push(TagRule {
            pattern: regex::Regex::new(r"bibliography|references").unwrap(),
            tag_name: "referenced".to_string(),
            tag_category: "structure".to_string(),
            confidence: 0.9,
        });

        // Technical content rules
        let mut technical_rules = Vec::new();
        technical_rules.push(TagRule {
            pattern: regex::Regex::new(r"api|endpoint|json|xml").unwrap(),
            tag_name: "api-documentation".to_string(),
            tag_category: "technical".to_string(),
            confidence: 0.8,
        });

        patterns.insert("format".to_string(), format_rules);
        patterns.insert("structure".to_string(), structure_rules);
        patterns.insert("technical".to_string(), technical_rules);

        patterns
    }
}

/// Placeholder for ML tagging model
struct TaggingModel {
    // TODO: Implement using linfa or Candle
}

/// Tag taxonomy for hierarchical tagging
#[derive(Debug, Clone)]
struct TagTaxonomy {
    categories: HashMap<String, Vec<String>>,
}

impl Default for TagTaxonomy {
    fn default() -> Self {
        let mut categories = HashMap::new();
        
        categories.insert("document-type".to_string(), vec![
            "legal-document".to_string(),
            "academic-paper".to_string(),
            "technical-documentation".to_string(),
            "business-document".to_string(),
        ]);

        categories.insert("domain".to_string(), vec![
            "legal".to_string(),
            "academic".to_string(),
            "technical".to_string(),
            "business".to_string(),
        ]);

        Self { categories }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::ner::{EntityType, ExtractedEntity};

    #[tokio::test]
    async fn test_auto_tagger_creation() {
        let tagger = AutoTagger::new();
        assert_eq!(tagger.config.min_confidence, 0.3);
        assert_eq!(tagger.config.max_tags, 20);
    }

    #[tokio::test]
    async fn test_rule_based_tagging() {
        let tagger = AutoTagger::new();
        let content = "This document contains a table of contents and references section.";
        
        let rule_tags = tagger.generate_rule_based_tags(content).unwrap();
        
        assert!(!rule_tags.is_empty());
        let structured_tags: Vec<_> = rule_tags
            .iter()
            .filter(|t| t.name == "structured" || t.name == "referenced")
            .collect();
        assert!(!structured_tags.is_empty());
    }

    #[tokio::test]
    async fn test_entity_based_tagging() {
        let tagger = AutoTagger::new();
        let entities = vec![
            ExtractedEntity::new(
                "John Smith".to_string(),
                EntityType::Person,
                0.9,
                0,
                10,
            ),
            ExtractedEntity::new(
                "Microsoft".to_string(),
                EntityType::Organization,
                0.8,
                15,
                24,
            ),
        ];
        
        let entity_tags = tagger.generate_entity_based_tags(&entities).unwrap();
        
        assert_eq!(entity_tags.len(), 2);
        assert!(entity_tags.iter().any(|t| t.name == "people-mentioned"));
        assert!(entity_tags.iter().any(|t| t.name == "organizations-mentioned"));
    }

    #[tokio::test]
    async fn test_category_based_tagging() {
        let tagger = AutoTagger::new();
        let category = DocumentCategory::Legal;
        
        let category_tags = tagger.generate_category_based_tags(&category).unwrap();
        
        assert!(!category_tags.is_empty());
        assert!(category_tags.iter().any(|t| t.name == "legal-document"));
        assert!(category_tags.iter().any(|t| t.name == "legal"));
    }

    #[tokio::test]
    async fn test_metadata_tagging() {
        let tagger = AutoTagger::new();
        let short_content = "Short document.";
        let long_content = "This is a much longer document with many words. ".repeat(50);
        
        let short_tags = tagger.generate_metadata_tags(short_content).unwrap();
        let long_tags = tagger.generate_metadata_tags(&long_content).unwrap();
        
        assert!(short_tags.iter().any(|t| t.name == "short"));
        assert!(long_tags.iter().any(|t| t.name == "long" || t.name == "very-long"));
    }

    #[test]
    fn test_generated_tag_creation() {
        let tag = GeneratedTag::new(
            "test-tag".to_string(),
            "test-category".to_string(),
            0.8,
            TagSource::Rule,
        );
        
        assert_eq!(tag.name, "test-tag");
        assert_eq!(tag.category, "test-category");
        assert_eq!(tag.confidence, 0.8);
        assert_eq!(tag.source, TagSource::Rule);
        assert_eq!(tag.full_name(), "test-category:test-tag");
    }

    #[test]
    fn test_tagging_config_default() {
        let config = TaggingConfig::default();
        assert!(config.rule_based);
        assert!(config.ml_based);
        assert!(config.entity_based);
        assert!(config.category_based);
        assert_eq!(config.min_confidence, 0.3);
        assert_eq!(config.max_tags, 20);
    }
} 