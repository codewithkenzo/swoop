use std::sync::Arc;
use std::time::Instant;
use tracing::info;

use swoop::{
    Result, 
    storage::{memory::MemoryStorage, Storage},
    rate_limiter::{RateLimiter, RateLimitConfig},
    extractors::{DataExtractor, ExtractorConfig, ExtractionResult},
    intelligence::{IntelligenceProcessor, IntelligenceConfig},
    models::{Document, Metadata},
};

/// Modern High-Performance Demo for Swoop
/// 
/// Demonstrates concurrent document processing with:
/// - High-throughput extraction 
/// - Intelligent rate limiting
/// - Real-time performance metrics
/// - Modern async architecture
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("swoop=info,swoop_high_performance=info")
        .init();

    info!("🚀 Swoop High-Performance Demo v2.0");
    info!("⚡ Modern async Rust architecture");

    let demo = HighPerformanceDemo::new().await?;
    demo.run_performance_benchmarks().await?;
    
    info!("✅ High-performance demo completed successfully!");
    Ok(())
}

/// High-performance demonstration system
struct HighPerformanceDemo {
    storage: Arc<dyn Storage>,
    extractor: DataExtractor,
    intelligence: IntelligenceProcessor,
    rate_limiter: Arc<RateLimiter>,
}

impl HighPerformanceDemo {
    async fn new() -> Result<Self> {
        info!("🔧 Initializing high-performance components...");

        // Memory storage for maximum speed
        let storage: Arc<dyn Storage> = Arc::new(MemoryStorage::new());
        
        // High-performance extraction config
        let extractor_config = ExtractorConfig {
            extract_emails: true,
            extract_phones: true,
            detect_sensitive: true,
            email_validation: true,
            phone_formatting: true,
            ..Default::default()
        };
        let extractor = DataExtractor::new(extractor_config);

        // AI intelligence config
        let intelligence_config = IntelligenceConfig {
            extract_entities: true,
            generate_summary: true,
            enable_quality_analysis: true,
            enable_classification: true,
            min_quality_threshold: 0.7,
            ..Default::default()
        };
        let intelligence = IntelligenceProcessor::new(intelligence_config);

        // High-throughput rate limiting
        let rate_config = RateLimitConfig {
            requests_per_second: 100,
            burst_capacity: 200,
            window_seconds: 60,
            default_delay_ms: 10,
            ip_requests_per_minute: 2000,
            global_requests_per_second: 200,
            max_requests: 1000,
            enabled: true,
        };
        let rate_limiter = Arc::new(RateLimiter::new(rate_config));

        info!("✅ High-performance components initialized");

        Ok(Self {
            storage,
            extractor,
            intelligence,
            rate_limiter,
        })
    }

    async fn run_performance_benchmarks(&self) -> Result<()> {
        info!("📊 Running performance benchmarks...");

        // Benchmark 1: Single document extraction performance
        self.benchmark_extraction_speed().await?;
        
        // Benchmark 2: Concurrent processing throughput
        self.benchmark_concurrent_throughput().await?;
        
        // Benchmark 3: AI analysis performance
        self.benchmark_ai_performance().await?;
        
        info!("📈 All benchmarks completed successfully");
        Ok(())
    }

    async fn benchmark_extraction_speed(&self) -> Result<()> {
        info!("🔍 Benchmark 1: Extraction Speed Test");
        
        let test_content = r#"
        Contact us at support@company.com or call (555) 123-4567.
        Our sales team can be reached at sales@company.com or (555) 987-6543.
        Visit https://company.com for more information.
        Emergency contact: emergency@company.com, Phone: +1-800-HELP-NOW
        "#;

        let start = Instant::now();
        let result = self.extractor.extract_all(test_content, test_content)?;
        let duration = start.elapsed();

        info!("📊 Extraction Results:");
        info!("   Processing time: {:?}", duration);
        info!("   Emails found: {}", result.emails.len());
        info!("   Phone numbers: {}", result.phones.len());
        info!("   Links: {}", result.links.len());
        info!("   Quality score: {:.2}", result.quality_score);
        info!("   Throughput: {:.2} chars/ms", test_content.len() as f64 / duration.as_millis() as f64);

        // Create and store a document
        let document = Document {
            id: "perf_test_1".to_string(),
            title: "Extraction Speed Test".to_string(),
            content: test_content.to_string(),
            content_type: Some("text/plain".to_string()),
            file_size: Some(test_content.len() as u64),
            metadata: Metadata {
                source_url: Some("internal://extraction-test".to_string()),
                processed_at: chrono::Utc::now(),
                processor: Some("high-performance-extractor".to_string()),
                ..Default::default()
            },
            content_hash: Some("extraction_test_hash".to_string()),
            summary: Some("Performance test document for extraction speed".to_string()),
            extracted_at: chrono::Utc::now(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            quality_score: Some(result.quality_score),
            source_url: Some("internal://extraction-test".to_string()),
            document_type: Some("test".to_string()),
            language: Some("en".to_string()),
            word_count: Some(test_content.split_whitespace().count()),
            size_bytes: Some(test_content.len() as u64),
        };

        self.storage.store_document(&document).await?;
        info!("✅ Extraction speed benchmark completed");
        Ok(())
    }

    async fn benchmark_concurrent_throughput(&self) -> Result<()> {
        info!("⚡ Benchmark 2: Concurrent Throughput Test");
        
        let test_documents = vec![
            "Document 1: Contact john@example.com or call (555) 111-1111",
            "Document 2: Email support@company.org, Phone: 555-222-2222",
            "Document 3: Reach us at info@business.net or dial 555-333-3333",
            "Document 4: Sales at sales@store.com, Support: (555) 444-4444",
            "Document 5: Help desk: help@service.io, Call 555-555-5555",
            "Document 6: Contact admin@site.edu or phone 555-666-6666",
            "Document 7: Email team@project.gov, Call line: 555-777-7777",
            "Document 8: Support contact@platform.biz, Phone 555-888-8888",
        ];

        let start = Instant::now();
        let mut handles = Vec::new();

        for (idx, content) in test_documents.into_iter().enumerate() {
            let extractor = self.extractor.clone();
            let storage = self.storage.clone();
            let content = content.to_string();
            
            let handle = tokio::spawn(async move {
                let result = extractor.extract_all(&content, &content)?;
                
                let document = Document {
                    id: format!("concurrent_test_{idx}"),
                    title: format!("Concurrent Test Document {}", idx + 1),
                    content: content.clone(),
                    content_type: Some("text/plain".to_string()),
                    file_size: Some(content.len() as u64),
                    metadata: Metadata {
                        source_url: Some(format!("internal://concurrent-test-{idx}")),
                        processed_at: chrono::Utc::now(),
                                                 processor: Some("concurrent-extractor".to_string()),
                        ..Default::default()
                    },
                    content_hash: Some(format!("concurrent_hash_{idx}")),
                    summary: Some(format!("Concurrent test document {}", idx + 1)),
                    extracted_at: chrono::Utc::now(),
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                    quality_score: Some(result.quality_score),
                    source_url: Some(format!("internal://concurrent-test-{idx}")),
                    document_type: Some("test".to_string()),
                    language: Some("en".to_string()),
                    word_count: Some(content.split_whitespace().count()),
                    size_bytes: Some(content.len() as u64),
                };

                storage.store_document(&document).await?;
                Ok::<ExtractionResult, swoop::Error>(result)
            });
            
            handles.push(handle);
        }

        let results = futures::future::try_join_all(handles).await
            .map_err(|e| swoop::Error::Other(format!("Join error: {e}")))?;
            
        let duration = start.elapsed();
        let successful_extractions = results.into_iter().collect::<Result<Vec<_>>>()?;

        info!("📊 Concurrent Processing Results:");
        info!("   Total time: {:?}", duration);
        info!("   Documents processed: {}", successful_extractions.len());
        info!("   Average per document: {:?}", duration / successful_extractions.len() as u32);
        info!("   Throughput: {:.2} docs/sec", successful_extractions.len() as f64 / duration.as_secs_f64());
        info!("   Success rate: 100%");

        info!("✅ Concurrent throughput benchmark completed");
        Ok(())
    }

    async fn benchmark_ai_performance(&self) -> Result<()> {
        info!("🧠 Benchmark 3: AI Analysis Performance");
        
        let complex_content = r#"
        Artificial Intelligence and Machine Learning Technologies in 2024
        
        The AI industry continues to experience rapid growth across multiple sectors.
        Natural language processing has reached new milestones with advanced models
        capable of understanding context and generating human-like responses.
        
        Key developments include:
        - Advanced neural networks for document processing
        - Real-time language translation and understanding
        - Automated content analysis and categorization
        - Intelligent data extraction from unstructured sources
        
        For business inquiries, contact our AI division at ai-business@techcorp.com
        or call our dedicated AI hotline at 1-800-AI-FUTURE (1-800-243-8887).
        
        Research collaborations: research@ai-lab.edu
        Technical support: support@ai-platform.com, Phone: +1-555-AI-HELP
        "#;

        let start = Instant::now();
        let analysis_result = self.intelligence
            .process_content(complex_content, "ai_benchmark.txt", &["ai".to_string(), "benchmark".to_string()])
            .await?;
        let duration = start.elapsed();

        info!("📊 AI Analysis Results:");
        info!("   Analysis time: {:?}", duration);
        info!("   Content length: {} chars", complex_content.len());
        info!("   Processing speed: {:.2} chars/ms", complex_content.len() as f64 / duration.as_millis() as f64);
        info!("   Emails detected: {}", analysis_result.emails.len());
        info!("   Phone numbers: {}", analysis_result.phones.len());
        info!("   Classification: {}", analysis_result.classification);
        info!("   Quality score: {:.2}", analysis_result.quality_score);
        info!("   Metadata entries: {}", analysis_result.metadata.len());

        info!("✅ AI analysis benchmark completed");
        Ok(())
    }
} 