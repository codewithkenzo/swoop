use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr},
    sync::Arc,
    time::{Duration, Instant},
};

use clap::{Arg, Command};
use reqwest::Client;
use tokio::time::sleep;
use tracing::{error, info, span, warn, Level};
use tracing_subscriber::EnvFilter;

use swoop::{
    error::{Error, Result},
    models::{Document, ExtractorRule, Metadata, SelectorType},
    parser::Parser,
    rate_limiter::{RateLimitConfig, RateLimiter},
    storage::{filesystem::FileSystemStorage, memory::MemoryStorage, Storage},
};

#[derive(Debug, Clone)]
struct CrawlTarget {
    url: String,
    name: String,
}

struct HighPerformanceCrawler {
    parser: Parser,
    rate_limiter: Arc<RateLimiter>,
    storage: Arc<dyn Storage>,
    client: Client,
    stats: Arc<tokio::sync::RwLock<CrawlStats>>,
}

#[derive(Debug, Default)]
struct CrawlStats {
    total_requests: usize,
    successful_crawls: usize,
    failed_crawls: usize,
    rate_limited: usize,
    total_processing_time: Duration,
    concurrent_requests: usize,
    start_time: Instant,
}

impl HighPerformanceCrawler {
    async fn new(use_filesystem: bool, storage_path: Option<String>, max_concurrent: usize) -> Result<Self> {
        // High-performance rate limiting configuration
        let rate_config = RateLimitConfig {
            requests_per_second: 50,     // 50 requests per second
            burst_capacity: 100,         // Allow bursts up to 100
            default_delay_ms: 20,        // Minimal 20ms delay
            ip_requests_per_minute: 1000, // High IP limit
            global_requests_per_second: 100, // High global limit
        };

        let storage: Arc<dyn Storage> = if use_filesystem {
            let path = storage_path.unwrap_or_else(|| "./swoop_data".to_string());
            info!("🗄️  High-performance filesystem storage: {}", path);
            Arc::new(FileSystemStorage::new(&path).await?)
        } else {
            info!("🗄️  High-performance in-memory storage");
            Arc::new(MemoryStorage::new())
        };

        // High-performance HTTP client configuration
        let client = Client::builder()
            .timeout(Duration::from_secs(5))  // Faster timeout
            .pool_max_idle_per_host(max_concurrent) // Connection pooling
            .pool_idle_timeout(Duration::from_secs(30))
            .user_agent("Swoop/2.0 (High-Performance Rust Crawler)")
            .build()
            .map_err(|e| Error::Http(e))?;

        Ok(Self {
            parser: Parser::new(),
            rate_limiter: Arc::new(RateLimiter::new(rate_config)),
            storage,
            client,
            stats: Arc::new(tokio::sync::RwLock::new(CrawlStats {
                start_time: Instant::now(),
                concurrent_requests: max_concurrent,
                ..Default::default()
            })),
        })
    }

    async fn setup_extraction_rules(&self) {
        info!("⚙️  Setting up high-performance extraction rules...");

        let rules = vec![
            ExtractorRule {
                name: "title".to_string(),
                selector_type: SelectorType::CSS,
                selector: "title, h1, h2".to_string(),
                attribute: None,
                multiple: false,
                required: true,
                default_value: Some("Untitled Page".to_string()),
            },
            ExtractorRule {
                name: "description".to_string(),
                selector_type: SelectorType::CSS,
                selector: "meta[name='description'], meta[property='og:description']".to_string(),
                attribute: Some("content".to_string()),
                multiple: false,
                required: false,
                default_value: Some("No description available".to_string()),
            },
        ];

        for rule in rules {
            self.parser.add_rule(rule.clone()).await;
        }
        info!("✅ High-performance extraction rules configured");
    }

    fn get_performance_test_targets() -> Vec<CrawlTarget> {
        vec![
            CrawlTarget {
                url: "https://httpbin.org/html".to_string(),
                name: "HTTPBin HTML Test".to_string(),
            },
            CrawlTarget {
                url: "https://example.com".to_string(),
                name: "Example.com".to_string(),
            },
            CrawlTarget {
                url: "https://httpbin.org/json".to_string(),
                name: "HTTPBin JSON Test".to_string(),
            },
            CrawlTarget {
                url: "https://httpbin.org/xml".to_string(),
                name: "HTTPBin XML Test".to_string(),
            },
            CrawlTarget {
                url: "https://httpbin.org/robots.txt".to_string(),
                name: "HTTPBin Robots.txt".to_string(),
            },
        ]
    }

    async fn crawl_url_concurrent(&self, target: &CrawlTarget) -> Result<Option<Document>> {
        let start_time = Instant::now();

        {
            let mut stats = self.stats.write().await;
            stats.total_requests += 1;
        }

        // Fast rate limit check
        let client_ip = IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1));
        if let Err(Error::RateLimit(_)) = self.rate_limiter.check_request(&target.url, Some(client_ip)).await {
            // Minimal wait for rate limiting
            sleep(Duration::from_millis(10)).await;
            self.rate_limiter.check_request(&target.url, Some(client_ip)).await?;
        }

        // High-performance HTTP request
        let response = self.client.get(&target.url).send().await
            .map_err(|e| Error::Http(e))?;

        let status_code = response.status().as_u16();
        let headers: HashMap<String, String> = response.headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let content_type = headers.get("content-type")
            .cloned()
            .unwrap_or_else(|| "text/html".to_string());

        let body_bytes = response.bytes().await
            .map_err(|e| Error::Http(e))?;

        let metadata = Metadata {
            url: target.url.clone(),
            content_type: content_type.clone(),
            fetch_time: chrono::Utc::now(),
            status_code,
            headers,
        };

        // Fast parsing
        let parse_result = self.parser.parse(&body_bytes, &content_type, &metadata).await?;

        let document_id = format!("doc_{}_{}", 
                                  chrono::Utc::now().timestamp_nanos(), 
                                  target.url.replace("https://", "").replace("/", "_"));

        let document = Document {
            id: document_id.clone(),
            url: target.url.clone(),
            title: parse_result.extracted.get("title")
                .map(|c| c.content.clone())
                .unwrap_or_else(|| target.name.clone()),
            content: parse_result.extracted.get("description")
                .map(|c| c.content.clone())
                .unwrap_or_else(|| "No content extracted".to_string()),
            html: String::from_utf8_lossy(&body_bytes).to_string(),
            text: parse_result.extracted.values()
                .map(|c| c.content.as_str())
                .collect::<Vec<_>>()
                .join(" "),
            metadata,
            links: parse_result.links,
            extracted: parse_result.extracted,
        };

        // Concurrent storage
        self.storage.store_document(&document).await?;
        
        {
            let mut stats = self.stats.write().await;
            stats.successful_crawls += 1;
            stats.total_processing_time += start_time.elapsed();
        }
        
        Ok(Some(document))
    }

    async fn run_concurrent_crawl(&self, targets: Vec<CrawlTarget>, max_concurrent: usize) -> Result<()> {
        info!("🚀 Starting HIGH-PERFORMANCE concurrent crawl with {} workers", max_concurrent);
        info!("🎯 Processing {} targets concurrently", targets.len());

        let semaphore = Arc::new(tokio::sync::Semaphore::new(max_concurrent));
        let mut handles = Vec::new();

        for (i, target) in targets.into_iter().enumerate() {
            let crawler = self.clone();
            let semaphore_clone = semaphore.clone();
            
            let handle = tokio::spawn(async move {
                let _permit = semaphore_clone.acquire().await.unwrap();
                
                info!("🌐 [Worker {}] Starting: {} ({})", i + 1, target.name, target.url);
                let start = Instant::now();
                
                match crawler.crawl_url_concurrent(&target).await {
                    Ok(Some(doc)) => {
                        let duration = start.elapsed();
                        info!("✅ [Worker {}] Completed in {:?}: {}", i + 1, duration, doc.title);
                        Ok(())
                    }
                    Err(e) => {
                        let duration = start.elapsed();
                        error!("❌ [Worker {}] Failed in {:?}: {} - {}", i + 1, duration, target.name, e);
                        Err(e)
                    }
                    _ => Ok(())
                }
            });
            
            handles.push(handle);
        }

        // Wait for all concurrent tasks to complete
        let results: Vec<_> = futures::future::join_all(handles).await;
        
        let mut successful = 0;
        let mut failed = 0;
        
        for result in results {
            match result {
                Ok(Ok(_)) => successful += 1,
                Ok(Err(_)) => failed += 1,
                Err(_) => failed += 1,
            }
        }

        {
            let mut stats = self.stats.write().await;
            stats.failed_crawls = failed;
        }

        info!("🎉 Concurrent crawl completed: {} successful, {} failed", successful, failed);
        Ok(())
    }

    async fn print_performance_statistics(&self) {
        info!("📈 === HIGH-PERFORMANCE STATISTICS ===");

        let stats = self.stats.read().await;
        let runtime = stats.start_time.elapsed();
        
        info!("⚡ Runtime: {:?}", runtime);
        info!("🚀 Concurrent workers: {}", stats.concurrent_requests);
        info!("📊 Total requests: {}", stats.total_requests);
        info!("✅ Successful crawls: {}", stats.successful_crawls);
        info!("❌ Failed crawls: {}", stats.failed_crawls);
        
        if stats.successful_crawls > 0 {
            let avg_time = stats.total_processing_time / stats.successful_crawls as u32;
            info!("📊 Average processing time: {:?}", avg_time);
            
            let requests_per_second = stats.total_requests as f64 / runtime.as_secs_f64();
            info!("🔥 Requests per second: {:.2}", requests_per_second);
        }

        let rate_stats = self.rate_limiter.get_stats().await;
        info!("🚦 Rate limiter - Total: {}, Blocked: {}", 
              rate_stats.total_requests, rate_stats.blocked_requests);
    }
}

// Implement Clone for concurrent usage
impl Clone for HighPerformanceCrawler {
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            rate_limiter: self.rate_limiter.clone(),
            storage: self.storage.clone(),
            client: self.client.clone(),
            stats: self.stats.clone(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let filter = EnvFilter::from_default_env()
        .add_directive("swoop_high_performance=info".parse().unwrap())
        .add_directive("swoop=info".parse().unwrap());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(false)
        .with_line_number(false)
        .init();

    let matches = Command::new("Swoop High-Performance Demo")
        .version("2.0.0")
        .about("Demonstrates Swoop's high-performance concurrent crawling capabilities")
        .arg(Arg::new("filesystem")
            .long("filesystem")
            .help("Use filesystem storage")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("concurrent")
            .long("concurrent")
            .short('c')
            .help("Number of concurrent workers")
            .value_name("NUM")
            .default_value("10"))
        .get_matches();

    let use_filesystem = matches.get_flag("filesystem");
    let max_concurrent: usize = matches.get_one::<String>("concurrent")
        .unwrap()
        .parse()
        .unwrap_or(10);

    info!("🚀 === SWOOP HIGH-PERFORMANCE DEMO ===");
    info!("💾 Storage: {}", if use_filesystem { "Filesystem" } else { "Memory" });
    info!("⚡ Concurrent workers: {}", max_concurrent);

    let crawler = HighPerformanceCrawler::new(use_filesystem, None, max_concurrent).await?;
    crawler.setup_extraction_rules().await;

    let targets = HighPerformanceCrawler::get_performance_test_targets();
    
    info!("🎯 Selected targets:");
    for target in &targets {
        info!("  • {} ({})", target.name, target.url);
    }

    // Run high-performance concurrent crawl
    crawler.run_concurrent_crawl(targets, max_concurrent).await?;

    crawler.print_performance_statistics().await;
    info!("🎉 High-performance demo completed successfully!");

    Ok(())
} 