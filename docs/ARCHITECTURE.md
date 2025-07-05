# Swoop Architecture Documentation

## System Overview

Swoop is a production-ready document intelligence platform built with a modern, scalable architecture. The system combines high-performance Rust backend services with a responsive React frontend to deliver real-time document processing and AI-powered analysis.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        Frontend Layer                           │
├─────────────────────────────────────────────────────────────────┤
│  React 18 + TypeScript + Vite                                   │
│  ├─ Components (UI/UX)                                          │
│  ├─ Pages (Dashboard, Documents, Chat)                          │
│  ├─ Hooks (useStreaming, useDocuments)                          │
│  ├─ API Client (axios, TanStack Query)                          │
│  └─ Real-time Updates (Server-Sent Events)                      │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                        API Gateway                              │
├─────────────────────────────────────────────────────────────────┤
│  Axum Web Framework (Rust)                                      │
│  ├─ Authentication & Authorization                              │
│  ├─ Rate Limiting & CORS                                        │
│  ├─ Request Validation & Sanitization                           │
│  ├─ Error Handling & Logging                                    │
│  └─ Health Checks & Monitoring                                  │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                     Business Logic Layer                        │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │  Document       │  │  Chat & AI      │  │  Web Crawler    │  │
│  │  Processor      │  │  Integration    │  │  Engine         │  │
│  │                 │  │                 │  │                 │  │
│  │ • Content       │  │ • OpenRouter    │  │ • Robots.txt    │  │
│  │   Extraction    │  │   API           │  │   Compliance    │  │
│  │ • Format        │  │ • Model         │  │ • Rate Limiting │  │
│  │   Detection     │  │   Selection     │  │ • Depth Control │  │
│  │ • Metadata      │  │ • Context Mgmt  │  │ • Content       │  │
│  │   Analysis      │  │ • Streaming     │  │   Extraction    │  │
│  │ • Quality       │  │   Responses     │  │ • Link          │  │
│  │   Scoring       │  │ • Embeddings    │  │   Discovery     │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  │
│                                                                 │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │  AI Analysis    │  │  Audio/TTS      │  │  Search &       │  │
│  │  Engine         │  │  Integration    │  │  Retrieval      │  │
│  │                 │  │                 │  │                 │  │
│  │ • Categorization│  │ • ElevenLabs    │  │ • Hybrid Search │  │
│  │ • Entity        │  │   Integration   │  │ • BM25 Keyword  │  │
│  │   Recognition   │  │ • Voice Options │  │ • Vector        │  │
│  │ • Sentiment     │  │ • Format        │  │   Similarity    │  │
│  │   Analysis      │  │   Support       │  │ • Relevance     │  │
│  │ • Complexity    │  │ • Streaming     │  │   Scoring       │  │
│  │   Scoring       │  │   Audio         │  │ • Faceted       │  │
│  │ • Summarization │  │ • Speech-to-Text│  │   Filtering     │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Storage Layer                              │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │  Primary        │  │  Vector         │  │  Cache &        │  │
│  │  Database       │  │  Database       │  │  Session        │  │
│  │                 │  │                 │  │                 │  │
│  │ • libSQL/SQLite │  │ • Qdrant        │  │ • Redis         │  │
│  │ • Document      │  │ • 384-dim       │  │ • Session Data  │  │
│  │   Metadata      │  │   Embeddings    │  │ • API Keys      │  │
│  │ • Analysis      │  │ • Similarity    │  │ • Rate Limits   │  │
│  │   Results       │  │   Search        │  │ • Temp Files    │  │
│  │ • User Data     │  │ • Clustering    │  │ • Job Status    │  │
│  │ • Crawl Jobs    │  │ • Indexing      │  │ • Metrics       │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                    External Services                            │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │  OpenRouter     │  │  ElevenLabs     │  │  Monitoring     │  │
│  │  API            │  │  TTS            │  │  & Observability│  │
│  │                 │  │                 │  │                 │  │
│  │ • 200+ Models   │  │ • Voice         │  │ • OpenTelemetry │  │
│  │ • GPT-4, Claude │  │   Synthesis     │  │ • Metrics       │  │
│  │ • Llama, Gemini │  │ • Multiple      │  │ • Tracing       │  │
│  │ • Model Routing │  │   Voices        │  │ • Logging       │  │
│  │ • Rate Limiting │  │ • Audio         │  │ • Health Checks │  │
│  │ • Cost Tracking │  │   Formats       │  │ • Alerts        │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Frontend Architecture

**Technology Stack:**
- **React 18** with TypeScript for type safety
- **Vite** for fast development and building
- **Tailwind CSS** for utility-first styling
- **Radix UI** for accessible component primitives
- **TanStack Query** for server state management
- **Framer Motion** for animations
- **Server-Sent Events** for real-time updates

**Key Features:**
- Responsive design with mobile-first approach
- Real-time progress tracking for uploads and analysis
- Optimistic UI updates for better UX
- Streaming responses for chat interfaces
- Comprehensive error handling and loading states

### 2. Backend Architecture

**Core Framework:**
- **Axum** - Fast, ergonomic Rust web framework
- **Tokio** - Asynchronous runtime for concurrent processing
- **Tower** - Middleware and service abstractions
- **Serde** - Serialization/deserialization
- **Clap** - Command-line argument parsing

**Request Processing Flow:**
```
HTTP Request → CORS Layer → Auth Layer → Rate Limiter → 
Request Validation → Business Logic → Response Serialization → 
HTTP Response
```

### 3. Document Processing Pipeline

**Stage 1: Content Extraction**
```rust
// Simplified processing flow
pub async fn process_document(file: &[u8], content_type: &str) -> Result<Document> {
    let content = extract_content(file, content_type)?;
    let metadata = analyze_metadata(&content)?;
    let analysis = perform_ai_analysis(&content).await?;
    let embeddings = generate_embeddings(&content).await?;
    
    Ok(Document {
        content,
        metadata,
        analysis,
        embeddings,
        ..Default::default()
    })
}
```

**Stage 2: AI Analysis**
- **Content Categorization**: Technical, business, legal, academic
- **Quality Assessment**: Readability, complexity, structure
- **Entity Recognition**: People, organizations, locations, dates
- **Sentiment Analysis**: Positive, negative, neutral with confidence scores
- **Key Topic Extraction**: Important themes and concepts
- **Summarization**: Concise content summaries

**Stage 3: Vector Embedding**
- **384-dimensional embeddings** for semantic similarity
- **Batch processing** for efficiency
- **Normalized vectors** for consistent similarity calculation
- **Indexed storage** in vector database

### 4. Storage Architecture

**Primary Database (libSQL/SQLite)**
```sql
-- Core document table
CREATE TABLE documents (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    content_type TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    metadata JSON,
    analysis JSON,
    tags JSON,
    embedding BLOB -- 384-dim vector
);

-- Crawl jobs tracking
CREATE TABLE crawl_jobs (
    id UUID PRIMARY KEY,
    url TEXT NOT NULL,
    status TEXT NOT NULL,
    progress JSON,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

**Vector Database (Qdrant)**
- **Collections** for different document types
- **Payload filtering** for metadata-based queries
- **Similarity search** with configurable distance metrics
- **Clustering** for document organization

**Cache Layer (Redis)**
- **Session storage** for user authentication
- **API rate limiting** counters
- **Temporary file storage** for uploads
- **Job status tracking** for async operations

### 5. AI Integration Architecture

**OpenRouter API Integration**
```rust
pub struct LLMClient {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl LLMClient {
    pub async fn chat_completion(&self, request: ChatRequest) -> Result<ChatResponse> {
        let response = self.client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;
        
        Ok(response.json().await?)
    }
}
```

**Model Selection Strategy:**
- **GPT-4** for complex analysis and reasoning
- **Claude** for long-form content and summarization
- **Llama** for cost-effective processing
- **Gemini** for multimodal tasks
- **Automatic failover** for reliability

### 6. Web Crawling Architecture

**Crawler Engine**
```rust
pub struct Crawler {
    client: reqwest::Client,
    robots_cache: Arc<RwLock<HashMap<String, RobotsTxt>>>,
    rate_limiter: Arc<RwLock<HashMap<String, Instant>>>,
}

impl Crawler {
    pub async fn crawl(&self, config: CrawlConfig) -> Result<Vec<Document>> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut results = Vec::new();
        
        queue.push_back((config.start_url, 0));
        
        while let Some((url, depth)) = queue.pop_front() {
            if depth > config.max_depth || visited.contains(&url) {
                continue;
            }
            
            if let Ok(page) = self.fetch_page(&url).await {
                visited.insert(url.clone());
                results.push(page.into_document());
                
                // Extract and queue new URLs
                for link in page.links {
                    if self.should_crawl(&link, &config) {
                        queue.push_back((link, depth + 1));
                    }
                }
            }
        }
        
        Ok(results)
    }
}
```

**Crawling Features:**
- **Robots.txt compliance** with caching
- **Rate limiting** per domain
- **Depth control** to prevent infinite crawling
- **URL filtering** with include/exclude patterns
- **Concurrent processing** with configurable limits
- **Progress tracking** with real-time updates

### 7. Real-time Communication

**Server-Sent Events (SSE)**
```rust
pub async fn document_processing_stream(
    document_id: String,
) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let stream = stream! {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        
        loop {
            interval.tick().await;
            
            if let Some(progress) = get_processing_progress(&document_id).await {
                let event = Event::default()
                    .event("progress")
                    .data(serde_json::to_string(&progress).unwrap());
                yield Ok(event);
                
                if progress.completed {
                    break;
                }
            }
        }
    };
    
    Sse::new(stream).keep_alive(KeepAlive::default())
}
```

**WebSocket Alternative:**
While SSE is used for server-to-client communication, WebSockets could be implemented for bidirectional real-time communication in chat interfaces.

## Data Flow

### Document Upload Flow
```
1. Frontend uploads file via multipart/form-data
2. API validates file type and size
3. Content extraction based on MIME type
4. Metadata analysis (word count, language detection)
5. AI analysis (categorization, sentiment, entities)
6. Vector embedding generation
7. Storage in primary database
8. Vector indexing in Qdrant
9. Real-time progress updates via SSE
10. Final result returned to frontend
```

### Search Query Flow
```
1. User submits search query
2. Query preprocessing and validation
3. Parallel execution:
   a. BM25 keyword search on content
   b. Vector similarity search on embeddings
4. Result scoring and ranking
5. Metadata filtering and faceting
6. Response serialization and caching
7. Results returned to frontend
```

### Chat Interaction Flow
```
1. User message received
2. Context retrieval (document or general)
3. Message history compilation
4. LLM API request with context
5. Streaming response handling
6. Real-time updates to frontend
7. Conversation state persistence
```

## Security Architecture

### Authentication & Authorization
- **API Key Authentication** with Bearer tokens
- **Rate limiting** per user/IP address
- **Input validation** and sanitization
- **CORS** configuration for cross-origin requests
- **Content Security Policy** headers

### Data Protection
- **Encryption at rest** for sensitive data
- **Secure file handling** with temporary storage
- **SQL injection prevention** with parameterized queries
- **XSS protection** with output encoding
- **File type validation** to prevent malicious uploads

### API Security
```rust
pub fn create_auth_layer() -> AuthLayer {
    AuthLayer::new(|req: &Request| {
        req.headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(|token| validate_api_key(token))
            .unwrap_or(false)
    })
}
```

## Performance Optimization

### Backend Optimizations
- **Async processing** with Tokio for I/O-bound operations
- **Connection pooling** for database connections
- **Batch processing** for embeddings and analysis
- **Lazy static initialization** for expensive resources
- **Memory-efficient streaming** for large files

### Frontend Optimizations
- **Code splitting** with dynamic imports
- **Lazy loading** for non-critical components
- **Memoization** for expensive computations
- **Virtual scrolling** for large document lists
- **Optimistic updates** for better UX

### Caching Strategy
- **Redis caching** for frequently accessed data
- **HTTP caching** headers for static assets
- **Query result caching** for search operations
- **Embedding caching** to avoid recomputation

## Scalability Considerations

### Horizontal Scaling
- **Stateless API design** for load balancing
- **Database sharding** for large datasets
- **Microservices architecture** for specific domains
- **Container orchestration** with Kubernetes

### Vertical Scaling
- **Memory optimization** for large document processing
- **CPU optimization** for AI computations
- **Storage optimization** for vector databases
- **Network optimization** for API responses

### Auto-scaling Triggers
- **CPU utilization** > 70% for 5 minutes
- **Memory utilization** > 80% for 3 minutes
- **Queue depth** > 100 pending jobs
- **Response time** > 2 seconds for 95th percentile

## Monitoring & Observability

### Metrics Collection
- **Application metrics** (request rate, error rate, latency)
- **Business metrics** (documents processed, users active)
- **Infrastructure metrics** (CPU, memory, disk, network)
- **Custom metrics** (AI model performance, crawl success rate)

### Logging Strategy
```rust
use tracing::{info, error, warn, debug};

#[tracing::instrument]
pub async fn process_document(id: &str) -> Result<Document> {
    info!("Starting document processing for {}", id);
    
    match extract_content(id).await {
        Ok(content) => {
            debug!("Content extracted successfully");
            // Processing logic
        }
        Err(e) => {
            error!("Failed to extract content: {}", e);
            return Err(e);
        }
    }
}
```

### Health Checks
- **Liveness probes** for container orchestration
- **Readiness probes** for traffic routing
- **Dependency health checks** for databases and external services
- **Circuit breakers** for external API calls

## Deployment Architecture

### Production Deployment
```yaml
# docker-compose.yml
version: '3.8'
services:
  swoop-api:
    image: swoop:latest
    environment:
      - DATABASE_URL=postgresql://user:pass@db:5432/swoop
      - REDIS_URL=redis://redis:6379
      - OPENROUTER_API_KEY=${OPENROUTER_API_KEY}
    depends_on:
      - db
      - redis
      - qdrant
    
  swoop-frontend:
    image: swoop-frontend:latest
    environment:
      - VITE_API_BASE_URL=https://api.swoop.dev
    
  db:
    image: postgres:15
    environment:
      - POSTGRES_DB=swoop
      - POSTGRES_USER=user
      - POSTGRES_PASSWORD=pass
    
  redis:
    image: redis:7-alpine
    
  qdrant:
    image: qdrant/qdrant:latest
```

### Environment Configuration
- **Development**: Single container with SQLite
- **Staging**: Multi-container with shared database
- **Production**: Kubernetes with auto-scaling
- **Edge**: Vercel Functions for global distribution

## Future Architecture Considerations

### Planned Enhancements
1. **Microservices Migration**: Break down monolith into specialized services
2. **Event-Driven Architecture**: Implement event sourcing and CQRS
3. **GraphQL API**: Provide more flexible query interface
4. **Real-time Collaboration**: WebSocket-based document editing
5. **Mobile Apps**: Native iOS/Android applications
6. **Offline Support**: Progressive Web App capabilities

### Technology Evolution
- **Rust async ecosystem** improvements
- **AI model hosting** for reduced latency
- **Vector database** performance optimizations
- **Edge computing** for global distribution
- **Quantum-ready cryptography** for future security