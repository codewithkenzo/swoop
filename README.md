# Swoop

> AI-Powered Document Intelligence Platform with Integrated Web Crawling and Analysis

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Deployment: Vercel](https://img.shields.io/badge/deploy-vercel-black.svg)](https://vercel.com)

**Swoop** is a next-generation, cloud-native platform for automated document analysis, intelligent web crawling, and content management — built for researchers, analysts, and teams who work with large volumes of structured and unstructured information.

Deployed as a scalable cloud app (one instance per user or organization), Swoop combines a high-performance Rust backend with a modern React + TypeScript frontend, enabling fast, secure, and intelligent data workflows.

---

## 🚀 Key Features

* **Web-Scale Crawling**: Extract structured data from websites using CSS selectors, XPath, and JSONPath
* **Semantic Content Analysis**: Built-in NLP and rule-based pipelines for understanding and filtering documents
* **Cloud Workspace**: Secure per-user instance for managing documents, rules, and exports
* **Real-Time Monitoring**: Prometheus-powered metrics and health insights for observability
* **Multi-Backend Storage**: File system, Redis, and SQLite support with pluggable architecture
* **Concurrent Processing**: Optimized for high-throughput, parallel document ingestion
* **Smart Organization**: Auto-tagging, categorization, and full-text indexing
* **Flexible Export Pipelines**: Output in JSON, Markdown, PDF, or custom formats

---

## 🧠 Platform Architecture

Swoop is designed as a modular, full-stack system with the following layers:

### 🌐 Cloud Application (Vercel + Serverless + Instance-Per-User Model)

Each signup provisions a dedicated instance:

* User data isolation
* Secure storage and configuration
* On-demand compute via Vercel functions and Rust modules

### 🧩 Rust Engine (Backend Services)

* High-performance crawling and extraction
* Semantic parsing and analysis
* Persistent workspace management
* Monitoring via Prometheus
* RESTful API layer (OpenAPI-compliant)

### 💻 Frontend (React + TypeScript)

* Responsive, cross-platform UI
* Visual rule builder for content extraction
* Workspace dashboard and document explorer
* Advanced filtering, previews, and export controls

---

## 🛠️ Developer Quick Start

```bash
# Clone the repository
git clone https://github.com/codewithkenzo/swoop.git
cd swoop

# Backend: build Rust services
cargo build --release

# Frontend: install and build React app
cd frontend
npm install
npm run build
```

---

## 🔍 Usage Example

### Web Document Processing (Rust API)

```rust
use swoop::{DocumentWorkspace, CrawlConfig, ExtractionRule};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = DocumentWorkspace::new("./data").await?;
    
    let mut config = CrawlConfig::default();
    config.add_rule(ExtractionRule::css("titles", "h1, h2, .headline"));
    
    let results = workspace.process_urls(vec![
        "https://example.com/docs",
        "https://blog.example.com/articles"
    ], config).await?;

    println!("Processed {} documents", results.len());
    Ok(())
}
```

### Content Analysis and Export

```rust
use swoop::{Analyzer, OutputFormat, ContentFilter};

let analysis = Analyzer::new()
    .with_semantic_extraction()
    .with_content_filtering(ContentFilter::legal_docs())
    .with_output_format(OutputFormat::json());

let report = analysis.process_directory("./inbox").await?;
report.export_to("./outbox").await?;
```

---

## 🧩 API Overview

### Document Processing

* `POST /api/documents` — Ingest and analyze documents from URLs or uploads
* `GET /api/documents/:id` — Retrieve processed document
* `POST /api/documents/search` — Search and filter across workspace

### Workspace Operations

* `GET /api/workspace/status` — View instance health and activity
* `POST /api/workspace/rules` — Define or update extraction rules
* `POST /api/workspace/analyze` — Trigger semantic analysis

### Monitoring

* `/health` — Health check
* `/metrics` — Prometheus metrics
* `/ready` — Readiness probe

---

## 🌩️ Deployment Models

### Cloud (Default)

* Automatic instance provisioning per user via Vercel
* Backend Rust services deployed as serverless functions or containers
* Isolated storage and config per account
* Ideal for research teams, legal firms, technical writers

### Self-Hosted (Optional)

* Use Docker or Kubernetes to run private deployments

#### Docker Example

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --features enterprise

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/swoop-server /usr/local/bin/
CMD ["swoop-server", "--config", "/etc/swoop/config.toml"]
```

---

## 🔧 Configuration

```toml
[workspace]
storage = "redis"
index_dir = "./index"
temp_dir = "./tmp"
max_file_size = "100MB"

[crawling]
concurrent = 10
timeout_ms = 30000
respect_robots = true
user_agent = "SwoopBot/1.0"

[analysis]
semantic = true
extract_images = true
summarize = true

[monitoring]
enable_prometheus = true
port = 9090
log_level = "info"
```

---

## 💼 Use Cases

* **Academic Research**: Collect and analyze scholarly documents
* **Legal Analysis**: Parse and organize legal texts, case studies, and regulations
* **Competitive Intelligence**: Monitor and extract structured insights from web sources
* **Enterprise Content Management**: Automate ingestion, classification, and export of internal documentation
* **Technical Documentation**: Analyze, index, and transform engineering documents or API specs

---

## 🤝 Contributing

We welcome community contributions!

1. Fork the repo
2. Create a feature branch: `git checkout -b feat/new-feature`
3. Add your implementation + tests
4. Ensure all tests pass: `cargo test && cd frontend && npm test`
5. Open a pull request with a clear summary

Please review our [Contributing Guidelines](CONTRIBUTING.md) and [Code of Conduct](CODE_OF_CONDUCT.md).

---

## 📄 License

Licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## 📬 Support & Resources

* **Docs**: [https://swoop-docs.dev](https://swoop-docs.dev)
* **Community**: [GitHub Discussions](https://github.com/codewithkenzo/swoop/discussions)
* **Support**: [contact@swoop-platform.com](mailto:contact@swoop-platform.com)

---

<p align="center">
  <strong>Built by <a href="https://github.com/codewithkenzo">@codewithkenzo</a> • Engineered for document intelligence in the cloud</strong>
</p>
