use clap::{Arg, Command};
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::sync::Semaphore;
use tracing::{error, info, warn};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// HTTP fetch function with retry logic and connection pooling
async fn fetch_url_simple(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    info!("Fetching URL: {}", url);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .pool_idle_timeout(Duration::from_secs(30))
        .tcp_keepalive(Duration::from_secs(60))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()?;
    
    // Retry logic - 2 attempts with short delay
    for attempt in 1..=2 {
        match client.get(url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.bytes().await {
                        Ok(bytes) => {
                            info!("Finished fetching URL: {} (attempt {})", url, attempt);
                            return Ok(bytes.to_vec());
                        }
                        Err(e) => {
                            if attempt == 2 {
                                return Err(format!("Failed to read response body: {}", e).into());
                            }
                            tokio::time::sleep(Duration::from_millis(200)).await;
                        }
                    }
                } else {
                    if attempt == 2 {
                        return Err(format!("HTTP {}", response.status()).into());
                    }
                    tokio::time::sleep(Duration::from_millis(200)).await;
                }
            }
            Err(e) => {
                if attempt == 2 {
                    return Err(e.into());
                }
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
        }
    }
    
    Err("All retry attempts failed".into())
}

/// Scraped data entry
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScrapedData {
    url: String,
    timestamp: DateTime<Utc>,
    content: String,
    status_code: Option<u16>,
    headers: HashMap<String, String>,
    response_time: u64,
    content_length: usize,
    content_type: Option<String>,
    title: Option<String>,
    success: bool,
    error: Option<String>,
}

/// CLI scraper state
#[derive(Debug)]
struct CliScraper {
    concurrency: usize,
    output_dir: PathBuf,
    scraped_data: Arc<Mutex<Vec<ScrapedData>>>,
}

impl CliScraper {
    fn new(concurrency: usize, output_dir: PathBuf) -> Self {
        fs::create_dir_all(&output_dir).expect("Failed to create output directory");
        Self {
            concurrency,
            output_dir,
            scraped_data: Arc::new(Mutex::new(Vec::new())),
        }
    }


    async fn scrape_urls(&self, urls: Vec<String>) {
        let semaphore = Arc::new(Semaphore::new(self.concurrency));
        let mut handles = Vec::new();

        info!("🚀 Starting to scrape {} URLs with concurrency {}", urls.len(), self.concurrency);

        for url in urls {
            let semaphore = semaphore.clone();
            let scraped_data = self.scraped_data.clone();
            let url_clone = url.clone();

            let handle = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                let result = Self::scrape_url_static(&url_clone).await;
                scraped_data.lock().unwrap().push(result);
            });

            handles.push(handle);
        }

        // Wait for all tasks to complete
        for handle in handles {
            handle.await.unwrap();
        }

        info!("✅ Completed scraping all URLs");
    }

    async fn scrape_url_static(url: &str) -> ScrapedData {
        let start_time = Instant::now();
        match fetch_url_simple(url).await {
            Ok(data) => {
                let duration = start_time.elapsed();
                let content = String::from_utf8_lossy(&data).to_string();
                info!("✅ Successfully scraped: {}", url);
                ScrapedData {
                    url: url.to_string(),
                    timestamp: Utc::now(),
                    content,
                    status_code: Some(200),
                    headers: HashMap::new(),
                    response_time: duration.as_millis() as u64,
                    content_length: data.len(),
                    content_type: Some("text/html".to_string()),
                    title: None,
                    success: true,
                    error: None,
                }
            }
            Err(e) => {
                error!("❌ Failed to scrape {}: {}", url, e);
                ScrapedData {
                    url: url.to_string(),
                    timestamp: Utc::now(),
                    content: String::new(),
                    status_code: None,
                    headers: HashMap::new(),
                    response_time: start_time.elapsed().as_millis() as u64,
                    content_length: 0,
                    content_type: None,
                    title: None,
                    success: false,
                    error: Some(e.to_string()),
                }
            }
        }
    }

    fn export_results(&self, format: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = self.scraped_data.lock().unwrap();
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        
        match format {
            "json" => {
                let file_path = self.output_dir.join(format!("scraped_data_{}.json", timestamp));
                let json_data = serde_json::to_string_pretty(&*data)?;
                fs::write(&file_path, json_data)?;
                info!("📄 Exported {} entries to {}", data.len(), file_path.display());
            }
            "csv" => {
                let file_path = self.output_dir.join(format!("scraped_data_{}.csv", timestamp));
                let mut csv_content = "URL,Timestamp,Status Code,Success,Response Time,Content Length,Title,Error\n".to_string();
                for item in data.iter() {
                    csv_content.push_str(&format!(
                        "{},{},{},{},{},{},{},{}\n",
                        item.url,
                        item.timestamp.format("%Y-%m-%d %H:%M:%S"),
                        item.status_code.unwrap_or(0),
                        item.success,
                        item.response_time,
                        item.content_length,
                        item.title.as_deref().unwrap_or(""),
                        item.error.as_deref().unwrap_or("")
                    ));
                }
                fs::write(&file_path, csv_content)?;
                info!("📄 Exported {} entries to {}", data.len(), file_path.display());
            }
            _ => {
                return Err("Unsupported format. Use 'json' or 'csv'".into());
            }
        }

        Ok(())
    }

    fn print_summary(&self) {
        let data = self.scraped_data.lock().unwrap();
        let total = data.len();
        let successful = data.iter().filter(|d| d.success).count();
        let failed = total - successful;
        let avg_response_time = if !data.is_empty() {
            data.iter().map(|d| d.response_time).sum::<u64>() / data.len() as u64
        } else {
            0
        };

        println!("\n📊 Scraping Summary:");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📈 Total URLs: {}", total);
        println!("✅ Successful: {}", successful);
        println!("❌ Failed: {}", failed);
        println!("⏱️  Average Response Time: {}ms", avg_response_time);
        println!("🎯 Success Rate: {:.1}%", if total > 0 { (successful as f64 / total as f64) * 100.0 } else { 0.0 });
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
}

pub async fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("swoop")
        .version("1.0")
        .about("High-performance web scraper")
        .arg(
            Arg::new("file")
                .long("file")
                .short('f')
                .value_name("FILE")
                .help("File containing URLs to scrape (one per line)")
                .conflicts_with("url")
        )
        .arg(
            Arg::new("url")
                .long("url")
                .short('u')
                .value_name("URL")
                .help("Single URL to scrape")
                .conflicts_with("file")
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .value_name("DIR")
                .help("Output directory for results")
                .default_value("./output")
        )
        .arg(
            Arg::new("concurrency")
                .long("concurrency")
                .short('c')
                .value_name("NUM")
                .help("Number of concurrent requests")
                .default_value("300")
        )
        .arg(
            Arg::new("format")
                .long("format")
                .value_name("FORMAT")
                .help("Output format (json, csv)")
                .default_value("json")
        )
        .get_matches();

    let output_dir = PathBuf::from(matches.get_one::<String>("dir").unwrap());
    let concurrency: usize = matches.get_one::<String>("concurrency").unwrap().parse()?;
    let format = matches.get_one::<String>("format").unwrap();

    let scraper = CliScraper::new(concurrency, output_dir);

    let urls = if let Some(file_path) = matches.get_one::<String>("file") {
        info!("📂 Loading URLs from file: {}", file_path);
        let contents = fs::read_to_string(file_path)?;
        let urls: Vec<String> = contents
            .lines()
            .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
            .map(|line| line.trim().to_string())
            .collect();
        info!("📋 Loaded {} URLs from file", urls.len());
        urls
    } else if let Some(url) = matches.get_one::<String>("url") {
        info!("🎯 Single URL mode: {}", url);
        vec![url.clone()]
    } else {
        warn!("⚠️  No URL or file specified. Use --help for usage information.");
        return Ok(());
    };

    if urls.is_empty() {
        warn!("⚠️  No URLs to scrape");
        return Ok(());
    }

    // Perform scraping
    scraper.scrape_urls(urls).await;

    // Print summary
    scraper.print_summary();

    // Export results
    scraper.export_results(format)?;

    Ok(())
}
