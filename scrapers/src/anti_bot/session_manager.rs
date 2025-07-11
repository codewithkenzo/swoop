//! Session management for persistent anti-bot evasion
//! 
//! This module implements comprehensive session management:
//! - Cookie and session persistence across requests
//! - Browser state management and restoration
//! - Session isolation and security
//! - Multi-platform session coordination

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

/// Session manager for maintaining persistent state
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, BrowserSession>>>,
    cookie_store: Arc<RwLock<CookieStore>>,
    config: SessionConfig,
}

impl SessionManager {
    /// Create a new session manager
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            cookie_store: Arc::new(RwLock::new(CookieStore::new())),
            config: SessionConfig::default(),
        })
    }

    /// Create a new session manager with custom config
    pub async fn new_with_config(config: SessionConfig) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            cookie_store: Arc::new(RwLock::new(CookieStore::new())),
            config,
        })
    }

    /// Create or retrieve a session for a platform
    pub async fn get_session(&self, platform: &str) -> Result<BrowserSession, Box<dyn std::error::Error + Send + Sync>> {
        let session_key = format!("session_{}", platform);
        
        // Check if session exists and is valid
        {
            let sessions = self.sessions.read().await;
            if let Some(session) = sessions.get(&session_key) {
                if !session.is_expired() {
                    return Ok(session.clone());
                }
            }
        }

        // Create new session
        self.create_new_session(platform).await
    }

    /// Create a new browser session
    async fn create_new_session(&self, platform: &str) -> Result<BrowserSession, Box<dyn std::error::Error + Send + Sync>> {
        let session = BrowserSession {
            platform: platform.to_string(),
            session_id: Self::generate_session_id(),
            created_at: Instant::now(),
            last_activity: Instant::now(),
            cookies: Vec::new(),
            local_storage: HashMap::new(),
            session_storage: HashMap::new(),
            user_agent: Self::generate_session_user_agent(platform),
            viewport: Self::generate_session_viewport(),
            headers: Self::generate_session_headers(platform),
            request_count: 0,
            success_count: 0,
        };

        // Store session
        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(format!("session_{}", platform), session.clone());
        }

        Ok(session)
    }

    /// Update session with request result
    pub async fn update_session(&self, platform: &str, success: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let session_key = format!("session_{}", platform);
        
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_key) {
            session.request_count += 1;
            session.last_activity = Instant::now();
            
            if success {
                session.success_count += 1;
            }
        }

        Ok(())
    }

    /// Store cookies for a session
    pub async fn store_cookies(&self, platform: &str, cookies: Vec<Cookie>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut cookie_store = self.cookie_store.write().await;
        cookie_store.store_cookies(platform, cookies).await?;

        // Update session cookies
        let session_key = format!("session_{}", platform);
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_key) {
            session.cookies = cookie_store.get_cookies(platform).await;
        }

        Ok(())
    }

    /// Get cookies for a session
    pub async fn get_cookies(&self, platform: &str) -> Vec<Cookie> {
        let cookie_store = self.cookie_store.read().await;
        cookie_store.get_cookies(platform).await
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&self) -> u32 {
        let mut sessions = self.sessions.write().await;
        let initial_count = sessions.len();
        
        sessions.retain(|_, session| !session.is_expired_with_config(&self.config));
        
        (initial_count - sessions.len()) as u32
    }

    /// Get session configuration
    pub fn get_config(&self) -> &SessionConfig {
        &self.config
    }

    /// Update session configuration
    pub fn update_config(&mut self, config: SessionConfig) {
        self.config = config;
    }

    /// Get session statistics
    pub async fn get_session_stats(&self) -> SessionStats {
        let sessions = self.sessions.read().await;
        let mut total_requests = 0;
        let mut total_successes = 0;
        let mut platform_stats = HashMap::new();

        for (_platform_key, session) in sessions.iter() {
            total_requests += session.request_count;
            total_successes += session.success_count;
            
            let platform = session.platform.clone();
            let stats = PlatformSessionStats {
                request_count: session.request_count,
                success_count: session.success_count,
                success_rate: if session.request_count > 0 {
                    session.success_count as f64 / session.request_count as f64
                } else {
                    0.0
                },
                session_age: session.created_at.elapsed(),
            };
            
            platform_stats.insert(platform, stats);
        }

        SessionStats {
            active_sessions: sessions.len() as u32,
            total_requests,
            total_successes,
            overall_success_rate: if total_requests > 0 {
                total_successes as f64 / total_requests as f64
            } else {
                0.0
            },
            platform_stats,
        }
    }

    // Helper methods

    fn generate_session_id() -> String {
        use rand::{Rng, thread_rng};
        let mut rng = thread_rng();
        format!("sess_{:016x}", rng.gen::<u64>())
    }

    fn generate_session_user_agent(platform: &str) -> String {
        match platform {
            "amazon" => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            "facebook" => "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            "instagram" => "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1".to_string(),
            _ => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
        }
    }

    fn generate_session_viewport() -> Viewport {
        use rand::{Rng, thread_rng};
        let mut rng = thread_rng();
        
        let common_viewports = [(1920, 1080),
            (1366, 768),
            (1440, 900),
            (1536, 864)];
        
        let viewport = common_viewports[rng.gen_range(0..common_viewports.len())];
        
        Viewport {
            width: viewport.0,
            height: viewport.1,
            device_pixel_ratio: rng.gen_range(1.0..2.0),
        }
    }

    fn generate_session_headers(platform: &str) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        
        headers.insert("Accept".to_string(), "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".to_string());
        headers.insert("Accept-Language".to_string(), "en-US,en;q=0.5".to_string());
        headers.insert("Accept-Encoding".to_string(), "gzip, deflate, br".to_string());
        headers.insert("DNT".to_string(), "1".to_string());
        headers.insert("Connection".to_string(), "keep-alive".to_string());
        headers.insert("Upgrade-Insecure-Requests".to_string(), "1".to_string());
        
        // Platform-specific headers
        match platform {
            "facebook" | "instagram" => {
                headers.insert("Sec-Fetch-Dest".to_string(), "document".to_string());
                headers.insert("Sec-Fetch-Mode".to_string(), "navigate".to_string());
                headers.insert("Sec-Fetch-Site".to_string(), "none".to_string());
            },
            _ => {}
        }
        
        headers
    }
}

/// Cookie storage and management
pub struct CookieStore {
    cookies: HashMap<String, Vec<Cookie>>,
}

impl CookieStore {
    fn new() -> Self {
        Self {
            cookies: HashMap::new(),
        }
    }

    async fn store_cookies(&mut self, platform: &str, cookies: Vec<Cookie>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Merge with existing cookies, updating duplicates
        let existing = self.cookies.entry(platform.to_string()).or_default();
        
        for new_cookie in cookies {
            // Remove existing cookie with same name/domain/path
            existing.retain(|cookie| {
                !(cookie.name == new_cookie.name && 
                  cookie.domain == new_cookie.domain && 
                  cookie.path == new_cookie.path)
            });
            
            // Add new cookie if not expired
            if !new_cookie.is_expired() {
                existing.push(new_cookie);
            }
        }

        Ok(())
    }

    async fn get_cookies(&self, platform: &str) -> Vec<Cookie> {
        self.cookies.get(platform)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter(|cookie| !cookie.is_expired())
            .collect()
    }
}

/// Browser session state
#[derive(Debug, Clone)]
pub struct BrowserSession {
    pub platform: String,
    pub session_id: String,
    pub created_at: Instant,
    pub last_activity: Instant,
    pub cookies: Vec<Cookie>,
    pub local_storage: HashMap<String, String>,
    pub session_storage: HashMap<String, String>,
    pub user_agent: String,
    pub viewport: Viewport,
    pub headers: HashMap<String, String>,
    pub request_count: u64,
    pub success_count: u64,
}

impl BrowserSession {
    /// Check if session has expired
    pub fn is_expired(&self) -> bool {
        self.last_activity.elapsed() > Duration::from_secs(1800) // 30 minutes
    }

    /// Check if session has expired using config timeout
    pub fn is_expired_with_config(&self, config: &SessionConfig) -> bool {
        self.last_activity.elapsed() > config.session_timeout
    }

    /// Get session success rate
    pub fn success_rate(&self) -> f64 {
        if self.request_count > 0 {
            self.success_count as f64 / self.request_count as f64
        } else {
            0.0
        }
    }
}

/// HTTP cookie representation
#[derive(Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub expires: Option<Instant>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<SameSite>,
}

impl Cookie {
    /// Check if cookie has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires {
            expires <= Instant::now()
        } else {
            false
        }
    }
}

/// SameSite cookie attribute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

/// Viewport configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
    pub device_pixel_ratio: f64,
}

/// Session configuration
#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub session_timeout: Duration,
    pub max_sessions_per_platform: u32,
    pub cookie_persistence: bool,
    pub auto_cleanup_interval: Duration,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            session_timeout: Duration::from_secs(1800), // 30 minutes
            max_sessions_per_platform: 5,
            cookie_persistence: true,
            auto_cleanup_interval: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// Session statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStats {
    pub active_sessions: u32,
    pub total_requests: u64,
    pub total_successes: u64,
    pub overall_success_rate: f64,
    pub platform_stats: HashMap<String, PlatformSessionStats>,
}

/// Platform-specific session statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformSessionStats {
    pub request_count: u64,
    pub success_count: u64,
    pub success_rate: f64,
    pub session_age: Duration,
}
