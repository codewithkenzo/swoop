/*!
 * Comprehensive Integration Tests for Crawl4AI Core
 * 
 * This test suite covers all core features with real-world scenarios:
 * - Parser functionality with diverse content types
 * - Storage systems with concurrent operations
 * - Error handling and edge cases
 * - Performance benchmarks
 */

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use serial_test::serial;

use swoop::{
    config::SelectorType,
    error::Result,
    models::{Document, Link, Metadata},
    parser::{Parser, ExtractorRule},
    storage::{Storage, memory::MemoryStorage, filesystem::FileSystemStorage},
};

#[allow(dead_code)]
const TEST_TIMEOUT: Duration = Duration::from_secs(30);

#[tokio::test]
#[serial]
async fn test_basic_html_parsing() -> Result<()> {
    println!("🔍 Testing basic HTML parsing...");
    
    let parser = Parser::new();
    
    // Test with our small test HTML file
    let html_content = include_str!("../test_data/small.html");
    let metadata = create_test_metadata("https://example.com/test");
    
    let result = parser.parse(html_content.as_bytes(), "text/html", &metadata).await?;
    
    // Verify parsing results
    assert!(!result.title.is_empty(), "Title should be extracted");
    assert!(!result.text.is_empty(), "Text content should be extracted");
    assert!(!result.html.is_empty(), "HTML should be preserved");
    assert!(!result.content.is_empty(), "Content should be extracted");
    
    println!("  ✅ HTML parsing test passed - Title: {}", result.title);
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_complex_html_parsing() -> Result<()> {
    println!("🌐 Testing complex HTML parsing...");
    
    let parser = Parser::new();
    
    // Test with our medium test HTML file
    let html_content = include_str!("../test_data/medium.html");
    let metadata = create_test_metadata("https://newstest.com/ai-article");
    
    let result = parser.parse(html_content.as_bytes(), "text/html", &metadata).await?;
    
    // Verify complex content extraction
    assert!(result.title.contains("Revolutionary AI System"), "Title should contain expected text");
    assert!(result.text.len() > 1000, "Should extract substantial text content");
    assert!(result.links.len() > 5, "Should extract multiple links");
    
    println!("  ✅ Complex HTML parsing test passed - {} characters, {} links", 
             result.text.len(), result.links.len());
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_json_parsing() -> Result<()> {
    println!("📄 Testing JSON parsing...");
    
    let parser = Parser::new();
    let json_content = r#"{
        "title": "Test API Response",
        "data": {
            "items": [1, 2, 3, 4, 5],
            "status": "success",
            "metadata": {
                "total": 100,
                "page": 1
            }
        },
        "links": [
            {"url": "https://api.example.com/item/1", "rel": "item"},
            {"url": "https://api.example.com/item/2", "rel": "item"}
        ]
    }"#;
    
    let metadata = create_test_metadata("https://api.example.com/data");
    let result = parser.parse(json_content.as_bytes(), "application/json", &metadata).await?;
    
    assert!(result.title.contains("Test API Response"), "Should extract title from JSON");
    assert!(!result.content.is_empty(), "Should have content");
    
    println!("  ✅ JSON parsing test passed - Content: {}", &result.content[..100.min(result.content.len())]);
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_xml_parsing() -> Result<()> {
    println!("📋 Testing XML parsing...");
    
    let parser = Parser::new();
    let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
    <root>
        <title>Test XML Document</title>
        <content>
            <item id="1">First item</item>
            <item id="2">Second item</item>
            <item id="3">Third item</item>
        </content>
        <metadata>
            <author>Test Author</author>
            <date>2024-01-15</date>
        </metadata>
    </root>"#;
    
    let metadata = create_test_metadata("https://example.com/data.xml");
    let result = parser.parse(xml_content.as_bytes(), "application/xml", &metadata).await?;
    
    assert!(result.title.contains("Test XML Document"), "Should extract title from XML");
    assert!(result.text.contains("First item"), "Should extract text content");
    
    println!("  ✅ XML parsing test passed - Title: {}", result.title);
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_extraction_rules() -> Result<()> {
    println!("📋 Testing extraction rules...");
    
    let parser = Parser::new();
    
    // Add custom extraction rules
    parser.add_rule(ExtractorRule {
        name: "main_title".to_string(),
        selector_type: SelectorType::CSS,
        selector: "h1.main-title".to_string(),
        attribute: None,
        multiple: false,
        required: false,
        default_value: None,
    }).await;
    
    parser.add_rule(ExtractorRule {
        name: "all_paragraphs".to_string(),
        selector_type: SelectorType::CSS,
        selector: "p.content".to_string(),
        attribute: None,
        multiple: true,
        required: false,
        default_value: None,
    }).await;
    
    let html_content = r#"
    <html>
        <body>
            <h1 class="main-title">Main Article Title</h1>
            <p class="content">First paragraph of content.</p>
            <p class="content">Second paragraph of content.</p>
            <p class="content">Third paragraph of content.</p>
        </body>
    </html>
    "#;
    
    let metadata = create_test_metadata("https://example.com/article");
    let result = parser.parse(html_content.as_bytes(), "text/html", &metadata).await?;
    
    // Verify extraction rules worked
    assert!(result.extracted.contains_key("main_title"), "Should extract main title");
    assert!(result.extracted.contains_key("all_paragraphs"), "Should extract paragraphs");
    
    let main_title = &result.extracted["main_title"];
    assert!(main_title.content.contains("Main Article Title"), "Title extraction should work");
    
    println!("  ✅ Extraction rules test passed - {} extracted fields", result.extracted.len());
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_memory_storage_operations() -> Result<()> {
    println!("💾 Testing memory storage operations...");
    
    let storage = Arc::new(MemoryStorage::new()) as Arc<dyn Storage>;
    
    // Test storing a document
    let document = create_test_document("test_doc_1", "https://example.com/doc1");
    storage.store_document(&document).await?;
    
    // Test retrieving the document
    let retrieved = storage.get_document("test_doc_1").await?;
    assert!(retrieved.is_some(), "Document should be retrievable");
    
    let retrieved_doc = retrieved.unwrap();
    assert_eq!(retrieved_doc.id, "test_doc_1");
    assert_eq!(retrieved_doc.url, "https://example.com/doc1");
    
    // Test updating the document
    let mut updated_doc = document.clone();
    updated_doc.title = "Updated Title".to_string();
    storage.store_document(&updated_doc).await?;
    
    let retrieved_updated = storage.get_document("test_doc_1").await?.unwrap();
    assert_eq!(retrieved_updated.title, "Updated Title");
    
    // Test search functionality
    let search_results = storage.search_documents("example", Some(10)).await?;
    assert!(!search_results.is_empty(), "Search should find the document");
    
    // Test storage stats
    let stats = storage.get_stats().await?;
    assert!(stats.total_documents >= 1, "Stats should show at least 1 document");
    
    println!("  ✅ Memory storage test passed - {} documents in storage", stats.total_documents);
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_concurrent_storage_operations() -> Result<()> {
    println!("🔄 Testing concurrent storage operations...");
    
    let storage = Arc::new(MemoryStorage::new()) as Arc<dyn Storage>;
    let mut handles = vec![];
    
    // Spawn multiple concurrent storage operations
    for i in 0..10 {
        let storage_clone: Arc<dyn Storage> = Arc::clone(&storage);
        let handle = tokio::spawn(async move {
            let document = create_test_document(
                &format!("concurrent_doc_{}", i),
                &format!("https://example.com/doc{}", i)
            );
            
            // Store document
            storage_clone.store_document(&document).await?;
            
            // Retrieve document
            let retrieved = storage_clone.get_document(&document.id).await?;
            assert!(retrieved.is_some(), "Document should be retrievable");
            
            // Search for document
            let search_results = storage_clone.search_documents("concurrent", Some(5)).await?;
            assert!(!search_results.is_empty(), "Search should find documents");
            
            Ok::<(), swoop::error::Error>(())
        });
        handles.push(handle);
    }
    
    // Wait for all operations to complete
    for handle in handles {
        handle.await.unwrap()?;
    }
    
    // Verify final state
    let stats = storage.get_stats().await?;
    assert_eq!(stats.total_documents, 10, "Should have stored 10 documents");
    
    println!("  ✅ Concurrent storage test passed - {} documents stored concurrently", 
             stats.total_documents);
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_filesystem_storage() -> Result<()> {
    println!("📁 Testing filesystem storage...");
    
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Arc::new(FileSystemStorage::new(&temp_dir.path().to_string_lossy()).await?) as Arc<dyn Storage>;
    
    // Test basic operations with filesystem storage
    let document = create_test_document("fs_test_doc", "https://example.com/fs-test");
    storage.store_document(&document).await?;
    
    let retrieved = storage.get_document("fs_test_doc").await?;
    assert!(retrieved.is_some(), "Document should be stored and retrievable from filesystem");
    
    let stats = storage.get_stats().await?;
    assert!(stats.total_documents >= 1, "Filesystem stats should show stored documents");
    
    println!("  ✅ Filesystem storage test passed");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_large_document_parsing() -> Result<()> {
    println!("🏗️ Testing large document parsing...");
    
    let parser = Parser::new();
    
    // Test with our large test HTML file
    let large_html = include_str!("../test_data/large.html");
    let metadata = create_test_metadata("https://testcommerce.com/catalog");
    
    let start_time = std::time::Instant::now();
    let result = parser.parse(large_html.as_bytes(), "text/html", &metadata).await?;
    let parse_time = start_time.elapsed();
    
    // Verify performance and content extraction
    assert!(result.text.len() > 5000, "Should extract substantial content from large document");
    assert!(result.links.len() > 10, "Should extract many links from large document");
    assert!(parse_time.as_secs() < 5, "Large document parsing should complete within 5 seconds");
    
    println!("  ✅ Large document parsing test passed - {} chars, {} links, {:?}", 
             result.text.len(), result.links.len(), parse_time);
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_error_handling() -> Result<()> {
    println!("🚨 Testing error handling...");
    
    let storage = Arc::new(MemoryStorage::new()) as Arc<dyn Storage>;
    let parser = Parser::new();
    
    // Test with malformed HTML
    let malformed_html = "<html><head><title>Test</head><body><p>Unclosed paragraph<div>Nested incorrectly</p></div></body>";
    let metadata = create_test_metadata("https://example.com/malformed");
    
    // Should handle malformed HTML gracefully
    let result = parser.parse(malformed_html.as_bytes(), "text/html", &metadata).await;
    assert!(result.is_ok(), "Parser should handle malformed HTML gracefully");
    
    // Test with non-existent document retrieval
    let non_existent = storage.get_document("non_existent_id").await?;
    assert!(non_existent.is_none(), "Should return None for non-existent documents");
    
    // Test with empty content
    let empty_result = parser.parse(b"", "text/html", &metadata).await?;
    assert!(!empty_result.content.is_empty(), "Should handle empty content");
    
    println!("  ✅ Error handling test passed");
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_performance_benchmarks() -> Result<()> {
    println!("⚡ Testing performance benchmarks...");
    
    let parser = Parser::new();
    let storage = Arc::new(MemoryStorage::new()) as Arc<dyn Storage>;
    
    // Test parsing performance with different content sizes
    let test_cases = vec![
        ("small", include_str!("../test_data/small.html")),
        ("medium", include_str!("../test_data/medium.html")),
        ("large", include_str!("../test_data/large.html")),
    ];
    
    for (size, content) in test_cases {
        let metadata = create_test_metadata(&format!("https://example.com/{}", size));
        
        let start_time = std::time::Instant::now();
        let result = parser.parse(content.as_bytes(), "text/html", &metadata).await?;
        let parse_time = start_time.elapsed();
        
        // Store the document
        let document = Document {
            id: format!("{}_perf_test", size),
            url: format!("https://example.com/{}", size),
            title: result.title,
            content: result.content,
            html: result.html,
            text: result.text,
            metadata: metadata.clone(),
            links: result.links,
            extracted: result.extracted,
        };
        
        let store_start = std::time::Instant::now();
        storage.store_document(&document).await?;
        let store_time = store_start.elapsed();
        
        println!("  📊 {} content: parse={:?}, store={:?}, size={} chars", 
                size, parse_time, store_time, content.len());
        
        // Performance assertions
        assert!(parse_time.as_millis() < 2000, "{} parsing should be under 2 seconds", size);
        assert!(store_time.as_millis() < 500, "{} storage should be under 500ms", size);
    }
    
    println!("  ✅ Performance benchmarks passed");
    Ok(())
}

// Helper functions

fn create_test_metadata(url: &str) -> Metadata {
    Metadata {
        url: url.to_string(),
        content_type: "text/html".to_string(),
        fetch_time: Utc::now(),
        status_code: 200,
        headers: HashMap::new(),
    }
}

fn create_test_document(id: &str, url: &str) -> Document {
    Document {
        id: id.to_string(),
        url: url.to_string(),
        title: format!("Test Document {}", id),
        content: format!("This is test content for document {}", id),
        html: format!("<html><body>Content for {}</body></html>", id),
        text: format!("Text content for document {}", id),
        metadata: create_test_metadata(url),
        links: vec![
            Link {
                url: format!("{}/link1", url),
                text: "Test Link 1".to_string(),
                source_url: url.to_string(),
                rel: None,
            }
        ],
        extracted: HashMap::new(),
    }
} 