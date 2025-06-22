# Swoop User Guide

## Table of Contents
1. [Getting Started](#getting-started)
2. [Document Processing](#document-processing)
3. [Web Crawling](#web-crawling)
4. [API Reference](#api-reference)
5. [Advanced Usage](#advanced-usage)
6. [Troubleshooting](#troubleshooting)

## Getting Started

### Installation

#### System Requirements
- Rust 1.75 or higher
- 4GB RAM minimum (8GB recommended for large documents)
- 100MB disk space for installation
- Network access for web crawling features

#### Quick Installation
```bash
# Clone repository
git clone https://github.com/your-org/swoop.git
cd swoop

# Build optimized binary
cargo build --release

# Start server
./target/release/swoop_server 3000
```

#### Verify Installation
```bash
# Check server status
curl http://localhost:3000/health

# Expected response:
# {"status":"healthy","version":"0.2.0","uptime_seconds":5}
```

### Basic Configuration

#### Environment Variables
```bash
export SWOOP_PORT=3000              # Server port (default: 3001)
export SWOOP_LOG_LEVEL=info         # Logging: debug, info, warn, error
export SWOOP_MAX_UPLOAD_SIZE=10MB   # Maximum file size
export SWOOP_WORKER_THREADS=4       # Processing threads
```

#### Command Line Options
```bash
# Start with custom port
swoop_server 8080

# Start with environment file
swoop_server --config .env

# Start with verbose logging
RUST_LOG=debug swoop_server
```

## Document Processing

### Supported Formats
- **HTML**: Full parsing with content extraction
- **Plain Text**: Direct processing with metadata analysis
- **PDF**: Text extraction with layout preservation (planned)
- **Markdown**: Structured content processing (planned)

### Upload Methods

#### Single Document Upload
```bash
# Upload HTML file
curl -F "file=@document.html" \
     -H "Content-Type: multipart/form-data" \
     http://localhost:3000/api/documents/upload

# Upload with custom filename
curl -F "file=@document.html;filename=custom-name.html" \
     http://localhost:3000/api/documents/upload
```

#### Batch Upload
```bash
# Upload multiple files
for file in *.html; do
    curl -F "file=@$file" http://localhost:3000/api/documents/upload
    sleep 0.1  # Rate limiting
done
```

### Document Analysis

#### Basic Analysis
```bash
# Get document analysis
curl -X POST http://localhost:3000/api/documents/{document_id}/analyze

# Response includes:
# - Word count and character statistics
# - Readability metrics (Flesch-Kincaid score)
# - Content classification
# - Language detection
# - Processing time metrics
```

#### Advanced Analysis Options
```bash
# Analysis with entity extraction
curl -X POST \
     -H "Content-Type: application/json" \
     -d '{"include_entities": true, "include_keywords": true}' \
     http://localhost:3000/api/documents/{document_id}/analyze

# Custom analysis parameters
curl -X POST \
     -H "Content-Type: application/json" \
     -d '{
       "analysis_depth": "comprehensive",
       "extract_summaries": true,
       "confidence_threshold": 0.8
     }' \
     http://localhost:3000/api/documents/{document_id}/analyze
```

### Content Extraction Examples

#### HTML Document Processing
Input HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>Sample Document</title>
    <script>console.log('removed');</script>
</head>
<body>
    <h1>Main Title</h1>
    <p>This is a sample paragraph with <strong>important</strong> content.</p>
    <style>.hidden { display: none; }</style>
</body>
</html>
```

Extracted Content:
```json
{
  "title": "Sample Document",
  "text": "Main Title\n\nThis is a sample paragraph with important content.",
  "word_count": 9,
  "character_count": 52,
  "metadata": {
    "has_title": true,
    "scripts_removed": 1,
    "styles_removed": 1
  }
}
```

## Web Crawling

### Basic Crawling

#### Single Page Crawl
```bash
curl -X POST \
     -H "Content-Type: application/json" \
     -d '{
       "start_url": "https://example.com",
       "max_pages": 1
     }' \
     http://localhost:3000/api/crawl
```

#### Site Crawling
```bash
curl -X POST \
     -H "Content-Type: application/json" \
     -d '{
       "start_url": "https://example.com",
       "max_depth": 3,
       "max_pages": 100,
       "rate_limit_ms": 1000,
       "follow_external": false
     }' \
     http://localhost:3000/api/crawl
```

### Crawl Configuration

#### Rate Limiting
```json
{
  "rate_limit_ms": 1000,        // Delay between requests
  "concurrent_requests": 5,      // Simultaneous connections
  "respect_robots_txt": true,    // Honor robots.txt
  "user_agent": "Swoop/0.2.0"   // Custom user agent
}
```

#### Content Filtering
```json
{
  "include_patterns": ["*.html", "*.htm"],
  "exclude_patterns": ["*/admin/*", "*/private/*"],
  "min_content_length": 100,
  "max_content_length": 1048576
}
```

### Monitoring Crawl Progress

#### Check Crawl Status
```bash
curl http://localhost:3000/api/crawl/{crawl_id}/status

# Response:
{
  "crawl_id": "crawl_abc123",
  "status": "running",
  "progress": {
    "pages_discovered": 47,
    "pages_processed": 23,
    "pages_failed": 2,
    "documents_extracted": 18
  },
  "performance": {
    "pages_per_minute": 15.2,
    "avg_response_time_ms": 850,
    "success_rate": 0.956
  }
}
```

## API Reference

### Authentication
Currently, Swoop operates without authentication. For production deployment, implement authentication middleware.

### Base URL
```
http://localhost:3000
```

### Error Handling
All API endpoints return structured error responses:

```json
{
  "error": {
    "code": "DOCUMENT_NOT_FOUND",
    "message": "Document with ID 'doc_123' not found",
    "details": {
      "document_id": "doc_123",
      "available_documents": 5
    }
  }
}
```

### Common HTTP Status Codes
- `200 OK`: Successful operation
- `201 Created`: Resource created successfully
- `400 Bad Request`: Invalid request parameters
- `404 Not Found`: Resource not found
- `413 Payload Too Large`: File size exceeds limit
- `422 Unprocessable Entity`: Invalid file format
- `500 Internal Server Error`: Server processing error

### Endpoints

#### System Endpoints
```bash
# Health check
GET /health
Response: {"status": "healthy", "version": "0.2.0"}

# System status
GET /api/status
Response: {"documents": 15, "crawls_active": 2, "uptime": 3600}
```

#### Document Endpoints
```bash
# Upload document
POST /api/documents/upload
Content-Type: multipart/form-data
Body: file=@document.html

# List documents
GET /api/documents
Query parameters:
  - limit: number of results (default: 50)
  - offset: pagination offset (default: 0)
  - sort: sort field (created_at, size, filename)

# Get specific document
GET /api/documents/{id}

# Analyze document
POST /api/documents/{id}/analyze

# Delete document
DELETE /api/documents/{id}
```

#### Crawling Endpoints
```bash
# Start crawl
POST /api/crawl
Body: {"start_url": "https://example.com", ...}

# Get crawl status
GET /api/crawl/{id}/status

# Stop crawl
POST /api/crawl/{id}/stop

# Get crawl results
GET /api/crawl/{id}/results
```

## Advanced Usage

### Batch Processing

#### Process Multiple Documents
```bash
#!/bin/bash
# batch_process.sh

for file in documents/*.html; do
    echo "Processing $file..."
    
    # Upload document
    response=$(curl -s -F "file=@$file" \
                    http://localhost:3000/api/documents/upload)
    
    # Extract document ID
    doc_id=$(echo $response | jq -r '.id')
    
    # Analyze document
    curl -s -X POST \
         http://localhost:3000/api/documents/$doc_id/analyze \
         | jq '.analysis' > "results/${file%.html}.json"
    
    echo "Analysis saved to results/${file%.html}.json"
done
```

### Performance Optimization

#### Concurrent Processing
```bash
# Process documents in parallel
find documents/ -name "*.html" | \
xargs -P 4 -I {} bash -c '
    response=$(curl -s -F "file=@{}" http://localhost:3000/api/documents/upload)
    doc_id=$(echo $response | jq -r ".id")
    curl -s -X POST http://localhost:3000/api/documents/$doc_id/analyze
'
```

#### Memory Management
```bash
# Monitor memory usage
curl http://localhost:3000/api/status | jq '.memory_usage'

# Clear document cache (if implemented)
curl -X POST http://localhost:3000/api/cache/clear
```

### Integration Examples

#### Python Integration
```python
import requests
import json

class SwoopClient:
    def __init__(self, base_url="http://localhost:3000"):
        self.base_url = base_url
    
    def upload_document(self, file_path):
        with open(file_path, 'rb') as f:
            files = {'file': f}
            response = requests.post(
                f"{self.base_url}/api/documents/upload",
                files=files
            )
        return response.json()
    
    def analyze_document(self, doc_id):
        response = requests.post(
            f"{self.base_url}/api/documents/{doc_id}/analyze"
        )
        return response.json()

# Usage
client = SwoopClient()
doc = client.upload_document("sample.html")
analysis = client.analyze_document(doc['id'])
print(f"Word count: {analysis['analysis']['word_count']}")
```

#### Node.js Integration
```javascript
const axios = require('axios');
const FormData = require('form-data');
const fs = require('fs');

class SwoopClient {
    constructor(baseUrl = 'http://localhost:3000') {
        this.baseUrl = baseUrl;
    }
    
    async uploadDocument(filePath) {
        const form = new FormData();
        form.append('file', fs.createReadStream(filePath));
        
        const response = await axios.post(
            `${this.baseUrl}/api/documents/upload`,
            form,
            { headers: form.getHeaders() }
        );
        
        return response.data;
    }
    
    async analyzeDocument(docId) {
        const response = await axios.post(
            `${this.baseUrl}/api/documents/${docId}/analyze`
        );
        
        return response.data;
    }
}

// Usage
const client = new SwoopClient();
client.uploadDocument('sample.html')
    .then(doc => client.analyzeDocument(doc.id))
    .then(analysis => console.log(analysis));
```

## Troubleshooting

### Common Issues

#### Server Won't Start
```bash
# Check if port is in use
netstat -tlnp | grep :3000

# Kill existing process
pkill -f swoop_server

# Start with different port
swoop_server 3001
```

#### Upload Failures
```bash
# Check file size
ls -lh document.html

# Verify file format
file document.html

# Test with small file
echo "<html><body>Test</body></html>" > test.html
curl -F "file=@test.html" http://localhost:3000/api/documents/upload
```

#### Performance Issues
```bash
# Monitor system resources
htop

# Check server logs
RUST_LOG=debug swoop_server

# Reduce concurrent processing
export SWOOP_WORKER_THREADS=2
```

### Debugging

#### Enable Debug Logging
```bash
RUST_LOG=debug swoop_server 2>&1 | tee swoop.log
```

#### API Response Debugging
```bash
# Verbose curl output
curl -v -F "file=@document.html" http://localhost:3000/api/documents/upload

# Pretty print JSON responses
curl -s http://localhost:3000/api/documents | jq .
```

#### Performance Profiling
```bash
# Time API calls
time curl -F "file=@large_document.html" \
          http://localhost:3000/api/documents/upload

# Monitor processing time
curl -X POST http://localhost:3000/api/documents/{id}/analyze | \
jq '.processing_time_ms'
```

### Getting Help

1. **Check Logs**: Enable debug logging to see detailed error messages
2. **Verify Configuration**: Ensure environment variables are set correctly
3. **Test with Simple Cases**: Use small, well-formed documents first
4. **Check System Resources**: Monitor CPU and memory usage
5. **Review API Documentation**: Ensure correct request format

For additional support, visit the [GitHub Issues](https://github.com/your-org/swoop/issues) page. 