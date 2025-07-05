# Quick Start Guide

Get Swoop running in under 2 minutes and process your first document.

## 🚀 One-Command Install

```bash
curl -fsSL https://install.swoop.dev | bash
```

**What this does:**
- Downloads the latest Swoop binary
- Sets up environment configuration
- Starts the server on `localhost:8080`
- Provides your API key

## 📋 Prerequisites

- **OS**: Linux, macOS, or Windows (WSL)
- **Memory**: 2GB+ available RAM
- **Storage**: 1GB+ free space
- **Network**: Internet connection for AI services

## 🎯 Alternative Installation Methods

### Docker (Recommended for Production)

```bash
# Quick start with Docker Compose
curl -o docker-compose.yml https://raw.githubusercontent.com/your-org/swoop/main/docker-compose.yml
docker-compose up -d

# Your server is now running at http://localhost:8080
```

### Manual Installation

```bash
# 1. Download binary
wget https://github.com/your-org/swoop/releases/latest/download/swoop-linux-x64.tar.gz
tar xzf swoop-linux-x64.tar.gz

# 2. Set up environment
cp .env.example .env
# Edit .env with your API keys

# 3. Run server
./swoop_server
```

### From Source

```bash
# 1. Clone repository
git clone https://github.com/your-org/swoop.git
cd swoop

# 2. Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 3. Build and run
cargo run --release --bin swoop_server
```

## 🔑 Get Your API Key

After installation, your API key is displayed in the terminal:

```bash
🎉 Swoop is running!
📍 Server: http://localhost:8080
🔑 API Key: swoop_sk_1234567890abcdef...
```

**Keep this key secure!** You'll need it for all API requests.

## 📄 Your First Document

### 1. Upload a Document

```bash
# Upload a PDF
curl -X POST "http://localhost:8080/api/documents/upload" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -F "file=@document.pdf"

# Upload with metadata
curl -X POST "http://localhost:8080/api/documents/upload" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -F "file=@report.pdf" \
  -F 'metadata={"title":"Q4 Report","tags":["finance","quarterly"]}'
```

**Response:**
```json
{
  "id": "doc_abc123",
  "title": "document.pdf",
  "status": "processing",
  "created_at": "2024-01-15T10:30:00Z",
  "analysis": {
    "category": "business",
    "quality_score": 85,
    "word_count": 2547
  }
}
```

### 2. Check Processing Status

```bash
# Get document details
curl -H "Authorization: Bearer YOUR_API_KEY" \
  http://localhost:8080/api/documents/doc_abc123
```

**When complete:**
```json
{
  "id": "doc_abc123",
  "title": "document.pdf",
  "status": "completed",
  "analysis": {
    "category": "business",
    "confidence": 0.92,
    "quality_score": 85,
    "sentiment": "neutral",
    "key_topics": ["revenue", "growth", "market analysis"],
    "entities": [
      {"text": "Q4 2023", "type": "date"},
      {"text": "Microsoft", "type": "organization"}
    ],
    "summary": "This document analyzes Q4 performance..."
  }
}
```

### 3. Search Your Documents

```bash
# Simple search
curl -H "Authorization: Bearer YOUR_API_KEY" \
  "http://localhost:8080/api/search?q=revenue+growth"

# Advanced search with filters
curl -H "Authorization: Bearer YOUR_API_KEY" \
  "http://localhost:8080/api/search?q=artificial+intelligence&category=technical&limit=5"
```

### 4. Chat with Your Document

```bash
# Ask questions about the document
curl -X POST "http://localhost:8080/api/documents/doc_abc123/chat" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"message": "What are the main financial highlights?"}'
```

**Response:**
```json
{
  "response": "Based on the document, the main financial highlights include: 1) Revenue increased 15% year-over-year to $2.1B, 2) Net profit margin improved to 12.3%, and 3) Cash flow from operations reached $450M...",
  "model": "gpt-4o",
  "sources": ["doc_abc123"],
  "timestamp": "2024-01-15T10:35:00Z"
}
```

## 🌐 Web Interface

Access the web dashboard at `http://localhost:8080`:

1. **📊 Dashboard** - Overview of all documents
2. **📤 Upload** - Drag-and-drop file upload
3. **🔍 Search** - Visual search interface
4. **💬 Chat** - Interactive document Q&A

## 📱 Real-time Updates

Monitor document processing in real-time:

```javascript
// JavaScript example for live updates
const eventSource = new EventSource(
  'http://localhost:8080/api/documents/doc_abc123/stream',
  {
    headers: {
      'Authorization': 'Bearer YOUR_API_KEY'
    }
  }
);

eventSource.onmessage = (event) => {
  const progress = JSON.parse(event.data);
  console.log(`Processing: ${progress.percentage}%`);
  
  if (progress.completed) {
    console.log('Document processing completed!');
    eventSource.close();
  }
};
```

## 🕷️ Web Crawling

Crawl websites to extract and analyze content:

```bash
# Start crawling a website
curl -X POST "http://localhost:8080/api/crawl" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://example.com",
    "max_depth": 2,
    "max_pages": 10,
    "include_patterns": ["*/blog/*", "*/docs/*"]
  }'

# Check crawl status
curl -H "Authorization: Bearer YOUR_API_KEY" \
  http://localhost:8080/api/crawl/job_xyz789/status

# Get crawl results
curl -H "Authorization: Bearer YOUR_API_KEY" \
  http://localhost:8080/api/crawl/job_xyz789/results
```

## 🎤 Voice Features

Generate audio from your documents:

```bash
# Get document audio (TTS)
curl -H "Authorization: Bearer YOUR_API_KEY" \
  "http://localhost:8080/api/audio/doc_abc123?voice=female&format=mp3" \
  --output document_audio.mp3

# Voice chat (upload audio, get audio response)
curl -X POST "http://localhost:8080/api/voice-chat" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -F "audio=@question.wav" \
  -F "document_id=doc_abc123"
```

## ⚙️ Configuration

### Environment Variables

Create a `.env` file or set environment variables:

```bash
# Required
OPENROUTER_API_KEY=your_openrouter_key
ELEVENLABS_API_KEY=your_elevenlabs_key

# Optional
DATABASE_URL=sqlite:./swoop.db
REDIS_URL=redis://localhost:6379
LOG_LEVEL=info
MAX_UPLOAD_SIZE=10485760  # 10MB
WORKER_THREADS=4
```

### Server Options

```bash
# Custom port
./swoop_server --port 3000

# Custom database
./swoop_server --database-url postgresql://user:pass@localhost:5432/swoop

# Development mode with debug logging
RUST_LOG=debug ./swoop_server

# Production mode
./swoop_server --release
```

## 🔧 Health Check

Verify your installation:

```bash
# Basic health check
curl http://localhost:8080/health

# Detailed system status
curl -H "Authorization: Bearer YOUR_API_KEY" \
  http://localhost:8080/api/status
```

**Healthy response:**
```json
{
  "status": "healthy",
  "version": "0.2.0",
  "uptime": 3600,
  "memory_usage": {
    "used": 524288000,
    "total": 8589934592
  },
  "storage": {
    "documents": 42,
    "total_size": 10485760
  }
}
```

## 🐛 Troubleshooting

### Common Issues

**Port already in use:**
```bash
# Kill existing process
pkill swoop_server
# Or use different port
./swoop_server --port 8081
```

**Database connection error:**
```bash
# Reset database
rm swoop.db
./swoop_server  # Will recreate database
```

**API key not working:**
```bash
# Check server logs
tail -f swoop.log
# Restart server to get new key
```

**Out of memory:**
```bash
# Reduce worker threads
WORKER_THREADS=2 ./swoop_server
```

### Getting Help

- **Logs**: Check `swoop.log` for detailed error messages
- **Documentation**: Visit full docs at `/docs`
- **Issues**: Report bugs on GitHub
- **Discord**: Join our community for real-time help

## 📚 What's Next?

Now that you have Swoop running:

1. **📖 [API Documentation](./openapi.yaml)** - Explore all endpoints
2. **💻 [Integration Examples](./INTEGRATION_EXAMPLES.md)** - Client libraries and code samples
3. **🏗️ [Architecture Guide](./ARCHITECTURE.md)** - Understanding the system
4. **⚡ [Performance Tuning](./PERFORMANCE_TUNING.md)** - Optimization for production
5. **🚀 [Deployment Guide](./ARCHITECTURE.md#deployment-architecture)** - Production deployment

## 🎯 Quick Examples

### Batch Upload
```bash
# Upload multiple files
for file in *.pdf; do
  curl -X POST "http://localhost:8080/api/documents/upload" \
    -H "Authorization: Bearer YOUR_API_KEY" \
    -F "file=@$file" &
done
wait
```

### Document Analysis
```bash
# Get detailed analysis
curl -H "Authorization: Bearer YOUR_API_KEY" \
  "http://localhost:8080/api/documents?category=technical&quality_min=80"
```

### Smart Search
```bash
# Find similar documents
curl -H "Authorization: Bearer YOUR_API_KEY" \
  "http://localhost:8080/api/search?q=machine+learning&semantic_weight=0.8"
```

---

**🎉 Congratulations!** You now have a fully functional document intelligence platform. Start uploading documents and exploring the AI-powered features.