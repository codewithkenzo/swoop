# Swoop Integration Examples

## Overview

This guide provides practical examples for integrating Swoop into your applications. Whether you're building a document management system, knowledge base, or AI-powered application, these examples will help you get started quickly.

## Table of Contents

1. [JavaScript/TypeScript Integration](#javascripttypescript-integration)
2. [Python Integration](#python-integration)
3. [cURL Examples](#curl-examples)
4. [React Components](#react-components)
5. [Node.js Backend Integration](#nodejs-backend-integration)
6. [Webhook Integration](#webhook-integration)
7. [Batch Processing](#batch-processing)
8. [Real-time Updates](#real-time-updates)

## JavaScript/TypeScript Integration

### Basic Client Setup

```typescript
// swoop-client.ts
import axios, { AxiosInstance } from 'axios';

export interface SwoopConfig {
  baseURL: string;
  apiKey: string;
  timeout?: number;
}

export class SwoopClient {
  private client: AxiosInstance;

  constructor(config: SwoopConfig) {
    this.client = axios.create({
      baseURL: config.baseURL,
      timeout: config.timeout || 10000,
      headers: {
        'Authorization': `Bearer ${config.apiKey}`,
        'Content-Type': 'application/json',
      },
    });

    // Response interceptor for error handling
    this.client.interceptors.response.use(
      (response) => response,
      (error) => {
        console.error('Swoop API Error:', error.response?.data || error.message);
        return Promise.reject(error);
      }
    );
  }

  // Document operations
  async uploadDocument(file: File, metadata?: any) {
    const formData = new FormData();
    formData.append('file', file);
    
    if (metadata) {
      formData.append('metadata', JSON.stringify(metadata));
    }

    const response = await this.client.post('/api/documents/upload', formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    });

    return response.data;
  }

  async getDocuments(params?: {
    page?: number;
    limit?: number;
    category?: string;
    tag?: string;
  }) {
    const response = await this.client.get('/api/documents', { params });
    return response.data;
  }

  async getDocument(id: string) {
    const response = await this.client.get(`/api/documents/${id}`);
    return response.data;
  }

  async deleteDocument(id: string) {
    const response = await this.client.delete(`/api/documents/${id}`);
    return response.data;
  }

  // Search operations
  async searchDocuments(query: string, options?: {
    limit?: number;
    category?: string;
    semanticWeight?: number;
  }) {
    const response = await this.client.get('/api/search', {
      params: { q: query, ...options },
    });
    return response.data;
  }

  // Chat operations
  async chat(message: string, documentId?: string) {
    const response = await this.client.post('/api/chat', {
      message,
      documentId,
    });
    return response.data;
  }

  async chatWithDocument(documentId: string, message: string) {
    const response = await this.client.post(`/api/documents/${documentId}/chat`, {
      message,
    });
    return response.data;
  }

  // Web crawling
  async startCrawl(url: string, options?: {
    maxDepth?: number;
    maxPages?: number;
    includePatterns?: string[];
    excludePatterns?: string[];
  }) {
    const response = await this.client.post('/api/crawl', {
      url,
      ...options,
    });
    return response.data;
  }

  async getCrawlStatus(jobId: string) {
    const response = await this.client.get(`/api/crawl/${jobId}/status`);
    return response.data;
  }

  async getCrawlResults(jobId: string) {
    const response = await this.client.get(`/api/crawl/${jobId}/results`);
    return response.data;
  }

  // Audio operations
  async getDocumentAudio(documentId: string, options?: {
    voice?: 'male' | 'female' | 'neutral';
    format?: 'mp3' | 'wav' | 'ogg';
    speed?: number;
  }) {
    const response = await this.client.get(`/api/audio/${documentId}`, {
      params: options,
      responseType: 'blob',
    });
    return response.data;
  }

  // System operations
  async getSystemStatus() {
    const response = await this.client.get('/api/status');
    return response.data;
  }

  async healthCheck() {
    const response = await this.client.get('/health');
    return response.data;
  }
}
```

### Usage Example

```typescript
// app.ts
import { SwoopClient } from './swoop-client';

const swoop = new SwoopClient({
  baseURL: 'https://api.swoop.dev',
  apiKey: 'your-api-key-here',
});

async function main() {
  try {
    // Upload a document
    const fileInput = document.getElementById('fileInput') as HTMLInputElement;
    if (fileInput.files && fileInput.files[0]) {
      const file = fileInput.files[0];
      const document = await swoop.uploadDocument(file, {
        title: 'My Document',
        tags: ['important', 'work'],
      });
      console.log('Document uploaded:', document);
    }

    // Search documents
    const searchResults = await swoop.searchDocuments('artificial intelligence', {
      limit: 10,
      category: 'technical',
    });
    console.log('Search results:', searchResults);

    // Chat with a document
    const chatResponse = await swoop.chatWithDocument(
      'doc-id-123',
      'What are the main points of this document?'
    );
    console.log('Chat response:', chatResponse);

    // Start web crawling
    const crawlJob = await swoop.startCrawl('https://example.com', {
      maxDepth: 2,
      maxPages: 50,
      includePatterns: ['*/blog/*', '*/docs/*'],
    });
    console.log('Crawl job started:', crawlJob);

  } catch (error) {
    console.error('Error:', error);
  }
}

main();
```

## Python Integration

### Python Client

```python
# swoop_client.py
import requests
from typing import Optional, Dict, Any, List
import json

class SwoopClient:
    def __init__(self, base_url: str, api_key: str, timeout: int = 30):
        self.base_url = base_url.rstrip('/')
        self.api_key = api_key
        self.timeout = timeout
        self.session = requests.Session()
        self.session.headers.update({
            'Authorization': f'Bearer {api_key}',
            'Content-Type': 'application/json',
        })

    def _request(self, method: str, endpoint: str, **kwargs) -> Dict[Any, Any]:
        """Make HTTP request with error handling."""
        url = f"{self.base_url}{endpoint}"
        
        try:
            response = self.session.request(
                method, url, timeout=self.timeout, **kwargs
            )
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            print(f"Swoop API Error: {e}")
            raise

    # Document operations
    def upload_document(self, file_path: str, metadata: Optional[Dict] = None) -> Dict:
        """Upload a document for processing."""
        with open(file_path, 'rb') as f:
            files = {'file': f}
            data = {}
            
            if metadata:
                data['metadata'] = json.dumps(metadata)
            
            # Remove default Content-Type header for multipart
            headers = {k: v for k, v in self.session.headers.items() 
                      if k.lower() != 'content-type'}
            
            response = requests.post(
                f"{self.base_url}/api/documents/upload",
                files=files,
                data=data,
                headers=headers,
                timeout=self.timeout
            )
            response.raise_for_status()
            return response.json()

    def get_documents(self, page: int = 0, limit: int = 20, 
                     category: Optional[str] = None, 
                     tag: Optional[str] = None) -> Dict:
        """Get list of documents with optional filtering."""
        params = {'page': page, 'limit': limit}
        if category:
            params['category'] = category
        if tag:
            params['tag'] = tag
            
        return self._request('GET', '/api/documents', params=params)

    def get_document(self, document_id: str) -> Dict:
        """Get a specific document by ID."""
        return self._request('GET', f'/api/documents/{document_id}')

    def delete_document(self, document_id: str) -> Dict:
        """Delete a document."""
        return self._request('DELETE', f'/api/documents/{document_id}')

    def analyze_document(self, document_id: str, force: bool = False) -> Dict:
        """Trigger re-analysis of a document."""
        data = {'force': force}
        return self._request('POST', f'/api/documents/{document_id}/analyze', 
                           json=data)

    # Search operations
    def search_documents(self, query: str, limit: int = 10, 
                        category: Optional[str] = None,
                        semantic_weight: float = 0.5) -> Dict:
        """Search documents using hybrid search."""
        params = {
            'q': query,
            'limit': limit,
            'semantic_weight': semantic_weight
        }
        if category:
            params['category'] = category
            
        return self._request('GET', '/api/search', params=params)

    # Chat operations
    def chat(self, message: str, document_id: Optional[str] = None, 
             model: Optional[str] = None) -> Dict:
        """Send a chat message."""
        data = {'message': message}
        if document_id:
            data['document_id'] = document_id
        if model:
            data['model'] = model
            
        return self._request('POST', '/api/chat', json=data)

    def chat_with_document(self, document_id: str, message: str,
                          model: Optional[str] = None) -> Dict:
        """Chat with a specific document."""
        data = {'message': message}
        if model:
            data['model'] = model
            
        return self._request('POST', f'/api/documents/{document_id}/chat',
                           json=data)

    # Web crawling
    def start_crawl(self, url: str, max_depth: int = 1, max_pages: int = 10,
                   include_patterns: Optional[List[str]] = None,
                   exclude_patterns: Optional[List[str]] = None) -> Dict:
        """Start a web crawling job."""
        data = {
            'url': url,
            'max_depth': max_depth,
            'max_pages': max_pages
        }
        if include_patterns:
            data['include_patterns'] = include_patterns
        if exclude_patterns:
            data['exclude_patterns'] = exclude_patterns
            
        return self._request('POST', '/api/crawl', json=data)

    def get_crawl_status(self, job_id: str) -> Dict:
        """Get the status of a crawling job."""
        return self._request('GET', f'/api/crawl/{job_id}/status')

    def get_crawl_results(self, job_id: str) -> Dict:
        """Get the results of a completed crawl job."""
        return self._request('GET', f'/api/crawl/{job_id}/results')

    # Audio operations
    def get_document_audio(self, document_id: str, voice: str = 'neutral',
                          format: str = 'mp3', speed: float = 1.0) -> bytes:
        """Get audio (TTS) for a document."""
        params = {'voice': voice, 'format': format, 'speed': speed}
        
        response = self.session.get(
            f"{self.base_url}/api/audio/{document_id}",
            params=params,
            timeout=self.timeout
        )
        response.raise_for_status()
        return response.content

    # System operations
    def get_system_status(self) -> Dict:
        """Get system status information."""
        return self._request('GET', '/api/status')

    def health_check(self) -> Dict:
        """Perform health check."""
        return self._request('GET', '/health')


# Usage example
if __name__ == "__main__":
    # Initialize client
    client = SwoopClient(
        base_url="https://api.swoop.dev",
        api_key="your-api-key-here"
    )
    
    try:
        # Upload document
        document = client.upload_document(
            "path/to/document.pdf",
            metadata={"title": "Important Document", "tags": ["work", "important"]}
        )
        print(f"Document uploaded: {document['id']}")
        
        # Search documents
        results = client.search_documents("machine learning", limit=5)
        print(f"Found {len(results['results'])} documents")
        
        # Chat with document
        chat_response = client.chat_with_document(
            document['id'],
            "What are the key findings in this document?"
        )
        print(f"Chat response: {chat_response['response']}")
        
        # Start crawling
        crawl_job = client.start_crawl(
            "https://example.com",
            max_depth=2,
            include_patterns=["*/blog/*"]
        )
        print(f"Crawl job started: {crawl_job['job_id']}")
        
    except Exception as e:
        print(f"Error: {e}")
```

## cURL Examples

### Document Upload

```bash
# Upload a PDF document
curl -X POST "https://api.swoop.dev/api/documents/upload" \
  -H "Authorization: Bearer your-api-key" \
  -F "file=@document.pdf" \
  -F 'metadata={"title":"My Document","tags":["important"]}'
```

### Document Search

```bash
# Search documents
curl -X GET "https://api.swoop.dev/api/search" \
  -H "Authorization: Bearer your-api-key" \
  -G \
  -d "q=artificial intelligence" \
  -d "limit=10" \
  -d "category=technical"
```

### Chat with Document

```bash
# Chat with a specific document
curl -X POST "https://api.swoop.dev/api/documents/doc-123/chat" \
  -H "Authorization: Bearer your-api-key" \
  -H "Content-Type: application/json" \
  -d '{"message":"What are the main points?"}'
```

### Web Crawling

```bash
# Start crawling
curl -X POST "https://api.swoop.dev/api/crawl" \
  -H "Authorization: Bearer your-api-key" \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://example.com",
    "max_depth": 2,
    "max_pages": 50,
    "include_patterns": ["*/blog/*", "*/docs/*"]
  }'

# Check crawl status
curl -X GET "https://api.swoop.dev/api/crawl/job-123/status" \
  -H "Authorization: Bearer your-api-key"
```

## React Components

### Document Upload Component

```tsx
// DocumentUpload.tsx
import React, { useState } from 'react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { SwoopClient } from './swoop-client';

interface DocumentUploadProps {
  onUploadSuccess?: (document: any) => void;
}

const swoop = new SwoopClient({
  baseURL: process.env.REACT_APP_SWOOP_API_URL!,
  apiKey: process.env.REACT_APP_SWOOP_API_KEY!,
});

export const DocumentUpload: React.FC<DocumentUploadProps> = ({ 
  onUploadSuccess 
}) => {
  const [file, setFile] = useState<File | null>(null);
  const [metadata, setMetadata] = useState({ title: '', tags: '' });
  const queryClient = useQueryClient();

  const uploadMutation = useMutation({
    mutationFn: async (data: { file: File; metadata: any }) => {
      return swoop.uploadDocument(data.file, data.metadata);
    },
    onSuccess: (document) => {
      queryClient.invalidateQueries({ queryKey: ['documents'] });
      onUploadSuccess?.(document);
      setFile(null);
      setMetadata({ title: '', tags: '' });
    },
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!file) return;

    const tags = metadata.tags.split(',').map(tag => tag.trim()).filter(Boolean);
    
    uploadMutation.mutate({
      file,
      metadata: {
        title: metadata.title || file.name,
        tags,
      },
    });
  };

  return (
    <div className="max-w-md mx-auto bg-white rounded-lg shadow-md p-6">
      <h2 className="text-xl font-semibold mb-4">Upload Document</h2>
      
      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            File
          </label>
          <input
            type="file"
            accept=".pdf,.txt,.md,.html"
            onChange={(e) => setFile(e.target.files?.[0] || null)}
            className="block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Title (optional)
          </label>
          <input
            type="text"
            value={metadata.title}
            onChange={(e) => setMetadata(prev => ({ ...prev, title: e.target.value }))}
            className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
            placeholder="Document title"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Tags (comma-separated)
          </label>
          <input
            type="text"
            value={metadata.tags}
            onChange={(e) => setMetadata(prev => ({ ...prev, tags: e.target.value }))}
            className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
            placeholder="tag1, tag2, tag3"
          />
        </div>

        <button
          type="submit"
          disabled={!file || uploadMutation.isPending}
          className="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {uploadMutation.isPending ? 'Uploading...' : 'Upload Document'}
        </button>
      </form>

      {uploadMutation.error && (
        <div className="mt-4 text-red-600 text-sm">
          Error: {uploadMutation.error.message}
        </div>
      )}
    </div>
  );
};
```

### Document Search Component

```tsx
// DocumentSearch.tsx
import React, { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { SwoopClient } from './swoop-client';

const swoop = new SwoopClient({
  baseURL: process.env.REACT_APP_SWOOP_API_URL!,
  apiKey: process.env.REACT_APP_SWOOP_API_KEY!,
});

export const DocumentSearch: React.FC = () => {
  const [query, setQuery] = useState('');
  const [category, setCategory] = useState('');
  const [debouncedQuery, setDebouncedQuery] = useState('');

  // Debounce search query
  React.useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedQuery(query);
    }, 500);

    return () => clearTimeout(timer);
  }, [query]);

  const { data: searchResults, isLoading, error } = useQuery({
    queryKey: ['search', debouncedQuery, category],
    queryFn: () => swoop.searchDocuments(debouncedQuery, {
      category: category || undefined,
      limit: 20,
    }),
    enabled: debouncedQuery.length > 0,
  });

  return (
    <div className="max-w-4xl mx-auto p-6">
      <h2 className="text-2xl font-bold mb-6">Search Documents</h2>
      
      <div className="mb-6 flex gap-4">
        <div className="flex-1">
          <input
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="Search documents..."
            className="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          />
        </div>
        
        <select
          value={category}
          onChange={(e) => setCategory(e.target.value)}
          className="px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500"
        >
          <option value="">All Categories</option>
          <option value="technical">Technical</option>
          <option value="business">Business</option>
          <option value="legal">Legal</option>
          <option value="academic">Academic</option>
        </select>
      </div>

      {isLoading && (
        <div className="text-center py-8">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto"></div>
          <p className="mt-2 text-gray-600">Searching...</p>
        </div>
      )}

      {error && (
        <div className="bg-red-50 border border-red-200 rounded-md p-4 mb-6">
          <p className="text-red-800">Error: {error.message}</p>
        </div>
      )}

      {searchResults && (
        <div className="space-y-4">
          <p className="text-gray-600">
            Found {searchResults.total} documents
          </p>
          
          {searchResults.results.map((doc: any) => (
            <div key={doc.id} className="bg-white border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow">
              <h3 className="text-lg font-semibold text-blue-600 mb-2">
                {doc.title}
              </h3>
              
              <div className="flex items-center gap-4 text-sm text-gray-500 mb-2">
                <span>Score: {doc.score?.toFixed(2)}</span>
                <span>Category: {doc.analysis?.category}</span>
                <span>Quality: {doc.analysis?.quality_score}/100</span>
              </div>
              
              {doc.highlights && (
                <div className="mb-2">
                  {doc.highlights.map((highlight: string, index: number) => (
                    <p key={index} className="text-sm text-gray-700 mb-1">
                      ...{highlight}...
                    </p>
                  ))}
                </div>
              )}
              
              {doc.tags && doc.tags.length > 0 && (
                <div className="flex flex-wrap gap-1">
                  {doc.tags.map((tag: string) => (
                    <span key={tag} className="px-2 py-1 bg-blue-100 text-blue-800 text-xs rounded">
                      {tag}
                    </span>
                  ))}
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
```

## Node.js Backend Integration

### Express.js Integration

```javascript
// server.js
const express = require('express');
const multer = require('multer');
const { SwoopClient } = require('./swoop-client');

const app = express();
const upload = multer({ dest: 'uploads/' });

// Initialize Swoop client
const swoop = new SwoopClient({
  baseURL: process.env.SWOOP_API_URL,
  apiKey: process.env.SWOOP_API_KEY,
});

app.use(express.json());

// Proxy document upload to Swoop
app.post('/api/documents/upload', upload.single('file'), async (req, res) => {
  try {
    const file = req.file;
    const metadata = req.body.metadata ? JSON.parse(req.body.metadata) : {};
    
    // Add user context to metadata
    metadata.uploaded_by = req.user?.id;
    metadata.uploaded_at = new Date().toISOString();
    
    const document = await swoop.uploadDocument(file.path, metadata);
    
    // Store document reference in your database
    await db.documents.create({
      id: document.id,
      user_id: req.user?.id,
      swoop_document_id: document.id,
      title: document.title,
      status: 'processing',
    });
    
    res.json(document);
  } catch (error) {
    console.error('Upload error:', error);
    res.status(500).json({ error: 'Upload failed' });
  }
});

// Enhanced search with user context
app.get('/api/search', async (req, res) => {
  try {
    const { q, category, limit = 10 } = req.query;
    
    // Get user's documents from your database
    const userDocuments = await db.documents.findMany({
      where: { user_id: req.user?.id },
      select: { swoop_document_id: true },
    });
    
    const results = await swoop.searchDocuments(q, {
      category,
      limit: parseInt(limit),
    });
    
    // Filter results to user's documents
    const filteredResults = results.results.filter(doc => 
      userDocuments.some(ud => ud.swoop_document_id === doc.id)
    );
    
    res.json({
      ...results,
      results: filteredResults,
    });
  } catch (error) {
    console.error('Search error:', error);
    res.status(500).json({ error: 'Search failed' });
  }
});

// Chat with document
app.post('/api/documents/:id/chat', async (req, res) => {
  try {
    const { id } = req.params;
    const { message } = req.body;
    
    // Verify user owns the document
    const document = await db.documents.findFirst({
      where: { 
        swoop_document_id: id,
        user_id: req.user?.id,
      },
    });
    
    if (!document) {
      return res.status(404).json({ error: 'Document not found' });
    }
    
    const response = await swoop.chatWithDocument(id, message);
    
    // Log chat interaction
    await db.chat_logs.create({
      document_id: document.id,
      user_id: req.user?.id,
      message,
      response: response.response,
      timestamp: new Date(),
    });
    
    res.json(response);
  } catch (error) {
    console.error('Chat error:', error);
    res.status(500).json({ error: 'Chat failed' });
  }
});

// Batch document processing
app.post('/api/documents/batch-process', async (req, res) => {
  try {
    const { document_ids } = req.body;
    
    const results = await Promise.allSettled(
      document_ids.map(id => swoop.analyzeDocument(id, { force: true }))
    );
    
    const processed = results.map((result, index) => ({
      document_id: document_ids[index],
      status: result.status,
      data: result.status === 'fulfilled' ? result.value : null,
      error: result.status === 'rejected' ? result.reason.message : null,
    }));
    
    res.json({ results: processed });
  } catch (error) {
    console.error('Batch processing error:', error);
    res.status(500).json({ error: 'Batch processing failed' });
  }
});

app.listen(3000, () => {
  console.log('Server running on port 3000');
});
```

## Webhook Integration

### Webhook Handler

```javascript
// webhook-handler.js
const express = require('express');
const crypto = require('crypto');

const app = express();

// Webhook signature verification
function verifyWebhookSignature(payload, signature, secret) {
  const computedSignature = crypto
    .createHmac('sha256', secret)
    .update(payload)
    .digest('hex');
  
  return signature === `sha256=${computedSignature}`;
}

// Webhook endpoint
app.post('/webhooks/swoop', express.raw({ type: 'application/json' }), async (req, res) => {
  const signature = req.headers['x-swoop-signature'];
  const payload = req.body;
  
  // Verify webhook signature
  if (!verifyWebhookSignature(payload, signature, process.env.WEBHOOK_SECRET)) {
    return res.status(401).json({ error: 'Invalid signature' });
  }
  
  const event = JSON.parse(payload.toString());
  
  try {
    switch (event.type) {
      case 'document.processed':
        await handleDocumentProcessed(event.data);
        break;
      
      case 'document.analysis.completed':
        await handleAnalysisCompleted(event.data);
        break;
      
      case 'crawl.completed':
        await handleCrawlCompleted(event.data);
        break;
      
      case 'crawl.failed':
        await handleCrawlFailed(event.data);
        break;
      
      default:
        console.log(`Unhandled event type: ${event.type}`);
    }
    
    res.status(200).json({ success: true });
  } catch (error) {
    console.error('Webhook processing error:', error);
    res.status(500).json({ error: 'Processing failed' });
  }
});

async function handleDocumentProcessed(data) {
  const { document_id, status, analysis } = data;
  
  // Update document status in your database
  await db.documents.update({
    where: { swoop_document_id: document_id },
    data: {
      status: 'processed',
      analysis: JSON.stringify(analysis),
      processed_at: new Date(),
    },
  });
  
  // Send notification to user
  const document = await db.documents.findFirst({
    where: { swoop_document_id: document_id },
    include: { user: true },
  });
  
  if (document && document.user) {
    await sendNotification(document.user.id, {
      type: 'document_processed',
      title: 'Document Processing Complete',
      message: `Your document "${document.title}" has been processed successfully.`,
      document_id: document.id,
    });
  }
}

async function handleAnalysisCompleted(data) {
  const { document_id, analysis, quality_score } = data;
  
  // Update analysis results
  await db.documents.update({
    where: { swoop_document_id: document_id },
    data: {
      analysis: JSON.stringify(analysis),
      quality_score,
      last_analyzed: new Date(),
    },
  });
  
  // Trigger additional processing if needed
  if (quality_score < 50) {
    await db.document_reviews.create({
      document_id,
      reason: 'low_quality',
      status: 'pending',
    });
  }
}

async function handleCrawlCompleted(data) {
  const { job_id, documents_count, statistics } = data;
  
  // Update crawl job status
  await db.crawl_jobs.update({
    where: { swoop_job_id: job_id },
    data: {
      status: 'completed',
      documents_count,
      statistics: JSON.stringify(statistics),
      completed_at: new Date(),
    },
  });
  
  // Notify user
  const crawlJob = await db.crawl_jobs.findFirst({
    where: { swoop_job_id: job_id },
    include: { user: true },
  });
  
  if (crawlJob && crawlJob.user) {
    await sendNotification(crawlJob.user.id, {
      type: 'crawl_completed',
      title: 'Web Crawling Complete',
      message: `Successfully crawled ${documents_count} documents from ${crawlJob.url}.`,
      job_id: crawlJob.id,
    });
  }
}

async function sendNotification(userId, notification) {
  // Implement your notification system
  // This could be email, push notification, WebSocket, etc.
  console.log(`Sending notification to user ${userId}:`, notification);
}

app.listen(3001, () => {
  console.log('Webhook server running on port 3001');
});
```

## Batch Processing

### Batch Upload Script

```python
# batch_upload.py
import os
import asyncio
from pathlib import Path
from swoop_client import SwoopClient

async def batch_upload_documents(client, directory_path, batch_size=5):
    """Upload documents in batches for better performance."""
    directory = Path(directory_path)
    
    # Find all supported files
    supported_extensions = {'.pdf', '.txt', '.md', '.html'}
    files = [f for f in directory.rglob('*') if f.suffix.lower() in supported_extensions]
    
    print(f"Found {len(files)} files to upload")
    
    # Process in batches
    for i in range(0, len(files), batch_size):
        batch = files[i:i + batch_size]
        print(f"Processing batch {i//batch_size + 1}/{(len(files)-1)//batch_size + 1}")
        
        tasks = []
        for file_path in batch:
            # Extract metadata from file path
            metadata = {
                'title': file_path.stem,
                'tags': [file_path.parent.name],
                'source': 'batch_upload',
                'file_path': str(file_path),
            }
            
            # Create upload task
            task = upload_single_document(client, file_path, metadata)
            tasks.append(task)
        
        # Execute batch
        results = await asyncio.gather(*tasks, return_exceptions=True)
        
        # Process results
        for j, result in enumerate(results):
            if isinstance(result, Exception):
                print(f"Error uploading {batch[j]}: {result}")
            else:
                print(f"Successfully uploaded {batch[j].name} (ID: {result['id']})")
        
        # Brief pause between batches
        await asyncio.sleep(1)

async def upload_single_document(client, file_path, metadata):
    """Upload a single document with retry logic."""
    max_retries = 3
    
    for attempt in range(max_retries):
        try:
            return client.upload_document(str(file_path), metadata)
        except Exception as e:
            if attempt == max_retries - 1:
                raise e
            
            print(f"Retry {attempt + 1} for {file_path}: {e}")
            await asyncio.sleep(2 ** attempt)  # Exponential backoff

async def main():
    client = SwoopClient(
        base_url="https://api.swoop.dev",
        api_key="your-api-key"
    )
    
    await batch_upload_documents(client, "./documents", batch_size=3)

if __name__ == "__main__":
    asyncio.run(main())
```

## Real-time Updates

### Server-Sent Events Client

```javascript
// sse-client.js
class SwoopSSEClient {
  constructor(baseURL, apiKey) {
    this.baseURL = baseURL;
    this.apiKey = apiKey;
    this.eventSources = new Map();
  }

  // Monitor document processing
  monitorDocumentProcessing(documentId, callbacks) {
    const url = `${this.baseURL}/api/documents/${documentId}/stream`;
    
    const eventSource = new EventSource(url, {
      headers: {
        'Authorization': `Bearer ${this.apiKey}`,
      },
    });

    eventSource.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        callbacks.onProgress?.(data);
      } catch (error) {
        callbacks.onError?.(error);
      }
    };

    eventSource.addEventListener('completed', (event) => {
      const data = JSON.parse(event.data);
      callbacks.onComplete?.(data);
      this.closeStream(documentId);
    });

    eventSource.addEventListener('error', (event) => {
      const data = JSON.parse(event.data);
      callbacks.onError?.(data);
      this.closeStream(documentId);
    });

    eventSource.onerror = (error) => {
      callbacks.onError?.(error);
      this.closeStream(documentId);
    };

    this.eventSources.set(documentId, eventSource);
    return eventSource;
  }

  // Monitor crawl job
  monitorCrawlJob(jobId, callbacks) {
    const url = `${this.baseURL}/api/crawl/${jobId}/stream`;
    
    const eventSource = new EventSource(url, {
      headers: {
        'Authorization': `Bearer ${this.apiKey}`,
      },
    });

    eventSource.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        callbacks.onProgress?.(data);
      } catch (error) {
        callbacks.onError?.(error);
      }
    };

    eventSource.addEventListener('completed', (event) => {
      const data = JSON.parse(event.data);
      callbacks.onComplete?.(data);
      this.closeStream(jobId);
    });

    eventSource.addEventListener('failed', (event) => {
      const data = JSON.parse(event.data);
      callbacks.onError?.(data);
      this.closeStream(jobId);
    });

    this.eventSources.set(jobId, eventSource);
    return eventSource;
  }

  closeStream(id) {
    const eventSource = this.eventSources.get(id);
    if (eventSource) {
      eventSource.close();
      this.eventSources.delete(id);
    }
  }

  closeAllStreams() {
    this.eventSources.forEach((eventSource) => {
      eventSource.close();
    });
    this.eventSources.clear();
  }
}

// Usage example
const sseClient = new SwoopSSEClient(
  'https://api.swoop.dev',
  'your-api-key'
);

// Monitor document processing
sseClient.monitorDocumentProcessing('doc-123', {
  onProgress: (data) => {
    console.log('Processing progress:', data);
    updateProgressBar(data.percentage);
  },
  
  onComplete: (data) => {
    console.log('Document processed:', data);
    showSuccessMessage('Document processed successfully!');
  },
  
  onError: (error) => {
    console.error('Processing error:', error);
    showErrorMessage('Processing failed');
  },
});

// Monitor crawl job
sseClient.monitorCrawlJob('job-456', {
  onProgress: (data) => {
    console.log('Crawl progress:', data);
    updateCrawlStatus(data.pages_crawled, data.pages_queued);
  },
  
  onComplete: (data) => {
    console.log('Crawl completed:', data);
    showCrawlResults(data.statistics);
  },
  
  onError: (error) => {
    console.error('Crawl error:', error);
    showErrorMessage('Crawl failed');
  },
});
```

This comprehensive integration guide provides practical examples for using Swoop in various environments and scenarios. Each example includes error handling, best practices, and real-world usage patterns to help you integrate Swoop effectively into your applications.