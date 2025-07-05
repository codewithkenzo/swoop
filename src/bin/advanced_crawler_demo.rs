/*!
 * Advanced Crawler Demo for Swoop
 * 
 * Demonstrates the enhanced crawler with recursive sublink extraction,
 * intelligent link classification, and comprehensive site mapping.
 */

use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;

use log::{info, warn};
use tokio::time::sleep;
use tracing::{span, Level};

use swoop::{
    error::Result,
    crawler::{LinkExtractor, ExtractedLink, LinkType, CrawlQueue},
    storage::memory::MemoryStorage,
};

#[derive(Debug)]
struct CrawlResult {
    url: String,
    title: String,
    links_found: usize,
    depth: usize,
    link_type: LinkType,
    parent_url: String,
    processing_time_ms: u64,
}

#[derive(Debug)]
struct AdvancedCrawlerDemo {
    storage: Arc<MemoryStorage>,
    link_extractor: LinkExtractor,
    crawl_queue: CrawlQueue,
    results: Vec<CrawlResult>,
    client: reqwest::Client,
}

impl AdvancedCrawlerDemo {
    /// Initialize the advanced crawler demo
    pub async fn new(base_url: &str, max_depth: usize) -> Result<Self> {
        info!("🚀 Initializing Advanced Crawler Demo");
        info!("   Base URL: {base_url}");
        info!("   Max Depth: {max_depth}");
        
        let storage = Arc::new(MemoryStorage::new());
        
        // Create link extractor with intelligent domain handling
        let allowed_domains = vec![
            "example.com".to_string(),
            "docs.rs".to_string(),
            "github.com".to_string(),
        ];
        
        let link_extractor = LinkExtractor::new(base_url, allowed_domains)?;
        let crawl_queue = CrawlQueue::new(max_depth);
        
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Swoop-AdvancedCrawler/2.0")
            .build()?;
        
        Ok(Self {
            storage,
            link_extractor,
            crawl_queue,
            results: Vec::new(),
            client,
        })
    }
    
    /// Run the comprehensive crawler demonstration
    pub async fn run_demo(&mut self, start_urls: Vec<String>) -> Result<()> {
        let demo_start = Instant::now();
        
        info!("🎯 Starting Advanced Crawler Demonstration");
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        // Initialize queue with starting URLs
        let initial_links: Vec<ExtractedLink> = start_urls.into_iter().map(|url| {
            ExtractedLink {
                url: url.clone(),
                text: "Starting URL".to_string(),
                link_type: LinkType::SameDomain,
                depth: 0,
                parent_url: "root".to_string(),
            }
        }).collect();
        
        self.crawl_queue.add_links(initial_links);
        
        info!("📋 Phase 1: Recursive Link Discovery & Extraction");
        self.recursive_crawl().await?;
        
        info!("📋 Phase 2: Link Analysis & Classification");
        self.analyze_crawl_results();
        
        info!("📋 Phase 3: Site Structure Mapping");
        self.generate_site_map();
        
        let total_time = demo_start.elapsed();
        
        info!("📈 ═══════════════════════════════════════════════");
        info!("📈 ADVANCED CRAWLER DEMO - FINAL REPORT");
        info!("📈 ═══════════════════════════════════════════════");
        info!("⏱️  Total execution time: {:.2}s", total_time.as_secs_f64());
        info!("🔗 Total URLs crawled: {}", self.results.len());
        info!("📊 Average processing time: {:.2}ms", 
              self.results.iter().map(|r| r.processing_time_ms).sum::<u64>() as f64 / self.results.len() as f64);
        info!("🌐 Site structure mapped successfully");
        info!("🏆 Status: Advanced crawling completed");
        info!("📈 ═══════════════════════════════════════════════");
        
        Ok(())
    }
    
    /// Perform recursive crawling with sublink extraction
    async fn recursive_crawl(&mut self) -> Result<()> {
        let mut crawl_count = 0;
        let max_crawls = 10; // Limit for demo purposes
        
        while !self.crawl_queue.is_empty() && crawl_count < max_crawls {
            if let Some(link) = self.crawl_queue.pop() {
                crawl_count += 1;
                
                let crawl_span = span!(Level::INFO, "crawl_page", url = %link.url, depth = link.depth);
                let _enter = crawl_span.enter();
                
                info!("🌐 Crawling [{}]: {} (depth: {})", crawl_count, link.url, link.depth);
                
                match self.crawl_page(link).await {
                    Ok(result) => {
                        info!("   ✅ Success: {} links found in {:.2}ms", 
                              result.links_found, result.processing_time_ms);
                        self.results.push(result);
                    }
                    Err(e) => {
                        warn!("   ⚠️  Failed to crawl: {e}");
                    }
                }
                
                // Polite delay between requests
                sleep(Duration::from_millis(500)).await;
            }
        }
        
        info!("🎉 Recursive crawling completed: {crawl_count} pages processed");
        Ok(())
    }
    
    /// Crawl a single page and extract sublinks
    async fn crawl_page(&mut self, link: ExtractedLink) -> Result<CrawlResult> {
        let start_time = Instant::now();
        
        // Fetch the page content
        let response = self.client.get(&link.url).send().await?;
        let html = response.text().await?;
        
        // Extract title (simple extraction for demo)
        let title = self.extract_title(&html);
        
        // Extract all sublinks from the page
        let extracted_links = self.link_extractor.extract_links(&html, &link.url, link.depth);
        
        // Add new links to the crawl queue
        self.crawl_queue.add_links(extracted_links.clone());
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        Ok(CrawlResult {
            url: link.url,
            title,
            links_found: extracted_links.len(),
            depth: link.depth,
            link_type: link.link_type,
            parent_url: link.parent_url,
            processing_time_ms: processing_time,
        })
    }
    
    /// Simple title extraction from HTML
    fn extract_title(&self, html: &str) -> String {
        use scraper::{Html, Selector};
        
        let document = Html::parse_document(html);
        let title_selector = Selector::parse("title").unwrap();
        
        if let Some(title_element) = document.select(&title_selector).next() {
            title_element.text().collect::<Vec<_>>().join(" ").trim().to_string()
        } else {
            "Untitled".to_string()
        }
    }
    
    /// Analyze crawl results and provide insights
    fn analyze_crawl_results(&self) {
        let mut link_type_counts: HashMap<String, usize> = HashMap::new();
        let mut depth_counts: HashMap<usize, usize> = HashMap::new();
        
        for result in &self.results {
            let link_type_str = format!("{:?}", result.link_type);
            *link_type_counts.entry(link_type_str).or_insert(0) += 1;
            *depth_counts.entry(result.depth).or_insert(0) += 1;
        }
        
        info!("📊 Link Type Distribution:");
        for (link_type, count) in &link_type_counts {
            info!("   {link_type}: {count} pages");
        }
        
        info!("📊 Depth Distribution:");
        for (depth, count) in &depth_counts {
            info!("   Depth {depth}: {count} pages");
        }
        
        let total_links_found: usize = self.results.iter().map(|r| r.links_found).sum();
        info!("📊 Total sublinks discovered: {total_links_found}");
    }
    
    /// Generate a visual site map representation
    fn generate_site_map(&self) {
        info!("🗺️  Site Structure Map:");
        
        // Group by depth for hierarchical display
        let mut depth_groups: HashMap<usize, Vec<&CrawlResult>> = HashMap::new();
        for result in &self.results {
            depth_groups.entry(result.depth).or_default().push(result);
        }
        
        for depth in 0..=3 {
            if let Some(pages) = depth_groups.get(&depth) {
                let indent = "  ".repeat(depth);
                info!("{}📁 Depth {} ({} pages):", indent, depth, pages.len());
                
                for (i, page) in pages.iter().take(3).enumerate() { // Show first 3 for brevity
                    info!("{}  ├─ {} [{}ms] ({})", 
                          indent, 
                          page.title.chars().take(40).collect::<String>(),
                          page.processing_time_ms,
                          page.links_found);
                }
                
                if pages.len() > 3 {
                    info!("{}  └─ ... and {} more", indent, pages.len() - 3);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    info!("🚀 Swoop Advanced Crawler Demo v2.0");
    info!("⚡ Featuring: Recursive Sublink Extraction, Intelligent Classification, Site Mapping");
    
    // Demo URLs for testing (using safe, well-behaved sites)
    let demo_urls = vec![
        "https://httpbin.org/".to_string(),
        "https://httpbin.org/html".to_string(),
    ];
    
    // Create and run the advanced crawler demo
    let mut demo = AdvancedCrawlerDemo::new("httpbin.org", 2).await?;
    demo.run_demo(demo_urls).await?;
    
    info!("✨ Advanced Crawler Demo completed successfully!");
    info!("🔧 Features demonstrated:");
    info!("   • Recursive sublink extraction");
    info!("   • Intelligent link classification");
    info!("   • Site structure mapping");
    info!("   • Polite crawling with delays");
    info!("   • Comprehensive analytics");
    
    Ok(())
} 