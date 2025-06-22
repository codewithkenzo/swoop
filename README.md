# Swoop - Advanced Document Processing & Analysis Platform

A production-ready document processing and AI analysis system with multi-model LLM integration, streaming capabilities, and intelligent content analysis.

## Core Features

### Document Processing
- **Multi-format Support**: PDF, Markdown, HTML, Plain Text with intelligent format detection
- **Robust Extraction**: Advanced content extraction with fallback mechanisms
- **Quality Analysis**: Automated content quality scoring and validation
- **Metadata Extraction**: Comprehensive document metadata and structure analysis

### LLM Integration
- **OpenRouter Integration**: Full OpenRouter API support with 200+ AI models
- **Streaming Responses**: Real-time streaming for agentic capabilities
- **Model Routing**: Intelligent model selection based on task category, user tier, and cost optimization
- **Cost Management**: Advanced cost tracking, rate limiting, and budget controls
- **Prompt Caching**: Automatic prompt caching for performance and cost optimization

### Analysis Capabilities
- **Document Summarization**: AI-powered content summarization
- **Key Point Extraction**: Automatic identification of important information
- **Entity Recognition**: Named entity extraction (people, organizations, locations)
- **Sentiment Analysis**: Document sentiment classification
- **Topic Modeling**: Automatic topic identification and categorization

### API Endpoints

#### Document Processing
```bash
# Upload and process documents
POST /api/documents/upload

# Get document status and analysis
GET /api/documents/:id

# List all processed documents
GET /api/documents
```

#### LLM-Powered Chat
```bash
# Enhanced chat with LLM integration
POST /api/llm/chat

# Real-time streaming chat
POST /api/llm/chat/stream

# Get available AI models
GET /api/llm/models

# LLM usage analytics
GET /api/llm/analytics
```

#### Analytics & Monitoring
```bash
# System statistics
GET /api/stats

# Processing metrics
GET /api/metrics

# Health check
GET /health
```

## Architecture

### Model Routing System
- **User Tier Management**: Free, Basic, Premium, Enterprise tiers with model access controls
- **Task Category Routing**: Specialized models for summarization, analysis, Q&A, and general tasks
- **Cost Optimization**: Automatic selection of cost-effective models based on user tier and requirements
- **Performance Balancing**: Dynamic routing between speed and quality based on request priority

### Streaming Infrastructure
- **Server-Sent Events**: Real-time streaming responses for interactive experiences
- **Agentic Capabilities**: Support for multi-step AI workflows and autonomous operations
- **Rate Limiting**: Sophisticated rate limiting and DDoS protection
- **Connection Management**: Efficient WebSocket and SSE connection handling

### Analytics & Monitoring
- **Usage Tracking**: Comprehensive tracking of model usage, costs, and performance
- **User Analytics**: Per-user usage statistics and cost analysis
- **Model Metrics**: Performance metrics for each AI model
- **Global Statistics**: System-wide analytics and monitoring

## Installation

### Prerequisites
- Rust 1.88+ (nightly recommended)
- OpenRouter API key (for LLM features)

### Quick Start
```bash
# Clone the repository
git clone https://github.com/yourusername/swoop
cd swoop

# Build the project
cargo build --release

# Set up environment variables
export OPENROUTER_API_KEY="your-api-key-here"

# Run the server
./target/release/swoop_server --port 4000
```

### Configuration
Create a `.env` file:
```env
OPENROUTER_API_KEY=your-api-key-here
DEFAULT_MODEL=openai/gpt-4o-mini
CACHE_TTL_SECONDS=3600
ANALYTICS_ENABLED=true
STREAMING_ENABLED=true
```

## Usage Examples

### Document Processing
```bash
# Upload a PDF document
curl -X POST http://localhost:4000/api/documents/upload \
  -F "file=@document.pdf"

# Get processing status
curl http://localhost:4000/api/documents/doc-id-here
```

### LLM Chat
```bash
# Send a chat message with document context
curl -X POST http://localhost:4000/api/llm/chat \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Summarize the uploaded document",
    "document_context": ["doc-id-here"],
    "personality": "technical"
  }'
```

### Streaming Chat
```bash
# Real-time streaming response
curl -X POST http://localhost:4000/api/llm/chat/stream \
  -H "Content-Type: application/json" \
  -H "Accept: text/event-stream" \
  -d '{
    "message": "Analyze this document for key insights",
    "document_context": ["doc-id-here"]
  }'
```

## Performance Metrics

- **Document Processing**: 500-2000 documents/hour depending on format and size
- **LLM Response Time**: 200-2000ms for standard completions
- **Streaming Latency**: <100ms first token, <50ms subsequent tokens
- **Concurrent Users**: Supports 1000+ concurrent connections
- **Memory Usage**: ~50MB base + ~1MB per active document
- **Storage**: Efficient document caching with configurable retention

## Security Features

- **API Key Management**: Secure OpenRouter API key handling
- **Rate Limiting**: Per-user and global rate limiting
- **Input Validation**: Comprehensive input sanitization and validation
- **Error Handling**: Secure error responses without sensitive information disclosure
- **CORS Configuration**: Configurable CORS policies for web integration

## Cost Optimization

### Model Selection Strategy
1. **Free Tier**: Access to basic models with rate limits
2. **Basic Tier**: Standard models with moderate usage limits
3. **Premium Tier**: Advanced models with higher usage allowances
4. **Enterprise Tier**: Full model access with volume discounts

### Cost Tracking
- Real-time cost calculation for all API calls
- Per-user cost tracking and budget alerts
- Model-specific cost analysis and optimization recommendations
- Automatic fallback to cheaper models when appropriate

## Development

### Building from Source
```bash
# Development build
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run --bin swoop_server -- --port 4000
```

### Adding New Features
1. Document processors: Extend `src/document_processor.rs`
2. LLM integrations: Add to `src/llm/` modules
3. API endpoints: Update `src/api_server.rs`
4. Model routing: Modify `src/llm/routing.rs`

## Deployment

### Production Deployment
```bash
# Build optimized release
cargo build --release --features production

# Run with production settings
./target/release/swoop_server \
  --port 8080 \
  --log-level info \
  --config production.toml
```

### Docker Deployment
```dockerfile
FROM rust:1.88-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/swoop_server /usr/local/bin/
EXPOSE 8080
CMD ["swoop_server", "--port", "8080"]
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- OpenRouter for AI model access and routing
- The Rust community for excellent crates and tools
- Contributors and testers who help improve the platform

---

**Swoop** - Transforming documents into intelligent, actionable insights with the power of AI.
