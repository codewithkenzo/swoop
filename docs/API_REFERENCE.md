# Swoop API Reference Documentation

## Overview

Swoop is an advanced document intelligence platform that provides comprehensive document processing, analysis, and chat capabilities. This API documentation covers all available endpoints, request/response formats, authentication requirements, and practical usage examples.

**Base URL:** `http://localhost:3001` (configurable via PORT environment variable)  
**API Version:** v1  
**Content-Type:** `application/json` (unless specified otherwise)

## Table of Contents

1. [Authentication & Security](#authentication--security)
2. [Core API Endpoints](#core-api-endpoints)
3. [Document Management](#document-management)
4. [Web Crawling](#web-crawling)
5. [Chat & AI Interface](#chat--ai-interface)
6. [Audio & TTS](#audio--tts)
7. [Server-Sent Events (SSE)](#server-sent-events-sse)
8. [Error Handling](#error-handling)
9. [Rate Limiting](#rate-limiting)
10. [Examples](#examples)

---

## Authentication & Security

### Current Status
- **Authentication:** None required (development mode)
- **CORS:** Permissive (all origins allowed)
- **Rate Limiting:** Not implemented yet

### Security Features
- Input validation and sanitization
- Safe file upload handling
- Content type detection
- Error message sanitization

---

## Core API Endpoints

### Service Information

#### GET /
Get service information and available endpoints.

**Response:**
```json
{
  "service": "Swoop Document Intelligence Platform",
  "version": "0.2.0",
  "status": "operational",
  "features": [
    "document_upload",
    "content_extraction",
    "text_analysis",
    "robust_processing",
    "chat_interface",
    "document_chat",
    "pdf_support",
    "markdown_support"
  ],
  "endpoints": {
    "health": "/health",
    "upload": "/api/documents/upload",
    "list": "/api/documents",
    "analyze": "/api/documents/{id}/analyze",
    "chat": "/api/chat",
    "document_chat": "/api/documents/{id}/chat"
  }
}
```

#### GET /health
Health check endpoint for monitoring and load balancers.

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-15T10:30:00Z",
  "uptime": "running",
  "service": "swoop-platform",
  "documents_loaded": 42,
  "features_active": [
    "document_processing",
    "text_extraction",
    "storage",
    "robust_parsing"
  ]
}
```

#### GET /api/status
Detailed API status information.

**Response:**
```json
{
  "api_version": "v1",
  "status": "active",
  "endpoints_available": 5,
  "documents_processed": 42,
  "capabilities": [
    "multipart_upload",
    "html_processing",
    "text_analysis",
    "error_recovery"
  ]
}
```

---

## Document Management

### Upload Document

#### POST /api/documents/upload
Upload and process a document file.

**Request:**
- **Content-Type:** `multipart/form-data`
- **Form Fields:**
  - `file`: File to upload (required)

**Supported File Types:**
- Text files (.txt, .md, .html)
- PDF documents
- Word documents
- Various text-based formats

**Response:**
```json
{
  "status": "success",
  "message": "Document uploaded and processed",
  "document": {
    "id": "doc_a1b2c3d4",
    "filename": "sample.pdf",
    "content_type": "application/pdf",
    "size_bytes": 1024000,
    "processed": true
  }
}
```

**Error Response:**
```json
{
  "error": "Invalid file format",
  "status": "error"
}
```

**Example cURL:**
```bash
curl -X POST \
  http://localhost:3001/api/documents/upload \
  -F 'file=@/path/to/document.pdf'
```

### List Documents

#### GET /api/documents
Retrieve a list of all processed documents.

**Response:**
```json
{
  "documents": [
    {
      "id": "doc_a1b2c3d4",
      "url": "https://example.com/document.pdf",
      "title": "Sample Document",
      "content_length": 5000,
      "extracted_at": "2024-01-15T10:30:00Z",
      "status": "processed"
    }
  ],
  "total_count": 1,
  "status": "success"
}
```

### Analyze Document

#### POST /api/documents/{id}/analyze
Perform detailed analysis on a specific document.

**Parameters:**
- `id` (path): Document ID

**Response:**
```json
{
  "status": "success",
  "analysis": {
    "document_id": "doc_a1b2c3d4",
    "statistics": {
      "word_count": 1250,
      "character_count": 7500,
      "line_count": 150,
      "sentence_count": 85,
      "avg_sentence_length": 88.2
    },
    "insights": {
      "readability": "moderate",
      "content_type": "medium_form",
      "language": "detected_english"
    },
    "summary": {
      "first_sentence": "This document provides an overview of...",
      "key_topics": ["document_analysis", "text_processing"]
    }
  }
}
```

---

## Web Crawling

### Start Crawl Job

#### POST /api/crawl
Start a new web crawling job.

**Request:**
```json
{
  "url": "https://example.com",
  "max_depth": 3,
  "max_pages": 100
}
```

**Response:**
```json
{
  "job_id": "crawl_xyz123"
}
```

### Get Crawl Status

#### GET /api/crawl/{job_id}
Check the status of a crawling job.

**Parameters:**
- `job_id` (path): Crawl job ID

**Response:**
```json
{
  "job_id": "crawl_xyz123",
  "status": "running",
  "urls_processed": 45,
  "successful_fetches": 42,
  "failed_fetches": 3,
  "documents_extracted": 38,
  "links_discovered": 156,
  "bytes_downloaded": 2048000,
  "avg_fetch_time_ms": 750.5
}
```

### Stop Crawl Job

#### POST /api/crawl/{job_id}/stop
Stop a running crawl job.

**Parameters:**
- `job_id` (path): Crawl job ID

**Response:**
```json
{
  "stopped": true
}
```

### Get Crawl Results

#### GET /api/crawl/{job_id}/results
Retrieve the results of a completed crawl job.

**Parameters:**
- `job_id` (path): Crawl job ID

**Response:**
```json
{
  "pages": [
    {
      "url": "https://example.com/page1",
      "status_code": 200,
      "text_length": 2500,
      "fetched_at": "2024-01-15T10:30:00Z"
    }
  ]
}
```

### List Crawl Jobs

#### GET /api/crawl
Get a list of all crawl jobs.

**Response:**
```json
{
  "jobs": [
    {
      "id": "crawl_xyz123",
      "status": "completed",
      "url": "https://example.com",
      "created_at": "2024-01-15T10:00:00Z"
    }
  ]
}
```

---

## Chat & AI Interface

### General Chat

#### POST /api/chat
Interact with the AI assistant for general queries.

**Request:**
```json
{
  "message": "How do I upload a document?",
  "context_window": 500
}
```

**Response:**
```json
{
  "status": "success",
  "chat": {
    "response": "To upload a document, send a POST request to /api/documents/upload with a multipart form containing your file.",
    "document_context": null,
    "confidence": 0.8,
    "sources": ["swoop_platform"],
    "timestamp": "2024-01-15T10:30:00Z"
  }
}
```

### Document-Specific Chat

#### POST /api/documents/{id}/chat
Chat with AI about a specific document.

**Parameters:**
- `id` (path): Document ID

**Request:**
```json
{
  "message": "What is the main topic of this document?",
  "context_window": 500
}
```

**Response:**
```json
{
  "status": "success",
  "document_id": "doc_a1b2c3d4",
  "chat": {
    "response": "Based on the document content, the main topic appears to be...",
    "document_context": "The document discusses various aspects of...",
    "confidence": 0.7,
    "sources": ["doc_a1b2c3d4"],
    "timestamp": "2024-01-15T10:30:00Z"
  }
}
```

**Supported Chat Commands:**
- `summary` or `summarize`: Get document summary
- `word count` or `length`: Get document statistics
- `search [term]` or `find [term]`: Search within document
- General questions about document content

---

## Audio & TTS

### Get Document Audio

#### GET /api/audio/{id}
Generate and retrieve audio version of a document.

**Parameters:**
- `id` (path): Document ID
- `voice` (query, optional): Voice ID (default: "en-us-female")

**Response:**
- **Content-Type:** `audio/wav`
- **Body:** Binary audio data

**Example:**
```bash
curl -X GET \
  "http://localhost:3001/api/audio/doc_a1b2c3d4?voice=en-us-female" \
  -o document_audio.wav
```

### Voice Chat (Streaming)

#### POST /api/voice-chat
Interactive voice chat with streaming responses.

**Request:**
```json
{
  "text": "Hello, how are you?",
  "voice": "en-us-female",
  "model": "default",
  "stream": true
}
```

**Response:**
- **Content-Type:** `text/event-stream`
- **Body:** Server-sent events with text tokens and audio data

**Stream Format:**
```
data: {"role":"assistant","text":"Hello","audio_b64":null}

data: {"role":"assistant","text":"there!","audio_b64":null}

data: {"role":"assistant","text":null,"audio_b64":"UklGRn4AAABXQVZFZm10..."}
```

---

## Server-Sent Events (SSE)

### Voice Chat Streaming

The `/api/voice-chat` endpoint uses Server-Sent Events to provide real-time streaming of both text tokens and audio data.

**Connection:**
```javascript
const eventSource = new EventSource('/api/voice-chat');
eventSource.onmessage = function(event) {
  const data = JSON.parse(event.data);
  if (data.text) {
    console.log('Text token:', data.text);
  }
  if (data.audio_b64) {
    console.log('Audio data received');
    // Process base64 audio data
  }
};
```

---

## Error Handling

### Error Response Format

All API endpoints return consistent error responses:

```json
{
  "error": "Error description",
  "status": "error",
  "code": "ERROR_CODE",
  "details": "Additional error details"
}
```

### HTTP Status Codes

- **200 OK**: Successful request
- **400 Bad Request**: Invalid request parameters
- **404 Not Found**: Resource not found
- **500 Internal Server Error**: Server error
- **501 Not Implemented**: Feature not available

### Common Error Scenarios

1. **File Upload Errors:**
   - Invalid file format
   - File too large
   - Corrupted file data

2. **Document Processing Errors:**
   - Document not found
   - Processing timeout
   - Extraction failure

3. **Crawling Errors:**
   - Invalid URL
   - Network timeout
   - Crawl job not found

---

## Rate Limiting

### Current Implementation
Rate limiting is not currently implemented but is planned for future releases.

### Recommended Client Behavior
- Implement client-side rate limiting
- Use exponential backoff for retries
- Monitor response times and adjust request frequency

---

## Examples

### Complete Document Processing Workflow

```javascript
// 1. Upload document
const formData = new FormData();
formData.append('file', fileInput.files[0]);

const uploadResponse = await fetch('/api/documents/upload', {
  method: 'POST',
  body: formData
});

const uploadResult = await uploadResponse.json();
const documentId = uploadResult.document.id;

// 2. Analyze document
const analysisResponse = await fetch(`/api/documents/${documentId}/analyze`, {
  method: 'POST'
});

const analysisResult = await analysisResponse.json();

// 3. Chat with document
const chatResponse = await fetch(`/api/documents/${documentId}/chat`, {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    message: "What are the key points in this document?",
    context_window: 500
  })
});

const chatResult = await chatResponse.json();
```

### Web Crawling Workflow

```javascript
// 1. Start crawl
const crawlResponse = await fetch('/api/crawl', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    url: 'https://example.com',
    max_depth: 2,
    max_pages: 50
  })
});

const crawlResult = await crawlResponse.json();
const jobId = crawlResult.job_id;

// 2. Monitor progress
const checkStatus = async () => {
  const statusResponse = await fetch(`/api/crawl/${jobId}`);
  const status = await statusResponse.json();
  
  if (status.status === 'completed') {
    // Get results
    const resultsResponse = await fetch(`/api/crawl/${jobId}/results`);
    const results = await resultsResponse.json();
    console.log('Crawl completed:', results);
  } else if (status.status === 'running') {
    // Check again in 5 seconds
    setTimeout(checkStatus, 5000);
  }
};

checkStatus();
```

### Voice Chat Integration

```javascript
// Initialize voice chat
const startVoiceChat = async (message) => {
  const response = await fetch('/api/voice-chat', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      text: message,
      voice: 'en-us-female',
      stream: true
    })
  });

  const reader = response.body.getReader();
  const decoder = new TextDecoder();

  while (true) {
    const { done, value } = await reader.read();
    if (done) break;

    const chunk = decoder.decode(value);
    const lines = chunk.split('\n');
    
    for (const line of lines) {
      if (line.startsWith('data: ')) {
        const data = JSON.parse(line.slice(6));
        
        if (data.text) {
          console.log('Text token:', data.text);
          // Display text in UI
        }
        
        if (data.audio_b64) {
          console.log('Audio received');
          // Play audio
          const audioBlob = new Blob([
            new Uint8Array(atob(data.audio_b64).split('').map(c => c.charCodeAt(0)))
          ], { type: 'audio/wav' });
          
          const audioUrl = URL.createObjectURL(audioBlob);
          const audio = new Audio(audioUrl);
          audio.play();
        }
      }
    }
  }
};
```

---

## Additional Features

### Document Types Supported
- **Text Files**: .txt, .md, .html
- **PDF Documents**: Full text extraction
- **Word Documents**: .docx support
- **Web Pages**: HTML content extraction
- **Markdown**: Full markdown parsing

### Processing Features
- **Robust Text Extraction**: Multiple fallback methods
- **Content Analysis**: Statistics, readability, language detection
- **Metadata Extraction**: File information, timestamps
- **Error Recovery**: Graceful handling of corrupted files

### AI Capabilities
- **Document Summarization**: Automatic content summarization
- **Question Answering**: Query documents directly
- **Content Search**: Semantic and keyword search
- **Topic Extraction**: Key topic identification

### TTS Integration
- **Voice Synthesis**: Convert documents to speech
- **Multiple Voices**: Support for different voice models
- **Streaming Audio**: Real-time audio generation
- **Audio Formats**: WAV output format

---

## Configuration

### Environment Variables

```bash
# Server Configuration
PORT=3001                           # Server port (default: 3001)
RUST_LOG=info                      # Log level (debug, info, warn, error)

# TTS Configuration (optional)
ELEVENLABS_API_KEY=your_api_key    # ElevenLabs API key for TTS
ELEVENLABS_VOICE_ID=voice_id       # Default voice ID

# Development Configuration
VITE_API_BASE_URL=http://localhost:3001  # Frontend API base URL
VITE_DEMO_MODE=true                # Enable demo mode (bypass auth)
```

### CLI Options

```bash
# Start server with custom port
./swoop_server --port 8080

# Set log level
./swoop_server --log-level debug

# Use configuration file
./swoop_server --config config.toml
```

---

## SDK Examples

### JavaScript/TypeScript SDK

```typescript
import { SwoopClient } from './swoop-client';

const client = new SwoopClient({
  baseUrl: 'http://localhost:3001',
  apiKey: 'optional-api-key'
});

// Upload and analyze document
const document = await client.documents.upload(file);
const analysis = await client.documents.analyze(document.id);

// Start crawl job
const job = await client.crawl.start('https://example.com');
const results = await client.crawl.waitForCompletion(job.id);

// Chat with document
const response = await client.chat.document(document.id, 'What is this about?');
```

### Python SDK

```python
from swoop_client import SwoopClient

client = SwoopClient(base_url='http://localhost:3001')

# Upload document
with open('document.pdf', 'rb') as f:
    document = client.documents.upload(f)

# Analyze document
analysis = client.documents.analyze(document['id'])

# Chat with document
response = client.chat.document(document['id'], 'Summarize this document')
```

---

## Troubleshooting

### Common Issues

1. **File Upload Fails**
   - Check file size limits
   - Verify file format support
   - Ensure proper multipart encoding

2. **TTS Not Working**
   - Verify TTS feature is enabled
   - Check ElevenLabs API key configuration
   - Ensure internet connectivity

3. **Crawling Errors**
   - Verify URL accessibility
   - Check robots.txt compliance
   - Monitor network connectivity

### Debug Mode

Enable debug logging for detailed information:

```bash
RUST_LOG=debug ./swoop_server
```

### Performance Tuning

- Adjust server port for optimal performance
- Monitor memory usage during document processing
- Consider file size limits for large documents

---

## Changelog

### Version 0.2.0
- Added voice chat with streaming support
- Implemented web crawling functionality
- Enhanced document analysis capabilities
- Added TTS integration with ElevenLabs
- Improved error handling and validation

### Version 0.1.0
- Initial release with basic document processing
- Core API endpoints for upload and analysis
- Simple chat interface
- Health check and status endpoints

---

## Support

For issues, questions, or contributions:

- **GitHub Repository**: https://github.com/codewithkenzo/swoop
- **Documentation**: https://docs.rs/swoop
- **Issues**: Use GitHub issue tracker
- **Email**: codewithkenzo@github.com

---

*This documentation is automatically generated and maintained. Last updated: 2024-01-15*