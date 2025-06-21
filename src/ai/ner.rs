//! # Named Entity Recognition (NER) Module
//!
//! This module provides named entity extraction from documents using transformer models.
//! It uses Candle framework for running pre-trained NER models efficiently.
//!
//! ## Supported Entity Types
//!
//! - **PERSON**: People names (John Smith, Mary Johnson)
//! - **ORG**: Organizations (Microsoft, University of California)
//! - **LOC**: Locations (New York, Paris, California)
//! - **DATE**: Dates and times (2024-01-15, January 2024)
//! - **MONEY**: Monetary amounts ($100, €50, £25)
//! - **MISC**: Miscellaneous entities (products, events, etc.)
//!
//! ## Architecture
//!
//! Uses pre-trained transformer models via Candle:
//! - BERT-based NER models for high accuracy
//! - Tokenization via tokenizers crate
//! - Efficient inference with GPU acceleration when available
//!
//! ## Usage
//!
//! ```rust,ignore
//! let extractor = EntityExtractor::new().await?;
//! let (entities, confidence) = extractor.extract_entities(text).await?;
//! ```

use crate::SwoopError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Types of named entities that can be extracted
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    Person,
    Organization,
    Location,
    Date,
    Money,
    Miscellaneous,
}

impl EntityType {
    /// Get human-readable name for the entity type
    pub fn name(&self) -> &'static str {
        match self {
            Self::Person => "Person",
            Self::Organization => "Organization",
            Self::Location => "Location",
            Self::Date => "Date",
            Self::Money => "Money",
            Self::Miscellaneous => "Miscellaneous",
        }
    }

    /// Get all possible entity types
    pub fn all() -> Vec<Self> {
        vec![
            Self::Person,
            Self::Organization,
            Self::Location,
            Self::Date,
            Self::Money,
            Self::Miscellaneous,
        ]
    }
}

/// An extracted named entity with its metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedEntity {
    /// The entity text as it appears in the document
    pub text: String,
    /// The type of entity
    pub entity_type: EntityType,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
    /// Start position in the original text
    pub start_pos: usize,
    /// End position in the original text
    pub end_pos: usize,
    /// Additional metadata (e.g., normalized form, context)
    pub metadata: HashMap<String, String>,
}

impl ExtractedEntity {
    /// Create a new extracted entity
    pub fn new(
        text: String,
        entity_type: EntityType,
        confidence: f32,
        start_pos: usize,
        end_pos: usize,
    ) -> Self {
        Self {
            text,
            entity_type,
            confidence,
            start_pos,
            end_pos,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to the entity
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Named entity extractor using transformer models
pub struct EntityExtractor {
    /// Tokenizer for text preprocessing
    tokenizer: Option<TokenizerModel>,
    /// NER model for entity extraction
    ner_model: Option<NerModel>,
    /// Rule-based patterns for high-confidence extraction
    patterns: HashMap<EntityType, Vec<regex::Regex>>,
}

impl EntityExtractor {
    /// Create a new entity extractor
    pub async fn new() -> Result<Self, SwoopError> {
        let mut extractor = Self {
            tokenizer: None,
            ner_model: None,
            patterns: Self::build_regex_patterns()?,
        };

        // TODO: Load pre-trained transformer model
        // For now, we'll use rule-based extraction
        extractor.initialize_models().await?;

        Ok(extractor)
    }

    /// Initialize NER models (placeholder for actual implementation)
    async fn initialize_models(&mut self) -> Result<(), SwoopError> {
        // TODO: Implement actual model loading using Candle
        // This would involve:
        // 1. Loading pre-trained BERT-NER model
        // 2. Setting up tokenizer
        // 3. Configuring GPU acceleration if available
        // 4. Model warm-up

        tracing::info!("Entity extractor initialized (rule-based mode)");
        Ok(())
    }

    /// Extract entities from text and return with overall confidence
    pub async fn extract_entities(
        &self,
        text: &str,
    ) -> Result<(Vec<ExtractedEntity>, f32), SwoopError> {
        // First, try rule-based extraction
        let rule_based_entities = self.extract_with_patterns(text);

        // TODO: Use transformer model for more accurate extraction
        // For now, return rule-based results
        let overall_confidence = if rule_based_entities.is_empty() {
            0.0
        } else {
            rule_based_entities.iter().map(|e| e.confidence).sum::<f32>()
                / rule_based_entities.len() as f32
        };

        Ok((rule_based_entities, overall_confidence))
    }

    /// Extract entities using regex patterns
    fn extract_with_patterns(&self, text: &str) -> Vec<ExtractedEntity> {
        let mut entities = Vec::new();

        for (entity_type, patterns) in &self.patterns {
            for pattern in patterns {
                for mat in pattern.find_iter(text) {
                    let entity = ExtractedEntity::new(
                        mat.as_str().to_string(),
                        entity_type.clone(),
                        0.7, // Rule-based confidence
                        mat.start(),
                        mat.end(),
                    );
                    entities.push(entity);
                }
            }
        }

        // Remove duplicates and overlapping entities
        self.deduplicate_entities(entities)
    }

    /// Remove duplicate and overlapping entities
    fn deduplicate_entities(&self, mut entities: Vec<ExtractedEntity>) -> Vec<ExtractedEntity> {
        // Sort by start position
        entities.sort_by_key(|e| e.start_pos);

        let mut deduplicated = Vec::new();
        let mut last_end = 0;

        for entity in entities {
            // Skip overlapping entities (keep the first one)
            if entity.start_pos >= last_end {
                last_end = entity.end_pos;
                deduplicated.push(entity);
            }
        }

        deduplicated
    }

    /// Build regex patterns for entity extraction
    fn build_regex_patterns() -> Result<HashMap<EntityType, Vec<regex::Regex>>, SwoopError> {
        let mut patterns = HashMap::new();

        // Person names (simple patterns)
        patterns.insert(
            EntityType::Person,
            vec![
                regex::Regex::new(r"\b[A-Z][a-z]+ [A-Z][a-z]+\b")
                    .map_err(|e| SwoopError::Configuration(format!("Regex error: {}", e)))?,
                regex::Regex::new(r"\b(Mr|Mrs|Ms|Dr|Prof)\. [A-Z][a-z]+ [A-Z][a-z]+\b")
                    .map_err(|e| SwoopError::Configuration(format!("Regex error: {}", e)))?,
            ],
        );

        // Organizations
        patterns.insert(
            EntityType::Organization,
            vec![
                regex::Regex::new(r"\b[A-Z][a-z]+ (Inc|LLC|Corp|Ltd|Company|Corporation)\b")
                    .map_err(|e| SwoopError::Configuration(format!("Regex error: {}", e)))?,
                regex::Regex::new(r"\b(University of|College of) [A-Z][a-z]+\b")
                    .map_err(|e| SwoopError::Configuration(format!("Regex error: {}", e)))?,
            ],
        );

        // Locations
        patterns.insert(
            EntityType::Location,
            vec![
                regex::Regex::new(r"\b[A-Z][a-z]+, [A-Z]{2}\b") // City, State
                    .map_err(|e| SwoopError::Configuration(format!("Regex error: {}", e)))?,
                regex::Regex::new(r"\b(New York|Los Angeles|Chicago|Houston|Phoenix|Philadelphia|San Antonio|San Diego|Dallas|San Jose)\b")
                    .map_err(|e| SwoopError::Configuration(format!("Regex error: {}", e)))?,
            ],
        );

        // Dates
        patterns.insert(
            EntityType::Date,
            vec![
                regex::Regex::new(r"\b\d{1,2}/\d{1,2}/\d{4}\b") // MM/DD/YYYY
                    .map_err(|e| SwoopError::Configuration(format!("Regex error: {}", e)))?,
                regex::Regex::new(r"\b\d{4}-\d{2}-\d{2}\b") // YYYY-MM-DD
                    .map_err(|e| SwoopError::Configuration(format!("Regex error: {}", e)))?,
                regex::Regex::new(r"\b(January|February|March|April|May|June|July|August|September|October|November|December) \d{1,2}, \d{4}\b")
                    .map_err(|e| SwoopError::Configuration(format!("Regex error: {}", e)))?,
            ],
        );

        // Money
        patterns.insert(
            EntityType::Money,
            vec![
                regex::Regex::new(r"\$\d{1,3}(,\d{3})*(\.\d{2})?") // $1,000.00
                    .map_err(|e| SwoopError::Configuration(format!("Regex error: {}", e)))?,
                regex::Regex::new(r"€\d{1,3}(,\d{3})*(\.\d{2})?") // €1,000.00
                    .map_err(|e| SwoopError::Configuration(format!("Regex error: {}", e)))?,
                regex::Regex::new(r"£\d{1,3}(,\d{3})*(\.\d{2})?") // £1,000.00
                    .map_err(|e| SwoopError::Configuration(format!("Regex error: {}", e)))?,
            ],
        );

        Ok(patterns)
    }
}

/// Placeholder for tokenizer model (to be implemented with tokenizers crate)
struct TokenizerModel {
    // TODO: Implement using tokenizers crate
}

/// Placeholder for NER model (to be implemented with Candle)
struct NerModel {
    // TODO: Implement using candle-transformers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_entity_extractor_creation() {
        let extractor = EntityExtractor::new().await;
        assert!(extractor.is_ok());
    }

    #[tokio::test]
    async fn test_person_extraction() {
        let extractor = EntityExtractor::new().await.unwrap();
        let text = "John Smith and Mary Johnson attended the meeting.";
        
        let (entities, confidence) = extractor.extract_entities(text).await.unwrap();
        
        assert!(!entities.is_empty());
        let person_entities: Vec<_> = entities
            .iter()
            .filter(|e| e.entity_type == EntityType::Person)
            .collect();
        assert!(!person_entities.is_empty());
        assert!(confidence > 0.0);
    }

    #[tokio::test]
    async fn test_date_extraction() {
        let extractor = EntityExtractor::new().await.unwrap();
        let text = "The meeting is scheduled for January 15, 2024 at 10:00 AM.";
        
        let (entities, _) = extractor.extract_entities(text).await.unwrap();
        
        let date_entities: Vec<_> = entities
            .iter()
            .filter(|e| e.entity_type == EntityType::Date)
            .collect();
        assert!(!date_entities.is_empty());
    }

    #[tokio::test]
    async fn test_money_extraction() {
        let extractor = EntityExtractor::new().await.unwrap();
        let text = "The contract is worth $1,500,000.00 and includes a bonus of €50,000.";
        
        let (entities, _) = extractor.extract_entities(text).await.unwrap();
        
        let money_entities: Vec<_> = entities
            .iter()
            .filter(|e| e.entity_type == EntityType::Money)
            .collect();
        assert!(!money_entities.is_empty());
    }

    #[test]
    fn test_entity_type_names() {
        assert_eq!(EntityType::Person.name(), "Person");
        assert_eq!(EntityType::Organization.name(), "Organization");
        assert_eq!(EntityType::Location.name(), "Location");
    }

    #[test]
    fn test_extracted_entity_creation() {
        let entity = ExtractedEntity::new(
            "John Smith".to_string(),
            EntityType::Person,
            0.9,
            0,
            10,
        );
        
        assert_eq!(entity.text, "John Smith");
        assert_eq!(entity.entity_type, EntityType::Person);
        assert_eq!(entity.confidence, 0.9);
        assert_eq!(entity.start_pos, 0);
        assert_eq!(entity.end_pos, 10);
    }

    #[test]
    fn test_entity_with_metadata() {
        let entity = ExtractedEntity::new(
            "Microsoft".to_string(),
            EntityType::Organization,
            0.8,
            0,
            9,
        ).with_metadata("industry".to_string(), "technology".to_string());
        
        assert_eq!(entity.metadata.get("industry"), Some(&"technology".to_string()));
    }
} 