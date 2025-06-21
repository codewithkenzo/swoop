//! # AI Module - Document Intelligence Core
//!
//! This module provides the core AI/ML functionality for Swoop's document intelligence platform.
//! All AI features are gated behind the `ai` feature flag for optional compilation and deployment.
//!
//! ## Features
//!
//! - **Document Categorization**: Classify documents by type (legal, academic, technical, etc.)
//! - **Named Entity Recognition**: Extract entities (people, organizations, locations, dates)
//! - **Vector Embeddings**: Generate semantic embeddings for similarity search
//! - **Auto-Tagging**: Intelligent tagging based on content analysis
//!
//! ## Architecture
//!
//! Each AI feature is implemented in its own module with <200-300 lines of code:
//! - `categorization`: Document type classification using linfa
//! - `ner`: Named entity recognition using Candle transformers
//! - `embeddings`: Vector embeddings using Candle models
//! - `tagging`: Auto-tagging logic combining rules and ML
//!
//! ## Usage
//!
//! ```rust,ignore
//! use swoop::ai::{DocumentAnalyzer, AnalysisConfig};
//!
//! let analyzer = DocumentAnalyzer::new().await?;
//! let config = AnalysisConfig::default()
//!     .with_categorization(true)
//!     .with_entity_extraction(true)
//!     .with_auto_tagging(true);
//!
//! let results = analyzer.analyze_document(content, config).await?;
//! ```

#[cfg(feature = "ai")]
pub mod categorization;
#[cfg(feature = "ai")]
pub mod embeddings;
#[cfg(feature = "ai")]
pub mod ner;
#[cfg(feature = "ai")]
pub mod tagging;

// Re-export main types when AI feature is enabled
#[cfg(feature = "ai")]
pub use categorization::{DocumentCategory, DocumentCategorizer};
#[cfg(feature = "ai")]
pub use embeddings::{DocumentEmbedder, EmbeddingModel};
#[cfg(feature = "ai")]
pub use ner::{EntityExtractor, ExtractedEntity};
#[cfg(feature = "ai")]
pub use tagging::{AutoTagger, TaggingConfig};

use crate::SwoopError;
use serde::{Deserialize, Serialize};

/// Configuration for document analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    /// Enable document categorization
    pub categorization: bool,
    /// Enable named entity recognition
    pub entity_extraction: bool,
    /// Enable vector embeddings generation
    pub embeddings: bool,
    /// Enable auto-tagging
    pub auto_tagging: bool,
    /// Language detection (always enabled via whatlang)
    pub language_detection: bool,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            categorization: true,
            entity_extraction: true,
            embeddings: true,
            auto_tagging: true,
            language_detection: true,
        }
    }
}

/// Results of document analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResults {
    /// Detected document category
    pub category: Option<DocumentCategory>,
    /// Extracted entities
    pub entities: Vec<ExtractedEntity>,
    /// Generated embeddings (as base64 encoded vector)
    pub embeddings: Option<String>,
    /// Auto-generated tags
    pub tags: Vec<String>,
    /// Detected language
    pub language: Option<String>,
    /// Confidence scores for each analysis
    pub confidence: AnalysisConfidence,
}

/// Confidence scores for different analysis components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfidence {
    pub categorization: Option<f32>,
    pub entity_extraction: Option<f32>,
    pub embeddings: Option<f32>,
    pub auto_tagging: Option<f32>,
    pub language_detection: Option<f32>,
}

/// Main document analyzer that orchestrates all AI features
#[cfg(feature = "ai")]
pub struct DocumentAnalyzer {
    categorizer: DocumentCategorizer,
    entity_extractor: EntityExtractor,
    embedder: DocumentEmbedder,
    auto_tagger: AutoTagger,
}

#[cfg(feature = "ai")]
impl DocumentAnalyzer {
    /// Create a new document analyzer with default models
    pub async fn new() -> Result<Self, SwoopError> {
        Ok(Self {
            categorizer: DocumentCategorizer::new().await?,
            entity_extractor: EntityExtractor::new().await?,
            embedder: DocumentEmbedder::new().await?,
            auto_tagger: AutoTagger::new(),
        })
    }

    /// Analyze a document with the given configuration
    pub async fn analyze_document(
        &self,
        content: &str,
        config: AnalysisConfig,
    ) -> Result<AnalysisResults, SwoopError> {
        let mut results = AnalysisResults {
            category: None,
            entities: Vec::new(),
            embeddings: None,
            tags: Vec::new(),
            language: None,
            confidence: AnalysisConfidence {
                categorization: None,
                entity_extraction: None,
                embeddings: None,
                auto_tagging: None,
                language_detection: None,
            },
        };

        // Language detection (always enabled)
        if config.language_detection {
            results.language = detect_language(content);
            results.confidence.language_detection = Some(0.9); // whatlang is quite reliable
        }

        // Document categorization
        if config.categorization {
            let (category, confidence) = self.categorizer.categorize(content).await?;
            results.category = Some(category);
            results.confidence.categorization = Some(confidence);
        }

        // Named entity recognition
        if config.entity_extraction {
            let (entities, confidence) = self.entity_extractor.extract_entities(content).await?;
            results.entities = entities;
            results.confidence.entity_extraction = Some(confidence);
        }

        // Vector embeddings
        if config.embeddings {
            let (embeddings, confidence) = self.embedder.generate_embeddings(content).await?;
            results.embeddings = Some(embeddings);
            results.confidence.embeddings = Some(confidence);
        }

        // Auto-tagging
        if config.auto_tagging {
            let (tags, confidence) = self.auto_tagger.generate_tags(content, &results).await?;
            results.tags = tags;
            results.confidence.auto_tagging = Some(confidence);
        }

        Ok(results)
    }
}

/// Detect language using whatlang (always available, not gated by AI feature)
pub fn detect_language(text: &str) -> Option<String> {
    #[cfg(feature = "ai")]
    {
        use whatlang::detect;
        detect(text).map(|info| info.lang().to_string())
    }
    #[cfg(not(feature = "ai"))]
    {
        // Fallback when AI features are disabled
        let _ = text;
        None
    }
}

/// Fallback implementations when AI feature is disabled
#[cfg(not(feature = "ai"))]
impl AnalysisResults {
    /// Create empty analysis results when AI is disabled
    pub fn empty() -> Self {
        Self {
            category: None,
            entities: Vec::new(),
            embeddings: None,
            tags: Vec::new(),
            language: None,
            confidence: AnalysisConfidence {
                categorization: None,
                entity_extraction: None,
                embeddings: None,
                auto_tagging: None,
                language_detection: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analysis_config_default() {
        let config = AnalysisConfig::default();
        assert!(config.categorization);
        assert!(config.entity_extraction);
        assert!(config.embeddings);
        assert!(config.auto_tagging);
        assert!(config.language_detection);
    }

    #[test]
    fn test_language_detection_fallback() {
        let result = detect_language("Hello world");
        // Should work if AI feature is enabled, return None if disabled
        #[cfg(feature = "ai")]
        assert!(result.is_some());
        #[cfg(not(feature = "ai"))]
        assert!(result.is_none());
    }
} 