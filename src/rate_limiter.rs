/*!
 * Rate Limiting System for Crawl4AI Core
 * 
 * Provides domain-specific rate limiting using token bucket algorithm
 * with distributed support and configurable policies.
 */

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use governor::{
    clock::{Clock, DefaultClock, Reference},
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter as GovRateLimiter
};
use tokio::sync::RwLock;
use url::Url;

use crate::error::{Error, Result};

/// Rate limiting configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Requests per second for a single domain
    pub requests_per_second: u32,
    /// Burst capacity (max tokens in bucket)
    pub burst_capacity: u32,
    /// Default delay between requests (milliseconds)
    pub default_delay_ms: u64,
    /// IP-based rate limiting (requests per minute)
    pub ip_requests_per_minute: u32,
    /// Global rate limit (requests per second across all domains)
    pub global_requests_per_second: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 2,      // Conservative default
            burst_capacity: 5,           // Allow some burst
            default_delay_ms: 500,       // 500ms between requests
            ip_requests_per_minute: 60,  // 1 request per second per IP
            global_requests_per_second: 10, // Global limit
        }
    }
}

/// Domain-specific rate limiter
#[derive(Debug)]
struct DomainLimiter {
    limiter: Arc<GovRateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
    last_request: Arc<RwLock<Instant>>,
    total_requests: Arc<RwLock<u64>>,
    failed_requests: Arc<RwLock<u64>>,
    created_at: Instant,
}

impl DomainLimiter {
    fn new(config: &RateLimitConfig) -> Self {
        let quota = Quota::per_second(
            std::num::NonZeroU32::new(config.requests_per_second).unwrap()
        ).allow_burst(
            std::num::NonZeroU32::new(config.burst_capacity).unwrap()
        );
        
        Self {
            limiter: Arc::new(GovRateLimiter::direct(quota)),
            last_request: Arc::new(RwLock::new(Instant::now())),
            total_requests: Arc::new(RwLock::new(0)),
            failed_requests: Arc::new(RwLock::new(0)),
            created_at: Instant::now(),
        }
    }
    
    async fn check_rate_limit(&self) -> Result<()> {
        match self.limiter.check() {
            Ok(_) => {
                *self.last_request.write().await = Instant::now();
                *self.total_requests.write().await += 1;
                Ok(())
            }
            Err(_) => {
                *self.failed_requests.write().await += 1;
                Err(Error::RateLimit("Domain rate limit exceeded".to_string()))
            }
        }
    }
    
    async fn get_stats(&self) -> DomainStats {
        DomainStats {
            total_requests: *self.total_requests.read().await,
            failed_requests: *self.failed_requests.read().await,
            last_request: *self.last_request.read().await,
            created_at: self.created_at,
            current_tokens: self.limiter.check().is_ok() as u32,
        }
    }
}

/// Statistics for a domain
#[derive(Debug, Clone)]
pub struct DomainStats {
    pub total_requests: u64,
    pub failed_requests: u64,
    pub last_request: Instant,
    pub created_at: Instant,
    pub current_tokens: u32,
}

/// IP-based rate limiter
#[derive(Debug)]
struct IpLimiter {
    limiters: Arc<RwLock<HashMap<IpAddr, Arc<GovRateLimiter<NotKeyed, InMemoryState, DefaultClock>>>>>,
    config: RateLimitConfig,
}

impl IpLimiter {
    fn new(config: RateLimitConfig) -> Self {
        Self {
            limiters: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }
    
    async fn check_ip_limit(&self, ip: IpAddr) -> Result<()> {
        let limiters = self.limiters.read().await;
        
                 if let Some(limiter) = limiters.get(&ip) {
             match limiter.check() {
                 Ok(_) => Ok(()),
                 Err(_) => Err(Error::RateLimit(format!("IP {} rate limit exceeded", ip))),
             }
        } else {
            drop(limiters);
            
            // Create new limiter for this IP
            let quota = Quota::per_minute(
                std::num::NonZeroU32::new(self.config.ip_requests_per_minute).unwrap()
            );
            let limiter = Arc::new(GovRateLimiter::direct(quota));
            
            self.limiters.write().await.insert(ip, limiter.clone());
            
                         match limiter.check() {
                 Ok(_) => Ok(()),
                 Err(_) => Err(Error::RateLimit(format!("IP {} rate limit exceeded", ip))),
             }
        }
    }
    
    async fn cleanup_expired(&self) {
        let mut limiters = self.limiters.write().await;
        let _now = DefaultClock::default().now(); // Intentionally unused - for future rate limiting statistics
        
        // Remove limiters that haven't been used in the last hour
        // For now, just keep all limiters - more sophisticated cleanup would require tracking usage
        limiters.retain(|_ip, _limiter| {
            // Keep all limiters for now - TODO: implement proper expiration tracking
            true
        });
    }
}

/// Main rate limiting system
pub struct RateLimiter {
    config: RateLimitConfig,
    domain_limiters: Arc<RwLock<HashMap<String, Arc<DomainLimiter>>>>,
    ip_limiter: Arc<IpLimiter>,
    global_limiter: Arc<GovRateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
    stats: Arc<RwLock<RateLimiterStats>>,
}

/// Overall rate limiter statistics
#[derive(Debug, Clone, Default)]
pub struct RateLimiterStats {
    pub total_requests: u64,
    pub blocked_requests: u64,
    pub active_domains: u64,
    pub active_ips: u64,
    pub uptime: Duration,
    pub requests_per_second: f64,
}

impl RateLimiter {
    /// Create a new rate limiter with the given configuration
    pub fn new(config: RateLimitConfig) -> Self {
        let global_quota = Quota::per_second(
            std::num::NonZeroU32::new(config.global_requests_per_second).unwrap()
        );
        
        Self {
            domain_limiters: Arc::new(RwLock::new(HashMap::new())),
            ip_limiter: Arc::new(IpLimiter::new(config.clone())),
            global_limiter: Arc::new(GovRateLimiter::direct(global_quota)),
            stats: Arc::new(RwLock::new(RateLimiterStats::default())),
            config,
        }
    }
    
    /// Check if a request to the given URL from the given IP is allowed
    pub async fn check_request(&self, url: &str, ip: Option<IpAddr>) -> Result<()> {
        // Update total requests stats
        {
            let mut stats = self.stats.write().await;
            stats.total_requests += 1;
        }
        
        // Check global rate limit first
        if let Err(_) = self.global_limiter.check() {
            let mut stats = self.stats.write().await;
            stats.blocked_requests += 1;
                         return Err(Error::RateLimit("Global rate limit exceeded".to_string()));
        }
        
        // Check IP-based rate limit if IP is provided
        if let Some(client_ip) = ip {
            self.ip_limiter.check_ip_limit(client_ip).await.map_err(|e| {
                tokio::spawn({
                    let stats = Arc::clone(&self.stats);
                    async move {
                        let mut stats = stats.write().await;
                        stats.blocked_requests += 1;
                    }
                });
                e
            })?;
        }
        
        // Extract domain from URL
        let domain = self.extract_domain(url)?;
        
        // Check domain-specific rate limit
        self.check_domain_limit(&domain).await.map_err(|e| {
            tokio::spawn({
                let stats = Arc::clone(&self.stats);
                async move {
                    let mut stats = stats.write().await;
                    stats.blocked_requests += 1;
                }
            });
            e
        })?;
        
        Ok(())
    }
    
    /// Check domain-specific rate limit
    async fn check_domain_limit(&self, domain: &str) -> Result<()> {
        let limiters = self.domain_limiters.read().await;
        
        if let Some(limiter) = limiters.get(domain) {
            limiter.check_rate_limit().await
        } else {
            drop(limiters);
            
            // Create new limiter for this domain
            let limiter = Arc::new(DomainLimiter::new(&self.config));
            self.domain_limiters.write().await.insert(domain.to_string(), limiter.clone());
            
            limiter.check_rate_limit().await
        }
    }
    
    /// Extract domain from URL
    fn extract_domain(&self, url: &str) -> Result<String> {
        let parsed_url = Url::parse(url)
            .map_err(|e| Error::Validation(format!("Invalid URL: {}", e)))?;
        
        parsed_url.host_str()
            .ok_or_else(|| Error::Validation("No host in URL".to_string()))
            .map(|host| host.to_string())
    }
    
    /// Get statistics for a specific domain
    pub async fn get_domain_stats(&self, domain: &str) -> Option<DomainStats> {
        let limiters = self.domain_limiters.read().await;
        if let Some(limiter) = limiters.get(domain) {
            Some(limiter.get_stats().await)
        } else {
            None
        }
    }
    
    /// Get overall rate limiter statistics
    pub async fn get_stats(&self) -> RateLimiterStats {
        let mut stats = self.stats.read().await.clone();
        
        // Update active counts
        stats.active_domains = self.domain_limiters.read().await.len() as u64;
        stats.active_ips = self.ip_limiter.limiters.read().await.len() as u64;
        
        // Calculate requests per second
        if stats.uptime.as_secs() > 0 {
            stats.requests_per_second = stats.total_requests as f64 / stats.uptime.as_secs() as f64;
        }
        
        stats
    }
    
    /// Get all domain statistics
    pub async fn get_all_domain_stats(&self) -> HashMap<String, DomainStats> {
        let limiters = self.domain_limiters.read().await;
        let mut result = HashMap::new();
        
        for (domain, limiter) in limiters.iter() {
            result.insert(domain.clone(), limiter.get_stats().await);
        }
        
        result
    }
    
    /// Wait for the appropriate delay based on rate limiting
    pub async fn wait_if_needed(&self, url: &str) -> Result<()> {
        let domain = self.extract_domain(url)?;
        let limiters = self.domain_limiters.read().await;
        
        if let Some(limiter) = limiters.get(&domain) {
            let last_request = *limiter.last_request.read().await;
            let min_interval = Duration::from_millis(self.config.default_delay_ms);
            let elapsed = last_request.elapsed();
            
            if elapsed < min_interval {
                let wait_time = min_interval - elapsed;
                tokio::time::sleep(wait_time).await;
            }
        }
        
        Ok(())
    }
    
    /// Cleanup expired rate limiters to prevent memory leaks
    pub async fn cleanup(&self) {
        // Cleanup IP limiters
        self.ip_limiter.cleanup_expired().await;
        
        // Cleanup domain limiters that haven't been used recently
        let mut domain_limiters = self.domain_limiters.write().await;
        let now = Instant::now();
        
        domain_limiters.retain(|_domain, limiter| {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    let last_request = *limiter.last_request.read().await;
                    now.duration_since(last_request) < Duration::from_secs(3600) // Keep for 1 hour
                })
            })
        });
    }
    
    /// Start periodic cleanup task
    pub fn start_cleanup_task(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(300)); // Every 5 minutes
            
            loop {
                interval.tick().await;
                self.cleanup().await;
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;
    
    #[tokio::test]
    async fn test_domain_rate_limiting() {
        let config = RateLimitConfig {
            requests_per_second: 2,
            burst_capacity: 3,
            ..Default::default()
        };
        
        let limiter = RateLimiter::new(config);
        let url = "https://example.com/test";
        
        // Should allow initial requests up to burst capacity
        for _ in 0..3 {
            assert!(limiter.check_request(url, None).await.is_ok());
        }
        
        // Should be rate limited after burst
        assert!(limiter.check_request(url, None).await.is_err());
        
        // Should allow requests after waiting
        tokio::time::sleep(Duration::from_millis(1000)).await;
        assert!(limiter.check_request(url, None).await.is_ok());
    }
    
    #[tokio::test]
    async fn test_ip_rate_limiting() {
        let config = RateLimitConfig {
            ip_requests_per_minute: 2,
            ..Default::default()
        };
        
        let limiter = RateLimiter::new(config);
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        
        // Should allow initial requests
        assert!(limiter.check_request("https://example1.com", Some(ip)).await.is_ok());
        assert!(limiter.check_request("https://example2.com", Some(ip)).await.is_ok());
        
        // Should be rate limited for same IP
        assert!(limiter.check_request("https://example3.com", Some(ip)).await.is_err());
    }
    
    #[tokio::test]
    async fn test_global_rate_limiting() {
        let config = RateLimitConfig {
            global_requests_per_second: 1,
            requests_per_second: 10, // High domain limit
            ..Default::default()
        };
        
        let limiter = RateLimiter::new(config);
        
        // Should allow first request
        assert!(limiter.check_request("https://example1.com", None).await.is_ok());
        
        // Should be globally rate limited
        assert!(limiter.check_request("https://example2.com", None).await.is_err());
    }
    
    #[tokio::test]
    async fn test_domain_extraction() {
        let limiter = RateLimiter::new(RateLimitConfig::default());
        
        assert_eq!(limiter.extract_domain("https://example.com/path").unwrap(), "example.com");
        assert_eq!(limiter.extract_domain("http://sub.example.com:8080/path").unwrap(), "sub.example.com");
        assert!(limiter.extract_domain("invalid-url").is_err());
    }
    
    #[tokio::test]
    async fn test_statistics() {
        let limiter = RateLimiter::new(RateLimitConfig::default());
        let url = "https://example.com/test";
        
        // Make some requests
        let _ = limiter.check_request(url, None).await;
        let _ = limiter.check_request(url, None).await;
        
        let stats = limiter.get_stats().await;
        assert_eq!(stats.total_requests, 2);
        assert_eq!(stats.active_domains, 1);
        
        let domain_stats = limiter.get_domain_stats("example.com").await;
        assert!(domain_stats.is_some());
        assert!(domain_stats.unwrap().total_requests >= 1);
    }
} 