//! Anti-bot evasion system tests
//! 
//! Comprehensive test suite for all anti-bot components including:
//! - Fingerprint spoofing effectiveness
//! - Proxy rotation and health monitoring
//! - Behavioral mimicking validation
//! - Stealth browser automation
//! - Session management

use scrapers::anti_bot::*;
use std::time::Duration;
use tokio::time::sleep;

pub mod fingerprint_tests;
pub mod proxy_tests;
pub mod behavior_tests;
pub mod stealth_tests;
pub mod session_tests;
pub mod integration_tests;

/// Test utilities for anti-bot testing
pub struct TestUtils;

impl TestUtils {
    /// Create a test fingerprint manager
    pub async fn create_test_fingerprint_manager() -> fingerprint_manager::FingerprintManager {
        fingerprint_manager::FingerprintManager::new().await.unwrap()
    }

    /// Create a test proxy rotator with mock proxies
    pub async fn create_test_proxy_rotator() -> proxy_rotator::ProxyRotator {
        let config = proxy_rotator::ProxyConfig {
            rotation_interval: Duration::from_secs(30),
            health_check_interval: Duration::from_secs(60),
            max_failures: 3,
            timeout: Duration::from_secs(10),
        };
        
        proxy_rotator::ProxyRotator::new(config).await.unwrap()
    }

    /// Create a test behavior engine
    pub async fn create_test_behavior_engine() -> behavior_engine::BehaviorEngine {
        behavior_engine::BehaviorEngine::new().await.unwrap()
    }

    /// Create a test stealth browser
    pub async fn create_test_stealth_browser() -> stealth_browser::StealthBrowser {
        let config = stealth_browser::StealthConfig::default();
        stealth_browser::StealthBrowser::new(config).await.unwrap()
    }

    /// Create a test session manager
    pub async fn create_test_session_manager() -> session_manager::SessionManager {
        session_manager::SessionManager::new().await.unwrap()
    }

    /// Validate that a fingerprint has been properly spoofed
    pub fn validate_fingerprint_spoofing(original: &str, spoofed: &str) -> bool {
        // Fingerprints should be different but valid
        original != spoofed && !spoofed.is_empty() && spoofed.len() > 10
    }

    /// Check if mouse movements appear human-like
    pub fn validate_human_mouse_movement(movements: &[behavior_engine::MouseEvent]) -> bool {
        if movements.len() < 2 {
            return false;
        }

        // Check for smooth curves and realistic timing
        let mut has_curves = false;
        let mut realistic_timing = true;

        for window in movements.windows(2) {
            let time_diff = window[1].timestamp.duration_since(window[0].timestamp);
            
            // Human movements should have some variance in timing
            if time_diff < Duration::from_millis(10) || time_diff > Duration::from_millis(500) {
                realistic_timing = false;
            }

            // Check for non-linear movement (curves)
            let dx = (window[1].x - window[0].x).abs();
            let dy = (window[1].y - window[0].y).abs();
            if dx > 0 && dy > 0 {
                has_curves = true;
            }
        }

        has_curves && realistic_timing
    }

    /// Validate typing patterns appear natural
    pub fn validate_natural_typing(events: &[behavior_engine::TypingEvent]) -> bool {
        if events.len() < 2 {
            return true; // Single character is always valid
        }

        let mut realistic_timing = true;
        let mut has_variance = false;
        let mut last_interval = Duration::from_millis(0);

        for window in events.windows(2) {
            let interval = window[1].timestamp.duration_since(window[0].timestamp);
            
            // Human typing should be between 50ms and 1000ms per character
            if interval < Duration::from_millis(50) || interval > Duration::from_millis(1000) {
                realistic_timing = false;
            }

            // Check for timing variance (humans don't type at constant speed)
            if last_interval != Duration::from_millis(0) {
                let variance = if interval > last_interval {
                    interval - last_interval
                } else {
                    last_interval - interval
                };
                
                if variance > Duration::from_millis(20) {
                    has_variance = true;
                }
            }
            
            last_interval = interval;
        }

        realistic_timing && has_variance
    }

    /// Mock HTTP response for testing
    pub fn create_mock_response(status: u16, body: &str) -> http::Response<hyper::body::Bytes> {
        http::Response::builder()
            .status(status)
            .header("content-type", "text/html")
            .body(hyper::body::Bytes::from(body.to_string()))
            .unwrap()
    }

    /// Create test proxy info
    pub fn create_test_proxy() -> proxy_rotator::ProxyInfo {
        proxy_rotator::ProxyInfo {
            host: "127.0.0.1".to_string(),
            port: 8080,
            proxy_type: proxy_rotator::ProxyType::Http,
            username: Some("test_user".to_string()),
            password: Some("test_pass".to_string()),
            country: Some("US".to_string()),
            region: Some("California".to_string()),
            city: Some("San Francisco".to_string()),
            isp: Some("Test ISP".to_string()),
            last_used: std::time::Instant::now(),
            success_count: 10,
            failure_count: 1,
            avg_response_time: Duration::from_millis(200),
            is_healthy: true,
        }
    }

    /// Wait for async operations to complete
    pub async fn wait_for_completion() {
        sleep(Duration::from_millis(100)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_utils_creation() {
        // Test that all test utilities can be created successfully
        let _fingerprint_manager = TestUtils::create_test_fingerprint_manager().await;
        let _proxy_rotator = TestUtils::create_test_proxy_rotator().await;
        let _behavior_engine = TestUtils::create_test_behavior_engine().await;
        let _stealth_browser = TestUtils::create_test_stealth_browser().await;
        let _session_manager = TestUtils::create_test_session_manager().await;
    }

    #[test]
    fn test_fingerprint_validation() {
        let original = "original_fingerprint_12345";
        let spoofed = "spoofed_fingerprint_67890";
        let invalid = "";

        assert!(TestUtils::validate_fingerprint_spoofing(original, spoofed));
        assert!(!TestUtils::validate_fingerprint_spoofing(original, invalid));
        assert!(!TestUtils::validate_fingerprint_spoofing(original, original));
    }

    #[test]
    fn test_proxy_creation() {
        let proxy = TestUtils::create_test_proxy();
        assert_eq!(proxy.host, "127.0.0.1");
        assert_eq!(proxy.port, 8080);
        assert!(proxy.is_healthy);
    }
}
