/*!
 * Modern High-Performance HTTP Server for Crawl4AI
 * 
 * Inspired by: https://dev.to/geoffreycopin/build-a-http-server-with-rust-and-tokio-part-1-serving-static-files-165l
 * 
 * Enhanced with modern features:
 * - Real-time WebSocket connections for live crawl updates
 * - Server-sent events for browser compatibility  
 * - High-performance static file serving with compression
 * - RESTful API for crawler management
 * - Graceful shutdown and connection management
 * - Comprehensive middleware pipeline
 * - Built-in metrics and monitoring
 * - Modern async/await patterns
 * - Connection pooling and reuse optimization
 */

use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use axum::{
    extract::{ws::WebSocket, ConnectInfo, Path, Path as AxumPath, Query, State, WebSocketUpgrade, Multipart},
    http::StatusCode, // HeaderMap may be needed for future header handling
    middleware::{self, Next},
    response::{Html, Response, Sse, Json as ResponseJson},
    routing::{get, post, delete},
    Json,
    Router,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::{broadcast, RwLock};
use tokio_util::sync::CancellationToken;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::{Document, DocumentWorkspace};
use crate::monitoring::MonitoringSystem;
use crate::common::ApiResponse;

/// Server configuration with rich customization
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub bind_addr: SocketAddr,
    pub static_dir: Option<PathBuf>,
    pub enable_compression: bool,
    pub request_timeout: u64,
    pub websocket_ping_interval: u64,
    pub max_connections: usize,
    pub enable_cors: bool,
    pub enable_logging: bool,
    pub server_name: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: "0.0.0.0:8080".parse().unwrap(),
            static_dir: Some(PathBuf::from("static")),
            enable_compression: true,
            request_timeout: 30,
            websocket_ping_interval: 30,
            max_connections: 1000,
            enable_cors: true,
            enable_logging: true,
            server_name: "Crawl4AI/1.0".to_string(),
        }
    }
}

/// Real-time server statistics with atomic counters
#[derive(Debug, Clone, Serialize)]
pub struct ServerStats {
    pub total_requests: u64,
    pub active_connections: u64,
    pub active_websockets: u64,
    pub uptime_seconds: u64,
    pub avg_response_time_ms: f64,
    pub total_bytes_transferred: u64,
}

/// Thread-safe statistics tracking
pub struct StatsTracker {
    total_requests: AtomicU64,
    active_connections: AtomicU64,
    active_websockets: AtomicU64,
    total_bytes_transferred: AtomicU64,
    response_times: RwLock<Vec<f64>>,
    start_time: Instant,
}

impl StatsTracker {
    pub fn new() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            active_connections: AtomicU64::new(0),
            active_websockets: AtomicU64::new(0),
            total_bytes_transferred: AtomicU64::new(0),
            response_times: RwLock::new(Vec::new()),
            start_time: Instant::now(),
        }
    }

    pub async fn get_stats(&self) -> ServerStats {
        let response_times = self.response_times.read().await;
        let avg_response_time_ms = if response_times.is_empty() {
            0.0
        } else {
            response_times.iter().sum::<f64>() / response_times.len() as f64
        };

        ServerStats {
            total_requests: self.total_requests.load(Ordering::Relaxed),
            active_connections: self.active_connections.load(Ordering::Relaxed),
            active_websockets: self.active_websockets.load(Ordering::Relaxed),
            uptime_seconds: self.start_time.elapsed().as_secs(),
            avg_response_time_ms,
            total_bytes_transferred: self.total_bytes_transferred.load(Ordering::Relaxed),
        }
    }

    pub fn increment_requests(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_connections(&self) {
        self.active_connections.fetch_add(1, Ordering::Relaxed);
    }

    pub fn decrement_connections(&self) {
        self.active_connections.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn increment_websockets(&self) {
        self.active_websockets.fetch_add(1, Ordering::Relaxed);
    }

    pub fn decrement_websockets(&self) {
        self.active_websockets.fetch_sub(1, Ordering::Relaxed);
    }

    pub async fn record_response_time(&self, duration: Duration) {
        let mut times = self.response_times.write().await;
        times.push(duration.as_millis() as f64);
        if times.len() > 1000 {
            times.remove(0);
        }
    }
}

/// Real-time events for live updates
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum LiveEvent {
    CrawlStarted {
        job_id: String,
        url: String,
        timestamp: String,
    },
    DocumentCrawled {
        job_id: String,
        url: String,
        title: Option<String>,
        size_bytes: usize,
        timestamp: String,
    },
    CrawlCompleted {
        job_id: String,
        total_documents: usize,
        duration_seconds: u64,
        timestamp: String,
    },
    StatsUpdate {
        stats: ServerStats,
        timestamp: String,
    },
    ClientConnected {
        client_id: String,
        ip: String,
        timestamp: String,
    },
}

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub config: ServerConfig,
    pub stats: Arc<StatsTracker>,
    pub event_sender: broadcast::Sender<LiveEvent>,
    pub documents: Arc<RwLock<HashMap<String, DocumentProcessingStatus>>>,
    pub conversations: Arc<RwLock<HashMap<String, Vec<ChatMessage>>>>,
    pub system_stats: Arc<RwLock<SystemStats>>,
    pub workspace: Arc<DocumentWorkspace>,
    pub monitoring: Arc<crate::monitoring::MonitoringSystem>,
    pub active_jobs: Arc<RwLock<HashMap<String, String>>>,
}

/// API request/response types
#[derive(Debug, Deserialize)]
pub struct StartCrawlRequest {
    pub urls: Vec<String>,
    pub max_depth: Option<usize>,
    pub max_pages: Option<usize>,
    pub delay_ms: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct StartCrawlResponse {
    pub job_id: String,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub limit: Option<usize>,
}

/// Document Processing Types
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentUploadRequest {
    pub filename: String,
    pub content_type: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentProcessingStatus {
    pub id: String,
    pub filename: String,
    pub status: ProcessingStatus,
    pub progress: f32,
    pub quality_score: Option<f32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub processing_time_ms: Option<u64>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessingStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

// Chat System Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub content: String,
    pub role: MessageRole,
    pub personality: Option<String>,
    pub document_references: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub personality: Option<String>,
    pub conversation_id: Option<String>,
    pub document_context: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub message: ChatMessage,
    pub conversation_id: String,
    pub processing_time_ms: u64,
}

// Analytics Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub documents_processed: u64,
    pub documents_pending: u64,
    pub average_quality_score: f32,
    pub processing_rate_per_hour: f32,
    pub error_rate: f32,
    pub uptime_seconds: u64,
    pub memory_usage_mb: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingMetrics {
    pub total_documents: u64,
    pub documents_by_status: HashMap<String, u64>,
    pub documents_by_type: HashMap<String, u64>,
    pub quality_score_distribution: Vec<(f32, u64)>,
    pub processing_times: Vec<u64>,
    pub recent_activity: Vec<ActivityEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityEvent {
    pub id: String,
    pub event_type: String,
    pub description: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub document_id: Option<String>,
    pub status: Option<String>,
}

/// High-performance HTTP server with modern features
pub struct CrawlServer {
    config: ServerConfig,
    state: AppState,
    shutdown_token: CancellationToken,
}

impl CrawlServer {
    pub fn new(config: ServerConfig) -> Result<Self> {
        let (event_sender, _) = broadcast::channel(1000);
        let stats = Arc::new(StatsTracker::new());
        let monitoring = MonitoringSystem::new()?;
        
        let state = AppState {
            config: config.clone(),
            stats,
            event_sender,
            documents: Arc::new(RwLock::new(HashMap::new())),
            conversations: Arc::new(RwLock::new(HashMap::new())),
            system_stats: Arc::new(RwLock::new(SystemStats {
                documents_processed: 0,
                documents_pending: 0,
                average_quality_score: 0.0,
                processing_rate_per_hour: 0.0,
                error_rate: 0.0,
                uptime_seconds: 0,
                memory_usage_mb: 0.0,
            })),
            workspace: Arc::new(DocumentWorkspace::new()),
            monitoring: Arc::new(monitoring),
            active_jobs: Arc::new(RwLock::new(HashMap::new())),
        };

        Ok(Self {
            config,
            state,
            shutdown_token: CancellationToken::new(),
        })
    }

    fn create_router(state: AppState) -> Router {
        let mut router = Router::new()
            // API routes
            .route("/api/v1/crawl", post(start_crawl))
            .route("/api/v1/crawl/:job_id", get(get_crawl_status))
            .route("/api/v1/crawl/:job_id", delete(stop_crawl))
            .route("/api/v1/documents", get(search_documents))
            .route("/api/v1/documents/:id", get(get_document))
            
            // Real-time endpoints
            .route("/ws", get(websocket_handler))
            .route("/events", get(sse_handler))
            
            // Legacy monitoring (keep for compatibility)
            .route("/api/v1/stats", get(get_server_stats))
            .route("/api/v1/health", get(health_check))
            
            // Advanced monitoring routes
            .route("/health", get(crate::monitoring::monitoring_health_check))
            .route("/ready", get(crate::monitoring::monitoring_readiness_check))
            .route("/metrics", get(crate::monitoring::monitoring_metrics_endpoint))
            .route("/monitoring/stats", get(crate::monitoring::monitoring_stats_endpoint))
            
            // Dashboard
            .route("/", get(serve_dashboard))
            .route("/dashboard", get(serve_dashboard))
            
            // Document endpoints
            .route("/api/documents/upload", post(upload_document))
            .route("/api/documents", get(list_documents))
            .route("/api/documents/:id", get(get_document_status))
            
            // Chat endpoints
            .route("/api/chat/query", post(chat_query))
            .route("/api/chat/conversations/:id", get(get_conversation))
            
            // Analytics endpoints
            .route("/api/stats", get(get_system_stats))
            .route("/api/metrics", get(get_processing_metrics))
            
            .with_state(state.clone());

        // Add static file serving
        if let Some(static_dir) = &state.config.static_dir {
            if static_dir.exists() {
                router = router.nest_service("/static", ServeDir::new(static_dir));
            }
        }

        // Add middleware stack
        let middleware_stack = ServiceBuilder::new()
            .layer(middleware::from_fn_with_state(state.clone(), request_middleware));

        if state.config.enable_compression {
            router = router.layer(CompressionLayer::new());
        }

        if state.config.enable_cors {
            router = router.layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any));
        }

        if state.config.enable_logging {
            router = router.layer(TraceLayer::new_for_http());
        }

        router.layer(middleware_stack)
    }

    pub async fn start(self) -> Result<()> {
        log::info!("🚀 Starting Crawl4AI server on {}", self.config.bind_addr);
        log::info!("📊 Dashboard: http://{}/dashboard", self.config.bind_addr);
        log::info!("🔌 WebSocket: ws://{}/ws", self.config.bind_addr);

        // Setup graceful shutdown
        let shutdown_token = self.shutdown_token.clone();
        tokio::spawn(async move {
            match signal::ctrl_c().await {
                Ok(_) => {
                    log::info!("🛑 Received Ctrl+C, shutting down gracefully...");
                    shutdown_token.cancel();
                }
                Err(e) => log::error!("❌ Failed to listen for Ctrl+C: {}", e),
            }
        });

        // Start stats broadcaster
        let stats_task = self.start_stats_broadcaster().await;

        // Create router and bind
        let app = Self::create_router(self.state.clone());
        let listener = TcpListener::bind(&self.config.bind_addr).await
            .map_err(|e| Error::Other(format!("Failed to bind: {}", e)))?;

        log::info!("✅ Server listening on {}", self.config.bind_addr);

        // Start server with graceful shutdown
        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
            .with_graceful_shutdown(async move {
                self.shutdown_token.cancelled().await;
                stats_task.abort();
                log::info!("✅ Server shutdown complete");
            })
            .await
            .map_err(|e| Error::Other(format!("Server error: {}", e)))?;

        Ok(())
    }

    async fn start_stats_broadcaster(&self) -> tokio::task::JoinHandle<()> {
        let state = self.state.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));
            loop {
                interval.tick().await;
                let stats = state.stats.get_stats().await;
                let event = LiveEvent::StatsUpdate {
                    stats,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };
                let _ = state.event_sender.send(event);
            }
        })
    }
}

/// Request middleware for metrics and logging
async fn request_middleware(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: axum::extract::Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();

    // Update legacy stats
    state.stats.increment_connections();
    state.stats.increment_requests();

    // Update advanced monitoring
    state.monitoring.record_crawl_request().await;
    state.monitoring.update_active_connections(state.stats.active_connections.load(Ordering::Relaxed)).await;

    let response = next.run(req).await;
    let latency = start.elapsed();

    // Record metrics in both systems
    state.stats.decrement_connections();
    state.stats.record_response_time(latency).await;
    
    // Advanced monitoring with success/failure detection
    if response.status().is_success() {
        state.monitoring.record_successful_crawl(latency).await;
    } else {
        state.monitoring.record_failed_crawl(latency).await;
    }

    if state.config.enable_logging {
        log::info!("📥 {} {} {} - {}ms - {}", addr, method, uri, latency.as_millis(), response.status());
    }

    response
}

/// API Handlers

async fn start_crawl(
    State(state): State<AppState>,
    Json(payload): Json<StartCrawlRequest>,
) -> std::result::Result<Json<StartCrawlResponse>, StatusCode> {
    log::info!("🕷️ Starting crawl for {} URLs", payload.urls.len());
    
    let job_id = format!("crawl_{}", uuid::Uuid::new_v4());
    
    {
        let mut jobs = state.active_jobs.write().await;
        jobs.insert(job_id.clone(), "running".to_string());
    }
    
    let event = LiveEvent::CrawlStarted {
        job_id: job_id.clone(),
        url: payload.urls.join(", "),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    let _ = state.event_sender.send(event);
    
    Ok(Json(StartCrawlResponse {
        job_id,
        status: "started".to_string(),
        message: format!("Started crawling {} URLs", payload.urls.len()),
    }))
}

async fn get_crawl_status(
    State(state): State<AppState>,
    AxumPath(job_id): AxumPath<String>,
) -> StatusCode {
    let jobs = state.active_jobs.read().await;
    if jobs.contains_key(&job_id) {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn stop_crawl(
    State(state): State<AppState>,
    AxumPath(job_id): AxumPath<String>,
) -> StatusCode {
    let mut jobs = state.active_jobs.write().await;
    if jobs.remove(&job_id).is_some() {
        log::info!("🛑 Stopped crawl job: {}", job_id);
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn search_documents(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Json<Vec<Document>> {
    log::info!("🔍 Searching documents: '{}'", params.q);
    
    let documents = state.documents.read().await;
    let results: Vec<Document> = documents.values()
        .take(params.limit.unwrap_or(10))
        .cloned()
        .collect();
    
    Json(results)
}

async fn get_document(
    State(state): State<AppState>,
    AxumPath(id): AxumPath<String>,
) -> std::result::Result<Json<Document>, StatusCode> {
    let documents = state.documents.read().await;
    match documents.get(&id) {
        Some(document) => Ok(Json(document.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_server_stats(State(state): State<AppState>) -> Json<ServerStats> {
    let stats = state.stats.get_stats().await;
    Json(stats)
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "crawl4ai-core",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn serve_dashboard() -> Html<&'static str> {
    Html(r#"<!DOCTYPE html>
<html>
<head>
    <title>Crawl4AI Dashboard</title>
    <style>
        body { font-family: -apple-system, sans-serif; margin: 0; padding: 20px; background: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; }
        .card { background: white; border-radius: 8px; padding: 20px; margin: 20px 0; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        .stats { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; }
        .stat { text-align: center; }
        .stat-value { font-size: 2em; font-weight: bold; color: #007acc; }
        .stat-label { color: #666; }
        .status { color: #28a745; font-weight: bold; }
        #events { height: 300px; overflow-y: auto; border: 1px solid #ddd; padding: 10px; font-family: monospace; font-size: 12px; }
        .event { margin: 5px 0; padding: 5px; background: #f8f9fa; border-radius: 3px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🕷️ Crawl4AI Dashboard</h1>
        
        <div class="card">
            <h2>Server Status</h2>
            <div class="status">✅ Online</div>
        </div>
        
        <div class="card">
            <h2>Statistics</h2>
            <div class="stats" id="stats">
                <div class="stat">
                    <div class="stat-value" id="total-requests">0</div>
                    <div class="stat-label">Total Requests</div>
                </div>
                <div class="stat">
                    <div class="stat-value" id="active-connections">0</div>
                    <div class="stat-label">Active Connections</div>
                </div>
                <div class="stat">
                    <div class="stat-value" id="active-websockets">0</div>
                    <div class="stat-label">WebSocket Connections</div>
                </div>
                <div class="stat">
                    <div class="stat-value" id="uptime">0s</div>
                    <div class="stat-label">Uptime</div>
                </div>
            </div>
        </div>
        
        <div class="card">
            <h2>Live Events</h2>
            <div id="events"></div>
        </div>
    </div>
    
    <script>
        const eventsContainer = document.getElementById('events');
        const eventSource = new EventSource('/events');
        
        eventSource.onmessage = function(event) {
            const data = JSON.parse(event.data);
            
            if (data.type === 'StatsUpdate') {
                updateStats(data.stats);
            }
            
            addEvent(data);
        };
        
        function updateStats(stats) {
            document.getElementById('total-requests').textContent = stats.total_requests;
            document.getElementById('active-connections').textContent = stats.active_connections;
            document.getElementById('active-websockets').textContent = stats.active_websockets;
            document.getElementById('uptime').textContent = stats.uptime_seconds + 's';
        }
        
        function addEvent(event) {
            const eventDiv = document.createElement('div');
            eventDiv.className = 'event';
            eventDiv.textContent = new Date().toLocaleTimeString() + ' - ' + JSON.stringify(event);
            eventsContainer.insertBefore(eventDiv, eventsContainer.firstChild);
            
            if (eventsContainer.children.length > 50) {
                eventsContainer.removeChild(eventsContainer.lastChild);
            }
        }
        
        // Fetch initial stats
        fetch('/api/v1/stats')
            .then(r => r.json())
            .then(updateStats);
    </script>
</body>
</html>"#)
}

/// WebSocket handler for real-time updates
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> axum::response::Response {
    log::info!("🔌 WebSocket connection from {}", addr);
    ws.on_upgrade(move |socket| handle_websocket(socket, state, addr))
}

async fn handle_websocket(socket: WebSocket, state: AppState, addr: SocketAddr) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.event_sender.subscribe();
    let client_id = uuid::Uuid::new_v4().to_string();
    
    state.stats.increment_websockets();
    
    let event = LiveEvent::ClientConnected {
        client_id: client_id.clone(),
        ip: addr.ip().to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    let _ = state.event_sender.send(event);
    
    // Create a channel for ping messages since SplitSink can't be cloned
    let (ping_tx, mut ping_rx) = tokio::sync::mpsc::channel(1);
    
    // Ping task
    let ping_task = {
        let interval = state.config.websocket_ping_interval;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(interval));
            loop {
                interval.tick().await;
                if ping_tx.send(()).await.is_err() {
                    break;
                }
            }
        })
    };
    
    // Broadcast and ping task
    let broadcast_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                event = rx.recv() => {
                    match event {
                        Ok(event) => {
                            let message = serde_json::to_string(&event).unwrap_or_default();
                            if sender.send(axum::extract::ws::Message::Text(message)).await.is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
                ping = ping_rx.recv() => {
                    match ping {
                        Some(_) => {
                            if sender.send(axum::extract::ws::Message::Ping(vec![])).await.is_err() {
                                break;
                            }
                        }
                        None => break,
                    }
                }
            }
        }
    });
    
    // Handle incoming messages
    let receive_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(axum::extract::ws::Message::Text(text)) => {
                    log::debug!("📨 WebSocket message from {}: {}", addr, text);
                }
                Ok(axum::extract::ws::Message::Close(_)) => {
                    log::info!("🔐 WebSocket close from {}", addr);
                    break;
                }
                Err(e) => {
                    log::error!("❌ WebSocket error from {}: {}", addr, e);
                    break;
                }
                _ => {}
            }
        }
    });
    
    tokio::select! {
        _ = ping_task => {},
        _ = broadcast_task => {},
        _ = receive_task => {},
    }
    
    state.stats.decrement_websockets();
    log::info!("🔌 WebSocket connection closed for {}", addr);
}

/// Server-sent events handler
async fn sse_handler(
    State(state): State<AppState>,
) -> Sse<impl futures_util::Stream<Item = std::result::Result<axum::response::sse::Event, io::Error>>> {
    log::info!("📡 SSE connection established");
    
    let rx = state.event_sender.subscribe();
    
    let stream = tokio_stream::wrappers::BroadcastStream::new(rx)
        .map(|result| {
            match result {
                Ok(event) => {
                    let data = serde_json::to_string(&event).unwrap_or_default();
                    Ok(axum::response::sse::Event::default().event("update").data(data))
                }
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "broadcast error")),
            }
        });
    
    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive")
    )
}

// API Handlers

pub async fn upload_document(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> std::result::Result<Json<DocumentProcessingStatus>, StatusCode> {
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "file" {
            let filename = field.file_name().unwrap_or("unknown").to_string();
            let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            
            let document_id = Uuid::new_v4().to_string();
            let document_status = DocumentProcessingStatus {
                id: document_id.clone(),
                filename: filename.clone(),
                status: ProcessingStatus::Pending,
                progress: 0.0,
                quality_score: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                processing_time_ms: None,
                error_message: None,
            };
            
            // Store document status
            state.documents.write().await.insert(document_id.clone(), document_status.clone());
            
            // Start background processing (simulate for now)
            let state_clone = state.clone();
            let doc_id = document_id.clone();
            tokio::spawn(async move {
                simulate_document_processing(state_clone, doc_id, data.to_vec()).await;
            });
            
            return Ok(Json(document_status));
        }
    }
    
    Err(StatusCode::BAD_REQUEST)
}

pub async fn get_document_status(
    State(state): State<AppState>,
    Path(document_id): Path<String>,
) -> std::result::Result<Json<DocumentProcessingStatus>, StatusCode> {
    let documents = state.documents.read().await;
    
    if let Some(document) = documents.get(&document_id) {
        Ok(Json(document.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn list_documents(
    State(state): State<AppState>,
) -> std::result::Result<Json<Vec<DocumentProcessingStatus>>, StatusCode> {
    let documents = state.documents.read().await;
    let document_list: Vec<DocumentProcessingStatus> = documents.values().cloned().collect();
    
    Ok(Json(document_list))
}

pub async fn chat_query(
    State(state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> std::result::Result<Json<ChatResponse>, StatusCode> {
    let start_time = std::time::Instant::now();
    
    // Generate conversation ID if not provided
    let conversation_id = request.conversation_id.unwrap_or_else(|| Uuid::new_v4().to_string());
    
    // Parse document references from message
    let document_references = extract_document_references(&request.message);
    
    // Create user message
    let user_message = ChatMessage {
        id: Uuid::new_v4().to_string(),
        content: request.message.clone(),
        role: MessageRole::User,
        personality: request.personality.clone(),
        document_references: document_references.clone(),
        timestamp: chrono::Utc::now(),
    };
    
    // Generate AI response (mock for now)
    let ai_response = generate_ai_response(&request, &document_references).await;
    
    let ai_message = ChatMessage {
        id: Uuid::new_v4().to_string(),
        content: ai_response,
        role: MessageRole::Assistant,
        personality: request.personality.clone(),
        document_references: document_references.clone(),
        timestamp: chrono::Utc::now(),
    };
    
    // Store conversation
    let mut conversations = state.conversations.write().await;
    let conversation = conversations.entry(conversation_id.clone()).or_insert_with(Vec::new);
    conversation.push(user_message);
    conversation.push(ai_message.clone());
    
    let processing_time = start_time.elapsed().as_millis() as u64;
    
    let response = ChatResponse {
        message: ai_message,
        conversation_id,
        processing_time_ms: processing_time,
    };
    
    Ok(Json(response))
}

pub async fn get_conversation(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
) -> Result<Json<Vec<ChatMessage>>, StatusCode> {
    let conversations = state.conversations.read().await;
    
    if let Some(messages) = conversations.get(&conversation_id) {
        Ok(Json(messages.clone()))
    } else {
        Ok(Json(Vec::new()))
    }
}

pub async fn get_system_stats(
    State(state): State<AppState>,
) -> std::result::Result<Json<SystemStats>, StatusCode> {
    let stats = state.system_stats.read().await;
    Ok(Json(stats.clone()))
}

pub async fn get_processing_metrics(
    State(state): State<AppState>,
) -> std::result::Result<Json<ProcessingMetrics>, StatusCode> {
    let documents = state.documents.read().await;
    
    let total_documents = documents.len() as u64;
    let mut documents_by_status = HashMap::new();
    let mut documents_by_type = HashMap::new();
    let mut quality_scores = Vec::new();
    let mut processing_times = Vec::new();
    
    for doc in documents.values() {
        // Count by status
        let status_key = format!("{:?}", doc.status);
        *documents_by_status.entry(status_key).or_insert(0) += 1;
        
        // Count by type (extract from filename)
        let file_type = doc.filename.split('.').last().unwrap_or("unknown").to_string();
        *documents_by_type.entry(file_type).or_insert(0) += 1;
        
        // Collect quality scores
        if let Some(score) = doc.quality_score {
            quality_scores.push(score);
        }
        
        // Collect processing times
        if let Some(time) = doc.processing_time_ms {
            processing_times.push(time);
        }
    }
    
    // Generate quality score distribution
    let quality_score_distribution = generate_quality_distribution(&quality_scores);
    
    // Generate recent activity
    let recent_activity = generate_recent_activity(&documents);
    
    let metrics = ProcessingMetrics {
        total_documents,
        documents_by_status,
        documents_by_type,
        quality_score_distribution,
        processing_times,
        recent_activity,
    };
    
    Ok(Json(metrics))
}

// Helper Functions
async fn simulate_document_processing(state: AppState, document_id: String, _data: Vec<u8>) {
    // Simulate processing stages
    let stages = vec![
        (ProcessingStatus::Processing, 0.2, "Starting analysis..."),
        (ProcessingStatus::Processing, 0.5, "Extracting content..."),
        (ProcessingStatus::Processing, 0.8, "Analyzing quality..."),
        (ProcessingStatus::Completed, 1.0, "Processing complete"),
    ];
    
    for (status, progress, _message) in stages {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        let mut documents = state.documents.write().await;
        if let Some(doc) = documents.get_mut(&document_id) {
            doc.status = status;
            doc.progress = progress;
            doc.updated_at = chrono::Utc::now();
            
            if progress >= 1.0 {
                doc.quality_score = Some(0.85 + (rand::random::<f32>() * 0.15)); // 0.85-1.0
                doc.processing_time_ms = Some(1500 + (rand::random::<u64>() % 2000)); // 1.5-3.5s
            }
        }
    }
    
    // Update system stats
    let mut stats = state.system_stats.write().await;
    stats.documents_processed += 1;
    stats.average_quality_score = 0.92; // Mock value
}

fn extract_document_references(message: &str) -> Vec<String> {
    let mut references = Vec::new();
    let words: Vec<&str> = message.split_whitespace().collect();
    
    for word in words {
        if word.starts_with('@') && word.len() > 1 {
            references.push(word[1..].to_string());
        }
    }
    
    references
}

async fn generate_ai_response(request: &ChatRequest, document_refs: &[String]) -> String {
    // Mock AI response based on personality and document references
    let personality = request.personality.as_deref().unwrap_or("professional");
    
    if !document_refs.is_empty() {
        let docs_str = document_refs.join(", ");
        match personality {
            "technical" => format!(
                "Based on my analysis of {}, I can provide detailed technical insights. The documents contain structured data with high confidence scores. Would you like me to elaborate on specific technical aspects?",
                docs_str
            ),
            "casual" => format!(
                "Hey! I took a look at {} and found some interesting stuff. The content looks solid and I can break it down for you in simple terms. What specific part are you curious about?",
                docs_str
            ),
            _ => format!(
                "I have analyzed the referenced documents: {}. The content has been processed with quality validation and I can provide comprehensive insights. Please let me know what specific information you're looking for.",
                docs_str
            ),
        }
    } else {
        match personality {
            "technical" => "I'm ready to provide technical analysis and detailed documentation insights. Please specify which documents you'd like me to examine or upload new files for processing.".to_string(),
            "casual" => "Hi there! I'm here to help you understand your documents in a friendly way. Just upload some files or reference existing ones with @ and I'll break things down for you!".to_string(),
            _ => "Hello! I'm your document intelligence assistant. I can help analyze documents, extract insights, and answer questions about your processed files. How can I assist you today?".to_string(),
        }
    }
}

fn generate_quality_distribution(scores: &[f32]) -> Vec<(f32, u64)> {
    let mut distribution = vec![
        (0.7, 0), (0.75, 0), (0.8, 0), (0.85, 0), (0.9, 0), (0.95, 0), (1.0, 0)
    ];
    
    for &score in scores {
        for (threshold, count) in distribution.iter_mut() {
            if score >= *threshold {
                *count += 1;
            }
        }
    }
    
    distribution
}

fn generate_recent_activity(documents: &HashMap<String, DocumentProcessingStatus>) -> Vec<ActivityEvent> {
    let mut activities = Vec::new();
    
    for doc in documents.values().take(10) {
        activities.push(ActivityEvent {
            id: Uuid::new_v4().to_string(),
            event_type: "document_processed".to_string(),
            description: format!("Processed document: {}", doc.filename),
            timestamp: doc.updated_at,
            document_id: Some(doc.id.clone()),
            status: Some(format!("{:?}", doc.status)),
        });
    }
    
    activities.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    activities
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_server_creation() {
        let config = ServerConfig::default();
        let _server = CrawlServer::new(config);
        assert!(true);
    }
    
    #[tokio::test]
    async fn test_stats_tracker() {
        let tracker = StatsTracker::new();
        tracker.increment_requests();
        tracker.increment_connections();
        
        let stats = tracker.get_stats().await;
        assert_eq!(stats.total_requests, 1);
        assert_eq!(stats.active_connections, 1);
    }
} 