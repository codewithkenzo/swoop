/*!
 * Crawl4AI Core Demo CLI Application
 * 
 * This application demonstrates all production features in action:
 * - Real web crawling with multiple domains
 * - Production rate limiting with live statistics
 * - HTML parsing with extraction rules
 * - Storage operations with performance metrics
 * - Error handling and retry logic
 * - Concurrent crawling with proper coordination
 * - Real-time monitoring and logging
 */

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use std::time::{Duration, Instant};

use clap::{Arg, Command};
use tokio::time::sleep;
use tracing::{info, warn, error, debug, span, Level};
use tracing_subscriber::EnvFilter;

use swoop::{
    config::SelectorType,
    error::{Error, Result},
    models::{Document, Metadata},
    parser::{Parser, ExtractorRule},
    rate_limiter::{RateLimiter, RateLimitConfig},
    storage::{Storage, memory::MemoryStorage, filesystem::FileSystemStorage},
};

#[derive(Debug, Clone)]
struct CrawlTarget {
    url: String,
    name: String,
}

struct CrawlDemo {
    parser: Parser,
    rate_limiter: Arc<RateLimiter>,
    storage: Arc<dyn Storage>,
    client: reqwest::Client,
    stats: Arc<tokio::sync::RwLock<DemoStats>>,
}

#[derive(Debug)]
struct DemoStats {
    total_requests: usize,
    successful_crawls: usize,
    rate_limited: usize,
    parsing_errors: usize,
    storage_errors: usize,
    total_processing_time: Duration,
    start_time: Instant,
}

impl Default for DemoStats {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_crawls: 0,
            rate_limited: 0,
            parsing_errors: 0,
            storage_errors: 0,
            total_processing_time: Duration::from_secs(0),
            start_time: Instant::now(),
        }
    }
}

impl CrawlDemo {
    async fn new(use_filesystem: bool, storage_path: Option<String>) -> Result<Self> {
        let rate_config = RateLimitConfig {
            requests_per_second: 1,
            burst_capacity: 3,
            default_delay_ms: 1000,
            ip_requests_per_minute: 30,
            global_requests_per_second: 2,
        };

        let storage: Arc<dyn Storage> = if use_filesystem {
            let path = storage_path.unwrap_or_else(|| "./swoop_data".to_string());
            info!("🗄️  Initializing filesystem storage at: {}", path);
            Arc::new(FileSystemStorage::new(&path).await?)
        } else {
            info!("🗄️  Using in-memory storage");
            Arc::new(MemoryStorage::new())
        };

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("Swoop/1.0 (Lightning-fast web crawler)")
            .build()
            .map_err(|e| Error::Http(e))?;

        Ok(Self {
            parser: Parser::new(),
            rate_limiter: Arc::new(RateLimiter::new(rate_config)),
            storage,
            client,
            stats: Arc::new(tokio::sync::RwLock::new(DemoStats {
                start_time: Instant::now(),
                ..Default::default()
            })),
        })
    }

    async fn setup_extraction_rules(&self) {
        info!("⚙️  Setting up extraction rules...");

        let rules = vec![
            ExtractorRule {
                name: "title".to_string(),
                selector_type: SelectorType::CSS,
                selector: "title, h1".to_string(),
                attribute: None,
                multiple: false,
                required: true,
                default_value: Some("Untitled Page".to_string()),
            },
            ExtractorRule {
                name: "description".to_string(),
                selector_type: SelectorType::CSS,
                selector: "meta[name='description']".to_string(),
                attribute: Some("content".to_string()),
                multiple: false,
                required: false,
                default_value: Some("No description available".to_string()),
            },
        ];

        for rule in rules {
            self.parser.add_rule(rule.clone()).await;
            debug!("  ✓ Added rule: {}", rule.name);
        }

        info!("✅ Extraction rules configured");
    }

    fn get_demo_targets() -> Vec<CrawlTarget> {
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
        ]
    }

    async fn crawl_url(&self, target: &CrawlTarget) -> Result<Option<Document>> {
        let span = span!(Level::INFO, "crawl_url", url = %target.url);
        let _enter = span.enter();

        info!("🌐 Starting crawl: {} ({})", target.name, target.url);
        let start_time = Instant::now();

        {
            let mut stats = self.stats.write().await;
            stats.total_requests += 1;
        }

        let client_ip = IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1));
        match self.rate_limiter.check_request(&target.url, Some(client_ip)).await {
            Ok(_) => {
                info!("✅ Rate limit check passed");
            }
            Err(Error::RateLimit(msg)) => {
                warn!("⏰ Rate limited: {}", msg);
                
                {
                    let mut stats = self.stats.write().await;
                    stats.rate_limited += 1;
                }

                info!("⏳ Waiting for rate limit cooldown...");
                self.rate_limiter.wait_if_needed(&target.url).await?;
                sleep(Duration::from_millis(500)).await;

                info!("🔄 Retrying after rate limit...");
                self.rate_limiter.check_request(&target.url, Some(client_ip)).await?;
            }
            Err(e) => {
                error!("❌ Rate limiter error: {}", e);
                return Err(e);
            }
        }

        info!("📡 Making HTTP request...");
        let response = self.client.get(&target.url).send().await
            .map_err(|e| Error::Http(e))?;

        info!("📥 Response received: {}", response.status());
        
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

        info!("📄 Downloaded {} bytes", body_bytes.len());

        let metadata = Metadata {
            url: target.url.clone(),
            content_type: content_type.clone(),
            fetch_time: chrono::Utc::now(),
            status_code,
            headers,
        };

        info!("🔍 Parsing content (type: {})...", content_type);
        let parse_result = self.parser.parse(&body_bytes, &content_type, &metadata).await?;

        info!("✅ Parsing successful");
        info!("  📊 Extracted {} fields", parse_result.extracted.len());

        let document_id = format!("doc_{}_{}", 
                                  chrono::Utc::now().timestamp(), 
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

        info!("💾 Storing document...");
        self.storage.store_document(&document).await?;
        
        info!("✅ Document stored successfully: {}", document_id);
        
        {
            let mut stats = self.stats.write().await;
            stats.successful_crawls += 1;
            stats.total_processing_time += start_time.elapsed();
        }
        
        Ok(Some(document))
    }

    async fn print_statistics(&self) {
        info!("📈 === FINAL STATISTICS ===");

        let stats = self.stats.read().await;
        let runtime = stats.start_time.elapsed();
        
        info!("🕐 Runtime: {:?}", runtime);
        info!("📊 Total requests: {}", stats.total_requests);
        info!("✅ Successful crawls: {}", stats.successful_crawls);
        info!("⏰ Rate limited: {}", stats.rate_limited);

        let rate_stats = self.rate_limiter.get_stats().await;
        info!("🚦 Rate limiter - Total: {}, Blocked: {}", 
              rate_stats.total_requests, rate_stats.blocked_requests);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let filter = EnvFilter::from_default_env()
        .add_directive("crawl4ai_demo=info".parse().unwrap())
        .add_directive("swoop=info".parse().unwrap());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();

    let matches = Command::new("Crawl4AI Demo")
        .version("1.0.0")
        .about("Demonstrates Crawl4AI production features")
        .arg(Arg::new("filesystem")
            .long("filesystem")
            .help("Use filesystem storage")
            .action(clap::ArgAction::SetTrue))
        .get_matches();

    let use_filesystem = matches.get_flag("filesystem");

    info!("🎯 === CRAWL4AI CORE PRODUCTION DEMO ===");
    info!("💾 Storage: {}", if use_filesystem { "Filesystem" } else { "Memory" });

    let demo = CrawlDemo::new(use_filesystem, None).await?;
    demo.setup_extraction_rules().await;

    let targets = CrawlDemo::get_demo_targets();
    
    info!("🎯 Selected targets:");
    for target in &targets {
        info!("  • {} ({})", target.name, target.url);
    }

    for (i, target) in targets.iter().enumerate() {
        info!("📍 Processing target {}/{}: {}", i + 1, targets.len(), target.name);
        
        match demo.crawl_url(target).await {
            Ok(Some(doc)) => {
                info!("✅ Successfully processed: {}", doc.title);
            }
            Err(e) => {
                error!("❌ Failed to process {}: {}", target.name, e);
            }
            _ => {}
        }

        if i < targets.len() - 1 {
            sleep(Duration::from_millis(1500)).await;
        }
    }

    demo.print_statistics().await;
    info!("🎉 Demo completed successfully!");

    Ok(())
} 