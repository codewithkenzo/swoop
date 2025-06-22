/*!
 * Production Features Integration Tests
 * 
 * This test suite validates all production-ready features working together:
 * - Rate limiting with real domains
 * - Error handling and retries
 * - Concurrent crawling with backoff
 * - Performance monitoring
 * - Storage persistence under load
 */

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use std::time::{Duration, Instant};

use serial_test::serial;
use tokio::time::sleep;

use swoop::{
    config::SelectorType,
    error::{Error, Result},
    models::{Document, Metadata},
    parser::{Parser, ExtractorRule},
    rate_limiter::{RateLimiter, RateLimitConfig},
    storage::{Storage, memory::MemoryStorage},
};

const TEST_TIMEOUT: Duration = Duration::from_secs(60);

/// Test data for production scenarios
fn create_test_html_content() -> &'static str {
    r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>Production Test Page</title>
        <meta name="description" content="A test page for production features">
    </head>
    <body>
        <header>
            <h1>Main Title</h1>
            <nav>
                <a href="/home">Home</a>
                <a href="/about">About</a>
                <a href="/products">Products</a>
            </nav>
        </header>
        <main>
            <article class="content">
                <h2>Article Title</h2>
                <p class="description">This is the main content of the article with important information.</p>
                <div class="metadata">
                    <span class="author">John Doe</span>
                    <span class="date">2024-01-15</span>
                    <span class="category">Technology</span>
                </div>
                <ul class="tags">
                    <li>web-crawling</li>
                    <li>rust</li>
                    <li>async</li>
                </ul>
            </article>
            <aside class="sidebar">
                <h3>Related Articles</h3>
                <ul>
                    <li><a href="/article1">Article 1</a></li>
                    <li><a href="/article2">Article 2</a></li>
                </ul>
            </aside>
        </main>
    </body>
    </html>
    "#
}

fn create_test_metadata(url: &str) -> Metadata {
    let mut headers = HashMap::new();
    headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
    headers.insert("server".to_string(), "nginx/1.20.1".to_string());
    
    Metadata {
        url: url.to_string(),
        content_type: "text/html".to_string(),
        fetch_time: chrono::Utc::now(),
        status_code: 200,
        headers,
    }
}

#[tokio::test]
#[serial]
async fn test_rate_limiting_production_scenario() -> Result<()> {
    let timeout = tokio::time::timeout(TEST_TIMEOUT, async {
        // Configure conservative rate limiting for production
        let config = RateLimitConfig {
            requests_per_second: 1,
            burst_capacity: 2,
            default_delay_ms: 1000,
            ip_requests_per_minute: 30,
            global_requests_per_second: 5,
        };
        
        let rate_limiter = Arc::new(RateLimiter::new(config));
        let client_ip = IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1)); // TEST-NET-3
        
        // Test burst allowance
        let start_time = Instant::now();
        
        // Should allow burst requests
        assert!(rate_limiter.check_request("https://example.com/page1", Some(client_ip)).await.is_ok());
        assert!(rate_limiter.check_request("https://example.com/page2", Some(client_ip)).await.is_ok());
        
        // Should be rate limited after burst
        let result = rate_limiter.check_request("https://example.com/page3", Some(client_ip)).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::RateLimit(_)));
        
        let elapsed = start_time.elapsed();
        println!("Rate limiting test completed in {:?}", elapsed);
        
        // Test statistics
        let stats = rate_limiter.get_stats().await;
        assert_eq!(stats.total_requests, 3);
        assert_eq!(stats.blocked_requests, 1);
        assert_eq!(stats.active_domains, 1);
        
        let domain_stats = rate_limiter.get_domain_stats("example.com").await;
        assert!(domain_stats.is_some());
        let domain_stats = domain_stats.unwrap();
        assert!(domain_stats.total_requests >= 2);
        assert!(domain_stats.failed_requests >= 1);
        
        Ok(())
    });
    
    timeout.await.map_err(|_| Error::Timeout("Rate limiting test timed out".to_string()))?
}

#[tokio::test]
#[serial]
async fn test_multi_domain_rate_limiting() -> Result<()> {
    let timeout = tokio::time::timeout(TEST_TIMEOUT, async {
        let config = RateLimitConfig {
            requests_per_second: 2,
            burst_capacity: 3,
            global_requests_per_second: 5,
            ..Default::default()
        };
        
        let rate_limiter = Arc::new(RateLimiter::new(config));
        
        // Test different domains should have separate limits
        assert!(rate_limiter.check_request("https://domain1.com/page", None).await.is_ok());
        assert!(rate_limiter.check_request("https://domain2.com/page", None).await.is_ok());
        assert!(rate_limiter.check_request("https://domain3.com/page", None).await.is_ok());
        
        // Each domain should allow burst
        assert!(rate_limiter.check_request("https://domain1.com/page2", None).await.is_ok());
        assert!(rate_limiter.check_request("https://domain1.com/page3", None).await.is_ok());
        
        // But should be limited within domain
        let result = rate_limiter.check_request("https://domain1.com/page4", None).await;
        assert!(result.is_err());
        
        let stats = rate_limiter.get_stats().await;
        assert_eq!(stats.active_domains, 3);
        
        Ok(())
    });
    
    timeout.await.map_err(|_| Error::Timeout("Multi-domain rate limiting test timed out".to_string()))?
}

#[tokio::test]
#[serial]
async fn test_integrated_crawling_with_rate_limiting() -> Result<()> {
    let timeout = tokio::time::timeout(TEST_TIMEOUT, async {
        // Setup components
        let storage = Arc::new(MemoryStorage::new()) as Arc<dyn Storage>;
        let parser = Parser::new();
        let rate_limiter = Arc::new(RateLimiter::new(RateLimitConfig::default()));
        
        // Add extraction rules
        parser.add_rule(ExtractorRule {
            name: "title".to_string(),
            selector_type: SelectorType::CSS,
            selector: "h1".to_string(),
            attribute: None,
            multiple: false,
            required: true,
            default_value: None,
        }).await;
        
        parser.add_rule(ExtractorRule {
            name: "description".to_string(),
            selector_type: SelectorType::CSS,
            selector: ".description".to_string(),
            attribute: None,
            multiple: false,
            required: false,
            default_value: Some("No description".to_string()),
        }).await;
        
        parser.add_rule(ExtractorRule {
            name: "tags".to_string(),
            selector_type: SelectorType::CSS,
            selector: ".tags li".to_string(),
            attribute: None,
            multiple: true,
            required: false,
            default_value: None,
        }).await;
        
        // Simulate crawling multiple URLs with rate limiting
        let urls = vec![
            "https://example.com/article1",
            "https://example.com/article2", 
            "https://example.com/article3",
            "https://different.com/page1",
            "https://different.com/page2",
        ];
        
        let mut successful_crawls = 0;
        let mut rate_limited_crawls = 0;
        
        for (i, url) in urls.iter().enumerate() {
            // Check rate limiting
            match rate_limiter.check_request(url, None).await {
                Ok(_) => {
                    // Simulate parsing and storage
                    let metadata = create_test_metadata(url);
                    let html_content = create_test_html_content();
                    
                    let parse_result = parser.parse(
                        html_content.as_bytes(),
                        "text/html",
                        &metadata
                    ).await?;
                    
                    // Create document
                    let document = Document {
                        id: format!("doc_{}", i),
                        url: url.to_string(),
                        title: parse_result.extracted.get("title")
                            .map(|c| c.content.clone())
                            .unwrap_or_else(|| "Untitled".to_string()),
                        content: parse_result.extracted.get("description")
                            .map(|c| c.content.clone())
                            .unwrap_or_else(|| "No content".to_string()),
                        html: html_content.to_string(),
                        text: "Extracted text content".to_string(),
                        metadata,
                        links: parse_result.links,
                        extracted: parse_result.extracted,
                    };
                    
                    // Store document
                    storage.store_document(&document).await?;
                    successful_crawls += 1;
                    
                    println!("Successfully crawled and stored: {}", url);
                }
                Err(Error::RateLimit(_)) => {
                    rate_limited_crawls += 1;
                    println!("Rate limited: {}", url);
                    
                    // Wait and retry
                    rate_limiter.wait_if_needed(url).await?;
                    sleep(Duration::from_millis(100)).await; // Additional safety margin
                    
                    // Retry after waiting
                    if rate_limiter.check_request(url, None).await.is_ok() {
                        println!("Retry successful for: {}", url);
                        successful_crawls += 1;
                    } else {
                        println!("Retry still rate limited: {}", url);
                    }
                }
                Err(e) => return Err(e),
            }
        }
        
        // Verify results
        assert!(successful_crawls > 0, "Should have successful crawls");
        println!("Crawling results: {} successful, {} rate limited", successful_crawls, rate_limited_crawls);
        
        // Verify storage
        let storage_stats = storage.get_stats().await?;
        assert!(storage_stats.total_documents > 0);
        assert!(storage_stats.successful_operations >= successful_crawls as u64);
        
        // Verify rate limiter stats
        let rate_stats = rate_limiter.get_stats().await;
        assert!(rate_stats.total_requests >= urls.len() as u64);
        assert!(rate_stats.active_domains >= 2); // Should have at least 2 domains
        
        Ok(())
    });
    
    timeout.await.map_err(|_| Error::Timeout("Integrated crawling test timed out".to_string()))?
}

#[tokio::test]
#[serial]
async fn test_concurrent_crawling_with_rate_limiting() -> Result<()> {
    let timeout = tokio::time::timeout(TEST_TIMEOUT, async {
        let config = RateLimitConfig {
            requests_per_second: 5,
            burst_capacity: 10,
            global_requests_per_second: 20,
            ..Default::default()
        };
        
        let rate_limiter = Arc::new(RateLimiter::new(config));
        let storage = Arc::new(MemoryStorage::new()) as Arc<dyn Storage>;
        
        // Create multiple concurrent tasks
        let mut tasks = Vec::new();
        
        for i in 0..20 {
            let rate_limiter_clone = Arc::clone(&rate_limiter);
            let storage_clone = Arc::clone(&storage);
            
            let task = tokio::spawn(async move {
                let url = format!("https://site{}.com/page{}", i % 5, i); // 5 different domains
                let client_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, (i % 10) as u8 + 1));
                
                // Attempt to crawl with rate limiting
                match rate_limiter_clone.check_request(&url, Some(client_ip)).await {
                    Ok(_) => {
                        // Simulate successful crawl
                        let metadata = create_test_metadata(&url);
                        let document = Document {
                            id: format!("concurrent_doc_{}", i),
                            url: url.clone(),
                            title: format!("Page {}", i),
                            content: format!("Content for page {}", i),
                            html: create_test_html_content().to_string(),
                            text: format!("Text content {}", i),
                            metadata,
                            links: vec![],
                            extracted: HashMap::new(),
                        };
                        
                        storage_clone.store_document(&document).await.map(|_| (i, true))
                    }
                    Err(Error::RateLimit(_)) => {
                        // Rate limited
                        Ok((i, false))
                    }
                    Err(e) => Err(e),
                }
            });
            
            tasks.push(task);
        }
        
        // Wait for all tasks
        let results = futures::future::join_all(tasks).await;
        
        let mut successful = 0;
        let mut rate_limited = 0;
        let mut errors = 0;
        
        for result in results {
            match result {
                Ok(Ok((_, true))) => successful += 1,
                Ok(Ok((_, false))) => rate_limited += 1,
                Ok(Err(_)) => errors += 1,
                Err(_) => errors += 1,
            }
        }
        
        println!("Concurrent crawling results: {} successful, {} rate limited, {} errors", 
                 successful, rate_limited, errors);
        
        // Verify that rate limiting is working
        assert!(rate_limited > 0, "Should have some rate limited requests");
        assert!(successful > 0, "Should have some successful requests");
        assert_eq!(errors, 0, "Should have no errors");
        
        // Verify storage
        let storage_stats = storage.get_stats().await?;
        assert_eq!(storage_stats.total_documents, successful as u64);
        
        // Verify rate limiter stats
        let rate_stats = rate_limiter.get_stats().await;
        assert_eq!(rate_stats.total_requests, 20);
        assert_eq!(rate_stats.blocked_requests, rate_limited as u64);
        assert!(rate_stats.active_domains <= 5); // Max 5 different domains
        
        Ok(())
    });
    
    timeout.await.map_err(|_| Error::Timeout("Concurrent crawling test timed out".to_string()))?
}

#[tokio::test]
#[serial]
async fn test_error_handling_and_retries() -> Result<()> {
    let timeout = tokio::time::timeout(TEST_TIMEOUT, async {
        let parser = Parser::new();
        
        // Test invalid HTML parsing
        let invalid_html = b"<html><body><unclosed-tag>content";
        let metadata = create_test_metadata("https://example.com/broken");
        
        let result = parser.parse(invalid_html, "text/html", &metadata).await;
        // Should not fail completely, but should handle gracefully
        assert!(result.is_ok(), "Parser should handle invalid HTML gracefully");
        
        // Test with malformed JSON
        let invalid_json = b"{\"key\": \"value\", \"malformed\": }";
        let result = parser.parse(invalid_json, "application/json", &metadata).await;
        // Should return error for malformed JSON
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.is_retryable() == false); // JSON parsing errors are not retryable
        }
        
        // Test storage error handling
        let storage = MemoryStorage::new();
        
        // Try to get non-existent document
        let result = storage.get_document("non-existent-id").await;
        assert!(result.is_ok()); // Should return Ok(None)
        assert!(result.unwrap().is_none());
        
        // Try to update non-existent document
        let fake_doc = Document {
            id: "fake-id".to_string(),
            url: "https://example.com/fake".to_string(),
            title: "Fake".to_string(),
            content: "Fake content".to_string(),
            html: "<html></html>".to_string(),
            text: "Fake text".to_string(),
            metadata: create_test_metadata("https://example.com/fake"),
            links: vec![],
            extracted: HashMap::new(),
        };
        
        let result = storage.update_document(&fake_doc).await;
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, Error::NotFound(_)));
            assert!(!e.is_retryable()); // NotFound errors are not retryable
        }
        
        println!("Error handling tests completed successfully");
        Ok(())
    });
    
    timeout.await.map_err(|_| Error::Timeout("Error handling test timed out".to_string()))?
}

#[tokio::test]
#[serial]
async fn test_performance_under_load() -> Result<()> {
    let timeout = tokio::time::timeout(TEST_TIMEOUT, async {
        let storage = Arc::new(MemoryStorage::new()) as Arc<dyn Storage>;
        let parser = Parser::new();
        
        // Add some extraction rules for realistic parsing
        parser.add_rule(ExtractorRule {
            name: "title".to_string(),
            selector_type: SelectorType::CSS,
            selector: "h1, h2, h3".to_string(),
            attribute: None,
            multiple: true,
            required: false,
            default_value: None,
        }).await;
        
        let start_time = Instant::now();
        let num_documents = 100;
        
        // Create and process many documents
        for i in 0..num_documents {
            let url = format!("https://performance-test.com/page{}", i);
            let metadata = create_test_metadata(&url);
            let html_content = create_test_html_content();
            
            // Parse
            let parse_result = parser.parse(
                html_content.as_bytes(),
                "text/html",
                &metadata
            ).await?;
            
            // Create document
            let document = Document {
                id: format!("perf_doc_{}", i),
                url: url.clone(),
                title: format!("Performance Test Page {}", i),
                content: format!("Content for performance test page {}", i),
                html: html_content.to_string(),
                text: format!("Text content for page {}", i),
                metadata,
                links: parse_result.links,
                extracted: parse_result.extracted,
            };
            
            // Store
            storage.store_document(&document).await?;
        }
        
        let processing_time = start_time.elapsed();
        let docs_per_second = num_documents as f64 / processing_time.as_secs_f64();
        
        println!("Processed {} documents in {:?} ({:.2} docs/sec)", 
                 num_documents, processing_time, docs_per_second);
        
        // Verify performance metrics
        assert!(docs_per_second > 10.0, "Should process at least 10 docs/sec, got {:.2}", docs_per_second);
        
        // Verify storage stats
        let stats = storage.get_stats().await?;
        assert_eq!(stats.total_documents, num_documents as u64);
        assert_eq!(stats.successful_operations, num_documents as u64);
        assert_eq!(stats.failed_operations, 0);
        assert!(stats.avg_operation_time_ms < 100.0, "Average operation time should be < 100ms");
        
        // Test search performance
        let search_start = Instant::now();
        let search_results = storage.search_documents("Performance Test", Some(10)).await?;
        let search_time = search_start.elapsed();
        
        println!("Search completed in {:?}, found {} results", search_time, search_results.len());
        assert!(search_time < Duration::from_millis(100), "Search should complete in < 100ms");
        assert!(search_results.len() > 0, "Should find matching documents");
        assert!(search_results.len() <= 10, "Should respect limit");
        
        Ok(())
    });
    
    timeout.await.map_err(|_| Error::Timeout("Performance test timed out".to_string()))?
}

#[tokio::test]
#[serial]
async fn test_cleanup_and_resource_management() -> Result<()> {
    let timeout = tokio::time::timeout(TEST_TIMEOUT, async {
        let rate_limiter = Arc::new(RateLimiter::new(RateLimitConfig::default()));
        
        // Generate many requests to different domains and IPs
        for i in 0..50 {
            let url = format!("https://cleanup-test-{}.com/page", i);
            let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, (i % 255) as u8 + 1));
            
            let _ = rate_limiter.check_request(&url, Some(ip)).await;
        }
        
        let stats_before = rate_limiter.get_stats().await;
        println!("Before cleanup: {} domains, {} IPs", 
                 stats_before.active_domains, stats_before.active_ips);
        
        // Test cleanup
        rate_limiter.cleanup().await;
        
        let stats_after = rate_limiter.get_stats().await;
        println!("After cleanup: {} domains, {} IPs", 
                 stats_after.active_domains, stats_after.active_ips);
        
        // Cleanup should preserve active limiters but can reduce inactive ones
        assert!(stats_after.active_domains <= stats_before.active_domains);
        assert!(stats_after.active_ips <= stats_before.active_ips);
        
        Ok(())
    });
    
    timeout.await.map_err(|_| Error::Timeout("Cleanup test timed out".to_string()))?
} 