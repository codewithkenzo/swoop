use anyhow::{anyhow, Result};
use dashmap::DashMap;
use futures_util::StreamExt;
use lru::LruCache;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};
use tokio_stream::wrappers::ReceiverStream;
use tracing::{debug, error, info, warn};

pub mod models;
pub mod routing;
pub mod streaming;
pub mod cache;
pub mod analytics;

use models::*;
use routing::*;
use streaming::*;
use cache::*;
use analytics::*;

/// Main LLM service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub openrouter_api_key: String,
    pub default_model: String,
    pub cache_config: CacheConfig,
    pub rate_limits: RateLimitConfig,
    pub user_tiers: HashMap<String, UserTier>,
    pub analytics_enabled: bool,
    pub streaming_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub requests_per_day: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            requests_per_hour: 1000,
            requests_per_day: 10000,
        }
    }
}

impl Default for LLMConfig {
    fn default() -> Self {
        let mut user_tiers = HashMap::new();
        
        // Default free tier
        user_tiers.insert("free".to_string(), UserTier::Free {
            daily_limit: 100,
            models: vec![
                "openai/gpt-3.5-turbo".to_string(),
                "meta-llama/llama-3.2-3b-instruct:free".to_string(),
            ],
        });

        // Default basic tier
        user_tiers.insert("basic".to_string(), UserTier::Basic {
            daily_limit: 1000,
            cost_limit: 10.0,
            models: vec![
                "openai/gpt-3.5-turbo".to_string(),
                "openai/gpt-4o-mini".to_string(),
                "anthropic/claude-3-haiku".to_string(),
            ],
        });

        Self {
            openrouter_api_key: std::env::var("OPENROUTER_API_KEY").unwrap_or_default(),
            default_model: "openai/gpt-3.5-turbo".to_string(),
            cache_config: CacheConfig::default(),
            rate_limits: RateLimitConfig::default(),
            user_tiers,
            analytics_enabled: true,
            streaming_enabled: true,
        }
    }
}

/// Main LLM service that orchestrates all components
#[derive(Clone)]
pub struct LLMService {
    pub client: Client,
    pub config: LLMConfig,
    pub model_registry: Arc<RwLock<ModelRegistry>>,
    pub router: ModelRouter,
    pub cache: Arc<PromptCache>,
    pub analytics: Arc<LLMAnalytics>,
    pub streaming: Arc<StreamingService>,
    pub rate_limiters: Arc<DashMap<String, Arc<Mutex<RateLimiter>>>>,
}

impl LLMService {
    pub async fn new(config: LLMConfig) -> Result<Self> {
        let client = Client::new();
        let model_registry = Arc::new(RwLock::new(ModelRegistry::new()));
        let router = ModelRouter::new();
        let cache = Arc::new(PromptCache::new(config.cache_config.ttl_seconds));
        let analytics = Arc::new(LLMAnalytics::new());
        let streaming = Arc::new(StreamingService::new(config.openrouter_api_key.clone()));
        let rate_limiters = Arc::new(DashMap::new());

        let service = Self {
            client,
            config,
            model_registry,
            router,
            cache,
            analytics,
            streaming,
            rate_limiters,
        };

        // Initialize model registry
        service.refresh_models().await?;

        info!("LLM Service initialized successfully");
        Ok(service)
    }

    /// Refresh available models from OpenRouter
    pub async fn refresh_models(&self) -> Result<()> {
        let response = self.client
            .get("https://openrouter.ai/api/v1/models")
            .header("Authorization", format!("Bearer {}", self.config.openrouter_api_key))
            .send()
            .await?;

        let models_response: ModelsResponse = response.json().await?;
        let mut registry = self.model_registry.write().await;

        for model in models_response.data {
            registry.add_model(model);
        }

        info!("Refreshed {} models in registry", registry.models.len());
        Ok(())
    }

    /// Process a completion request
    pub async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let start_time = Instant::now();
        
        // Check rate limits
        if !self.check_rate_limit(&request.user_id).await? {
            return Err(anyhow!("Rate limit exceeded"));
        }

        // Get user tier
        let user_tier = self.get_user_tier(&request.user_id).await?;

        // Check cache if enabled
        if self.config.cache_config.enabled {
            let cache_key = self.generate_cache_key(&request);
            if let Some(cached_response) = self.cache.get(&cache_key).await {
                self.analytics.record_cache_hit(&request.user_id).await;
                debug!("Cache hit for user: {}", request.user_id);
                return Ok(cached_response);
            }
        }

        // Select appropriate model
        let registry = self.model_registry.read().await;
        let selected_model = self.router.select_model(&request, &user_tier, &registry).await?;
        drop(registry);

        // Create OpenRouter request
        let openrouter_request = OpenRouterRequest {
            model: selected_model.id.clone(),
            messages: request.messages.clone(),
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
            frequency_penalty: request.frequency_penalty,
            presence_penalty: request.presence_penalty,
            stream: Some(false), // Non-streaming for regular completions
        };

        // Make API call
        let response = self.client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.config.openrouter_api_key))
            .header("Content-Type", "application/json")
            .json(&openrouter_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("OpenRouter API error: {}", error_text);
            return Err(anyhow!("OpenRouter API error: {}", error_text));
        }

        let completion_response: CompletionResponse = response.json().await?;
        let duration = start_time.elapsed();

        // Record analytics
        if self.config.analytics_enabled {
            self.analytics.record_completion(
                &request.user_id,
                &selected_model.id,
                duration,
                completion_response.usage.as_ref(),
            ).await;
        }

        // Cache response if enabled
        if self.config.cache_config.enabled {
            let cache_key = self.generate_cache_key(&request);
            self.cache.put(cache_key, completion_response.clone()).await;
        }

        info!("Completed request for user: {} in {:?}", request.user_id, duration);
        Ok(completion_response)
    }

    /// Stream a completion request
    pub async fn stream_complete(&self, request: CompletionRequest) -> Result<impl futures_util::Stream<Item = Result<axum::response::sse::Event, std::convert::Infallible>>> {
        // Check rate limits
        if !self.check_rate_limit(&request.user_id).await? {
            return Err(anyhow!("Rate limit exceeded"));
        }

        // Get user tier and select model
        let user_tier = self.get_user_tier(&request.user_id).await?;
        let registry = self.model_registry.read().await;
        let selected_model = self.router.select_model(&request, &user_tier, &registry).await?;
        drop(registry);

        // Create OpenRouter request for streaming
        let openrouter_request = OpenRouterRequest {
            model: selected_model.id.clone(),
            messages: request.messages.clone(),
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
            frequency_penalty: request.frequency_penalty,
            presence_penalty: request.presence_penalty,
            stream: Some(true),
        };

        self.streaming.stream_completion(openrouter_request).await
    }

    /// Get user tier configuration
    async fn get_user_tier(&self, user_id: &str) -> Result<UserTier> {
        // In a real implementation, this would query a database
        // For now, we'll use a simple mapping based on user ID patterns
        let tier = if user_id.starts_with("enterprise_") {
            UserTier::Enterprise {
                cost_limit: 1000.0,
                models: vec![
                    "openai/gpt-4".to_string(),
                    "anthropic/claude-3-opus".to_string(),
                    "openai/gpt-4o".to_string(),
                ],
                dedicated_resources: true,
                custom_models: vec![],
            }
        } else if user_id.starts_with("premium_") {
            UserTier::Premium {
                daily_limit: 5000,
                cost_limit: 100.0,
                models: vec![
                    "openai/gpt-4o-mini".to_string(),
                    "anthropic/claude-3-sonnet".to_string(),
                    "openai/gpt-3.5-turbo".to_string(),
                ],
                priority_access: true,
            }
        } else if user_id.starts_with("basic_") {
            self.config.user_tiers.get("basic").cloned().unwrap_or_default()
        } else {
            self.config.user_tiers.get("free").cloned().unwrap_or_default()
        };

        Ok(tier)
    }

    /// Check rate limits for a user
    async fn check_rate_limit(&self, user_id: &str) -> Result<bool> {
        let rate_limiter = self.rate_limiters
            .entry(user_id.to_string())
            .or_insert_with(|| Arc::new(Mutex::new(RateLimiter::new(self.config.rate_limits.clone()))));

        let mut limiter = rate_limiter.lock().await;
        Ok(limiter.check_limit())
    }

    /// Generate cache key for a request
    fn generate_cache_key(&self, request: &CompletionRequest) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        request.messages.hash(&mut hasher);
        request.max_tokens.hash(&mut hasher);
        request.temperature.map(|t| (t * 1000.0) as u32).hash(&mut hasher);
        request.task_category.hash(&mut hasher);

        format!("completion_{:x}", hasher.finish())
    }
}

impl Default for UserTier {
    fn default() -> Self {
        UserTier::Free {
            daily_limit: 100,
            models: vec!["openai/gpt-3.5-turbo".to_string()],
        }
    }
}

/// Rate limiter implementation
struct RateLimiter {
    config: RateLimitConfig,
    request_times: Vec<Instant>,
}

impl RateLimiter {
    fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            request_times: Vec::new(),
        }
    }

    fn check_limit(&mut self) -> bool {
        let now = Instant::now();
        
        // Clean old requests
        self.request_times.retain(|&time| now.duration_since(time) < Duration::from_secs(60));
        
        // Check minute limit
        if self.request_times.len() >= self.config.requests_per_minute as usize {
            return false;
        }
        
        // Add current request
        self.request_times.push(now);
        true
    }
} 