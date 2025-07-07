# Getting Started with Swoop

Welcome to Swoop, a high-performance web crawler built with Rust. This guide will help you get up and running quickly.

## Installation

### System Requirements

- **Rust**: Version 1.82.0 or higher
- **Memory**: Minimum 2GB RAM (8GB+ recommended for production)
- **Storage**: 1GB free space for binaries and cache
- **Network**: Stable internet connection

### Quick Install

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build Swoop
git clone https://github.com/codewithkenzo/swoop.git
cd swoop
cargo build --release
```

## First Steps

### 1. Basic HTTP Fetching

Start with simple HTTP requests:

```rust
use swoop_core::fetch_url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fetch_url("https://httpbin.org/json").await?;
    println!("Response: {}", String::from_utf8_lossy(&content));
    Ok(())
}
```

### 2. Content Extraction

Extract structured data from web pages:

```rust
use scrapers::{ScraperRegistry, platforms::GenericScraper};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ScraperRegistry::default();
    
    // Extract content from any website
    let result = registry.extract("https://example.com").await?;
    
    println!("Title: {:?}", result.title);
    println!("Text length: {}", result.text.as_ref().map_or(0, |t| t.len()));
    
    Ok(())
}
```

### 3. Running the TUI

Launch the interactive dashboard:

```bash
cargo run --bin tui
```

**TUI Controls:**
- `q` - Quit the application
- `Arrow keys` - Navigate between panels
- `Space` - Pause/resume operations
- `r` - Refresh data

## Configuration

### Basic Configuration

Create a `swoop.toml` file in your project root:

```toml
[scraper]
max_concurrent = 5          # Number of concurrent requests
rate_limit = 1.0            # Requests per second
timeout_secs = 30           # Request timeout
user_agent = "Swoop/1.0"    # Custom user agent

[scraper.headers]
"Accept" = "text/html,application/xhtml+xml"
"Accept-Language" = "en-US,en;q=0.5"
```

### Storage Configuration

Configure data storage backends:

```toml
[storage.scylla]
nodes = ["127.0.0.1:9042"]  # ScyllaDB cluster nodes
keyspace = "swoop"          # Database keyspace
timeout_secs = 30           # Connection timeout

[storage.s3]
endpoint = "https://s3.amazonaws.com"
bucket = "swoop-data"
region = "us-east-1"
# AWS credentials via environment variables:
# AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY
```

## Common Use Cases

### Website Monitoring

Monitor website changes over time:

```rust
use scrapers::{ScraperRegistry, extractors};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = ScraperRegistry::default();
    let url = "https://news.ycombinator.com";
    
    loop {
        match registry.extract(url).await {
            Ok(content) => {
                println!("Scraped at: {:?}", chrono::Utc::now());
                println!("Title: {:?}", content.title);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
        
        sleep(Duration::from_secs(300)).await; // Check every 5 minutes
    }
}
```

### Bulk URL Processing

Process multiple URLs efficiently:

```rust
use scrapers::ScraperRegistry;
use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec![
        "https://example.com",
        "https://httpbin.org/json",
        "https://rust-lang.org",
    ];
    
    let registry = ScraperRegistry::default();
    
    let results = stream::iter(urls)
        .map(|url| async move {
            let result = registry.extract(url).await;
            (url, result)
        })
        .buffer_unordered(3) // Process 3 URLs concurrently
        .collect::<Vec<_>>()
        .await;
    
    for (url, result) in results {
        match result {
            Ok(content) => println!("✓ {}: {}", url, content.title.unwrap_or_default()),
            Err(e) => println!("✗ {}: {}", url, e),
        }
    }
    
    Ok(())
}
```

## Next Steps

- [**API Reference**](../api/README.md) - Detailed API documentation
- [**Architecture Guide**](../architecture/overview.md) - Understanding Swoop's design
- [**Examples**](../../examples/) - More practical examples
- [**Security Guide**](security.md) - Best practices for secure crawling

## Troubleshooting

### Common Issues

**Build Errors**
```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
cargo build
```

**Network Timeouts**
```toml
# Increase timeout in swoop.toml
[scraper]
timeout_secs = 60
```

**Memory Issues**
```toml
# Reduce concurrency
[scraper]
max_concurrent = 2
```

Need help? Check our [GitHub Issues](https://github.com/codewithkenzo/swoop/issues) or start a [Discussion](https://github.com/codewithkenzo/swoop/discussions).