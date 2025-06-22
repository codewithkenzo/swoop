use anyhow::Result;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info};

use super::models::Usage;

/// LLM analytics service for tracking usage, costs, and performance
pub struct LLMAnalytics {
    user_analytics: Arc<DashMap<String, UserAnalytics>>,
    model_metrics: Arc<RwLock<HashMap<String, ModelMetrics>>>,
    global_stats: Arc<RwLock<GlobalStats>>,
}

impl LLMAnalytics {
    pub fn new() -> Self {
        Self {
            user_analytics: Arc::new(DashMap::new()),
            model_metrics: Arc::new(RwLock::new(HashMap::new())),
            global_stats: Arc::new(RwLock::new(GlobalStats::default())),
        }
    }

    /// Record a completion request
    pub async fn record_completion(
        &self,
        user_id: &str,
        model_id: &str,
        duration: Duration,
        usage: Option<&Usage>,
    ) {
        // Update user analytics
        let mut user_analytics = self.user_analytics.entry(user_id.to_string())
            .or_insert_with(|| UserAnalytics::new(user_id));
        
        user_analytics.record_request(model_id, duration, usage);

        // Update model metrics
        let mut model_metrics = self.model_metrics.write().await;
        let model_metric = model_metrics.entry(model_id.to_string())
            .or_insert_with(|| ModelMetrics::new(model_id));
        
        model_metric.record_request(duration, usage);

        // Update global stats
        let mut global_stats = self.global_stats.write().await;
        global_stats.record_request(duration, usage);

        debug!("Recorded completion for user: {}, model: {}", user_id, model_id);
    }

    /// Record a cache hit
    pub async fn record_cache_hit(&self, user_id: &str) {
        let mut user_analytics = self.user_analytics.entry(user_id.to_string())
            .or_insert_with(|| UserAnalytics::new(user_id));
        
        user_analytics.cache_hits += 1;

        let mut global_stats = self.global_stats.write().await;
        global_stats.cache_hits += 1;

        debug!("Recorded cache hit for user: {}", user_id);
    }

    /// Get analytics for a specific user
    pub async fn get_user_analytics(&self, user_id: &str) -> Result<UserAnalytics> {
        if let Some(analytics) = self.user_analytics.get(user_id) {
            Ok(analytics.clone())
        } else {
            Ok(UserAnalytics::new(user_id))
        }
    }

    /// Get metrics for all models
    pub async fn get_model_metrics(&self) -> Result<HashMap<String, ModelMetrics>> {
        let metrics = self.model_metrics.read().await;
        Ok(metrics.clone())
    }

    /// Get global statistics
    pub async fn get_global_stats(&self) -> Result<GlobalStats> {
        let stats = self.global_stats.read().await;
        Ok(stats.clone())
    }
}

/// User-specific analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAnalytics {
    pub user_id: String,
    pub total_requests: u64,
    pub total_cost: f64,
    pub total_tokens: u64,
    pub cache_hits: u64,
    pub model_usage: HashMap<String, ModelUsage>,
    pub first_request_time: Instant,
    pub last_request_time: Instant,
}

impl UserAnalytics {
    pub fn new(user_id: &str) -> Self {
        let now = Instant::now();
        Self {
            user_id: user_id.to_string(),
            total_requests: 0,
            total_cost: 0.0,
            total_tokens: 0,
            cache_hits: 0,
            model_usage: HashMap::new(),
            first_request_time: now,
            last_request_time: now,
        }
    }

    pub fn record_request(&mut self, model_id: &str, duration: Duration, usage: Option<&Usage>) {
        self.total_requests += 1;
        self.last_request_time = Instant::now();

        if let Some(usage) = usage {
            self.total_tokens += usage.total_tokens as u64;
            if let Some(cost) = usage.cost {
                self.total_cost += cost;
            }
        }

        let model_usage = self.model_usage.entry(model_id.to_string())
            .or_insert_with(|| ModelUsage::new(model_id));
        
        model_usage.record_request(duration, usage);
    }
}

/// Model-specific usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelUsage {
    pub model_id: String,
    pub requests: u64,
    pub total_cost: f64,
    pub total_tokens: u64,
    pub avg_response_time: Duration,
    pub last_used: Instant,
}

impl ModelUsage {
    pub fn new(model_id: &str) -> Self {
        Self {
            model_id: model_id.to_string(),
            requests: 0,
            total_cost: 0.0,
            total_tokens: 0,
            avg_response_time: Duration::from_millis(0),
            last_used: Instant::now(),
        }
    }

    pub fn record_request(&mut self, duration: Duration, usage: Option<&Usage>) {
        self.requests += 1;
        self.last_used = Instant::now();

        // Update average response time
        let total_time = self.avg_response_time.as_millis() as u64 * (self.requests - 1) + duration.as_millis() as u64;
        self.avg_response_time = Duration::from_millis(total_time / self.requests);

        if let Some(usage) = usage {
            self.total_tokens += usage.total_tokens as u64;
            if let Some(cost) = usage.cost {
                self.total_cost += cost;
            }
        }
    }
}

/// Model performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub model_id: String,
    pub total_requests: u64,
    pub total_cost: f64,
    pub total_tokens: u64,
    pub avg_response_time: Duration,
    pub error_rate: f64,
    pub unique_users: u64,
}

impl ModelMetrics {
    pub fn new(model_id: &str) -> Self {
        Self {
            model_id: model_id.to_string(),
            total_requests: 0,
            total_cost: 0.0,
            total_tokens: 0,
            avg_response_time: Duration::from_millis(0),
            error_rate: 0.0,
            unique_users: 0,
        }
    }

    pub fn record_request(&mut self, duration: Duration, usage: Option<&Usage>) {
        self.total_requests += 1;

        // Update average response time
        let total_time = self.avg_response_time.as_millis() as u64 * (self.total_requests - 1) + duration.as_millis() as u64;
        self.avg_response_time = Duration::from_millis(total_time / self.total_requests);

        if let Some(usage) = usage {
            self.total_tokens += usage.total_tokens as u64;
            if let Some(cost) = usage.cost {
                self.total_cost += cost;
            }
        }
    }
}

/// Global statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalStats {
    pub total_requests: u64,
    pub total_cost: f64,
    pub total_tokens: u64,
    pub cache_hits: u64,
    pub avg_response_time: Duration,
    pub unique_users: u64,
    pub unique_models: u64,
}

impl Default for GlobalStats {
    fn default() -> Self {
        Self {
            total_requests: 0,
            total_cost: 0.0,
            total_tokens: 0,
            cache_hits: 0,
            avg_response_time: Duration::from_millis(0),
            unique_users: 0,
            unique_models: 0,
        }
    }
}

impl GlobalStats {
    pub fn record_request(&mut self, duration: Duration, usage: Option<&Usage>) {
        self.total_requests += 1;

        // Update average response time
        let total_time = self.avg_response_time.as_millis() as u64 * (self.total_requests - 1) + duration.as_millis() as u64;
        self.avg_response_time = Duration::from_millis(total_time / self.total_requests);

        if let Some(usage) = usage {
            self.total_tokens += usage.total_tokens as u64;
            if let Some(cost) = usage.cost {
                self.total_cost += cost;
            }
        }
    }
}

impl Default for LLMAnalytics {
    fn default() -> Self {
        Self::new()
    }
} 