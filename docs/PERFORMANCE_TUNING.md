# Swoop Performance Tuning Guide

## Overview

This guide provides comprehensive strategies for optimizing Swoop's performance across different deployment scenarios. Whether you're running a single instance or a large-scale deployment, these optimizations will help you achieve maximum throughput and efficiency.

## Table of Contents

1. [System Requirements](#system-requirements)
2. [Database Optimization](#database-optimization)
3. [Memory Management](#memory-management)
4. [CPU Optimization](#cpu-optimization)
5. [Network Optimization](#network-optimization)
6. [AI Model Optimization](#ai-model-optimization)
7. [Storage Optimization](#storage-optimization)
8. [Caching Strategies](#caching-strategies)
9. [Monitoring and Metrics](#monitoring-and-metrics)
10. [Deployment Configurations](#deployment-configurations)

## System Requirements

### Minimum Requirements
- **CPU**: 2 cores, 2.4 GHz
- **RAM**: 4GB
- **Storage**: 20GB SSD
- **Network**: 100 Mbps

### Recommended for Production
- **CPU**: 8 cores, 3.2 GHz
- **RAM**: 16GB
- **Storage**: 100GB NVMe SSD
- **Network**: 1 Gbps

### High-Performance Setup
- **CPU**: 16+ cores, 3.5+ GHz
- **RAM**: 32GB+
- **Storage**: 500GB+ NVMe SSD
- **Network**: 10 Gbps
- **GPU**: Optional for local AI inference

## Database Optimization

### SQLite Configuration

```rust
// src/storage/sqlite.rs
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};

pub fn create_optimized_sqlite_options() -> SqliteConnectOptions {
    SqliteConnectOptions::new()
        .filename("swoop.db")
        .create_if_missing(true)
        // Performance optimizations
        .journal_mode(SqliteJournalMode::Wal)  // Write-Ahead Logging
        .synchronous(SqliteSynchronous::Normal)  // Faster writes
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(30))
        .pragma("cache_size", "-64000")  // 64MB cache
        .pragma("temp_store", "memory")
        .pragma("mmap_size", "268435456")  // 256MB memory mapping
        .pragma("optimize", "")  // Auto-optimize on close
}
```

### PostgreSQL Configuration (Production)

```sql
-- postgresql.conf optimizations
shared_buffers = 256MB                    # 25% of RAM
effective_cache_size = 1GB                # 75% of RAM
work_mem = 64MB                           # For complex queries
maintenance_work_mem = 256MB              # For maintenance operations
checkpoint_completion_target = 0.9
wal_buffers = 16MB
default_statistics_target = 100

-- Index optimizations
CREATE INDEX CONCURRENTLY idx_documents_created_at ON documents(created_at);
CREATE INDEX CONCURRENTLY idx_documents_content_type ON documents(content_type);
CREATE INDEX CONCURRENTLY idx_documents_analysis_category ON documents((analysis->>'category'));
CREATE INDEX CONCURRENTLY idx_documents_tags ON documents USING GIN (tags);
CREATE INDEX CONCURRENTLY idx_documents_embedding ON documents USING GIST (embedding);
```

### Connection Pooling

```rust
// src/storage/mod.rs
use sqlx::pool::PoolOptions;

pub async fn create_optimized_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    PoolOptions::new()
        .max_connections(20)              // Adjust based on CPU cores
        .min_connections(5)
        .max_lifetime(Duration::from_secs(3600))  // 1 hour
        .idle_timeout(Duration::from_secs(600))   // 10 minutes
        .acquire_timeout(Duration::from_secs(30))
        .test_before_acquire(true)
        .connect(&database_url)
        .await
}
```

## Memory Management

### Rust Memory Optimizations

```rust
// src/lib.rs
use std::sync::Arc;
use tokio::sync::RwLock;

// Efficient document processing
pub struct DocumentProcessor {
    // Use Arc for shared data
    models: Arc<RwLock<HashMap<String, Arc<Model>>>>,
    // Pre-allocated buffers
    content_buffer: Vec<u8>,
    analysis_buffer: Vec<u8>,
}

impl DocumentProcessor {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            models: Arc::new(RwLock::new(HashMap::new())),
            content_buffer: Vec::with_capacity(capacity),
            analysis_buffer: Vec::with_capacity(capacity),
        }
    }
    
    pub async fn process_document(&mut self, document: &Document) -> Result<ProcessedDocument> {
        // Reuse buffers to avoid allocations
        self.content_buffer.clear();
        self.analysis_buffer.clear();
        
        // Process with minimal allocations
        let content = self.extract_content_efficient(document)?;
        let analysis = self.analyze_content_efficient(&content).await?;
        
        Ok(ProcessedDocument {
            content,
            analysis,
            ..Default::default()
        })
    }
}

// Memory-efficient content extraction
pub fn extract_content_efficient(data: &[u8]) -> Result<String> {
    match detect_content_type(data) {
        ContentType::Pdf => {
            // Use streaming PDF parser
            let mut content = String::with_capacity(data.len());
            extract_pdf_streaming(data, &mut content)?;
            Ok(content)
        }
        ContentType::Html => {
            // Use efficient HTML parser
            let mut content = String::with_capacity(data.len() / 2);
            extract_html_efficient(data, &mut content)?;
            Ok(content)
        }
        _ => Ok(String::from_utf8_lossy(data).into_owned()),
    }
}
```

### Memory Monitoring

```rust
// src/monitoring/memory.rs
use sysinfo::{System, SystemExt};

pub struct MemoryMonitor {
    system: System,
    threshold: u64,
}

impl MemoryMonitor {
    pub fn new(threshold_mb: u64) -> Self {
        Self {
            system: System::new_all(),
            threshold: threshold_mb * 1024 * 1024,
        }
    }
    
    pub fn check_memory_usage(&mut self) -> MemoryStats {
        self.system.refresh_memory();
        
        let used = self.system.used_memory();
        let total = self.system.total_memory();
        let usage_percent = (used as f64 / total as f64) * 100.0;
        
        MemoryStats {
            used,
            total,
            usage_percent,
            should_gc: used > self.threshold,
        }
    }
    
    pub fn force_gc_if_needed(&mut self) {
        if self.check_memory_usage().should_gc {
            // Trigger garbage collection
            std::hint::black_box(Vec::<u8>::new());
        }
    }
}
```

## CPU Optimization

### Async Runtime Configuration

```rust
// src/main.rs
use tokio::runtime::Builder;

fn create_optimized_runtime() -> tokio::runtime::Runtime {
    Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .max_blocking_threads(512)
        .thread_name("swoop-worker")
        .thread_stack_size(2 * 1024 * 1024)  // 2MB stack
        .enable_all()
        .build()
        .expect("Failed to create runtime")
}

// CPU-bound task optimization
pub async fn process_documents_parallel(
    documents: Vec<Document>,
    max_concurrency: usize,
) -> Result<Vec<ProcessedDocument>> {
    let semaphore = Arc::new(Semaphore::new(max_concurrency));
    let futures = documents
        .into_iter()
        .map(|doc| {
            let semaphore = semaphore.clone();
            async move {
                let _permit = semaphore.acquire().await.unwrap();
                process_document_cpu_optimized(doc).await
            }
        })
        .collect::<Vec<_>>();
    
    futures::future::try_join_all(futures).await
}
```

### CPU-Intensive Operations

```rust
// src/processing/cpu.rs
use rayon::prelude::*;

// Parallel text processing
pub fn analyze_content_parallel(content: &str) -> ContentAnalysis {
    let chunks: Vec<&str> = content
        .split('\n')
        .collect::<Vec<_>>()
        .chunks(100)
        .map(|chunk| chunk.join("\n"))
        .collect();
    
    let analyses: Vec<ChunkAnalysis> = chunks
        .par_iter()
        .map(|chunk| analyze_chunk(chunk))
        .collect();
    
    merge_analyses(analyses)
}

// SIMD-optimized operations where possible
pub fn calculate_embeddings_optimized(text: &str) -> Vec<f32> {
    // Use vectorized operations for mathematical computations
    let tokens = tokenize_efficient(text);
    let mut embeddings = vec![0.0f32; 384];
    
    // Batch processing for better CPU utilization
    for batch in tokens.chunks(32) {
        process_token_batch(batch, &mut embeddings);
    }
    
    normalize_embeddings(&mut embeddings);
    embeddings
}
```

## Network Optimization

### HTTP Client Configuration

```rust
// src/llm/client.rs
use reqwest::Client;
use std::time::Duration;

pub fn create_optimized_client() -> Result<Client, reqwest::Error> {
    Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .pool_idle_timeout(Duration::from_secs(90))
        .pool_max_idle_per_host(10)
        .tcp_keepalive(Duration::from_secs(60))
        .tcp_nodelay(true)
        .gzip(true)
        .brotli(true)
        .http2_prior_knowledge()
        .build()
}

// Connection pooling for external APIs
pub struct APIConnectionPool {
    openrouter_client: Client,
    elevenlabs_client: Client,
    connection_limits: HashMap<String, Semaphore>,
}

impl APIConnectionPool {
    pub fn new() -> Self {
        Self {
            openrouter_client: create_optimized_client().unwrap(),
            elevenlabs_client: create_optimized_client().unwrap(),
            connection_limits: HashMap::from([
                ("openrouter".to_string(), Semaphore::new(10)),
                ("elevenlabs".to_string(), Semaphore::new(5)),
            ]),
        }
    }
    
    pub async fn make_request(&self, service: &str, request: Request) -> Result<Response> {
        let permit = self.connection_limits
            .get(service)
            .unwrap()
            .acquire()
            .await
            .unwrap();
        
        let client = match service {
            "openrouter" => &self.openrouter_client,
            "elevenlabs" => &self.elevenlabs_client,
            _ => return Err(Error::InvalidService),
        };
        
        let response = client.execute(request).await?;
        drop(permit);
        
        Ok(response)
    }
}
```

### Server Configuration

```rust
// src/server/mod.rs
use axum::http::HeaderValue;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;

pub fn create_optimized_app() -> Router {
    Router::new()
        .route("/api/documents/upload", post(upload_document))
        .route("/api/documents", get(list_documents))
        .route("/api/search", get(search_documents))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::DELETE])
                .allow_headers([CONTENT_TYPE, AUTHORIZATION])
                .max_age(Duration::from_secs(3600))
        )
        .layer(CompressionLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true))
        )
}

// Request size limits
pub fn configure_request_limits() -> RequestBodyLimitLayer {
    RequestBodyLimitLayer::new(
        10 * 1024 * 1024  // 10MB max request size
    )
}
```

## AI Model Optimization

### Model Selection Strategy

```rust
// src/llm/optimizer.rs
use std::collections::HashMap;

pub struct ModelOptimizer {
    model_performance: HashMap<String, ModelMetrics>,
    cost_limits: CostLimits,
}

impl ModelOptimizer {
    pub fn select_optimal_model(&self, task: &Task) -> String {
        match task.complexity {
            TaskComplexity::Simple => self.select_fast_model(task),
            TaskComplexity::Medium => self.select_balanced_model(task),
            TaskComplexity::Complex => self.select_powerful_model(task),
        }
    }
    
    fn select_fast_model(&self, task: &Task) -> String {
        // Use faster, cheaper models for simple tasks
        match task.task_type {
            TaskType::Categorization => "llama-3.1-8b".to_string(),
            TaskType::EntityExtraction => "llama-3.1-8b".to_string(),
            TaskType::Summarization => "claude-3-haiku".to_string(),
            _ => "gpt-4o-mini".to_string(),
        }
    }
    
    fn select_balanced_model(&self, task: &Task) -> String {
        // Balance between performance and cost
        match task.task_type {
            TaskType::QA => "gpt-4o".to_string(),
            TaskType::Analysis => "claude-3-sonnet".to_string(),
            TaskType::ContentGeneration => "gpt-4o".to_string(),
            _ => "gpt-4o-mini".to_string(),
        }
    }
    
    fn select_powerful_model(&self, task: &Task) -> String {
        // Use most capable models for complex tasks
        match task.task_type {
            TaskType::DeepAnalysis => "gpt-4".to_string(),
            TaskType::ComplexReasoning => "claude-3-opus".to_string(),
            TaskType::CodeGeneration => "gpt-4".to_string(),
            _ => "gpt-4o".to_string(),
        }
    }
}
```

### Request Batching

```rust
// src/llm/batch.rs
pub struct BatchProcessor {
    pending_requests: Vec<AIRequest>,
    batch_size: usize,
    batch_timeout: Duration,
}

impl BatchProcessor {
    pub async fn process_request(&mut self, request: AIRequest) -> Result<AIResponse> {
        self.pending_requests.push(request);
        
        if self.pending_requests.len() >= self.batch_size {
            self.process_batch().await
        } else {
            // Wait for more requests or timeout
            self.wait_for_batch_or_timeout().await
        }
    }
    
    async fn process_batch(&mut self) -> Result<Vec<AIResponse>> {
        let batch = std::mem::take(&mut self.pending_requests);
        
        // Combine requests for better API efficiency
        let combined_request = self.combine_requests(batch)?;
        let response = self.send_combined_request(combined_request).await?;
        
        self.split_response(response)
    }
    
    fn combine_requests(&self, requests: Vec<AIRequest>) -> Result<CombinedRequest> {
        // Combine similar requests into a single API call
        let mut combined = CombinedRequest::new();
        
        for request in requests {
            combined.add_request(request);
        }
        
        Ok(combined)
    }
}
```

## Storage Optimization

### Vector Database Optimization

```rust
// src/storage/vector.rs
use qdrant_client::prelude::*;

pub struct OptimizedVectorStore {
    client: QdrantClient,
    collection_name: String,
    batch_size: usize,
}

impl OptimizedVectorStore {
    pub async fn new(url: &str, collection_name: &str) -> Result<Self> {
        let client = QdrantClient::from_url(url).build()?;
        
        // Create optimized collection
        client.create_collection(&CreateCollection {
            collection_name: collection_name.to_string(),
            vectors_config: Some(VectorsConfig {
                params: Some(VectorParams {
                    size: 384,
                    distance: Distance::Cosine as i32,
                    hnsw_config: Some(HnswConfig {
                        m: 16,          // Optimize for search speed
                        ef_construct: 200,  // Build index quality
                        full_scan_threshold: 10000,
                        max_indexing_threads: 4,
                    }),
                    quantization_config: Some(QuantizationConfig {
                        // Use scalar quantization for memory efficiency
                        scalar: Some(ScalarQuantization {
                            r#type: QuantizationType::Int8 as i32,
                            quantile: Some(0.99),
                            always_ram: Some(true),
                        }),
                    }),
                }),
            }),
            optimizers_config: Some(OptimizersConfig {
                deleted_threshold: 0.2,
                vacuum_min_vector_number: 1000,
                default_segment_number: 4,
                max_segment_size: Some(20000),
                memmap_threshold: Some(50000),
                indexing_threshold: Some(20000),
                flush_interval_sec: 10,
                max_optimization_threads: 2,
            }),
        }).await?;
        
        Ok(Self {
            client,
            collection_name: collection_name.to_string(),
            batch_size: 100,
        })
    }
    
    pub async fn batch_upsert(&self, vectors: Vec<VectorData>) -> Result<()> {
        for batch in vectors.chunks(self.batch_size) {
            let points: Vec<PointStruct> = batch.iter()
                .map(|v| PointStruct {
                    id: Some(v.id.clone().into()),
                    vectors: Some(v.vector.clone().into()),
                    payload: v.payload.clone(),
                })
                .collect();
            
            self.client.upsert_points_batch(
                &self.collection_name,
                points,
                None,
                None,
                None,
            ).await?;
        }
        
        Ok(())
    }
    
    pub async fn optimized_search(&self, query: &[f32], limit: usize) -> Result<Vec<SearchResult>> {
        let search_request = SearchPoints {
            collection_name: self.collection_name.clone(),
            vector: query.to_vec(),
            limit: limit as u64,
            with_payload: Some(true.into()),
            params: Some(SearchParams {
                hnsw_ef: Some(128),  // Search quality vs speed
                exact: Some(false),
                quantization: Some(QuantizationSearchParams {
                    ignore: Some(false),
                    rescore: Some(true),
                    oversampling: Some(1.0),
                }),
            }),
        };
        
        let response = self.client.search_points(&search_request).await?;
        Ok(response.result)
    }
}
```

### File Storage Optimization

```rust
// src/storage/file.rs
use std::path::Path;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct OptimizedFileStore {
    base_path: PathBuf,
    compression_enabled: bool,
    cache_size: usize,
}

impl OptimizedFileStore {
    pub async fn store_document(&self, id: &str, content: &[u8]) -> Result<()> {
        let path = self.get_file_path(id);
        
        // Ensure directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        // Store with optional compression
        let data = if self.compression_enabled {
            compress_data(content)?
        } else {
            content.to_vec()
        };
        
        // Write atomically
        let temp_path = path.with_extension("tmp");
        fs::write(&temp_path, data).await?;
        fs::rename(temp_path, path).await?;
        
        Ok(())
    }
    
    pub async fn retrieve_document(&self, id: &str) -> Result<Vec<u8>> {
        let path = self.get_file_path(id);
        let data = fs::read(path).await?;
        
        if self.compression_enabled {
            decompress_data(&data)
        } else {
            Ok(data)
        }
    }
    
    fn get_file_path(&self, id: &str) -> PathBuf {
        // Use hierarchical directory structure for better file system performance
        let (dir1, dir2) = (&id[..2], &id[2..4]);
        self.base_path.join(dir1).join(dir2).join(format!("{}.dat", id))
    }
}

fn compress_data(data: &[u8]) -> Result<Vec<u8>> {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;
    
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    Ok(encoder.finish()?)
}
```

## Caching Strategies

### Multi-Level Caching

```rust
// src/cache/mod.rs
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use redis::AsyncCommands;

pub struct MultiLevelCache {
    // Level 1: In-memory cache (fastest)
    memory_cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    memory_capacity: usize,
    
    // Level 2: Redis cache (fast, shared)
    redis_client: redis::Client,
    
    // Level 3: Database (persistent)
    database: Arc<Database>,
}

impl MultiLevelCache {
    pub async fn get<T>(&self, key: &str) -> Result<Option<T>> 
    where 
        T: serde::de::DeserializeOwned + Clone,
    {
        // Try Level 1: Memory cache
        {
            let cache = self.memory_cache.read().await;
            if let Some(entry) = cache.get(key) {
                if !entry.is_expired() {
                    return Ok(Some(entry.value.clone()));
                }
            }
        }
        
        // Try Level 2: Redis cache
        let mut redis_conn = self.redis_client.get_async_connection().await?;
        if let Some(data) = redis_conn.get::<_, Option<String>>(key).await? {
            let value: T = serde_json::from_str(&data)?;
            
            // Store in memory cache for faster access
            self.set_memory_cache(key, value.clone()).await;
            
            return Ok(Some(value));
        }
        
        // Try Level 3: Database
        if let Some(value) = self.database.get(key).await? {
            // Store in both caches
            self.set_memory_cache(key, value.clone()).await;
            self.set_redis_cache(key, &value).await?;
            
            return Ok(Some(value));
        }
        
        Ok(None)
    }
    
    pub async fn set<T>(&self, key: &str, value: T, ttl: Duration) -> Result<()>
    where 
        T: serde::Serialize + Clone,
    {
        // Set in all levels
        self.set_memory_cache(key, value.clone()).await;
        self.set_redis_cache(key, &value).await?;
        self.database.set(key, &value).await?;
        
        Ok(())
    }
    
    async fn set_memory_cache<T>(&self, key: &str, value: T) 
    where 
        T: Clone,
    {
        let mut cache = self.memory_cache.write().await;
        
        // Evict oldest entries if at capacity
        if cache.len() >= self.memory_capacity {
            let oldest_key = cache.keys().next().unwrap().clone();
            cache.remove(&oldest_key);
        }
        
        cache.insert(key.to_string(), CacheEntry {
            value,
            created_at: Instant::now(),
            ttl: Duration::from_secs(3600),
        });
    }
    
    async fn set_redis_cache<T>(&self, key: &str, value: &T) -> Result<()>
    where 
        T: serde::Serialize,
    {
        let mut redis_conn = self.redis_client.get_async_connection().await?;
        let serialized = serde_json::to_string(value)?;
        
        redis_conn.setex(key, 3600, serialized).await?;
        Ok(())
    }
}
```

### Cache Warming

```rust
// src/cache/warming.rs
pub struct CacheWarmer {
    cache: Arc<MultiLevelCache>,
    database: Arc<Database>,
}

impl CacheWarmer {
    pub async fn warm_popular_documents(&self) -> Result<()> {
        // Get most accessed documents
        let popular_docs = self.database.get_popular_documents(100).await?;
        
        // Warm cache in batches
        for batch in popular_docs.chunks(10) {
            let futures = batch.iter().map(|doc| {
                let cache = self.cache.clone();
                async move {
                    cache.set(
                        &format!("doc:{}", doc.id),
                        doc.clone(),
                        Duration::from_secs(3600),
                    ).await
                }
            });
            
            futures::future::join_all(futures).await;
        }
        
        Ok(())
    }
    
    pub async fn warm_search_results(&self) -> Result<()> {
        // Get common search queries
        let common_queries = self.database.get_common_search_queries(50).await?;
        
        for query in common_queries {
            let results = self.database.search_documents(&query, 20).await?;
            
            self.cache.set(
                &format!("search:{}", query),
                results,
                Duration::from_secs(1800),
            ).await?;
        }
        
        Ok(())
    }
}
```

## Monitoring and Metrics

### Performance Metrics

```rust
// src/monitoring/metrics.rs
use prometheus::{Counter, Histogram, Gauge, register_counter, register_histogram, register_gauge};

pub struct PerformanceMetrics {
    // Request metrics
    pub request_count: Counter,
    pub request_duration: Histogram,
    pub active_requests: Gauge,
    
    // Document processing metrics
    pub documents_processed: Counter,
    pub processing_duration: Histogram,
    pub processing_errors: Counter,
    
    // System metrics
    pub memory_usage: Gauge,
    pub cpu_usage: Gauge,
    pub disk_usage: Gauge,
    
    // Cache metrics
    pub cache_hits: Counter,
    pub cache_misses: Counter,
    pub cache_evictions: Counter,
}

impl PerformanceMetrics {
    pub fn new() -> Result<Self> {
        Ok(Self {
            request_count: register_counter!(
                "swoop_requests_total",
                "Total number of requests"
            )?,
            request_duration: register_histogram!(
                "swoop_request_duration_seconds",
                "Request duration in seconds"
            )?,
            active_requests: register_gauge!(
                "swoop_active_requests",
                "Number of active requests"
            )?,
            documents_processed: register_counter!(
                "swoop_documents_processed_total",
                "Total number of documents processed"
            )?,
            processing_duration: register_histogram!(
                "swoop_processing_duration_seconds",
                "Document processing duration in seconds"
            )?,
            processing_errors: register_counter!(
                "swoop_processing_errors_total",
                "Total number of processing errors"
            )?,
            memory_usage: register_gauge!(
                "swoop_memory_usage_bytes",
                "Memory usage in bytes"
            )?,
            cpu_usage: register_gauge!(
                "swoop_cpu_usage_percent",
                "CPU usage percentage"
            )?,
            disk_usage: register_gauge!(
                "swoop_disk_usage_bytes",
                "Disk usage in bytes"
            )?,
            cache_hits: register_counter!(
                "swoop_cache_hits_total",
                "Total number of cache hits"
            )?,
            cache_misses: register_counter!(
                "swoop_cache_misses_total",
                "Total number of cache misses"
            )?,
            cache_evictions: register_counter!(
                "swoop_cache_evictions_total",
                "Total number of cache evictions"
            )?,
        })
    }
    
    pub fn record_request(&self, duration: Duration) {
        self.request_count.inc();
        self.request_duration.observe(duration.as_secs_f64());
    }
    
    pub fn record_document_processing(&self, duration: Duration, success: bool) {
        if success {
            self.documents_processed.inc();
        } else {
            self.processing_errors.inc();
        }
        self.processing_duration.observe(duration.as_secs_f64());
    }
    
    pub fn update_system_metrics(&self, memory: u64, cpu: f64, disk: u64) {
        self.memory_usage.set(memory as f64);
        self.cpu_usage.set(cpu);
        self.disk_usage.set(disk as f64);
    }
}
```

### Alerting

```rust
// src/monitoring/alerts.rs
pub struct AlertManager {
    metrics: Arc<PerformanceMetrics>,
    alert_rules: Vec<AlertRule>,
    notification_client: NotificationClient,
}

impl AlertManager {
    pub async fn check_alerts(&self) -> Result<()> {
        for rule in &self.alert_rules {
            if rule.should_alert(&self.metrics).await {
                self.send_alert(rule).await?;
            }
        }
        Ok(())
    }
    
    async fn send_alert(&self, rule: &AlertRule) -> Result<()> {
        let alert = Alert {
            severity: rule.severity,
            message: rule.message.clone(),
            timestamp: Utc::now(),
            metric_values: rule.get_current_values(&self.metrics).await,
        };
        
        self.notification_client.send_alert(alert).await
    }
}

pub struct AlertRule {
    pub name: String,
    pub condition: AlertCondition,
    pub severity: AlertSeverity,
    pub message: String,
    pub cooldown: Duration,
    pub last_triggered: Option<Instant>,
}

impl AlertRule {
    pub async fn should_alert(&mut self, metrics: &PerformanceMetrics) -> bool {
        // Check cooldown
        if let Some(last) = self.last_triggered {
            if last.elapsed() < self.cooldown {
                return false;
            }
        }
        
        // Check condition
        let should_alert = match &self.condition {
            AlertCondition::MemoryUsage { threshold } => {
                metrics.memory_usage.get() > *threshold
            }
            AlertCondition::ErrorRate { threshold, window } => {
                let error_rate = self.calculate_error_rate(metrics, *window).await;
                error_rate > *threshold
            }
            AlertCondition::ResponseTime { threshold, percentile } => {
                let response_time = self.calculate_response_time(metrics, *percentile).await;
                response_time > *threshold
            }
        };
        
        if should_alert {
            self.last_triggered = Some(Instant::now());
        }
        
        should_alert
    }
}
```

## Deployment Configurations

### Development Environment

```bash
# dev.env
RUST_LOG=debug
DATABASE_URL=sqlite:./swoop_dev.db
REDIS_URL=redis://localhost:6379
OPENROUTER_API_KEY=your_dev_key
WORKER_THREADS=2
MAX_CONNECTIONS=10
CACHE_SIZE=100MB
```

### Production Environment

```bash
# prod.env
RUST_LOG=info
DATABASE_URL=postgresql://user:pass@db-cluster:5432/swoop
REDIS_URL=redis://redis-cluster:6379
OPENROUTER_API_KEY=your_prod_key
WORKER_THREADS=16
MAX_CONNECTIONS=100
CACHE_SIZE=2GB
MAX_UPLOAD_SIZE=50MB
RATE_LIMIT_REQUESTS=1000
RATE_LIMIT_WINDOW=60
```

### Docker Optimization

```dockerfile
# Dockerfile.optimized
FROM rust:1.70-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source code
COPY src ./src
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -r -s /bin/false swoop

# Copy binary
COPY --from=builder /app/target/release/swoop_server /usr/local/bin/swoop_server

# Set ownership and permissions
RUN chown swoop:swoop /usr/local/bin/swoop_server
RUN chmod +x /usr/local/bin/swoop_server

# Switch to non-root user
USER swoop

# Optimize for production
ENV RUST_LOG=info
ENV MALLOC_ARENA_MAX=2

EXPOSE 8080
CMD ["swoop_server"]
```

### Kubernetes Deployment

```yaml
# k8s/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: swoop-api
spec:
  replicas: 3
  selector:
    matchLabels:
      app: swoop-api
  template:
    metadata:
      labels:
        app: swoop-api
    spec:
      containers:
      - name: swoop-api
        image: swoop:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: swoop-secrets
              key: database-url
        - name: REDIS_URL
          value: "redis://redis-service:6379"
        resources:
          requests:
            memory: "1Gi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "1000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
        # Performance optimizations
        env:
        - name: WORKER_THREADS
          value: "4"
        - name: MAX_CONNECTIONS
          value: "50"
        - name: RUST_LOG
          value: "info"
---
apiVersion: v1
kind: Service
metadata:
  name: swoop-api-service
spec:
  selector:
    app: swoop-api
  ports:
  - port: 80
    targetPort: 8080
  type: LoadBalancer
```

## Performance Benchmarks

### Load Testing

```bash
# Load test with Apache Bench
ab -n 10000 -c 100 -H "Authorization: Bearer your-token" \
   http://localhost:8080/api/documents

# Load test with wrk
wrk -t12 -c400 -d30s --header "Authorization: Bearer your-token" \
    http://localhost:8080/api/documents

# Document upload test
for i in {1..100}; do
  curl -X POST \
    -H "Authorization: Bearer your-token" \
    -F "file=@test-document.pdf" \
    http://localhost:8080/api/documents/upload &
done
wait
```

### Expected Performance Metrics

| Metric | Development | Production | High-Performance |
|--------|-------------|------------|------------------|
| Request/sec | 100-500 | 1000-5000 | 10000+ |
| Response time (95th) | <500ms | <200ms | <100ms |
| Memory usage | 1-2GB | 4-8GB | 16-32GB |
| CPU usage | 20-40% | 40-70% | 70-90% |
| Document processing | 1-5/sec | 10-50/sec | 100+/sec |

This performance tuning guide provides comprehensive strategies for optimizing Swoop across all deployment scenarios. Regular monitoring and incremental optimization based on actual usage patterns will yield the best results.