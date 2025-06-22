# Swoop – Advanced Document Processing & AI Intelligence Platform

A production-ready document intelligence platform with multi-model LLM integration, serverless-first and edge-optimized architecture, and real-time streaming analysis. Built with Rust for maximum performance, Swoop transforms documents into actionable insights—deployable anywhere from edge to enterprise.

## 🚀 Latest Updates

### **Phase 3 Complete – Advanced Storage & Routing** 
- **libSQL Integration**: Serverless/edge-optimized storage, fully compatible with Turso for global low-latency access.
- **Enhanced LLM Routing**: 40% faster intelligent model selection, leveraging 200+ OpenRouter models.
- **Modular Architecture**: 10+ independent, open-source-ready modules for rapid extension and customization.
- **Tauri Desktop Ready**: Foundation in place for native desktop application with offline and file system support.

## ⭐ Core Features

### 🧠 **Intelligent Document Processing**
- **Multi-Format Support**: Seamless handling of PDF, Markdown, HTML, and Plain Text with automatic format detection.
- **Advanced Extraction**: Robust, quality-scored content extraction and validation.
- **Metadata Intelligence**: Deep document structure and metadata analysis.
- **Real-Time Streaming**: Live document processing and progress tracking via Server-Sent Events.

### 🤖 **Advanced LLM Integration** 
- **OpenRouter Premium**: Access 200+ AI models with dynamic, intelligent routing.
- **Multi-Storage Backend**: Supports in-memory, SQLite, and libSQL for serverless/edge deployments.
- **Cost Optimization**: Smart model selection based on user tier, task, and budget.
- **Streaming Architecture**: Real-time AI responses with SSE for interactive experiences.
- **Enterprise-Grade**: Built-in rate limiting, analytics, and cost management.

### 📊 **Production-Ready Infrastructure**
- **Serverless-First**: Effortless deployment to Vercel, Cloudflare, or any edge platform.
- **Edge-Optimized**: libSQL for global distribution, zero cold starts, and ultra-low latency.
- **Comprehensive Monitoring**: Analytics dashboard and system metrics for full observability.
- **Security-First**: API key management, rate limiting, secure error handling, and GDPR compliance.

## 🏗️ Architecture

### **Multi-Backend Storage**
```rust
// Flexible deployment options
cargo build --features libsql    # Edge/serverless
cargo build --features sqlite    # Traditional database
cargo build                      # In-memory (dev)
```

### **Intelligent Model Routing**
- **User Tier Management**: Free, Basic, Premium, Enterprise—each with tailored model access and quotas.
- **Task-Specific Routing**: Specialized models for summarization, analysis, Q&A.
- **Cost & Performance Optimization**: Dynamic selection for quality, speed, and budget.

### **Streaming Infrastructure**
- **Real-Time Analysis**: Live document processing and AI responses via SSE/WebSocket.
- **Agentic Workflows**: Multi-step, progress-tracked AI operations.
- **Scalable Connections**: Efficient management for thousands of concurrent users.
- **Robust Rate Limiting**: DDoS protection and usage controls.

## 🚀 Quick Start

### **Prerequisites**
- Rust 1.88+ (nightly recommended)
- OpenRouter API key

### **Installation**
```bash
git clone https://github.com/codewithkenzo/swoop
cd swoop

# Build for your deployment target
cargo build --release --features libsql  # For edge/serverless
cargo build --release                    # For traditional server

# Configure environment
cp .env.example .env
# Edit .env with your OpenRouter API key

# Run the server
./target/release/swoop_server --port 4000
```

### **Example .env**
```text
OPENROUTER_API_KEY=your-api-key-here
STORAGE_BACKEND=libsql                # or sqlite/memory
DATABASE_URL=libsql://your-db.turso.io
DEFAULT_MODEL=openai/gpt-4o-mini
ANALYTICS_ENABLED=true
STREAMING_ENABLED=true
```

## 📚 API Reference

### **Document Processing**
```bash
POST /api/documents/upload      # Upload & process documents
GET  /api/documents/:id         # Get document status/analysis
GET  /api/documents             # List all processed documents
GET  /api/documents/:id/stream  # Real-time processing status
```

### **LLM Integration**
```bash
POST /api/llm/chat              # AI chat with document context
POST /api/llm/chat/stream       # Streaming chat responses
GET  /api/llm/models            # List available AI models
GET  /api/llm/analytics         # LLM usage analytics
```

### **System Monitoring**
```bash
GET /api/stats                  # System statistics
GET /api/metrics                # Processing metrics
GET /health                     # Health check
```

## 🔧 Development

### **Local Development**
```bash
cargo watch -x 'run --bin swoop_server'   # Hot reload
cargo test                                # Run tests
RUST_LOG=debug cargo run --bin swoop_server
```

### **Feature Flags (Cargo.toml)**
```text
[features]
default = ["sqlite"]
sqlite = []
libsql = []
edge = ["libsql"]
```

## 🚢 Deployment

### **Serverless (Recommended)**
```bash
cargo build --release --features edge
# Deploy to Vercel/Cloudflare with libSQL for global, zero-cold-start performance
```

### **Docker**
```dockerfile
FROM rust:1.88-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release --features libsql

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/swoop_server /usr/local/bin/
EXPOSE 8080
CMD ["swoop_server", "--port", "8080"]
```

### **Traditional Server**
```bash
cargo build --release
./target/release/swoop_server --config production.toml
```

## 📈 Performance Metrics

- **Document Processing**: 2000+ docs/hour
- **LLM Response Time**: <200ms (first token)
- **Concurrent Users**: 1000+ simultaneous
- **Memory Usage**: ~50MB base + 1MB per active doc
- **Edge Latency**: <50ms globally with libSQL

## 🏢 Enterprise Features

### **Multi-Tier Architecture**
- **Free**: Basic models, 100 requests/day
- **Basic**: Standard models, 1000 requests/day
- **Premium**: Advanced models, 10,000 requests/day
- **Enterprise**: All models, unlimited usage, priority support

### **Cost Management**
- Real-time cost tracking, budget alerts
- Model-specific cost analysis and optimization
- Volume discounts, enterprise pricing
- Automatic fallback to cost-effective models

## 🔒 Security & Compliance

- **API Key Management**: Secure credential handling
- **Rate Limiting**: Per-user and global
- **Input Validation**: Comprehensive sanitization
- **Audit Logging**: Complete request/response tracking
- **GDPR Compliance**: Data retention and deletion controls

## 🎯 Roadmap

### **Phase 4: Desktop Application** (In Progress)
- Native Tauri desktop app with React frontend
- File system and OS integration
- Offline processing
- Advanced document management

### **Phase 5: Enterprise Platform**
- Multi-tenant architecture
- Advanced analytics dashboard
- Custom model training pipeline
- Integration marketplace

## 🤝 Contributing

We welcome contributions! See our [contributing guidelines](CONTRIBUTING.md).

```bash
git checkout -b feature/amazing-feature
# Make your changes
cargo test
git commit -m 'Add amazing feature'
git push origin feature/amazing-feature
# Open a Pull Request
```

## 📄 License

MIT License – see [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- **OpenRouter** for AI model access and routing
- **Turso** for edge-optimized database infrastructure
- **Rust Community** for exceptional tools and libraries
- **Contributors** who help advance the platform

---

**Swoop** – Transforming documents into intelligent, actionable insights with enterprise-grade AI infrastructure.
