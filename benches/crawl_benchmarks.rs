/*!
 * Performance Benchmarks for Crawl4AI Core
 * 
 * Comprehensive benchmarking suite using Criterion to measure:
 * - HTML parsing performance
 * - Storage system throughput  
 * - Concurrent crawling performance
 * - Memory usage patterns
 * - Network request processing
 */

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::runtime::Runtime;


use swoop::{
    config::SelectorType,
    models::{Document, Metadata, Link},
    parser::{Parser, ExtractorRule},
    storage::{Storage, memory::MemoryStorage},
};

// Test data constants
const SMALL_HTML: &str = include_str!("../test_data/small.html");
const MEDIUM_HTML: &str = include_str!("../test_data/medium.html");
const LARGE_HTML: &str = include_str!("../test_data/large.html");

fn create_test_metadata(url: &str) -> Metadata {
    Metadata {
        url: url.to_string(),
        content_type: "text/html".to_string(),
        fetch_time: chrono::Utc::now(),
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

fn parsing_benchmarks(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("parsing");
    
    // Benchmark HTML parsing with different sizes
    for (name, content) in [
        ("small_html", SMALL_HTML),
        ("medium_html", MEDIUM_HTML),
        ("large_html", LARGE_HTML),
    ] {
        group.throughput(Throughput::Bytes(content.len() as u64));
        group.bench_with_input(BenchmarkId::new("parse_html", name), content, |b, content| {
            let parser = Parser::new();
            let metadata = create_test_metadata("https://example.com/bench");
            
            b.iter(|| {
                rt.block_on(async {
                    let result = parser.parse(
                        black_box(content.as_bytes()),
                        black_box("text/html"),
                        black_box(&metadata)
                    ).await;
                    black_box(result)
                })
            });
        });
    }
    
    group.finish();
}

fn storage_benchmarks(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("storage");
    
    // Benchmark document storage operations
    group.bench_function("store_document", |b| {
        let storage = Arc::new(MemoryStorage::new()) as Arc<dyn Storage>;
        let document = create_test_document("bench_doc", "https://example.com/bench");
        
        b.iter(|| {
            rt.block_on(async {
                let result = storage.store_document(black_box(&document)).await;
                black_box(result)
            })
        });
    });
    
    // Benchmark document retrieval
    group.bench_function("get_document", |b| {
        let storage = Arc::new(MemoryStorage::new()) as Arc<dyn Storage>;
        let document = create_test_document("bench_doc", "https://example.com/bench");
        
        rt.block_on(async {
            storage.store_document(&document).await.unwrap();
        });
        
        b.iter(|| {
            rt.block_on(async {
                let result = storage.get_document(black_box("bench_doc")).await;
                black_box(result)
            })
        });
    });
    
    group.finish();
}

fn concurrent_operations_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("concurrent");
    
    // Benchmark concurrent parsing
    group.bench_function("concurrent_parsing", |b| {
        b.iter(|| {
            rt.block_on(async {
                let tasks: Vec<_> = (0..10).map(|i| {
                    let parser = Parser::new();
                    let metadata = create_test_metadata(&format!("https://example.com/bench{}", i));
                    
                    tokio::spawn(async move {
                        parser.parse(
                            black_box(SMALL_HTML.as_bytes()),
                            black_box("text/html"),
                            black_box(&metadata)
                        ).await
                    })
                }).collect();
                
                let results = futures::future::join_all(tasks).await;
                black_box(results)
            })
        });
    });
    
    // Benchmark concurrent storage operations
    group.bench_function("concurrent_storage", |b| {
        let storage = Arc::new(MemoryStorage::new()) as Arc<dyn Storage>;
        
        b.iter(|| {
            rt.block_on(async {
                let tasks: Vec<_> = (0..10).map(|i| {
                    let storage_clone = Arc::clone(&storage);
                    let document = create_test_document(
                        &format!("bench_doc_{}", i),
                        &format!("https://example.com/bench{}", i)
                    );
                    
                    tokio::spawn(async move {
                        storage_clone.store_document(black_box(&document)).await
                    })
                }).collect();
                
                let results = futures::future::join_all(tasks).await;
                black_box(results)
            })
        });
    });
    
    group.finish();
}

fn memory_usage_benchmark(c: &mut Criterion) {
    let rt = Runtime::new();
    
    let mut group = c.benchmark_group("memory");
    
    // Benchmark memory usage with many documents
    for size in [100, 1000, 5000] {
        group.bench_with_input(BenchmarkId::new("store_many_documents", size), &size, |b, &size| {
            b.iter(|| {
                rt.as_ref().unwrap().block_on(async {
                    let storage = Arc::new(MemoryStorage::new()) as Arc<dyn Storage>;
                    
                    for i in 0..size {
                        let document = create_test_document(
                            &format!("doc_{}", i),
                            &format!("https://example.com/doc{}", i)
                        );
                        storage.store_document(&document).await.unwrap();
                    }
                    
                    let stats = storage.get_stats().await.unwrap();
                    black_box(stats)
                })
            });
        });
    }
    
    group.finish();
}

fn extraction_rules_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("extraction_rules");
    
    // Benchmark parsing with many extraction rules
    for num_rules in [1, 5, 10, 20] {
        group.bench_with_input(BenchmarkId::new("parse_with_rules", num_rules), &num_rules, |b, &num_rules| {
            b.iter(|| {
                rt.block_on(async {
                    let parser = Parser::new();
                    
                    // Add extraction rules
                    for i in 0..num_rules {
                        parser.add_rule(ExtractorRule {
                            name: format!("rule_{}", i),
                            selector_type: SelectorType::CSS,
                            selector: format!("h{}", (i % 6) + 1), // h1 to h6
                            attribute: None,
                            multiple: false,
                            required: false,
                            default_value: None,
                        }).await;
                    }
                    
                    let metadata = create_test_metadata("https://example.com/bench");
                    let result = parser.parse(
                        black_box(MEDIUM_HTML.as_bytes()),
                        black_box("text/html"),
                        black_box(&metadata)
                    ).await;
                    
                    black_box(result)
                })
            });
        });
    }
    
    group.finish();
}

fn search_benchmarks(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("search");
    
    // Setup storage with many documents
    let storage = Arc::new(MemoryStorage::new()) as Arc<dyn Storage>;
    rt.block_on(async {
        for i in 0..1000 {
            let document = create_test_document(
                &format!("search_doc_{}", i),
                &format!("https://example.com/search{}", i)
            );
            storage.store_document(&document).await.unwrap();
        }
    });
    
    // Benchmark search operations
    group.bench_function("search_documents", |b| {
        b.iter(|| {
            rt.block_on(async {
                let result = storage.search_documents(
                    black_box("Test Document"),
                    black_box(Some(10))
                ).await;
                black_box(result)
            })
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    parsing_benchmarks,
    storage_benchmarks,
    concurrent_operations_benchmark,
    memory_usage_benchmark,
    extraction_rules_benchmark,
    search_benchmarks
);

criterion_main!(benches); 