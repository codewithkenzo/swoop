//! # AI Integration Test
//!
//! This module provides integration tests to verify that all AI components
//! work together correctly and can analyze documents end-to-end.

#[cfg(test)]
mod tests {
    use crate::ai::{
        DocumentAnalyzer, AnalysisConfig, AnalysisResults,
        DocumentCategory, EntityType,
        detect_language,
    };

    #[tokio::test]
    async fn test_full_document_analysis() {
        // Test document with mixed content
        let test_content = r#"
            Legal Contract Agreement
            
            This agreement between Microsoft Corporation and John Smith,
            dated January 15, 2024, establishes the terms for software licensing.
            
            The total contract value is $1,500,000.00 and will be paid
            in quarterly installments to the contractor located in New York, NY.
            
            Whereas the plaintiff seeks damages, this contract shall be governed
            by the laws of California.
        "#;

        // Create analyzer
        let analyzer = DocumentAnalyzer::new().await.expect("Failed to create analyzer");
        
        // Configure analysis
        let config = AnalysisConfig {
            categorization: true,
            entity_extraction: true,
            embeddings: true,
            auto_tagging: true,
            language_detection: true,
        };

        // Perform analysis
        let results = analyzer.analyze_document(test_content, config).await
            .expect("Failed to analyze document");

        // Verify categorization
        assert!(results.category.is_some());
        assert_eq!(results.category.unwrap(), DocumentCategory::Legal);

        // Verify entity extraction
        assert!(!results.entities.is_empty());
        
        // Check for expected entities
        let person_entities: Vec<_> = results.entities.iter()
            .filter(|e| e.entity_type == EntityType::Person)
            .collect();
        assert!(!person_entities.is_empty());

        let org_entities: Vec<_> = results.entities.iter()
            .filter(|e| e.entity_type == EntityType::Organization)
            .collect();
        assert!(!org_entities.is_empty());

        let money_entities: Vec<_> = results.entities.iter()
            .filter(|e| e.entity_type == EntityType::Money)
            .collect();
        assert!(!money_entities.is_empty());

        // Verify embeddings
        assert!(results.embeddings.is_some());

        // Verify auto-tagging
        assert!(!results.tags.is_empty());
        
        // Check for expected tags
        assert!(results.tags.iter().any(|tag| tag.contains("legal")));
        assert!(results.tags.iter().any(|tag| tag.contains("contract") || tag.contains("agreement")));

        // Verify language detection
        assert!(results.language.is_some());
        
        // Verify confidence scores
        assert!(results.confidence.categorization.unwrap() > 0.0);
        assert!(results.confidence.entity_extraction.unwrap() > 0.0);
        assert!(results.confidence.embeddings.unwrap() > 0.0);
        assert!(results.confidence.auto_tagging.unwrap() > 0.0);
        assert!(results.confidence.language_detection.unwrap() > 0.0);

        println!("✅ Full document analysis test passed!");
        println!("📊 Analysis Results:");
        println!("   Category: {:?}", results.category);
        println!("   Entities: {} found", results.entities.len());
        println!("   Tags: {:?}", results.tags);
        println!("   Language: {:?}", results.language);
    }

    #[tokio::test]
    async fn test_academic_document_analysis() {
        let academic_content = r#"
            Abstract
            
            This research study analyzes the effectiveness of machine learning
            algorithms in natural language processing tasks. The methodology
            employed by Smith et al. (2023) was adapted for this investigation.
            
            Our hypothesis was that transformer-based models would outperform
            traditional approaches. The analysis revealed significant improvements
            in accuracy and processing speed.
            
            References:
            1. Smith, J., Johnson, M., & Brown, K. (2023). "Advanced NLP Techniques"
            2. University of California Research Department (2024)
        "#;

        let analyzer = DocumentAnalyzer::new().await.unwrap();
        let config = AnalysisConfig::default();
        
        let results = analyzer.analyze_document(academic_content, config).await.unwrap();

        // Should be categorized as academic
        assert_eq!(results.category.unwrap(), DocumentCategory::Academic);

        // Should have academic-related tags
        assert!(results.tags.iter().any(|tag| tag.contains("academic") || tag.contains("research")));

        println!("✅ Academic document analysis test passed!");
    }

    #[tokio::test]
    async fn test_technical_document_analysis() {
        let technical_content = r#"
            API Documentation
            
            This endpoint accepts POST requests to /api/documents with the following parameters:
            
            function processDocument(content: string, config: ProcessingConfig): Promise<Document> {
                // Implementation details
                return new Promise((resolve, reject) => {
                    // Processing logic here
                });
            }
            
            The algorithm processes the input using advanced parsing techniques
            and returns a structured Document object with extracted metadata.
        "#;

        let analyzer = DocumentAnalyzer::new().await.unwrap();
        let config = AnalysisConfig::default();
        
        let results = analyzer.analyze_document(technical_content, config).await.unwrap();

        // Should be categorized as technical
        assert_eq!(results.category.unwrap(), DocumentCategory::Technical);

        // Should have technical-related tags
        assert!(results.tags.iter().any(|tag| tag.contains("technical") || tag.contains("api")));

        println!("✅ Technical document analysis test passed!");
    }

    #[test]
    fn test_language_detection_standalone() {
        // Test with English text
        let english_text = "This is a test document written in English.";
        let detected = detect_language(english_text);
        
        #[cfg(feature = "ai")]
        assert!(detected.is_some());
        
        #[cfg(not(feature = "ai"))]
        assert!(detected.is_none());

        println!("✅ Language detection test passed!");
    }

    #[tokio::test]
    async fn test_analysis_config_customization() {
        let content = "Test document content.";
        let analyzer = DocumentAnalyzer::new().await.unwrap();

        // Test with minimal config
        let minimal_config = AnalysisConfig {
            categorization: true,
            entity_extraction: false,
            embeddings: false,
            auto_tagging: false,
            language_detection: true,
        };

        let results = analyzer.analyze_document(content, minimal_config).await.unwrap();

        // Should have category and language
        assert!(results.category.is_some());
        assert!(results.language.is_some());

        // Should not have entities, embeddings, or tags
        assert!(results.entities.is_empty());
        assert!(results.embeddings.is_none());
        assert!(results.tags.is_empty());

        println!("✅ Analysis config customization test passed!");
    }

    #[tokio::test]
    async fn test_empty_content_handling() {
        let analyzer = DocumentAnalyzer::new().await.unwrap();
        let config = AnalysisConfig::default();

        // Test with empty content
        let results = analyzer.analyze_document("", config).await.unwrap();

        // Should handle gracefully
        assert!(results.entities.is_empty());
        assert!(results.tags.is_empty());

        println!("✅ Empty content handling test passed!");
    }
} 