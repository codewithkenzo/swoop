use swoop::{
    error::Result,
    extractors::{DataExtractor, ExtractorConfig},
    loaders::{BulkLoader, LoaderConfig},
};
use tokio::fs;
use reqwest::Client;
use serde_json::Value;
use std::time::Instant;
use futures::future::join_all;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    println!("⚡ REAL ASYNC SWOOP DEMO - Concurrent Processing");
    println!("{}", "=".repeat(60));
    
    // Test 1: Sequential vs Concurrent comparison
    println!("\n📊 Performance Comparison: Sequential vs Concurrent");
    performance_comparison().await?;
    
    // Test 2: Real concurrent crawling
    println!("\n🚀 Step 1: Concurrent URL Loading and Processing");
    concurrent_url_processing().await?;
    
    // Test 3: Concurrent server operations  
    println!("\n🌐 Step 2: Concurrent Server Operations");
    concurrent_server_ops().await?;
    
    // Test 4: Bulk concurrent extraction
    println!("\n⚡ Step 3: Bulk Concurrent Data Extraction");
    bulk_concurrent_extraction().await?;
    
    println!("\n🎉 Real async demo completed with concurrent operations!");
    println!("📁 Check data/async_demo/ for concurrent extraction results");
    
    Ok(())
}

async fn performance_comparison() -> Result<()> {
    let urls = vec![
        "https://httpbin.org/delay/1",  // 1 second delay
        "https://httpbin.org/delay/2",  // 2 second delay  
        "https://httpbin.org/delay/1",  // 1 second delay
        "https://jsonplaceholder.typicode.com/users/1",
        "https://jsonplaceholder.typicode.com/users/2",
    ];
    
    let client = Client::new();
    
    // Sequential processing
    println!("🐌 Sequential processing (one after another):");
    let start = Instant::now();
    for (i, url) in urls.iter().enumerate() {
        println!("   Processing URL {}: {}", i + 1, url);
        let resp_start = Instant::now();
        match client.get(url).send().await {
            Ok(response) => {
                let status = response.status();
                let _response_text = response.text().await?;
                println!("   ✅ URL {} completed in {:?} - Status: {}", 
                         i + 1, resp_start.elapsed(), status);
            },
            Err(e) => println!("   ❌ URL {} failed: {}", i + 1, e),
        }
    }
    let sequential_time = start.elapsed();
    println!("   📊 Total sequential time: {:?}\n", sequential_time);
    
    // Concurrent processing
    println!("🚀 Concurrent processing (all at once):");
    let start = Instant::now();
    
    let tasks: Vec<JoinHandle<_>> = urls.iter().enumerate().map(|(i, url)| {
        let client = client.clone();
        let url = url.to_string();
        tokio::spawn(async move {
            println!("   🚀 Starting concurrent request {}: {}", i + 1, url);
            let resp_start = Instant::now();
            match client.get(url).send().await {
                Ok(response) => {
                    let status = response.status();
                    let _response_text = response.text().await.unwrap_or_default();
                    println!("   ✅ Concurrent request {} completed in {:?} - Status: {}", 
                             i + 1, resp_start.elapsed(), status);
                    (i + 1, true, resp_start.elapsed())
                },
                Err(e) => {
                    println!("   ❌ Concurrent request {} failed: {}", i + 1, e);
                    (i + 1, false, resp_start.elapsed())
                }
            }
        })
    }).collect();
    
    let results = join_all(tasks).await;
    let concurrent_time = start.elapsed();
    
    let success_count = results.iter().filter(|r| r.is_ok() && r.as_ref().unwrap().1).count();
    println!("   📊 Total concurrent time: {:?}", concurrent_time);
    println!("   📊 Successful requests: {}/{}", success_count, urls.len());
    println!("   📊 Speed improvement: {:.2}x faster!", 
             sequential_time.as_secs_f64() / concurrent_time.as_secs_f64());
    
    Ok(())
}

async fn concurrent_url_processing() -> Result<()> {
    let config = LoaderConfig {
        batch_size: 50,
        max_concurrent: 10,
        timeout_seconds: 30,
        max_urls: 1000,
        skip_invalid: true,
        validate_urls: true,
        deduplicate: true,
    };
    
    let mut loader = BulkLoader::new(config);
    let url_entries = loader.load_from_csv("data/test_files/real_urls.csv").await?;
    let urls: Vec<String> = url_entries.iter().map(|e| e.url.clone()).collect();
    
    println!("✅ Loaded {} URLs for concurrent processing", urls.len());
    
    // Create client for concurrent operations
    let client = Client::new();
    
    // Submit multiple crawl jobs concurrently to the server
    println!("🚀 Submitting multiple crawl jobs concurrently...");
    let start = Instant::now();
    
    // Split URLs into batches for concurrent submission
    let batch_size = 2;
    let batches: Vec<Vec<String>> = urls.chunks(batch_size)
        .map(|chunk| chunk.to_vec())
        .collect();
    
    let crawl_tasks: Vec<JoinHandle<_>> = batches.into_iter().enumerate().map(|(i, batch)| {
        let client = client.clone();
        tokio::spawn(async move {
            let payload = serde_json::json!({
                "urls": batch,
                "max_depth": 1,
                "max_pages": 5,
                "settings": {
                    "respect_robots_txt": true,
                    "user_agent": format!("Swoop/1.0 Async Demo Batch {}", i + 1),
                    "delay_ms": 1000
                }
            });
            
            println!("   📤 Submitting batch {} with {} URLs", i + 1, batch.len());
            
            match client
                .post("http://localhost:3056/api/v1/crawl")
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
                .await
            {
                Ok(response) => {
                    let status = response.status();
                    let _response_text = response.text().await.unwrap_or_default();
                    if status.is_success() {
                        if let Ok(json) = serde_json::from_str::<Value>(&_response_text) {
                            if let Some(job_id) = json.get("job_id").and_then(|v| v.as_str()) {
                                println!("   ✅ Batch {} job started: {}", i + 1, job_id);
                                return Some((i + 1, job_id.to_string()));
                            }
                        }
                    }
                    println!("   ❌ Batch {} failed: {} - {}", i + 1, status, _response_text);
                    None
                },
                Err(e) => {
                    println!("   ❌ Batch {} network error: {}", i + 1, e);
                    None
                }
            }
        })
    }).collect();
    
    let job_results = join_all(crawl_tasks).await;
    let submit_time = start.elapsed();
    
    let successful_jobs: Vec<_> = job_results.into_iter()
        .filter_map(|r| r.ok().flatten())
        .collect();
    
    println!("📊 Concurrent submission completed in {:?}", submit_time);
    println!("📊 Successfully started {} crawl jobs", successful_jobs.len());
    
    // Monitor all jobs concurrently
    if !successful_jobs.is_empty() {
        println!("🔄 Monitoring all jobs concurrently...");
        let monitor_tasks: Vec<JoinHandle<_>> = successful_jobs.into_iter().map(|(batch_id, job_id)| {
            let client = client.clone();
            tokio::spawn(async move {
                for attempt in 1..=3 {
                    match client
                        .get(&format!("http://localhost:3056/api/v1/crawl/{}", job_id))
                        .send()
                        .await
                    {
                        Ok(response) => {
                            let _response_text = response.text().await.unwrap_or_default();
                            println!("   📊 Batch {} (attempt {}): Job status received", batch_id, attempt);
                        },
                        Err(e) => println!("   ❌ Batch {} monitoring error: {}", batch_id, e),
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
                batch_id
            })
        }).collect();
        
        let _monitor_results = join_all(monitor_tasks).await;
        println!("✅ All job monitoring completed concurrently");
    }
    
    Ok(())
}

async fn concurrent_server_ops() -> Result<()> {
    println!("🌐 Testing multiple server endpoints concurrently...");
    
    let client = Client::new();
    let start = Instant::now();
    
    // Create concurrent tasks for different server operations
    let server_tasks = vec![
        tokio::spawn({
            let client = client.clone();
            async move {
                match client.get("http://localhost:3056/api/v1/health").send().await {
                    Ok(response) => {
                        let _response_text = response.text().await.unwrap_or_default();
                        println!("   🏥 Health check completed: Server healthy");
                        Some(("health", _response_text))
                    },
                    Err(e) => {
                        println!("   ❌ Health check failed: {}", e);
                        None
                    }
                }
            }
        }),
        tokio::spawn({
            let client = client.clone();
            async move {
                match client.get("http://localhost:3056/api/v1/stats").send().await {
                    Ok(response) => {
                        let _response_text = response.text().await.unwrap_or_default();
                        println!("   📊 Stats retrieved: Server statistics");
                        Some(("stats", _response_text))
                    },
                    Err(e) => {
                        println!("   ❌ Stats failed: {}", e);
                        None
                    }
                }
            }
        }),
        tokio::spawn({
            let client = client.clone();
            async move {
                match client.get("http://localhost:3056/ready").send().await {
                    Ok(response) => {
                        let _response_text = response.text().await.unwrap_or_default();
                        println!("   ✅ Readiness check completed: Server ready");
                        Some(("ready", _response_text))
                    },
                    Err(e) => {
                        println!("   ❌ Readiness check failed: {}", e);
                        None
                    }
                }
            }
        }),
        tokio::spawn({
            let client = client.clone();
            async move {
                match client.get("http://localhost:3056/metrics").send().await {
                    Ok(response) => {
                        let _response_text = response.text().await.unwrap_or_default();
                        println!("   📈 Metrics retrieved: Prometheus metrics");
                        Some(("metrics", _response_text))
                    },
                    Err(e) => {
                        println!("   ❌ Metrics failed: {}", e);
                        None
                    }
                }
            }
        }),
    ];
    
    let results = join_all(server_tasks).await;
    let total_time = start.elapsed();
    
    let successful_ops = results.iter().filter(|r| r.is_ok() && r.as_ref().unwrap().is_some()).count();
    println!("📊 Completed {} concurrent server operations in {:?}", successful_ops, total_time);
    
    Ok(())
}

async fn bulk_concurrent_extraction() -> Result<()> {
    fs::create_dir_all("data/async_demo").await?;
    
    let urls = vec![
        "https://httpbin.org/html",
        "https://jsonplaceholder.typicode.com/users/1",
        "https://jsonplaceholder.typicode.com/users/2", 
        "https://jsonplaceholder.typicode.com/users/3",
        "https://example.com",
        "https://httpbin.org/json",
        "https://httpbin.org/xml",
        "https://jsonplaceholder.typicode.com/posts/1",
    ];
    
    println!("⚡ Extracting data from {} URLs concurrently...", urls.len());
    
    let client = Client::new();
    let extractor = DataExtractor::new(ExtractorConfig::default());
    let start = Instant::now();
    
    // Process all URLs concurrently
    let extraction_tasks: Vec<JoinHandle<_>> = urls.into_iter().enumerate().map(|(i, url)| {
        let client = client.clone();
        let extractor = extractor.clone();
        tokio::spawn(async move {
            println!("   🚀 Starting concurrent extraction {}: {}", i + 1, url);
            let extract_start = Instant::now();
            let url_clone = url.clone();
            
            match client.get(url).send().await {
                Ok(response) => {
                    match response.text().await {
                        Ok(content) => {
                            match extractor.extract_all(&content, &content) {
                                Ok(extracted) => {
                                    let extract_time = extract_start.elapsed();
                                    println!("   ✅ Extraction {} completed in {:?}: {} emails, {} phones", 
                                             i + 1, extract_time, extracted.emails.len(), extracted.phones.len());
                                    
                                    // Save results
                                    let filename = format!("data/async_demo/concurrent_extracted_{}.json", i + 1);
                                    if let Ok(json) = serde_json::to_string_pretty(&extracted) {
                                        let _ = fs::write(&filename, json).await;
                                    }
                                    
                                    Some((i + 1, url_clone, extracted, extract_time))
                                },
                                Err(e) => {
                                    println!("   ❌ Extraction {} failed: {}", i + 1, e);
                                    None
                                }
                            }
                        },
                        Err(e) => {
                            println!("   ❌ Content reading {} failed: {}", i + 1, e);
                            None
                        }
                    }
                },
                Err(e) => {
                    println!("   ❌ Request {} failed: {}", i + 1, e);
                    None
                }
            }
        })
    }).collect();
    
    let results = join_all(extraction_tasks).await;
    let total_time = start.elapsed();
    
    // Aggregate results
    let successful_extractions: Vec<_> = results.into_iter()
        .filter_map(|r| r.ok().flatten())
        .collect();
    
    let total_emails: usize = successful_extractions.iter().map(|(_, _, extracted, _)| extracted.emails.len()).sum();
    let total_phones: usize = successful_extractions.iter().map(|(_, _, extracted, _)| extracted.phones.len()).sum();
    let total_sensitive: usize = successful_extractions.iter().map(|(_, _, extracted, _)| extracted.sensitive_data.len()).sum();
    
    println!("📊 Concurrent extraction summary:");
    println!("   ⚡ Processed {} URLs concurrently in {:?}", successful_extractions.len(), total_time);
    println!("   📧 Total emails extracted: {}", total_emails);
    println!("   📞 Total phones extracted: {}", total_phones);
    println!("   🔒 Total sensitive data detected: {}", total_sensitive);
    
    // Save summary
    let summary = serde_json::json!({
        "test_type": "concurrent_bulk_extraction",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "total_time_ms": total_time.as_millis(),
        "urls_processed": successful_extractions.len(),
        "total_emails": total_emails,
        "total_phones": total_phones,
        "total_sensitive_data": total_sensitive,
        "concurrent_processing": true,
        "average_time_per_url_ms": total_time.as_millis() / successful_extractions.len().max(1) as u128
    });
    
    fs::write("data/async_demo/concurrent_summary.json", 
             serde_json::to_string_pretty(&summary)?)
        .await?;
    
    println!("💾 Concurrent extraction summary saved");
    
    Ok(())
} 