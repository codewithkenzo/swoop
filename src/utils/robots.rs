/*!
 * Robots.txt parsing and caching module for Crawl4AI
 * 
 * This module provides functionality to fetch, parse, and cache robots.txt files
 * to ensure compliance with website crawling policies.
 */

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

// use lazy_static::lazy_static; // For future caching implementation
use log::{debug, info, warn};
use regex::Regex;
use reqwest::Client;
use tokio::sync::RwLock;
use url::Url;

// use crate::error::{Error, Result}; // For future error handling

/// Represents a parsed robots.txt rule
#[derive(Debug, Clone)]
pub struct RobotRule {
    /// User agent this rule applies to
    pub user_agent: String,
    /// Whether this is an allow or disallow rule
    pub allow: bool,
    /// Path pattern for the rule
    pub path: String,
    /// Compiled regex for pattern matching
    pattern: Option<Regex>,
}

impl RobotRule {
    /// Create a new robot rule
    pub fn new(user_agent: String, allow: bool, path: String) -> Self {
        let pattern = Self::compile_pattern(&path);
        Self {
            user_agent,
            allow,
            path,
            pattern,
        }
    }
    
    /// Compile a robots.txt path pattern into a regex
    fn compile_pattern(path: &str) -> Option<Regex> {
        // Convert robots.txt wildcards to regex
        let mut pattern = regex::escape(path);
        pattern = pattern.replace(r"\*", ".*");
        pattern = format!("^{}", pattern);
        
        Regex::new(&pattern).ok()
    }
    
    /// Check if a path matches this rule
    pub fn matches(&self, path: &str) -> bool {
        if let Some(ref pattern) = self.pattern {
            pattern.is_match(path)
        } else {
            // Fallback to simple string matching
            path.starts_with(&self.path)
        }
    }
}

/// Represents parsed robots.txt content for a domain
#[derive(Debug, Clone)]
pub struct RobotRules {
    /// Domain these rules apply to
    pub domain: String,
    /// List of rules
    pub rules: Vec<RobotRule>,
    /// Crawl delay in seconds
    pub crawl_delay: Option<u64>,
    /// Sitemap URLs
    pub sitemaps: Vec<String>,
    /// When these rules were fetched
    pub fetched_at: Instant,
}

impl RobotRules {
    /// Create new empty robot rules
    pub fn new(domain: String) -> Self {
        Self {
            domain,
            rules: Vec::new(),
            crawl_delay: None,
            sitemaps: Vec::new(),
            fetched_at: Instant::now(),
        }
    }
    
    /// Check if a path is allowed for a given user agent
    pub fn is_allowed(&self, path: &str, user_agent: &str) -> bool {
        let normalized_path = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{}", path)
        };
        
        let normalized_user_agent = user_agent.to_lowercase();
        
        // Find the most specific rule that matches
        let mut applicable_rules: Vec<&RobotRule> = self.rules
            .iter()
            .filter(|rule| {
                let rule_ua = rule.user_agent.to_lowercase();
                rule_ua == "*" || 
                rule_ua == normalized_user_agent ||
                normalized_user_agent.starts_with(&rule_ua)
            })
            .filter(|rule| rule.matches(&normalized_path))
            .collect();
        
        // Sort by specificity (longer paths first, then by allow/disallow)
        applicable_rules.sort_by(|a, b| {
            let len_cmp = b.path.len().cmp(&a.path.len());
            if len_cmp == std::cmp::Ordering::Equal {
                // Prefer disallow rules over allow rules for same path
                b.allow.cmp(&a.allow)
            } else {
                len_cmp
            }
        });
        
        // If we have matching rules, use the most specific one
        if let Some(rule) = applicable_rules.first() {
            rule.allow
        } else {
            // No matching rules means allowed by default
            true
        }
    }
    
    /// Get the crawl delay for a user agent
    pub fn get_crawl_delay(&self, _user_agent: &str) -> Option<Duration> {
        self.crawl_delay.map(Duration::from_secs)
    }
    
    /// Parse robots.txt content
    pub fn parse(domain: String, content: &str) -> Self {
        let mut rules = Self::new(domain);
        let mut current_user_agents = Vec::new();
        
        for line in content.lines() {
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Parse line
            if let Some(colon_pos) = line.find(':') {
                let key = line[..colon_pos].trim().to_lowercase();
                let value = line[colon_pos + 1..].trim();
                
                match key.as_str() {
                    "user-agent" => {
                        current_user_agents.clear();
                        current_user_agents.push(value.to_string());
                    }
                    "disallow" => {
                        for user_agent in &current_user_agents {
                            if !value.is_empty() {
                                rules.rules.push(RobotRule::new(
                                    user_agent.clone(),
                                    false,
                                    value.to_string(),
                                ));
                            }
                        }
                    }
                    "allow" => {
                        for user_agent in &current_user_agents {
                            if !value.is_empty() {
                                rules.rules.push(RobotRule::new(
                                    user_agent.clone(),
                                    true,
                                    value.to_string(),
                                ));
                            }
                        }
                    }
                    "crawl-delay" => {
                        if let Ok(delay) = value.parse::<u64>() {
                            rules.crawl_delay = Some(delay);
                        }
                    }
                    "sitemap" => {
                        rules.sitemaps.push(value.to_string());
                    }
                    _ => {
                        // Unknown directive, ignore
                    }
                }
            }
        }
        
        rules
    }
}

/// Cache for robots.txt rules
#[derive(Debug)]
pub struct RobotsCache {
    /// HTTP client for fetching robots.txt
    client: Client,
    /// Cache of parsed rules by domain
    cache: Arc<RwLock<HashMap<String, Arc<RobotRules>>>>,
    /// Cache TTL in seconds
    cache_ttl: Duration,
}

impl RobotsCache {
    /// Create a new RobotsCache
    pub fn new(client: Client) -> Self {
        Self {
            client,
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl: Duration::from_secs(3600), // 1 hour default TTL
        }
    }
    
    /// Create a new RobotsCache with custom TTL
    pub fn with_ttl(client: Client, ttl: Duration) -> Self {
        Self {
            client,
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl: ttl,
        }
    }
    
    /// Get robots.txt rules for a domain
    pub async fn get_rules(&self, domain: &str) -> Arc<RobotRules> {
        let domain_key = domain.to_lowercase();
        
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(rules) = cache.get(&domain_key) {
                // Check if rules are still fresh
                if rules.fetched_at.elapsed() < self.cache_ttl {
                    debug!("Using cached robots.txt for domain: {}", domain);
                    return rules.clone();
                }
            }
        }
        
        // Fetch and parse robots.txt
        info!("Fetching robots.txt for domain: {}", domain);
        let rules = self.fetch_and_parse_robots(domain).await;
        
        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(domain_key, rules.clone());
        }
        
        rules
    }
    
    /// Check if a URL is allowed to be crawled
    pub async fn is_allowed(&self, url: &str, user_agent: &str) -> bool {
        match Url::parse(url) {
            Ok(parsed_url) => {
                if let Some(domain) = parsed_url.host_str() {
                    let rules = self.get_rules(domain).await;
                    let path = parsed_url.path();
                    rules.is_allowed(path, user_agent)
                } else {
                    // Invalid domain, disallow
                    false
                }
            }
            Err(_) => {
                // Invalid URL, disallow
                false
            }
        }
    }
    
    /// Get crawl delay for a domain and user agent
    pub async fn get_crawl_delay(&self, domain: &str, user_agent: &str) -> Option<Duration> {
        let rules = self.get_rules(domain).await;
        rules.get_crawl_delay(user_agent)
    }
    
    /// Fetch and parse robots.txt for a domain
    async fn fetch_and_parse_robots(&self, domain: &str) -> Arc<RobotRules> {
        let robots_url = format!("https://{}/robots.txt", domain);
        
        match self.client.get(&robots_url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text().await {
                        Ok(content) => {
                            debug!("Successfully fetched robots.txt for {}", domain);
                            Arc::new(RobotRules::parse(domain.to_string(), &content))
                        }
                        Err(e) => {
                            warn!("Failed to read robots.txt content for {}: {}", domain, e);
                            Arc::new(RobotRules::new(domain.to_string()))
                        }
                    }
                } else {
                    info!("No robots.txt found for {} (status: {})", domain, response.status());
                    Arc::new(RobotRules::new(domain.to_string()))
                }
            }
            Err(e) => {
                warn!("Failed to fetch robots.txt for {}: {}", domain, e);
                Arc::new(RobotRules::new(domain.to_string()))
            }
        }
    }
    
    /// Clear the cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        info!("Robots.txt cache cleared");
    }
    
    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> (usize, usize) {
        let cache = self.cache.read().await;
        let total_entries = cache.len();
        let fresh_entries = cache
            .values()
            .filter(|rules| rules.fetched_at.elapsed() < self.cache_ttl)
            .count();
        
        (total_entries, fresh_entries)
    }
}

impl Default for RobotsCache {
    fn default() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Crawl4AI/1.0 (+https://quantumscribe.ai/bot)")
            .build()
            .unwrap_or_else(|_| Client::new());
        
        Self::new(client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_robot_rule_matching() {
        let rule = RobotRule::new("*".to_string(), false, "/admin".to_string());
        assert!(rule.matches("/admin"));
        assert!(rule.matches("/admin/"));
        assert!(rule.matches("/admin/users"));
        assert!(!rule.matches("/public"));
        
        let wildcard_rule = RobotRule::new("*".to_string(), false, "/temp*".to_string());
        assert!(wildcard_rule.matches("/temp"));
        assert!(wildcard_rule.matches("/temporary"));
        assert!(!wildcard_rule.matches("/public"));
    }
    
    #[test]
    fn test_robots_parsing() {
        let content = r#"
            User-agent: *
            Disallow: /admin
            Disallow: /private
            Allow: /public
            Crawl-delay: 1
            Sitemap: https://example.com/sitemap.xml
            
            User-agent: Googlebot
            Disallow: /temp
        "#;
        
        let rules = RobotRules::parse("example.com".to_string(), content);
        
        assert_eq!(rules.rules.len(), 4);
        assert_eq!(rules.crawl_delay, Some(1));
        assert_eq!(rules.sitemaps.len(), 1);
        
        // Test specific rules
        assert!(!rules.is_allowed("/admin", "*"));
        assert!(rules.is_allowed("/public", "*"));
        assert!(!rules.is_allowed("/temp", "Googlebot"));
        assert!(rules.is_allowed("/temp", "Mozilla"));
    }
    
    #[tokio::test]
    async fn test_robots_cache() {
        let client = Client::new();
        let cache = RobotsCache::new(client);
        
        // Test cache stats
        let (total, fresh) = cache.get_cache_stats().await;
        assert_eq!(total, 0);
        assert_eq!(fresh, 0);
        
        // Test cache clearing
        cache.clear_cache().await;
        let (total, fresh) = cache.get_cache_stats().await;
        assert_eq!(total, 0);
    }
} 