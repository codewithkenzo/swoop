# Frequently Asked Questions

## General Questions

### What is Swoop?

Swoop is an AI-powered document intelligence platform that transforms documents into searchable, actionable insights. It provides document processing, AI analysis, semantic search, and chat capabilities through a single REST API.

### How is Swoop different from other document processing tools?

- **AI-Native**: Built-in integration with 200+ AI models for analysis
- **Real-time**: Live progress updates and streaming responses
- **Hybrid Search**: Combines keyword and semantic search
- **Production Ready**: Enterprise security, monitoring, and scalability
- **Open Source**: Full source code available on GitHub

### What document formats are supported?

Currently supported formats:
- PDF files
- HTML documents
- Markdown files
- Plain text files

Additional formats (coming soon):
- Microsoft Word (.docx)
- PowerPoint (.pptx)
- Excel (.xlsx)
- Images with OCR

### Is Swoop free to use?

Swoop is open source and free to self-host. You only pay for:
- AI model usage (OpenRouter API)
- Text-to-speech (ElevenLabs API)
- Your own infrastructure costs

We also offer a managed cloud service with usage-based pricing.

## Installation & Setup

### What are the system requirements?

**Minimum:**
- CPU: 2 cores, 2.4 GHz
- RAM: 4GB
- Storage: 20GB SSD
- Network: 100 Mbps

**Recommended:**
- CPU: 8 cores, 3.2 GHz
- RAM: 16GB
- Storage: 100GB NVMe SSD
- Network: 1 Gbps

### Can I run Swoop without Docker?

Yes! Swoop provides multiple installation options:
- One-command installer (recommended)
- Docker/Docker Compose
- Manual binary installation
- Build from source

### Do I need API keys to run Swoop?

For basic document upload and storage: **No**

For AI features you'll need:
- **OpenRouter API key** - for AI analysis and chat (required for most features)
- **ElevenLabs API key** - for text-to-speech (optional)

### How do I get API keys?

**OpenRouter:**
1. Sign up at [openrouter.ai](https://openrouter.ai)
2. Add credits to your account
3. Copy your API key from the dashboard

**ElevenLabs:**
1. Sign up at [elevenlabs.io](https://elevenlabs.io)
2. Get free credits for testing
3. Copy your API key from settings

## Usage & Features

### How accurate is the AI analysis?

AI analysis accuracy varies by content type:
- **Categorization**: 90-95% accuracy
- **Entity Recognition**: 85-90% accuracy
- **Quality Scoring**: 80-85% correlation with human ratings
- **Sentiment Analysis**: 85-90% accuracy

Accuracy improves with higher-quality input documents.

### Can I use custom AI models?

Yes! Swoop supports:
- Any OpenRouter-compatible model
- Custom model endpoints (enterprise)
- Local model hosting (coming soon)

### How does hybrid search work?

Swoop combines two search methods:
1. **BM25 (keyword)**: Traditional full-text search
2. **Vector similarity**: Semantic understanding using 384-dimensional embeddings

Results are ranked using a weighted combination (default 50/50, configurable).

### Can I search across multiple languages?

Currently, Swoop works best with English documents. Multi-language support is planned for Q2 2024.

### How real-time are the updates?

- **Document processing**: Live progress via Server-Sent Events
- **Search results**: Sub-100ms response time
- **Chat responses**: Streaming with ~50ms first token

## Performance & Scaling

### How many documents can Swoop handle?

Performance depends on your infrastructure:
- **Single server**: 10,000-100,000 documents
- **Clustered setup**: 1,000,000+ documents
- **Cloud deployment**: Virtually unlimited with auto-scaling

### What's the maximum file size?

Default limits:
- **File size**: 10MB per document
- **Content length**: 1M characters after extraction
- **Batch uploads**: 50 files per request

These limits are configurable in production deployments.

### How fast is document processing?

Typical processing times:
- **PDF (1-10 pages)**: 1-3 seconds
- **HTML page**: 0.5-1 second
- **Large documents (50+ pages)**: 5-15 seconds

Processing includes content extraction, AI analysis, and vector embedding generation.

### Can Swoop handle concurrent uploads?

Yes! Swoop is built for concurrency:
- **Default**: 10 concurrent uploads
- **Configurable**: Up to 100+ with adequate resources
- **Queue management**: Automatic overflow handling

## Security & Privacy

### Is my data secure?

Swoop implements enterprise-grade security:
- **Encryption**: Data encrypted at rest and in transit
- **Authentication**: API key-based access control
- **Rate limiting**: DDoS protection
- **Audit logging**: Complete activity tracking
- **No data sharing**: Your documents stay on your infrastructure

### Where is my data stored?

With self-hosted Swoop:
- **Documents**: Your local database (SQLite/PostgreSQL)
- **Embeddings**: Your vector database (Qdrant)
- **Metadata**: Your Redis cache

With managed cloud service:
- **Data residency**: Choose your region
- **Compliance**: SOC2, GDPR, CCPA ready
- **Backup**: Automated with encryption

### Do you train AI models on my documents?

**Never.** Swoop:
- Uses external AI APIs (OpenRouter) with no-training guarantees
- Doesn't store your content on AI provider servers
- Only sends necessary context for analysis
- Respects data processing agreements

### Can I run Swoop offline?

Partially. You can:
- Upload and store documents ✅
- Search stored content ✅
- View document metadata ✅

But you need internet for:
- AI analysis and chat ❌
- Text-to-speech ❌
- New embeddings generation ❌

## Troubleshooting

### Swoop won't start

**Check the basics:**
```bash
# Verify port isn't in use
lsof -i :8080

# Check system resources
free -h
df -h

# Review logs
tail -f swoop.log
```

**Common solutions:**
- Use a different port: `--port 8081`
- Increase memory: Close other applications
- Check disk space: Free up storage

### API requests fail with 401 Unauthorized

**Verify authentication:**
```bash
# Test with your API key
curl -H "Authorization: Bearer YOUR_KEY" \
  http://localhost:8080/api/status
```

**Common issues:**
- Missing `Bearer ` prefix in header
- Expired or invalid API key
- Special characters in key (URL encode)

### Document upload fails

**Check file format:**
```bash
# Verify file type
file document.pdf
```

**Common issues:**
- Unsupported file format
- File size exceeds limit (default 10MB)
- Corrupted file
- Network timeout on large files

### AI analysis returns errors

**Check API keys:**
```bash
# Test OpenRouter connection
curl -H "Authorization: Bearer YOUR_OPENROUTER_KEY" \
  https://openrouter.ai/api/v1/models
```

**Common issues:**
- Invalid or expired API keys
- Insufficient credits
- Rate limiting from AI provider
- Network connectivity issues

### Search returns no results

**Verify data:**
```bash
# Check if documents exist
curl -H "Authorization: Bearer YOUR_KEY" \
  http://localhost:8080/api/documents
```

**Common issues:**
- Documents not fully processed
- Search query too specific
- Embeddings not generated
- Database connectivity issues

### High memory usage

**Check memory consumption:**
```bash
# Monitor memory
top -p $(pgrep swoop_server)
```

**Optimization steps:**
- Reduce worker threads: `WORKER_THREADS=2`
- Limit concurrent uploads
- Increase swap space
- Restart server periodically

### Slow performance

**Performance checklist:**
- Adequate CPU cores for workload
- SSD storage for database
- Sufficient RAM (16GB+ recommended)
- Fast network connection
- Database optimization (indexes, vacuum)

## Integration & Development

### How do I integrate Swoop with my application?

Multiple integration options:
1. **REST API**: Direct HTTP requests
2. **Client libraries**: JavaScript, Python, Go
3. **Webhooks**: Event-driven notifications
4. **Web interface**: Embed Swoop UI components

### Can I customize the web interface?

Yes! The frontend is built with React:
- Fork the repository
- Modify components in `frontend/src/`
- Rebuild with `npm run build`
- Host your custom version

### How do I contribute to Swoop?

We welcome contributions!
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

See our [Contributing Guide](./CONTRIBUTING.md) for details.

### Is there a community?

Join our growing community:
- **GitHub**: [Issues and discussions](https://github.com/your-org/swoop)
- **Discord**: [Real-time chat](https://discord.gg/swoop)
- **Twitter**: [@swoopAI](https://twitter.com/swoopAI)

## Pricing & Licensing

### What's the license?

Swoop is released under the MIT License:
- ✅ Commercial use allowed
- ✅ Modification allowed
- ✅ Distribution allowed
- ✅ Private use allowed
- ❗ No warranty provided

### What are the costs?

**Self-hosted (free):**
- Swoop software: $0
- Infrastructure: Your costs
- AI APIs: Pay-per-use

**Managed cloud service:**
- Documents: $0.01 per document processed
- Storage: $0.10 per GB per month
- Chat: $0.001 per message
- Enterprise: Custom pricing

### Do you offer enterprise support?

Yes! Enterprise features include:
- Priority support
- Custom integrations
- On-premise deployment
- Training and consulting
- SLA guarantees

Contact us at enterprise@swoop.dev for details.

## Still Have Questions?

- **Documentation**: Check our [complete docs](/docs)
- **GitHub Issues**: [Report bugs or request features](https://github.com/your-org/swoop/issues)
- **Discord**: [Join our community](https://discord.gg/swoop)
- **Email**: [support@swoop.dev](mailto:support@swoop.dev)