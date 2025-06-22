# Swoop User Guide

## Table of Contents
1. [Getting Started](#getting-started)
2. [Document Processing](#document-processing)
3. [LLM Integration](#llm-integration)
4. [Storage Backends](#storage-backends)
5. [API Reference](#api-reference)
6. [Advanced Usage](#advanced-usage)
7. [Troubleshooting](#troubleshooting)

## Getting Started

### Installation

#### System Requirements
- Rust 1.88+ (nightly recommended)
- 4GB RAM minimum (8GB recommended for large documents)
- 100MB disk space for installation
- OpenRouter API key for LLM features
- Network access for web crawling features

#### Quick Installation
```bash
# Clone repository
git clone https://github.com/codewithkenzo/swoop.git
cd swoop

# Build optimized binary (choose your deployment target)
cargo build --release --features libsql  # For edge/serverless
cargo build --release                    # For traditional server

# Configure environment
cp .env.example .env
# Edit .env with your OpenRouter API key

# Start server
./target/release/swoop_server --port 4000
```

#### Verify Installation
```bash
# Check server status
curl http://localhost:4000/health

# Expected response:
# {"status":"healthy","version":"0.3.0","uptime_seconds":5}
```

### Basic Configuration

#### Environment Variables
```bash
export OPENROUTER_API_KEY=sk-or-v1-xxx    # Required for LLM features
export STORAGE_BACKEND=libsql              # or sqlite/memory
export DATABASE_URL=libsql://your-db.turso.io
export DEFAULT_MODEL=openai/gpt-4o-mini
export ANALYTICS_ENABLED=true
export STREAMING_ENABLED=true
export SWOOP_PORT=4000                     # Server port
export SWOOP_LOG_LEVEL=info               # Logging: debug, info, warn, error
```

#### Command Line Options
```bash
# Start with custom port
swoop_server --port 8080

# Start with configuration file
swoop_server --config production.toml

# Start with verbose logging
RUST_LOG=debug swoop_server --port 4000
```

## Document Processing

### Supported Formats
- **HTML**: Full parsing with content extraction and metadata analysis
- **Plain Text**: Direct processing with intelligent structure detection
- **Markdown**: Structured content processing with heading extraction
- **PDF**: Text extraction with layout preservation (via external libraries)

### Upload Methods

#### Single Document Upload
```bash
# Upload HTML file
curl -F "file=@document.html" \
     -H "Content-Type: multipart/form-data" \
     http://localhost:4000/api/documents/upload

# Response includes document ID and initial analysis
{
  "id": "doc_abc123",
  "status": "processed",
  "analysis": {
    "word_count": 1250,
    "character_count": 8500,
    "readability_score": 65.2
  }
}
```

#### Batch Upload with Progress Tracking
```bash
# Upload multiple files with streaming progress
for file in documents/*.html; do
    echo "Processing $file..."
    
    response=$(curl -s -F "file=@$file" \
                    http://localhost:4000/api/documents/upload)
    
    doc_id=$(echo $response | jq -r '.id')
    echo "Document ID: $doc_id"
    
    # Monitor processing progress
    curl -N http://localhost:4000/api/documents/$doc_id/stream
done
```

### Document Analysis

#### AI-Powered Analysis
```bash
# Get comprehensive document analysis
curl -X POST \
     -H "Content-Type: application/json" \
     -d '{
       "analysis_type": "comprehensive",
       "include_summary": true,
       "include_entities": true,
       "include_sentiment": true
     }' \
     http://localhost:4000/api/documents/{document_id}/analyze

# Response includes:
# - AI-generated summary
# - Named entity recognition
# - Sentiment analysis
# - Topic classification
# - Readability metrics
```

#### Real-Time Processing Status
```bash
# Stream processing status
curl -N http://localhost:4000/api/documents/{document_id}/stream

# Server-Sent Events format:
# event: progress
# data: {"stage": "extracting", "progress": 0.3}
#
# event: complete
# data: {"status": "processed", "analysis": {...}}
```

## LLM Integration

### OpenRouter Model Access

#### Available Models
```bash
# List all available models
curl http://localhost:4000/api/llm/models

# Filter by capabilities
curl "http://localhost:4000/api/llm/models?capability=document_analysis"

# Get model details
curl http://localhost:4000/api/llm/models/openai/gpt-4o-mini
```

#### Chat with Document Context
```bash
# Chat with specific document
curl -X POST \
     -H "Content-Type: application/json" \
     -d '{
       "messages": [
         {"role": "user", "content": "Summarize the key points"}
       ],
       "document_ids": ["doc_abc123"],
       "model": "openai/gpt-4o-mini"
     }' \
     http://localhost:4000/api/llm/chat

# Streaming chat responses
curl -X POST \
     -H "Content-Type: application/json" \
     -d '{
       "messages": [
         {"role": "user", "content": "What are the main themes?"}
       ],
       "document_ids": ["doc_abc123"],
       "stream": true
     }' \
     http://localhost:4000/api/llm/chat/stream
```

### User Tier Management

#### Tier Capabilities
```bash
# Free Tier: Basic models, 100 requests/day
# Basic Tier: Standard models, 1000 requests/day  
# Premium Tier: Advanced models, 10,000 requests/day
# Enterprise Tier: All models, unlimited usage

# Check current tier and usage
curl -H "Authorization: Bearer your-api-key" \
     http://localhost:4000/api/user/tier

# Response:
{
  "tier": "premium",
  "usage": {
    "requests_today": 245,
    "requests_limit": 10000,
    "cost_today": 2.45
  }
}
```

## Storage Backends

### libSQL (Recommended for Production)

#### Turso Setup
```bash
# Install Turso CLI
curl -sSfL https://get.tur.so/install.sh | bash

# Create database
turso db create swoop-production
turso db show swoop-production

# Get connection details
turso db tokens create swoop-production

# Configure Swoop
export DATABASE_URL="libsql://swoop-production-[username].turso.io"
export TURSO_AUTH_TOKEN="your-auth-token"
```

#### Local Development with libSQL
```bash
# Use local libSQL file
export DATABASE_URL="file:./swoop.db"
cargo run --features libsql
```

### SQLite (Traditional)
```bash
# Configure SQLite backend
export STORAGE_BACKEND=sqlite
export DATABASE_URL="sqlite:./swoop.sqlite"
cargo run --features sqlite
```

### Memory Storage (Development)
```bash
# In-memory storage (data not persisted)
export STORAGE_BACKEND=memory
cargo run
```

## API Reference

### Authentication
```bash
# API key authentication
curl -H "Authorization: Bearer your-api-key" \
     http://localhost:4000/api/protected-endpoint
```

### Document Endpoints

#### Core Operations
```bash
# Upload document
POST /api/documents/upload
Content-Type: multipart/form-data
Body: file=@document.html

# Get document details
GET /api/documents/{id}

# List documents with filtering
GET /api/documents?limit=50&offset=0&format=html

# Analyze document with AI
POST /api/documents/{id}/analyze
Body: {"analysis_type": "comprehensive"}

# Stream processing status
GET /api/documents/{id}/stream
Accept: text/event-stream

# Delete document
DELETE /api/documents/{id}
```

#### Search and Query
```bash
# Search documents by content
GET /api/documents/search?q=machine+learning&limit=20

# Advanced search with filters
POST /api/documents/search
Body: {
  "query": "artificial intelligence",
  "filters": {
    "date_range": "2024-01-01:2024-12-31",
    "content_type": "html",
    "min_word_count": 1000
  }
}
```

### LLM Endpoints

#### Chat Interface
```bash
# Standard chat
POST /api/llm/chat
Body: {
  "messages": [...],
  "model": "openai/gpt-4o-mini",
  "document_context": ["doc_123"]
}

# Streaming chat
POST /api/llm/chat/stream
Body: {
  "messages": [...],
  "stream": true
}
```

#### Model Management
```bash
# List available models
GET /api/llm/models

# Get model pricing
GET /api/llm/models/{model_id}/pricing

# Model performance metrics
GET /api/llm/models/{model_id}/metrics
```

#### Analytics
```bash
# Usage analytics
GET /api/llm/analytics
Query params: ?start_date=2024-01-01&end_date=2024-01-31

# Cost breakdown
GET /api/llm/analytics/costs

# Performance metrics
GET /api/llm/analytics/performance
```

### System Endpoints
```bash
# Health check
GET /health

# System statistics
GET /api/stats

# Performance metrics
GET /api/metrics

# Configuration
GET /api/config
```

## Advanced Usage

### Batch Processing with AI Analysis

#### Intelligent Document Processing Pipeline
```python
import requests
import json
import time

class SwoopClient:
    def __init__(self, base_url="http://localhost:4000", api_key=None):
        self.base_url = base_url
        self.headers = {}
        if api_key:
            self.headers['Authorization'] = f'Bearer {api_key}'
    
    def process_document_batch(self, file_paths, analysis_config):
        """Process multiple documents with AI analysis"""
        results = []
        
        for file_path in file_paths:
            # Upload document
            with open(file_path, 'rb') as f:
                files = {'file': f}
                response = requests.post(
                    f"{self.base_url}/api/documents/upload",
                    files=files,
                    headers=self.headers
                )
            
            doc_data = response.json()
            doc_id = doc_data['id']
            
            # AI analysis
            analysis_response = requests.post(
                f"{self.base_url}/api/documents/{doc_id}/analyze",
                json=analysis_config,
                headers=self.headers
            )
            
            results.append({
                'file': file_path,
                'document_id': doc_id,
                'analysis': analysis_response.json()
            })
            
            time.sleep(0.1)  # Rate limiting
        
        return results

# Usage
client = SwoopClient(api_key="your-api-key")
results = client.process_document_batch(
    ['doc1.html', 'doc2.html', 'doc3.html'],
    {
        'analysis_type': 'comprehensive',
        'include_summary': True,
        'include_entities': True
    }
)
```

### Performance Optimization

#### Concurrent Processing
```bash
# Process documents in parallel with xargs
find documents/ -name "*.html" | \
xargs -P 4 -I {} bash -c '
    response=$(curl -s -F "file=@{}" http://localhost:4000/api/documents/upload)
    doc_id=$(echo $response | jq -r ".id")
    curl -s -X POST http://localhost:4000/api/documents/$doc_id/analyze
'
```

#### Connection Pooling and Caching
```bash
# Monitor connection pool status
curl http://localhost:4000/api/stats | jq '.connection_pool'

# Cache performance metrics
curl http://localhost:4000/api/metrics | jq '.cache_stats'
```

### Integration Examples

#### Node.js Integration with Streaming
```javascript
const axios = require('axios');
const FormData = require('form-data');
const EventSource = require('eventsource');

class SwoopClient {
    constructor(baseUrl = 'http://localhost:4000', apiKey = null) {
        this.baseUrl = baseUrl;
        this.headers = apiKey ? { 'Authorization': `Bearer ${apiKey}` } : {};
    }
    
    async uploadDocument(filePath) {
        const form = new FormData();
        form.append('file', fs.createReadStream(filePath));
        
        const response = await axios.post(
            `${this.baseUrl}/api/documents/upload`,
            form,
            { 
                headers: { 
                    ...form.getHeaders(),
                    ...this.headers 
                }
            }
        );
        
        return response.data;
    }
    
    async chatWithDocument(docId, message) {
        const response = await axios.post(
            `${this.baseUrl}/api/llm/chat`,
            {
                messages: [{ role: 'user', content: message }],
                document_ids: [docId]
            },
            { headers: this.headers }
        );
        
        return response.data;
    }
    
    streamProcessing(docId, callback) {
        const eventSource = new EventSource(
            `${this.baseUrl}/api/documents/${docId}/stream`
        );
        
        eventSource.onmessage = (event) => {
            const data = JSON.parse(event.data);
            callback(data);
        };
        
        return eventSource;
    }
}

// Usage
const client = new SwoopClient('http://localhost:4000', 'your-api-key');

// Upload and analyze
client.uploadDocument('document.html')
    .then(doc => {
        console.log('Document uploaded:', doc.id);
        
        // Stream processing status
        client.streamProcessing(doc.id, (status) => {
            console.log('Processing status:', status);
        });
        
        // Chat with document
        return client.chatWithDocument(doc.id, 'What are the main topics?');
    })
    .then(response => console.log('AI Response:', response));
```

## Troubleshooting

### Common Issues

#### Server Won't Start
```bash
# Check if port is in use
netstat -tlnp | grep :4000

# Kill existing process
pkill -f swoop_server

# Start with different port
swoop_server --port 4001
```

#### OpenRouter API Issues
```bash
# Test API key
curl -H "Authorization: Bearer $OPENROUTER_API_KEY" \
     https://openrouter.ai/api/v1/models

# Check API quota
curl -H "Authorization: Bearer $OPENROUTER_API_KEY" \
     https://openrouter.ai/api/v1/auth/key
```

#### Storage Backend Issues
```bash
# libSQL connection test
curl -X POST \
     -H "Content-Type: application/json" \
     -d '{"query": "SELECT 1"}' \
     "$DATABASE_URL/v1/execute"

# SQLite file permissions
ls -la swoop.sqlite
chmod 666 swoop.sqlite
```

### Performance Issues

#### Monitor System Resources
```bash
# Check memory usage
curl http://localhost:4000/api/stats | jq '.memory_usage'

# Monitor processing queue
curl http://localhost:4000/api/metrics | jq '.processing_queue'

# Database performance
curl http://localhost:4000/api/metrics | jq '.database_stats'
```

#### Optimize Configuration
```bash
# Reduce concurrent processing
export SWOOP_WORKER_THREADS=2

# Increase memory limits
export SWOOP_MAX_DOCUMENT_SIZE=50MB

# Adjust rate limiting
export SWOOP_RATE_LIMIT=100/minute
```

### Debugging

#### Enable Debug Logging
```bash
RUST_LOG=debug swoop_server --port 4000 2>&1 | tee swoop.log
```

#### API Response Debugging
```bash
# Verbose curl output
curl -v -F "file=@document.html" http://localhost:4000/api/documents/upload

# Monitor streaming responses
curl -N -v http://localhost:4000/api/documents/doc_123/stream
```

### Getting Help

1. **Check Logs**: Enable debug logging for detailed error messages
2. **Verify API Keys**: Ensure OpenRouter API key is valid and has credits
3. **Test Storage**: Verify database connectivity and permissions
4. **Monitor Resources**: Check CPU, memory, and disk usage
5. **Review Configuration**: Ensure environment variables are set correctly

For additional support, visit the [GitHub Issues](https://github.com/codewithkenzo/swoop/issues) page. 