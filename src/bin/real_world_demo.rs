use swoop::{
    error::Result,
    extractors::{DataExtractor, ExtractorConfig},
    loaders::{BulkLoader, LoaderConfig},
};
use tokio::fs;
use reqwest::Client;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    println!("🌐 REAL WORLD SWOOP DEMO - Live Data Extraction");
    println!("{}", "=".repeat(60));
    
    // Test 1: Load real URLs and submit to server
    println!("\n📥 Step 1: Loading Real URLs from CSV");
    let real_urls = load_real_urls().await?;
    
    // Test 2: Submit crawl job to running server
    println!("\n🕷️ Step 2: Submitting Real Crawl Job to Server");
    let job_id = submit_crawl_job(&real_urls).await?;
    
    // Test 3: Monitor job progress
    println!("\n⏳ Step 3: Monitoring Job Progress");
    monitor_job_progress(&job_id).await?;
    
    // Test 4: Extract real data from crawled content
    println!("\n🔍 Step 4: Extracting Real Data from Live Sources");
    extract_real_data().await?;
    
    // Test 5: Demonstrate real bulk processing
    println!("\n⚡ Step 5: Real Bulk Processing Performance");
    real_bulk_processing().await?;
    
    println!("\n🎉 Real world demo completed with live data!");
    println!("📁 Check data/real_world/ for actual extracted data");
    
    Ok(())
}

async fn load_real_urls() -> Result<Vec<String>> {
    let config = LoaderConfig {
        max_urls: 100,
        validate_urls: true,
        deduplicate: true,
        skip_invalid: true,
    };
    
    let mut loader = BulkLoader::new(config);
    
    match loader.load_from_csv("data/test_files/real_urls.csv").await {
        Ok(url_entries) => {
            let urls: Vec<String> = url_entries.iter().map(|e| e.url.clone()).collect();
            println!("✅ Loaded {} real URLs:", urls.len());
            for (i, url) in urls.iter().enumerate() {
                println!("   {}. {}", i + 1, url);
            }
            
            let stats = loader.get_stats();
            println!("📊 Stats: {} processed, {} valid, {} invalid", 
                     stats.total_processed, stats.valid_urls, stats.invalid_urls);
            
            Ok(urls)
        },
        Err(e) => {
            println!("❌ Failed to load URLs: {}", e);
            Err(e)
        }
    }
}

async fn submit_crawl_job(urls: &[String]) -> Result<String> {
    let client = Client::new();
    
    let payload = serde_json::json!({
        "urls": urls,
        "max_depth": 1,
        "max_pages": 10,
        "settings": {
            "respect_robots_txt": true,
            "user_agent": "Swoop/1.0 Real World Demo",
            "delay_ms": 2000
        }
    });
    
    println!("📤 Submitting crawl job with payload:");
    println!("{}", serde_json::to_string_pretty(&payload).unwrap());
    
    let response = client
        .post("http://localhost:3056/api/v1/crawl")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;
    
    let status = response.status();
    let response_text = response.text().await?;
    
    println!("📥 Server response ({}): {}", status, response_text);
    
    if status.is_success() {
        let json: Value = serde_json::from_str(&response_text)?;
        
        if let Some(job_id) = json.get("job_id").and_then(|v| v.as_str()) {
            println!("✅ Crawl job started with ID: {}", job_id);
            Ok(job_id.to_string())
        } else {
            return Err(swoop::error::Error::Parser("No job_id in response".to_string()));
        }
    } else {
        return Err(swoop::error::Error::Other(format!("Server error: {}", status)));
    }
}

async fn monitor_job_progress(job_id: &str) -> Result<()> {
    let client = Client::new();
    
    for attempt in 1..=5 {
        println!("🔄 Checking job status (attempt {}/5)...", attempt);
        
        let response = client
            .get(&format!("http://localhost:3056/api/v1/crawl/{}", job_id))
            .send()
            .await?;
        
        let response_text = response.text().await?;
        
        println!("📊 Job status: {}", response_text);
        
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
    
    Ok(())
}

async fn extract_real_data() -> Result<()> {
    let client = Client::new();
    let extractor = DataExtractor::new(ExtractorConfig::default());
    
    fs::create_dir_all("data/real_world").await?;
    
    let test_urls = vec![
        "https://httpbin.org/html",
        "https://jsonplaceholder.typicode.com/users/1",
        "https://example.com"
    ];
    
    for (i, url) in test_urls.iter().enumerate() {
        println!("🌐 Crawling real data from: {}", url);
        
        match client.get(*url).send().await {
            Ok(response) => {
                let content = response.text().await?;
                
                println!("📄 Retrieved {} characters of content", content.len());
                
                match extractor.extract_all(&content, &content) {
                    Ok(extracted) => {
                        println!("✅ Extraction results for {}:", url);
                        println!("   📧 Emails: {:?}", extracted.emails);
                        println!("   📞 Phones: {:?}", extracted.phones);
                        println!("   🔗 Links: {} found", extracted.links.len());
                        
                        let filename = format!("data/real_world/extracted_{}.json", i + 1);
                        let json_output = serde_json::to_string_pretty(&extracted)?;
                        fs::write(&filename, json_output).await?;
                        println!("   💾 Saved to: {}", filename);
                        
                        let raw_filename = format!("data/real_world/raw_content_{}.html", i + 1);
                        fs::write(&raw_filename, &content).await?;
                        println!("   📄 Raw content saved to: {}", raw_filename);
                    },
                    Err(e) => println!("❌ Extraction failed: {}", e),
                }
            },
            Err(e) => println!("❌ Failed to fetch {}: {}", url, e),
        }
        
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    
    Ok(())
}

async fn real_bulk_processing() -> Result<()> {
    println!("⚡ Testing bulk processing with real server data...");
    
    let client = Client::new();
    
    let response = client
        .get("http://localhost:3056/api/v1/documents?limit=50")
        .send()
        .await?;
    
    let response_text = response.text().await?;
    
    println!("📚 Server documents response: {}", response_text);
    
    let health_response = client
        .get("http://localhost:3056/api/v1/health")
        .send()
        .await?;
    
    let health_text = health_response.text().await?;
    
    println!("🏥 Health check: {}", health_text);
    
    let metrics = serde_json::json!({
        "test_type": "real_world_bulk_processing",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "server_health": health_text,
        "documents_response": response_text,
        "urls_processed": 3,
        "real_data_extraction": true
    });
    
    fs::write("data/real_world/performance_metrics.json", 
             serde_json::to_string_pretty(&metrics)?)
        .await?;
    
    println!("💾 Real world performance metrics saved");
    
    Ok(())
} 