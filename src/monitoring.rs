/*!
 * Advanced Monitoring Module
 * 
 * Provides comprehensive monitoring capabilities for Crawl4AI Core:
 * - Prometheus metrics collection
 * - Health check endpoints (/health, /ready, /metrics)
 * - Performance statistics and dashboards
 * - Real-time monitoring integration
 */

use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
// Simplified monitoring without axum_prometheus dependency
use metrics::{counter, gauge, histogram, describe_counter, describe_gauge, describe_histogram};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::info;

use crate::error::Result;
use crate::server::AppState;

/// Comprehensive monitoring system for Crawl4AI Core
#[derive(Clone)]
pub struct MonitoringSystem {
    pub stats: Arc<RwLock<SystemStats>>,
    pub health_checker: Arc<HealthChecker>,
    startup_time: Instant,
}

/// System-wide statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub requests_total: u64,
    pub successful_crawls: u64,
    pub failed_crawls: u64,
    pub rate_limited_requests: u64,
    pub parser_operations: u64,
    pub storage_operations: u64,
    pub active_connections: u64,
    pub uptime_seconds: u64,
    pub memory_usage_bytes: u64,
    pub crawl_latency_stats: LatencyStats,
    pub storage_latency_stats: LatencyStats,
}

/// Latency statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyStats {
    pub min_ms: f64,
    pub max_ms: f64,
    pub avg_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
}

/// Health check status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub uptime_seconds: u64,
    pub checks: HashMap<String, ComponentHealth>,
}

/// Individual component health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub healthy: bool,
    pub message: String,
    pub last_check: String,
}

/// Health checker for system components
pub struct HealthChecker {
    storage_healthy: Arc<RwLock<bool>>,
    rate_limiter_healthy: Arc<RwLock<bool>>,
    parser_healthy: Arc<RwLock<bool>>,
}

impl Default for SystemStats {
    fn default() -> Self {
        Self {
            requests_total: 0,
            successful_crawls: 0,
            failed_crawls: 0,
            rate_limited_requests: 0,
            parser_operations: 0,
            storage_operations: 0,
            active_connections: 0,
            uptime_seconds: 0,
            memory_usage_bytes: 0,
            crawl_latency_stats: LatencyStats::default(),
            storage_latency_stats: LatencyStats::default(),
        }
    }
}

impl Default for LatencyStats {
    fn default() -> Self {
        Self {
            min_ms: 0.0,
            max_ms: 0.0,
            avg_ms: 0.0,
            p95_ms: 0.0,
            p99_ms: 0.0,
        }
    }
}

impl MonitoringSystem {
    /// Initialize the monitoring system with basic metrics
    pub fn new() -> Result<Self> {
        // Register custom metrics
        Self::register_metrics();
        
        let health_checker = Arc::new(HealthChecker::new());
        let stats = Arc::new(RwLock::new(SystemStats::default()));
        
        info!("🔧 Advanced monitoring system initialized");
        
        Ok(Self {
            stats,
            health_checker,
            startup_time: Instant::now(),
        })
    }

    /// Register all custom metrics with descriptions
    fn register_metrics() {
        // Counters
        describe_counter!("crawl4ai_requests_total", "Total number of crawl requests");
        describe_counter!("crawl4ai_successful_crawls", "Number of successful crawls");
        describe_counter!("crawl4ai_failed_crawls", "Number of failed crawls");
        describe_counter!("crawl4ai_rate_limited_requests", "Number of rate limited requests");
        describe_counter!("crawl4ai_parser_operations", "Number of parser operations");
        describe_counter!("crawl4ai_storage_operations", "Number of storage operations");

        // Gauges
        describe_gauge!("crawl4ai_active_connections", "Current number of active connections");
        describe_gauge!("crawl4ai_memory_usage_bytes", "Current memory usage in bytes");
        describe_gauge!("crawl4ai_uptime_seconds", "System uptime in seconds");

        // Histograms
        describe_histogram!("crawl4ai_crawl_duration_seconds", "Time taken to complete crawl requests");
        describe_histogram!("crawl4ai_storage_operation_duration_seconds", "Time taken for storage operations");
        describe_histogram!("crawl4ai_parser_duration_seconds", "Time taken for parsing operations");
    }

    /// Create monitoring routes for health checks and metrics
    /// Note: These routes are integrated directly into the server router

    /// Record a crawl request
    pub async fn record_crawl_request(&self) {
        counter!("crawl4ai_requests_total").increment(1);
        
        let mut stats = self.stats.write().await;
        stats.requests_total += 1;
    }

    /// Record a successful crawl
    pub async fn record_successful_crawl(&self, duration: Duration) {
        counter!("crawl4ai_successful_crawls").increment(1);
        histogram!("crawl4ai_crawl_duration_seconds").record(duration.as_secs_f64());
        
        let mut stats = self.stats.write().await;
        stats.successful_crawls += 1;
        
        // Update latency stats
        self.update_latency_stats(&mut stats.crawl_latency_stats, duration.as_millis() as f64).await;
    }

    /// Record a failed crawl
    pub async fn record_failed_crawl(&self, duration: Duration) {
        counter!("crawl4ai_failed_crawls").increment(1);
        histogram!("crawl4ai_crawl_duration_seconds").record(duration.as_secs_f64());
        
        let mut stats = self.stats.write().await;
        stats.failed_crawls += 1;
    }

    /// Record rate limiting
    pub async fn record_rate_limited(&self) {
        counter!("crawl4ai_rate_limited_requests").increment(1);
        
        let mut stats = self.stats.write().await;
        stats.rate_limited_requests += 1;
    }

    /// Record parser operation
    pub async fn record_parser_operation(&self, duration: Duration) {
        counter!("crawl4ai_parser_operations").increment(1);
        histogram!("crawl4ai_parser_duration_seconds").record(duration.as_secs_f64());
        
        let mut stats = self.stats.write().await;
        stats.parser_operations += 1;
    }

    /// Record storage operation
    pub async fn record_storage_operation(&self, duration: Duration) {
        counter!("crawl4ai_storage_operations").increment(1);
        histogram!("crawl4ai_storage_operation_duration_seconds").record(duration.as_secs_f64());
        
        let mut stats = self.stats.write().await;
        stats.storage_operations += 1;
        
        // Update storage latency stats
        self.update_latency_stats(&mut stats.storage_latency_stats, duration.as_millis() as f64).await;
    }

    /// Update active connections count
    pub async fn update_active_connections(&self, count: u64) {
        gauge!("crawl4ai_active_connections").set(count as f64);
        
        let mut stats = self.stats.write().await;
        stats.active_connections = count;
    }

    /// Update memory usage
    pub async fn update_memory_usage(&self, bytes: u64) {
        gauge!("crawl4ai_memory_usage_bytes").set(bytes as f64);
        
        let mut stats = self.stats.write().await;
        stats.memory_usage_bytes = bytes;
    }

    /// Update system uptime
    pub async fn update_uptime(&self) {
        let uptime = self.startup_time.elapsed().as_secs();
        gauge!("crawl4ai_uptime_seconds").set(uptime as f64);
        
        let mut stats = self.stats.write().await;
        stats.uptime_seconds = uptime;
    }

    /// Update latency statistics
    async fn update_latency_stats(&self, latency_stats: &mut LatencyStats, duration_ms: f64) {
        // Simple running statistics - in production, you might use a more sophisticated approach
        if latency_stats.min_ms == 0.0 || duration_ms < latency_stats.min_ms {
            latency_stats.min_ms = duration_ms;
        }
        
        if duration_ms > latency_stats.max_ms {
            latency_stats.max_ms = duration_ms;
        }
        
        // Simple running average - in production, consider using exponential moving average
        latency_stats.avg_ms = (latency_stats.avg_ms + duration_ms) / 2.0;
        
        // Note: p95 and p99 would require a histogram in production
        latency_stats.p95_ms = latency_stats.avg_ms * 1.5;
        latency_stats.p99_ms = latency_stats.avg_ms * 2.0;
    }

    /// Get current system statistics
    pub async fn get_stats(&self) -> SystemStats {
        self.update_uptime().await;
        self.stats.read().await.clone()
    }
}

impl HealthChecker {
    fn new() -> Self {
        Self {
            storage_healthy: Arc::new(RwLock::new(true)),
            rate_limiter_healthy: Arc::new(RwLock::new(true)),
            parser_healthy: Arc::new(RwLock::new(true)),
        }
    }

    /// Update storage health status
    pub async fn update_storage_health(&self, healthy: bool) {
        *self.storage_healthy.write().await = healthy;
    }

    /// Update rate limiter health status
    pub async fn update_rate_limiter_health(&self, healthy: bool) {
        *self.rate_limiter_healthy.write().await = healthy;
    }

    /// Update parser health status
    pub async fn update_parser_health(&self, healthy: bool) {
        *self.parser_healthy.write().await = healthy;
    }

    /// Get comprehensive health status
    pub async fn get_health_status(&self, uptime_seconds: u64) -> HealthStatus {
        let mut checks = HashMap::new();
        
        let storage_healthy = *self.storage_healthy.read().await;
        let rate_limiter_healthy = *self.rate_limiter_healthy.read().await;
        let parser_healthy = *self.parser_healthy.read().await;
        
        checks.insert("storage".to_string(), ComponentHealth {
            healthy: storage_healthy,
            message: if storage_healthy { "Storage is operational".to_string() } else { "Storage has issues".to_string() },
            last_check: chrono::Utc::now().to_rfc3339(),
        });
        
        checks.insert("rate_limiter".to_string(), ComponentHealth {
            healthy: rate_limiter_healthy,
            message: if rate_limiter_healthy { "Rate limiter is operational".to_string() } else { "Rate limiter has issues".to_string() },
            last_check: chrono::Utc::now().to_rfc3339(),
        });
        
        checks.insert("parser".to_string(), ComponentHealth {
            healthy: parser_healthy,
            message: if parser_healthy { "Parser is operational".to_string() } else { "Parser has issues".to_string() },
            last_check: chrono::Utc::now().to_rfc3339(),
        });
        
        let overall_healthy = storage_healthy && rate_limiter_healthy && parser_healthy;
        
        HealthStatus {
            status: if overall_healthy { "healthy".to_string() } else { "unhealthy".to_string() },
            uptime_seconds,
            checks,
        }
    }
}

// HTTP endpoint handlers

/// Basic health check endpoint  
pub async fn monitoring_health_check(State(state): State<AppState>) -> impl IntoResponse {
    let stats = state.monitoring.get_stats().await;
    let health_status = state.monitoring.health_checker.get_health_status(stats.uptime_seconds).await;
    
    if health_status.status == "healthy" {
        (StatusCode::OK, Json(health_status)).into_response()
    } else {
        // Return 503 Service Unavailable for unhealthy status
        (StatusCode::SERVICE_UNAVAILABLE, Json(health_status)).into_response()
    }
}

/// Readiness check endpoint (for Kubernetes)
pub async fn monitoring_readiness_check(State(state): State<AppState>) -> impl IntoResponse {
    let stats = state.monitoring.get_stats().await;
    let health_status = state.monitoring.health_checker.get_health_status(stats.uptime_seconds).await;
    
    // Readiness is more strict - checks if system can handle new requests
    let ready = health_status.status == "healthy" && stats.active_connections < 1000;
    
    if ready {
        (StatusCode::OK, Json(serde_json::json!({
            "status": "ready",
            "uptime_seconds": stats.uptime_seconds,
            "active_connections": stats.active_connections
        }))).into_response()
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({
            "status": "not_ready",
            "reason": "System overloaded or unhealthy"
        }))).into_response()
    }
}

/// Prometheus metrics endpoint
pub async fn monitoring_metrics_endpoint(State(state): State<AppState>) -> impl IntoResponse {
    // Return basic metrics in text format
    let stats = state.monitoring.get_stats().await;
    let metrics_text = format!(
        "# HELP crawl4ai_requests_total Total number of crawl requests\n# TYPE crawl4ai_requests_total counter\ncrawl4ai_requests_total {}\n\n# HELP crawl4ai_successful_crawls Number of successful crawls\n# TYPE crawl4ai_successful_crawls counter\ncrawl4ai_successful_crawls {}\n\n# HELP crawl4ai_failed_crawls Number of failed crawls\n# TYPE crawl4ai_failed_crawls counter\ncrawl4ai_failed_crawls {}\n\n# HELP crawl4ai_active_connections Current number of active connections\n# TYPE crawl4ai_active_connections gauge\ncrawl4ai_active_connections {}\n\n# HELP crawl4ai_uptime_seconds System uptime in seconds\n# TYPE crawl4ai_uptime_seconds gauge\ncrawl4ai_uptime_seconds {}\n",
        stats.requests_total,
        stats.successful_crawls,
        stats.failed_crawls,
        stats.active_connections,
        stats.uptime_seconds
    );
    
    axum::response::Response::builder()
        .header("content-type", "text/plain; version=0.0.4; charset=utf-8")
        .body(metrics_text)
        .unwrap()
}

/// Comprehensive statistics endpoint
pub async fn monitoring_stats_endpoint(State(state): State<AppState>) -> impl IntoResponse {
    let stats = state.monitoring.get_stats().await;
    Json(stats)
} 