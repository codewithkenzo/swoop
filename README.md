# Swoop

> Intelligent document analysis and management platform with advanced web crawling capabilities

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

**Swoop** is a comprehensive document analysis and management platform that combines advanced web crawling, intelligent content extraction, and sophisticated document organization capabilities. Built with modern technologies, it serves both as a powerful backend engine and a complete desktop application for document research and analysis workflows.

## Key Capabilities

- **Document Intelligence**: Advanced content extraction using CSS selectors, XPath, and JSONPath with semantic analysis
- **Unified Workspace**: Desktop application with React/TypeScript frontend and Tauri integration
- **Multi-Source Ingestion**: Web crawling, file system monitoring, and direct document import
- **Production Monitoring**: Comprehensive observability with Prometheus metrics and health monitoring
- **Enterprise Storage**: Multiple backend options including distributed Redis, SQLite, and filesystem
- **Concurrent Processing**: Thread-safe architecture with intelligent rate limiting and resource management
- **Content Organization**: Smart document categorization, tagging, and full-text search capabilities
- **Export Workflows**: Multiple output formats with customizable processing pipelines

## Architecture Overview

Swoop operates as a full-stack platform with three primary components:

### Core Engine (Rust)
High-performance document processing and web crawling engine with production-grade monitoring and storage management.

### Desktop Application (Tauri + React + TypeScript)
Cross-platform desktop application providing:
- Document workspace and file management
- Visual content extraction rule builder
- Real-time crawling and processing monitoring
- Advanced search and filtering interfaces
- Export and workflow management

### Web Interface
Browser-based dashboard for monitoring, configuration, and remote management of document processing workflows.

## Installation

### Desktop Application
Download the latest release for your platform:
- **Windows**: `swoop-setup.exe`
- **macOS**: `swoop.dmg`
- **Linux**: `swoop.AppImage` or package manager

### Developer Installation
```toml
[dependencies]
swoop = "0.1"
tokio = { version = "1.0", features = ["full"] }
```

## Usage Examples

### Document Workspace Integration
```rust
use swoop::{DocumentWorkspace, CrawlConfig, ExtractionRule};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = DocumentWorkspace::new("./documents").await?;
    
    // Configure intelligent extraction
    let mut config = CrawlConfig::new();
    config.add_rule(ExtractionRule::css("articles", "article h2, .content"));
    config.add_rule(ExtractionRule::xpath("metadata", "//meta[@name='description']/@content"));
    
    // Process and organize documents
    let results = workspace.process_urls(vec![
        "https://research.example.com/papers",
        "https://docs.example.com/guides"
    ], config).await?;
    
    // Results are automatically organized and indexed
    println!("Processed {} documents", results.len());
    Ok(())
}
```

### Advanced Content Analysis
```rust
use swoop::{Analyzer, ContentFilter, OutputFormat};

let analyzer = Analyzer::new()
    .with_semantic_extraction()
    .with_content_filtering(ContentFilter::academic_papers())
    .with_output_format(OutputFormat::structured_json());

let analysis = analyzer.process_directory("./research_docs").await?;
analysis.export_to("./analysis_results").await?;
```

### Enterprise Deployment
```rust
use swoop::{EnterpriseServer, MonitoringConfig, StorageConfig};

let server = EnterpriseServer::builder()
    .with_monitoring(MonitoringConfig::prometheus())
    .with_storage(StorageConfig::distributed_redis("cluster.internal"))
    .with_concurrent_workers(50)
    .bind("0.0.0.0:8080")
    .await?;

server.run().await?;
```

## Configuration

### Workspace Configuration
```toml
[workspace]
default_storage = "filesystem"
index_path = "./workspace/index"
temp_directory = "./workspace/temp"
max_file_size = "100MB"

[crawling]
concurrent_requests = 10
request_timeout = 30000
respect_robots_txt = true
user_agent = "Swoop Document Analyzer/1.0"

[extraction]
enable_semantic_analysis = true
extract_images = true
preserve_formatting = true
generate_summaries = true

[monitoring]
enable_prometheus = true
metrics_port = 9090
log_level = "info"
```

## Desktop Application Features

### Document Workspace
- **File System Integration**: Native file monitoring and organization
- **Visual Rule Builder**: Drag-and-drop interface for creating extraction rules
- **Preview System**: Real-time preview of extraction results
- **Batch Processing**: Queue management for large document sets

### Analysis Dashboard
- **Content Visualization**: Interactive charts and content analysis
- **Search Interface**: Advanced filtering and full-text search
- **Export Tools**: Multiple format support with custom templates
- **Workflow Management**: Automated processing pipelines

### Monitoring Console
- **Real-time Metrics**: System performance and processing statistics
- **Health Monitoring**: Component status and error tracking
- **Resource Management**: Memory, storage, and network utilization
- **Audit Logging**: Complete activity tracking and reporting

## Deployment Options

### Standalone Desktop
Single-user installation with local storage and processing capabilities.

### Enterprise Server
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --features enterprise

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/swoop-server /usr/local/bin/
EXPOSE 8080 9090
CMD ["swoop-server", "--config", "/etc/swoop/config.toml"]
```

### Kubernetes Deployment
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: swoop-enterprise
spec:
  replicas: 3
  selector:
    matchLabels:
      app: swoop
  template:
    spec:
      containers:
      - name: swoop
        image: swoop/enterprise:latest
        ports:
        - containerPort: 8080
        - containerPort: 9090
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "2000m"
```

## API Reference

### Document Processing
- `POST /api/v1/documents/process` - Process documents from URLs or files
- `GET /api/v1/documents/{id}` - Retrieve processed document
- `POST /api/v1/documents/search` - Full-text search across document corpus
- `GET /api/v1/documents/export` - Export documents in various formats

### Workspace Management
- `GET /api/v1/workspace/status` - Workspace health and statistics
- `POST /api/v1/workspace/rules` - Create or update extraction rules
- `GET /api/v1/workspace/files` - List and manage workspace files
- `POST /api/v1/workspace/analyze` - Run analysis on document sets

### Monitoring Endpoints
- `/health` - Application health status
- `/ready` - Kubernetes readiness probe
- `/metrics` - Prometheus metrics endpoint
- `/api/v1/monitoring/stats` - Detailed system statistics

## Development

### Building from Source
```bash
git clone https://github.com/codewithkenzo/swoop.git
cd swoop

# Build core engine
cargo build --release

# Build desktop application
cd desktop
npm install
npm run tauri build

# Run development server
cargo run --bin swoop-server
```

### Testing
```bash
# Unit and integration tests
cargo test

# Frontend tests
cd desktop && npm test

# End-to-end testing
cargo test --test e2e_tests
```

## Use Cases

**Research and Analysis**
- Academic paper collection and analysis
- Market research document processing
- Legal document review and organization
- Technical documentation management

**Enterprise Content Management**
- Automated content ingestion from multiple sources
- Document compliance and audit workflows
- Knowledge base construction and maintenance
- Competitive intelligence gathering

**Development and Documentation**
- API documentation extraction and organization
- Code repository analysis and documentation
- Technical specification management
- Automated reporting and documentation generation

## Contributing

We welcome contributions to improve Swoop's capabilities. Please read our [Contributing Guidelines](CONTRIBUTING.md) and [Code of Conduct](CODE_OF_CONDUCT.md).

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/enhancement`
3. Implement changes with appropriate tests
4. Ensure all tests pass: `cargo test && cd desktop && npm test`
5. Submit a pull request with detailed description

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support and Documentation

- **Documentation**: [https://swoop-docs.dev](https://swoop-docs.dev)
- **Community**: [GitHub Discussions](https://github.com/codewithkenzo/swoop/discussions)
- **Issues**: [GitHub Issues](https://github.com/codewithkenzo/swoop/issues)
- **Enterprise Support**: [contact@swoop-platform.com](mailto:contact@swoop-platform.com)

---

<p align="center">
  <strong>Built by <a href="https://github.com/codewithkenzo">@codewithkenzo</a> • Made for document intelligence professionals</strong>
</p> 