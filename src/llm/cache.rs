use lru::LruCache;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info};
use serde::{Serialize, Deserialize};

use super::models::CompletionResponse;

/// Prompt cache for storing and retrieving completion responses
pub struct PromptCache {
    cache: Arc<RwLock<LruCache<String, CacheEntry>>>,
    ttl: Duration,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    response: CompletionResponse,
    created_at: Instant,
}

impl PromptCache {
    pub fn new(ttl_seconds: u64) -> Self {
        let cache_size = NonZeroUsize::new(1000).unwrap(); // Cache up to 1000 entries
        
        Self {
            cache: Arc::new(RwLock::new(LruCache::new(cache_size))),
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    pub async fn get(&self, key: &str) -> Option<CompletionResponse> {
        let mut cache = self.cache.write().await;
        
        if let Some(entry) = cache.get(key) {
            // Check if entry is still valid
            if entry.created_at.elapsed() < self.ttl {
                debug!("Cache hit for key: {}", key);
                return Some(entry.response.clone());
            } else {
                // Entry expired, remove it
                cache.pop(key);
                debug!("Cache entry expired for key: {}", key);
            }
        }
        
        debug!("Cache miss for key: {}", key);
        None
    }

    pub async fn put(&self, key: String, response: CompletionResponse) {
        let mut cache = self.cache.write().await;
        
        let entry = CacheEntry {
            response,
            created_at: Instant::now(),
        };
        
        cache.put(key.clone(), entry);
        debug!("Cached response for key: {}", key);
    }

    pub async fn invalidate(&self, key: &str) {
        let mut cache = self.cache.write().await;
        cache.pop(key);
        debug!("Invalidated cache entry for key: {}", key);
    }

    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        info!("Cleared all cache entries");
    }

    pub async fn get_stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        
        CacheStats {
            size: cache.len(),
            capacity: cache.cap().get(),
            hit_rate: 0.0, // Would need separate tracking for accurate hit rate
        }
    }

    /// Clean up expired entries
    pub async fn cleanup_expired(&self) {
        let mut cache = self.cache.write().await;
        let now = Instant::now();
        
        let expired_keys: Vec<String> = cache.iter()
            .filter_map(|(key, entry)| {
                if now.duration_since(entry.created_at) > self.ttl {
                    Some(key.clone())
                } else {
                    None
                }
            })
            .collect();

        let expired_count = expired_keys.len();
        for key in &expired_keys {
            cache.pop(key);
        }

        if expired_count > 0 {
            debug!("Cleaned up {} expired cache entries", expired_count);
        }
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub size: usize,
    pub capacity: usize,
    pub hit_rate: f64,
}

/// Advanced caching with semantic similarity
pub struct SemanticCache {
    cache: Arc<RwLock<HashMap<String, Vec<SemanticCacheEntry>>>>,
    similarity_threshold: f64,
    ttl: Duration,
}

#[derive(Debug, Clone)]
struct SemanticCacheEntry {
    prompt_hash: String,
    response: CompletionResponse,
    embedding: Vec<f32>, // Would be computed using an embedding model
    created_at: Instant,
}

impl SemanticCache {
    pub fn new(ttl_seconds: u64, similarity_threshold: f64) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            similarity_threshold,
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    pub async fn get_similar(&self, prompt: &str, _embedding: Vec<f32>) -> Option<CompletionResponse> {
        let cache = self.cache.read().await;
        let prompt_key = self.get_prompt_category(prompt);
        
        if let Some(entries) = cache.get(&prompt_key) {
            for entry in entries {
                if entry.created_at.elapsed() < self.ttl {
                    // In a real implementation, we would compute cosine similarity
                    // between the query embedding and cached embeddings
                    // For now, we'll use a simple string similarity
                    if self.simple_similarity(prompt, &entry.prompt_hash) > self.similarity_threshold {
                        debug!("Semantic cache hit for prompt category: {}", prompt_key);
                        return Some(entry.response.clone());
                    }
                }
            }
        }
        
        None
    }

    fn get_prompt_category(&self, prompt: &str) -> String {
        // Simple categorization based on keywords
        let prompt_lower = prompt.to_lowercase();
        
        if prompt_lower.contains("summarize") || prompt_lower.contains("summary") {
            "summarization".to_string()
        } else if prompt_lower.contains("analyze") || prompt_lower.contains("analysis") {
            "analysis".to_string()
        } else if prompt_lower.contains("extract") || prompt_lower.contains("extraction") {
            "extraction".to_string()
        } else if prompt_lower.contains("translate") || prompt_lower.contains("translation") {
            "translation".to_string()
        } else {
            "general".to_string()
        }
    }

    fn simple_similarity(&self, a: &str, b: &str) -> f64 {
        // Simple Jaccard similarity
        let set_a: std::collections::HashSet<&str> = a.split_whitespace().collect();
        let set_b: std::collections::HashSet<&str> = b.split_whitespace().collect();
        
        let intersection = set_a.intersection(&set_b).count();
        let union = set_a.union(&set_b).count();
        
        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub ttl_seconds: u64,
    pub semantic_enabled: bool,
    pub similarity_threshold: f64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            ttl_seconds: 300, // 5 minutes
            semantic_enabled: false, // Disabled by default as it requires embedding models
            similarity_threshold: 0.8,
        }
    }
} 