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
```

### Running the TUI Dashboard

The TUI provides a real-time dashboard for monitoring scraper performance and status.

```bash
# Run the TUI interface
cargo run --bin swoop-tui
```

**TUI Controls:**
- **`q`** or **`Esc`**: Quit the application
- **`Tab`** / **`Shift+Tab`**: Cycle through tabs
- **`←`** / **`→`**: Navigate panes within the Overview tab
- **`↑`** / **`↓`**: Scroll through lists (Logs, Targets, etc.)
- **`Spacebar`**: Pause or resume the scraping engine
- **`i`**: Enter input mode to add new target URLs
- **`l`**: Load URLs from the default file (`urls.txt`)
- **`e`**: Switch to the Export tab
- **`d`**: Launch the advanced, standalone dashboard

### Running the CLI Scraper

For command-line operations, use the `swoop-cli` binary.

**Scrape a single URL:**
```bash
cargo run --bin swoop-cli -- --url "https://example.com"
```

**Scrape a list of URLs from a file:**
```bash
cargo run --bin swoop-cli -- --file urls.txt
```

**CLI Options:**
- `--url <URL>`: Scrape a single URL.
- `--file <PATH>`: Scrape URLs from a file (one per line).
- `--concurrency <NUM>`: Set the number of concurrent requests (default: 10).
- `--output-dir <DIR>`: Specify the directory for saving results (default: `./test_output`).
- `--format <FORMAT>`: Set the output format (`json` or `csv`, default: `json`).

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

Swoop uses environment variables for configuration. Create a `.env` file in the root directory:

```env
# Scraper settings
MAX_CONCURRENT=10
RATE_LIMIT=1.0
USER_AGENT="Swoop/1.0"

# ScyllaDB settings
SCYLLA_NODES="127.0.0.1:9042"
SCYLLA_KEYSPACE="swoop"

# S3 settings
S3_BUCKET="swoop-data"
S3_REGION="us-east-1"
AWS_ACCESS_KEY_ID="YOUR_ACCESS_KEY"
AWS_SECRET_ACCESS_KEY="YOUR_SECRET_KEY"
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
