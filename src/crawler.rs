/*!
 * Crawler module for Crawl4AI
 * 
 * This module provides the core crawling functionality with multi-threaded processing,
 * rate limiting, robots.txt compliance, and configurable crawl strategies.
 */

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Context;
use chrono::{DateTime, Utc};
use futures::stream::{FuturesUnordered, StreamExt};
use log::{debug, error, info, warn};
use parking_lot::{Mutex, RwLock};
use regex::Regex;
use reqwest::{Client, ClientBuilder, Proxy};
use tokio::sync::Semaphore;
use tokio::time::sleep;
use url::Url;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::{Document, Link, Metadata, CrawlJobConfig as CrawlConfig};
use crate::storage::Storage;

/// Stub Parser implementation for compilation
#[derive(Debug, Default)]
pub struct Parser;

#[derive(Debug)]
pub struct ParseResult {
    pub title: String,
    pub content: String,
    pub html: String,
    pub text: String,
    pub extracted: Vec<String>,
}

impl Parser {
    pub async fn parse(&self, _url: &str, _content: &str) -> Result<ParseResult> {
        Ok(ParseResult {
            title: "Untitled".to_string(),
            content: "Content".to_string(),
            html: "".to_string(),
            text: "".to_string(),
            extracted: vec![],
        })
    }
}

/// Stub RobotsCache implementation for compilation
#[derive(Debug)]
pub struct RobotsCache {
    client: Client,
}

impl RobotsCache {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
    
    pub async fn can_fetch(&self, _url: &str, _user_agent: &str) -> bool {
        true // Allow all for now
    }
}

/// Statistics for a crawl job
#[derive(Debug, Clone)]
pub struct CrawlStats {
    /// Unique identifier for the crawl job
    pub job_id: String,
    /// Time when the crawl started
    pub start_time: DateTime<Utc>,
    /// Time when the crawl ended (if completed)
    pub end_time: Option<DateTime<Utc>>,
    /// Number of URLs processed
    pub urls_processed: usize,
    /// Number of successful fetches
    pub successful_fetches: usize,
    /// Number of failed fetches
    pub failed_fetches: usize,
    /// Number of bytes downloaded
    pub bytes_downloaded: usize,
    /// Number of documents extracted
    pub documents_extracted: usize,
    /// Number of links discovered
    pub links_discovered: usize,
    /// Average fetch time in milliseconds
    pub avg_fetch_time_ms: f64,
    /// Status of the crawl job
    pub status: String,
}

impl CrawlStats {
    /// Create a new CrawlStats instance
    fn new(job_id: String) -> Self {
        Self {
            job_id,
            start_time: Utc::now(),
            end_time: None,
            urls_processed: 0,
            successful_fetches: 0,
            failed_fetches: 0,
            bytes_downloaded: 0,
            documents_extracted: 0,
            links_discovered: 0,
            avg_fetch_time_ms: 0.0,
            status: "running".to_string(),
        }
    }
    
    /// Mark the crawl job as completed
    fn mark_completed(&mut self) {
        self.end_time = Some(Utc::now());
        self.status = "completed".to_string();
    }
    
    /// Mark the crawl job as failed
    fn mark_failed(&mut self, reason: &str) {
        self.end_time = Some(Utc::now());
        self.status = format!("failed: {}", reason);
    }
    
    /// Update fetch statistics
    fn update_fetch_stats(&mut self, success: bool, bytes: usize, time_ms: f64) {
        self.urls_processed += 1;
        
        if success {
            self.successful_fetches += 1;
            self.bytes_downloaded += bytes;
            
            // Update average fetch time
            let total_time = self.avg_fetch_time_ms * (self.successful_fetches - 1) as f64;
            self.avg_fetch_time_ms = (total_time + time_ms) / self.successful_fetches as f64;
        } else {
            self.failed_fetches += 1;
        }
    }
    
    /// Update document extraction statistics
    fn update_extraction_stats(&mut self, docs: usize, links: usize) {
        self.documents_extracted += docs;
        self.links_discovered += links;
    }
}

/// A crawl job represents a single crawling task
pub struct CrawlJob {
    /// Unique identifier for the crawl job
    pub id: String,
    /// Configuration for the crawl job
    pub config: CrawlConfig,
    /// Starting URLs for the crawl
    pub seeds: Vec<String>,
    /// Statistics for the crawl job
    pub stats: Arc<RwLock<CrawlStats>>,
    /// Queue of URLs to crawl
    queue: Arc<Mutex<VecDeque<String>>>,
    /// Set of visited URLs
    visited: Arc<RwLock<HashSet<String>>>,
    /// HTTP client for making requests
    client: Client,
    /// Robots.txt cache
    robots: Arc<RobotsCache>,
    /// Parser for extracting content
    parser: Arc<Parser>,
    /// Storage for saving results
    storage: Arc<dyn Storage>,
    /// Semaphore for limiting concurrent requests
    semaphore: Arc<Semaphore>,
}

impl std::fmt::Debug for CrawlJob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CrawlJob")
            .field("id", &self.id)
            .field("config", &self.config)
            .field("seeds", &self.seeds)
            .field("stats", &self.stats)
            .field("queue", &"<Queue>")
            .field("visited", &"<Visited>")
            .field("client", &"<Client>")
            .field("robots", &"<RobotsCache>")
            .field("parser", &"<Parser>")
            .field("storage", &"<dyn Storage>")
            .field("semaphore", &"<Semaphore>")
            .finish()
    }
}

impl CrawlJob {
    /// Create a new CrawlJob
    fn new(
        config: CrawlConfig,
        seeds: Vec<String>,
        parser: Arc<Parser>,
        storage: Arc<dyn Storage>,
    ) -> Result<Self> {
        let id = Uuid::new_v4().to_string();
        let stats = Arc::new(RwLock::new(CrawlStats::new(id.clone())));
        
        // Build HTTP client with appropriate configuration
        let mut client_builder = ClientBuilder::new()
            .timeout(Duration::from_secs(30)) // Default timeout
            .connect_timeout(Duration::from_secs(10)) // Default connect timeout
            .user_agent(&config.user_agent)
            .redirect(reqwest::redirect::Policy::limited(10)); // Default max redirects
        
        let client = client_builder.build().context("Failed to build HTTP client")?;
        
        // Initialize robots.txt cache
        let robots = Arc::new(RobotsCache::new(client.clone()));
        
        // Initialize URL queue and visited set
        let queue = Arc::new(Mutex::new(VecDeque::new()));
        let visited = Arc::new(RwLock::new(HashSet::new()));
        
        // Add seed URLs to queue
        {
            let mut queue_guard = queue.lock();
            for seed in &seeds {
                queue_guard.push_back(seed.clone());
            }
        }
        
        // Create semaphore for concurrency control
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent_requests));
        
        Ok(Self {
            id,
            config,
            seeds,
            stats,
            queue,
            visited,
            client,
            robots,
            parser,
            storage,
            semaphore,
        })
    }
    
    /// Run the crawl job
    pub async fn run(&self) -> Result<CrawlStats> {
        info!("Starting crawl job {}", self.id);
        
        // Process URLs until queue is empty or max URLs reached
        let mut tasks = FuturesUnordered::new();
        let mut processed_count = 0;
        
        loop {
            // Check if we've reached the maximum number of URLs
            if self.config.max_urls > 0 && processed_count >= self.config.max_urls {
                info!("Reached maximum number of URLs ({})", self.config.max_urls);
                break;
            }
            
            // Get next URL from queue
            let url_option = {
                let mut queue = self.queue.lock();
                queue.pop_front()
            };
            
            if let Some(url) = url_option {
                // Check if URL has already been visited
                let is_visited = {
                    let visited = self.visited.read();
                    visited.contains(&url)
                };
                
                if !is_visited {
                    // Mark URL as visited
                    {
                        let mut visited = self.visited.write();
                        visited.insert(url.clone());
                    }
                    
                    // Acquire semaphore permit
                    let permit = self.semaphore.clone().acquire_owned().await?;
                    
                    // Clone necessary references for the task
                    let client = self.client.clone();
                    let robots = self.robots.clone();
                    let parser = self.parser.clone();
                    let storage = self.storage.clone();
                    let queue = self.queue.clone();
                    let stats = self.stats.clone();
                    let config = self.config.clone();
                    
                    // Spawn task to process URL
                    tasks.push(tokio::spawn(async move {
                        let result = Self::process_url(
                            &url,
                            client,
                            robots,
                            parser,
                            storage,
                            queue,
                            stats,
                            &config,
                        ).await;
                        
                        // Release semaphore permit
                        drop(permit);
                        
                        result
                    }));
                    
                    processed_count += 1;
                }
            } else if tasks.is_empty() {
                // No more URLs in queue and no running tasks
                break;
            }
            
            // Wait for a task to complete or for new URLs
            if !tasks.is_empty() {
                match tasks.next().await {
                    Some(result) => {
                        if let Err(e) = result {
                            error!("Task panicked: {}", e);
                        }
                    }
                    None => break,
                }
            } else {
                // No tasks running, wait a bit for new URLs
                sleep(Duration::from_millis(100)).await;
            }
        }
        
        // Wait for all remaining tasks to complete
        while let Some(result) = tasks.next().await {
            if let Err(e) = result {
                error!("Task panicked: {}", e);
            }
        }
        
        // Mark crawl as completed
        {
            let mut stats = self.stats.write();
            stats.mark_completed();
        }
        
        // Return final stats
        Ok(self.stats.read().clone())
    }
    
    /// Process a single URL
    async fn process_url(
        url: &str,
        client: Client,
        robots: Arc<RobotsCache>,
        parser: Arc<Parser>,
        storage: Arc<dyn Storage>,
        queue: Arc<Mutex<VecDeque<String>>>,
        stats: Arc<RwLock<CrawlStats>>,
        config: &CrawlConfig,
    ) -> Result<()> {
        debug!("Processing URL: {}", url);
        
        // Parse URL
        let parsed_url = match Url::parse(url) {
            Ok(parsed) => parsed,
            Err(e) => {
                warn!("Failed to parse URL {}: {}", url, e);
                return Ok(());
            }
        };
        
        // Check robots.txt
        if config.respect_robots_txt {
            if !robots.is_allowed(url, &config.user_agent).await {
                debug!("URL disallowed by robots.txt: {}", url);
                return Ok(());
            }
        }
        
        // Fetch URL
        let start_time = Instant::now();
        let response = match client.get(url).send().await {
            Ok(resp) => resp,
            Err(e) => {
                warn!("Failed to fetch URL {}: {}", url, e);
                
                // Update stats
                let mut stats = stats.write();
                stats.update_fetch_stats(false, 0, 0.0);
                
                return Ok(());
            }
        };
        
        // Check status code
        if !response.status().is_success() {
            warn!("Non-success status code for URL {}: {}", url, response.status());
            
            // Update stats
            let mut stats = stats.write();
            stats.update_fetch_stats(false, 0, 0.0);
            
            return Ok(());
        }
        
        // Get content type and headers before consuming response
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("text/html")
            .to_string();
        
        let status_code = response.status().as_u16();
        let headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();
        
        // Get response body
        let body = match response.bytes().await {
            Ok(bytes) => bytes,
            Err(e) => {
                warn!("Failed to get response body for URL {}: {}", url, e);
                
                // Update stats
                let mut stats = stats.write();
                stats.update_fetch_stats(false, 0, 0.0);
                
                return Ok(());
            }
        };
        
        // Calculate fetch time
        let fetch_time_ms = start_time.elapsed().as_millis() as f64;
        
        // Update stats
        {
            let mut stats = stats.write();
            stats.update_fetch_stats(true, body.len(), fetch_time_ms);
        }
        
        // Create metadata
        let metadata = Metadata {
            url: url.to_string(),
            content_type: content_type.clone(),
            fetch_time: Utc::now(),
            status_code,
            headers,
        };
        
        // Parse content
        let parse_result = parser.parse(&body, &content_type, &metadata).await?;
        
        // Extract links if crawling is enabled
        let mut links = Vec::new();
        if config.follow_links && parse_result.links.len() > 0 {
            for link in &parse_result.links {
                // Normalize link
                let normalized_link = match Url::parse(&link.url) {
                    Ok(parsed) => parsed.to_string(),
                    Err(_) => {
                        // Relative URL, resolve against base URL
                        match parsed_url.join(&link.url) {
                            Ok(resolved) => resolved.to_string(),
                            Err(e) => {
                                debug!("Failed to resolve relative URL {}: {}", link.url, e);
                                continue;
                            }
                        }
                    }
                };
                
                // Apply URL filters
                if Self::should_follow_link(&normalized_link, config) {
                    links.push(Link {
                        url: normalized_link.clone(),
                        text: link.text.clone(),
                        source_url: url.to_string(),
                        rel: link.rel.clone(),
                    });
                    
                    // Add to queue if not visited
                    let is_visited = {
                        let visited = stats.read().urls_processed;
                        visited >= config.max_urls && config.max_urls > 0
                    };
                    
                    if !is_visited {
                        let mut queue = queue.lock();
                        queue.push_back(normalized_link);
                    }
                }
            }
        }
        
        // Create document
        let document = Document {
            id: format!("doc_{}", Uuid::new_v4().to_string().replace('-', "")[..8].to_string()),
            title: parse_result.title,
            content: parse_result.content,
            summary: None,
            metadata,
            quality_score: None,
            content_hash: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            source_url: Some(url.to_string()),
            document_type: Some("html".to_string()),
            language: None,
            word_count: Some(parse_result.content.split_whitespace().count()),
            size_bytes: Some(parse_result.content.len() as u64),
            content_type: Some("text/html".to_string()),
            file_size: Some(parse_result.content.len() as u64),
            extracted_at: chrono::Utc::now(),
        };
        
        // Store document
        storage.store_document(&document).await?;
        
        // Update stats
        {
            let mut stats = stats.write();
            stats.update_extraction_stats(1, document.links.len());
        }
        
        // Respect politeness delay
        if config.politeness_delay > 0 {
            sleep(Duration::from_millis(config.politeness_delay)).await;
        }
        
        Ok(())
    }
    
    /// Check if a link should be followed based on configuration
    fn should_follow_link(url: &str, config: &CrawlConfig) -> bool {
        // Parse URL
        let parsed_url = match Url::parse(url) {
            Ok(parsed) => parsed,
            Err(_) => return false,
        };
        
        // Check domain restrictions
        if !config.allowed_domains.is_empty() {
            let domain = parsed_url.host_str().unwrap_or("");
            if !config.allowed_domains.iter().any(|d| domain.ends_with(d)) {
                return false;
            }
        }
        
        // Check URL patterns
        if !config.url_patterns.is_empty() {
            let matches_pattern = config.url_patterns.iter().any(|pattern| {
                let regex = match Regex::new(pattern) {
                    Ok(re) => re,
                    Err(_) => return false,
                };
                regex.is_match(url)
            });
            
            if !matches_pattern {
                return false;
            }
        }
        
        // Check URL exclusions
        if !config.exclude_patterns.is_empty() {
            let matches_exclusion = config.exclude_patterns.iter().any(|pattern| {
                let regex = match Regex::new(pattern) {
                    Ok(re) => re,
                    Err(_) => return false,
                };
                regex.is_match(url)
            });
            
            if matches_exclusion {
                return false;
            }
        }
        
        // Check max depth
        if config.max_depth > 0 {
            // TODO: Implement depth tracking
        }
        
        true
    }
}

/// Builder for creating a Crawler
pub struct CrawlerBuilder {
    config: CrawlConfig,
    parser: Option<Arc<Parser>>,
    storage: Option<Arc<dyn Storage>>,
}

impl std::fmt::Debug for CrawlerBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CrawlerBuilder")
            .field("config", &self.config)
            .field("parser", &self.parser.as_ref().map(|_| "<Parser>"))
            .field("storage", &self.storage.as_ref().map(|_| "<dyn Storage>"))
            .finish()
    }
}

impl CrawlerBuilder {
    /// Create a new CrawlerBuilder with default configuration
    pub fn new() -> Self {
        Self {
            config: CrawlConfig::default(),
            parser: None,
            storage: None,
        }
    }
    
    /// Set the configuration
    pub fn with_config(mut self, config: CrawlConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Set the parser
    pub fn with_parser(mut self, parser: Arc<Parser>) -> Self {
        self.parser = Some(parser);
        self
    }
    
    /// Set the storage
    pub fn with_storage(mut self, storage: Arc<dyn Storage>) -> Self {
        self.storage = Some(storage);
        self
    }
    
    /// Build the Crawler
    pub fn build(self) -> Result<Crawler> {
        let parser = self.parser.unwrap_or_else(|| Arc::new(Parser::default()));
        let storage = self.storage.ok_or_else(|| Error::Configuration("Storage not provided".to_string()))?;
        
        Ok(Crawler {
            config: self.config,
            parser,
            storage,
            jobs: Arc::new(RwLock::new(HashMap::new())),
        })
    }
}

/// The main Crawler struct
pub struct Crawler {
    /// Configuration for the crawler
    pub config: CrawlConfig,
    /// Parser for extracting content
    parser: Arc<Parser>,
    /// Storage for saving results
    storage: Arc<dyn Storage>,
    /// Active crawl jobs
    jobs: Arc<RwLock<HashMap<String, Arc<CrawlJob>>>>,
}

impl std::fmt::Debug for Crawler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Crawler")
            .field("config", &self.config)
            .field("parser", &"<Parser>")
            .field("storage", &"<dyn Storage>")
            .field("jobs", &self.jobs)
            .finish()
    }
}

impl Crawler {
    /// Create a new Crawler with default configuration
    pub fn new(storage: Arc<dyn Storage>) -> Self {
        Self {
            config: CrawlConfig::default(),
            parser: Arc::new(Parser::default()),
            storage,
            jobs: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Start a new crawl job
    pub async fn start_crawl(&self, seeds: Vec<String>, config: Option<CrawlConfig>) -> Result<String> {
        let config = config.unwrap_or_else(|| self.config.clone());
        
        // Create crawl job
        let job = CrawlJob::new(
            config,
            seeds,
            self.parser.clone(),
            self.storage.clone(),
        )?;
        
        let job_id = job.id.clone();
        
        // Store job
        {
            let mut jobs = self.jobs.write();
            jobs.insert(job_id.clone(), Arc::new(job));
        }
        
        // Start job in background
        let jobs_clone = self.jobs.clone();
        let job_id_clone = job_id.clone();
        
        tokio::spawn(async move {
            let job = {
                let jobs = jobs_clone.read();
                jobs.get(&job_id_clone).cloned()
            };
            
            if let Some(job) = job {
                if let Err(e) = job.run().await {
                    error!("Crawl job {} failed: {}", job_id_clone, e);
                    
                    // Update stats
                    let mut stats = job.stats.write();
                    stats.mark_failed(&e.to_string());
                }
            }
        });
        
        Ok(job_id)
    }
    
    /// Get the status of a crawl job
    pub fn get_job_status(&self, job_id: &str) -> Option<CrawlStats> {
        let jobs = self.jobs.read();
        jobs.get(job_id).map(|job| job.stats.read().clone())
    }
    
    /// Stop a crawl job
    pub fn stop_job(&self, job_id: &str) -> bool {
        let mut jobs = self.jobs.write();
        jobs.remove(job_id).is_some()
    }
    
    /// Get all active job IDs
    pub fn get_active_jobs(&self) -> Vec<String> {
        let jobs = self.jobs.read();
        jobs.keys().cloned().collect()
    }
}

impl Default for Crawler {
    fn default() -> Self {
        unimplemented!("Crawler requires a storage implementation")
    }
}
