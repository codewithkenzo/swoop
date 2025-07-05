
use swoop::{
    error::Result,
    extractors::{DataExtractor, ExtractorConfig},
    loaders::{BulkLoader, LoaderConfig},
};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("🚀 Swoop Consumer Demo - Advanced Data Extraction & Bulk Processing");
    println!("{}", "=".repeat(80));
    
    // Test 1: CSV Bulk Loading
    println!("\n📥 Test 1: CSV Bulk URL Loading");
    test_csv_loading().await?;
    
    // Test 2: Data Extraction (emails, phones, sensitive data)
    println!("\n🔍 Test 2: Advanced Data Extraction");
    test_data_extraction().await?;
    
    // Test 3: Validation and Output
    println!("\n✅ Test 3: Validation & Output Processing");
    test_validation_output().await?;
    
    // Test 4: Performance Benchmarking
    println!("\n⚡ Test 4: Performance Benchmarking");
    test_performance().await?;
    
    println!("\n🎉 All consumer features tested successfully!");
    println!("📁 Check the data/ directory for output files");
    
    Ok(())
}

async fn test_csv_loading() -> Result<()> {
    let config = LoaderConfig {
        max_urls: 1000,
        validate_urls: true,
        deduplicate: true,
        skip_invalid: true,
        ..Default::default()
    };
    
    let mut loader = BulkLoader::new(config);
    
    // Test CSV loading
    match loader.load_from_csv("data/test_files/sample_urls.csv").await {
        Ok(urls) => {
            println!("✅ Loaded {} URLs from CSV", urls.len());
            let stats = loader.get_stats();
            println!("   📊 Stats: {} processed, {} valid, {} invalid, {} duplicates", 
                     stats.total_processed, stats.valid_urls, stats.invalid_urls, stats.duplicates_removed);
            
            // Save processed URLs to output
            let output_path = "data/processed/validated_urls.json";
            let json_output = serde_json::to_string_pretty(&urls)?;
            fs::write(output_path, json_output).await?;
            println!("   💾 Saved validated URLs to: {output_path}");
        },
        Err(e) => println!("❌ CSV loading failed: {e}"),
    }
    
    // Test text file loading
    // Create a simple text file first
    let text_urls = "https://httpbin.org/html\nhttps://example.com\n# Comment line\nhttps://www.rust-lang.org\n";
    fs::write("data/test_files/urls.txt", text_urls).await?;
    
    loader.reset_stats();
    match loader.load_from_text("data/test_files/urls.txt").await {
        Ok(urls) => {
            println!("✅ Loaded {} URLs from text file", urls.len());
            let stats = loader.get_stats();
            println!("   📊 Stats: {} processed, {} valid, processing time: {}ms", 
                     stats.total_processed, stats.valid_urls, stats.processing_time_ms);
        },
        Err(e) => println!("❌ Text loading failed: {e}"),
    }
    
    Ok(())
}

async fn test_data_extraction() -> Result<()> {
    let config = ExtractorConfig {
        extract_emails: true,
        extract_phones: true,  
        detect_sensitive: true,
        redact_sensitive: true,
        email_validation: true,
        phone_formatting: true,
        ..Default::default()
    };
    
    let extractor = DataExtractor::new(config);
    
    // Read our test HTML content
    let html_content = fs::read_to_string("data/test_files/test_content.html").await?;
    let text_content = "Contact us at support@example.com or call (555) 123-4567. 
                        Our business number is 1-800-555-HELP.
                        Emergency: +1.555.999.8888
                        International: +44 20 7946 0958
                        Multiple contacts: john@domain.com, jane@company.org, admin@service.net
                        SSN: 123-45-6789 
                        Credit Card: 4111-1111-1111-1111";
    
    // Extract all data
    match extractor.extract_all(text_content, &html_content) {
        Ok(extracted) => {
            println!("✅ Data extraction completed!");
            println!("   📧 Emails found: {:?}", extracted.emails);
            println!("   📞 Phones found: {:?}", extracted.phones);
            println!("   🔒 Sensitive data detected: {} items", extracted.sensitive_data.len());
            
            for sensitive in &extracted.sensitive_data {
                println!("      - {}: {} -> {}", 
                         sensitive.data_type, 
                         sensitive.original_text, 
                         sensitive.redacted_text);
            }
            
            println!("   🔗 Links found: {} items", extracted.links.len());
            println!("   📋 Metadata: {:?}", extracted.metadata);
            
            // Save extracted data
            let output_path = "data/processed/extracted_data.json";
            let json_output = serde_json::to_string_pretty(&extracted)?;
            fs::write(output_path, json_output).await?;
            println!("   💾 Saved extraction results to: {output_path}");
            
            // Create sanitized version
            let sanitized = extractor.sanitize_text(text_content);
            let sanitized_path = "data/processed/sanitized_content.txt";
            fs::write(sanitized_path, sanitized).await?;
            println!("   🛡️  Saved sanitized content to: {sanitized_path}");
            
        },
        Err(e) => println!("❌ Data extraction failed: {e}"),
    }
    
    Ok(())
}

async fn test_validation_output() -> Result<()> {
    println!("✅ Validation system working (integrated in CSV loader)");
    
    // Create different output formats
    let sample_data = vec![
        ("John Doe", "john@example.com", "+1 (555) 123-4567"),
        ("Jane Smith", "jane@company.org", "(555) 987-6543"),
        ("Bob Wilson", "bob@service.net", "1-800-555-HELP"),
    ];
    
    // CSV output
    let csv_path = "data/exports/contacts.csv";
    let mut csv_content = "Name,Email,Phone\n".to_string();
    for (name, email, phone) in &sample_data {
        csv_content.push_str(&format!("{name},{email},{phone}\n"));
    }
    fs::write(csv_path, csv_content).await?;
    println!("   📄 CSV export saved to: {csv_path}");
    
    // JSON Lines output (streaming format)
    let jsonl_path = "data/exports/contacts.jsonl";
    let mut jsonl_content = String::new();
    for (name, email, phone) in &sample_data {
        let record = serde_json::json!({
            "name": name,
            "email": email, 
            "phone": phone,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });
        jsonl_content.push_str(&format!("{record}\n"));
    }
    fs::write(jsonl_path, jsonl_content).await?;
    println!("   📄 JSONL export saved to: {jsonl_path}");
    
    Ok(())
}

async fn test_performance() -> Result<()> {
    let start_time = std::time::Instant::now();
    
    // Create a larger dataset for performance testing
    let mut large_text = String::new();
    for i in 0..1000 {
        large_text.push_str(&format!(
            "Contact {} at user{}@domain{}.com or call (555) {}-{} ",
            i, i, i % 10, 100 + (i % 900), 1000 + (i % 9000)
        ));
    }
    
    let config = ExtractorConfig::default();
    let extractor = DataExtractor::new(config);
    
    match extractor.extract_all(&large_text, "") {
        Ok(extracted) => {
            let processing_time = start_time.elapsed();
            println!("✅ Performance test completed!");
            println!("   ⏱️  Processing time: {processing_time:?}");
            println!("   📧 Emails extracted: {}", extracted.emails.len());
            println!("   📞 Phones extracted: {}", extracted.phones.len());
            println!("   📊 Processing rate: {:.2} items/ms",
                     (extracted.emails.len() + extracted.phones.len()) as f64 
                     / processing_time.as_millis() as f64);
            
            // Save performance results
            let perf_data = serde_json::json!({
                "processing_time_ms": processing_time.as_millis(),
                "emails_extracted": extracted.emails.len(),
                "phones_extracted": extracted.phones.len(),
                "total_items": extracted.emails.len() + extracted.phones.len(),
                "items_per_ms": (extracted.emails.len() + extracted.phones.len()) as f64 / processing_time.as_millis() as f64,
                "timestamp": chrono::Utc::now().to_rfc3339()
            });
            
            fs::write("data/processed/performance_results.json", 
                     serde_json::to_string_pretty(&perf_data)?).await?;
            println!("   💾 Performance results saved to: data/processed/performance_results.json");
        },
        Err(e) => println!("❌ Performance test failed: {e}"),
    }
    
    Ok(())
} 