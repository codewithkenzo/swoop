use anyhow::Result;
use governor::clock::{Clock, QuantaClock};
use governor::{DefaultDirectRateLimiter, Quota};
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

pub struct DistributedRateLimiter {
    // Per-domain rate limiters
    domain_limiters: Arc<RwLock<HashMap<String, DefaultDirectRateLimiter>>>,
    // Global rate limiter
    global_limiter: DefaultDirectRateLimiter,
    // Configuration
    requests_per_domain: NonZeroU32,
    requests_per_second_global: NonZeroU32,
}

impl DistributedRateLimiter {
    pub fn new(requests_per_domain: u32, requests_per_second_global: u32) -> Result<Self> {
        Ok(Self {
            domain_limiters: Arc::new(RwLock::new(HashMap::new())),
            global_limiter: DefaultDirectRateLimiter::direct(Quota::per_second(
                NonZeroU32::new(requests_per_second_global)
                    .ok_or_else(|| anyhow::anyhow!("Global rate limit must be > 0"))?,
            )),
            requests_per_domain: NonZeroU32::new(requests_per_domain)
                .ok_or_else(|| anyhow::anyhow!("Domain rate limit must be > 0"))?,
            requests_per_second_global: NonZeroU32::new(requests_per_second_global)
                .ok_or_else(|| anyhow::anyhow!("Global rate limit must be > 0"))?,
        })
    }

    pub async fn check_rate_limit(&self, domain: &str) -> Result<()> {
        // Check global rate limit first
        self.global_limiter.until_ready().await;

        // Check domain-specific rate limit
        {
            let mut limiters = self.domain_limiters.write().await;
            let limiter = limiters.entry(domain.to_string()).or_insert_with(|| {
                DefaultDirectRateLimiter::direct(Quota::per_second(self.requests_per_domain))
            });
            limiter.until_ready().await;
        }

        Ok(())
    }

    /// Get the current rate limit status for a domain
    pub async fn get_domain_status(&self, domain: &str) -> Option<Duration> {
        let limiters = self.domain_limiters.read().await;
        if let Some(limiter) = limiters.get(domain) {
            // Check if rate limited and return wait time
            match limiter.check() {
                Ok(_) => None, // Not rate limited
                Err(negative) => Some(negative.wait_time_from(QuantaClock::default().now())),
            }
        } else {
            None
        }
    }

    /// Clear rate limits for a specific domain
    pub async fn reset_domain(&self, domain: &str) {
        let mut limiters = self.domain_limiters.write().await;
        limiters.remove(domain);
    }

    /// Get statistics about current rate limiting
    pub async fn get_stats(&self) -> RateLimiterStats {
        let limiters = self.domain_limiters.read().await;
        RateLimiterStats {
            total_domains: limiters.len(),
            global_rate_limit: self.requests_per_second_global.get(),
            domain_rate_limit: self.requests_per_domain.get(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RateLimiterStats {
    pub total_domains: usize,
    pub global_rate_limit: u32,
    pub domain_rate_limit: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_rate_limiter_creation() {
        let limiter = DistributedRateLimiter::new(5, 10).unwrap();
        let stats = limiter.get_stats().await;
        assert_eq!(stats.global_rate_limit, 10);
        assert_eq!(stats.domain_rate_limit, 5);
        assert_eq!(stats.total_domains, 0);
    }

    #[tokio::test]
    async fn test_domain_rate_limiting() {
        let limiter = DistributedRateLimiter::new(2, 10).unwrap();

        // First request should succeed immediately
        let start = std::time::Instant::now();
        limiter.check_rate_limit("example.com").await.unwrap();
        assert!(start.elapsed() < Duration::from_millis(100));

        // Second request should succeed immediately
        let start = std::time::Instant::now();
        limiter.check_rate_limit("example.com").await.unwrap();
        assert!(start.elapsed() < Duration::from_millis(100));

        // Third request should be rate limited (wait for next second)
        let start = std::time::Instant::now();
        limiter.check_rate_limit("example.com").await.unwrap();
        assert!(start.elapsed() >= Duration::from_millis(400)); // Should wait ~500ms
    }

    #[tokio::test]
    async fn test_different_domains() {
        let limiter = DistributedRateLimiter::new(1, 10).unwrap();

        // Requests to different domains should not interfere
        let start = std::time::Instant::now();
        limiter.check_rate_limit("example.com").await.unwrap();
        limiter.check_rate_limit("another.com").await.unwrap();
        assert!(start.elapsed() < Duration::from_millis(100));
    }

    #[tokio::test]
    async fn test_reset_domain() {
        let limiter = DistributedRateLimiter::new(1, 10).unwrap();

        // Use up the rate limit
        limiter.check_rate_limit("example.com").await.unwrap();

        // Reset the domain
        limiter.reset_domain("example.com").await;

        // Should be able to make request immediately
        let start = std::time::Instant::now();
        limiter.check_rate_limit("example.com").await.unwrap();
        assert!(start.elapsed() < Duration::from_millis(100));
    }
}
