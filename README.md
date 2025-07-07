# Swoop Web Crawler

> A high-performance, production-ready web crawler built with Rust, designed for scalable data extraction from social media platforms and web content.

[![Rust](https://img.shields.io/badge/rust-1.82+-orange.svg)](https://www.rust-lang.org)
[![CI](https://github.com/codewithkenzo/swoop/workflows/CI/badge.svg)](https://github.com/codewithkenzo/swoop/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## ✨ Features

- **High-Performance HTTP Client** - Built on `hyper` with connection pooling and async operations
- **Real-Time TUI Dashboard** - Monitor crawling operations with an interactive terminal interface  
- **Modular Architecture** - Extensible platform-specific scrapers for social media sites
- **Scalable Storage** - ScyllaDB for time-series data and S3-compatible archival storage
- **Anti-Bot Evasion** - Rate limiting, proxy rotation, and stealth capabilities
- **Compliance Ready** - GDPR compliance features and robots.txt respect

## 🚀 Quick Start

### Prerequisites

- Rust 1.82.0 or higher
- Optional: ScyllaDB for storage (Docker available)
- Optional: S3-compatible storage

### Installation

```bash
# Clone the repository
git clone https://github.com/codewithkenzo/swoop.git
cd swoop

# Build the project
cargo build --release

# Run the TUI interface
cargo run --bin tui
```

### Basic Usage

```rust
use swoop_core::fetch_url;
use scrapers::{ScraperRegistry, ScraperConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Simple HTTP fetch
    let content = fetch_url("https://example.com").await?;
    println!("Fetched {} bytes", content.len());

    // Platform-specific scraping
    let registry = ScraperRegistry::default();
    let extracted = registry.extract("https://example.com").await?;
    println!("Title: {:?}", extracted.title);
    
    Ok(())
}
```

## 📚 Documentation

- [**User Guide**](docs/guide/getting-started.md) - Complete setup and usage instructions
- [**API Reference**](docs/api/README.md) - Detailed API documentation for all crates
- [**Architecture**](docs/architecture/overview.md) - System design and component overview
- [**Examples**](examples/) - Practical usage examples and tutorials

## 🏗️ Project Structure

```
swoop/
├── core/           # HTTP client and networking utilities
├── scrapers/       # Platform-specific content extraction
├── storage/        # Data persistence layer (ScyllaDB + S3)
├── tui/           # Terminal user interface
├── docs/          # Documentation
└── examples/      # Usage examples
```

## 🔧 Configuration

Swoop uses TOML configuration files for easy customization:

```toml
# swoop.toml
[scraper]
max_concurrent = 10
rate_limit = 1.0
user_agent = "Swoop/1.0"

[storage.scylla]
nodes = ["127.0.0.1:9042"]
keyspace = "swoop"

[storage.s3]
bucket = "swoop-data"
region = "us-east-1"
```

## 🛠️ Development

### Building from Source

```bash
# Development build
cargo build

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt
```

### Running the TUI

```bash
# Start the interactive dashboard
cargo run --bin tui

# Controls:
# - 'q' to quit
# - Arrow keys to navigate
# - Space to pause/resume
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙋 Support

- **Issues**: [GitHub Issues](https://github.com/codewithkenzo/swoop/issues)
- **Discussions**: [GitHub Discussions](https://github.com/codewithkenzo/swoop/discussions)
- **Security**: Please report security vulnerabilities to [security@swoop.dev](mailto:security@swoop.dev)

---

Built with ❤️ using Rust