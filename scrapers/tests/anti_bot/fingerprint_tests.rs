//! Fingerprint spoofing tests
//! 
//! Tests for browser fingerprint evasion including:
//! - Canvas fingerprint randomization
//! - WebGL parameter spoofing
//! - TLS signature manipulation
//! - AudioContext fingerprint evasion

use super::TestUtils;
use scrapers::anti_bot::fingerprint_manager::*;
use std::collections::HashSet;

#[tokio::test]
async fn test_fingerprint_manager_creation() {
    let manager = TestUtils::create_test_fingerprint_manager().await;
    
    // Verify manager is properly initialized
    assert!(manager.generate_canvas_fingerprint().await.is_ok());
    assert!(manager.generate_webgl_fingerprint().await.is_ok());
    assert!(manager.generate_audio_fingerprint().await.is_ok());
}

#[tokio::test]
async fn test_canvas_fingerprint_randomization() {
    let manager = TestUtils::create_test_fingerprint_manager().await;
    
    // Generate multiple canvas fingerprints
    let mut fingerprints = HashSet::new();
    
    for _ in 0..10 {
        let fingerprint = manager.generate_canvas_fingerprint().await.unwrap();
        fingerprints.insert(fingerprint);
    }
    
    // All fingerprints should be unique (randomized)
    assert_eq!(fingerprints.len(), 10, "Canvas fingerprints should be randomized");
    
    // Each fingerprint should be a valid hash
    for fingerprint in &fingerprints {
        assert!(!fingerprint.is_empty());
        assert!(fingerprint.len() >= 32); // Minimum hash length
    }
}

#[tokio::test]
async fn test_webgl_fingerprint_spoofing() {
    let manager = TestUtils::create_test_fingerprint_manager().await;
    
    // Generate multiple WebGL fingerprints
    let mut fingerprints = Vec::new();
    
    for _ in 0..5 {
        let fingerprint = manager.generate_webgl_fingerprint().await.unwrap();
        fingerprints.push(fingerprint);
    }
    
    // Verify fingerprints are different
    for i in 0..fingerprints.len() {
        for j in i+1..fingerprints.len() {
            assert_ne!(fingerprints[i], fingerprints[j], 
                      "WebGL fingerprints should be unique");
        }
    }
    
    // Verify fingerprints contain expected WebGL parameters
    for fingerprint in &fingerprints {
        assert!(fingerprint.contains("renderer") || fingerprint.contains("vendor"));
    }
}

#[tokio::test]
async fn test_tls_signature_randomization() {
    let manager = TestUtils::create_test_fingerprint_manager().await;
    
    // Generate TLS signatures
    let sig1 = manager.generate_tls_signature().await.unwrap();
    let sig2 = manager.generate_tls_signature().await.unwrap();
    
    // Signatures should be different
    assert_ne!(sig1, sig2, "TLS signatures should be randomized");
    
    // Verify signature format (JA3/JA4 style)
    assert!(sig1.len() >= 32);
    assert!(sig2.len() >= 32);
    
    // Should contain hex characters
    assert!(sig1.chars().all(|c| c.is_ascii_hexdigit() || c == '-'));
    assert!(sig2.chars().all(|c| c.is_ascii_hexdigit() || c == '-'));
}

#[tokio::test]
async fn test_audio_context_fingerprint() {
    let manager = TestUtils::create_test_fingerprint_manager().await;
    
    // Generate audio fingerprints
    let mut fingerprints = Vec::new();
    
    for _ in 0..3 {
        let fingerprint = manager.generate_audio_fingerprint().await.unwrap();
        fingerprints.push(fingerprint);
    }
    
    // Verify uniqueness
    for i in 0..fingerprints.len() {
        for j in i+1..fingerprints.len() {
            assert_ne!(fingerprints[i], fingerprints[j], 
                      "Audio fingerprints should be unique");
        }
    }
    
    // Verify fingerprint format (should be numeric hash)
    for fingerprint in &fingerprints {
        assert!(!fingerprint.is_empty());
        assert!(fingerprint.len() >= 10);
    }
}

#[tokio::test]
async fn test_viewport_randomization() {
    let manager = TestUtils::create_test_fingerprint_manager().await;
    
    // Generate multiple viewports
    let mut viewports = Vec::new();
    
    for _ in 0..10 {
        let viewport = manager.generate_viewport().await.unwrap();
        viewports.push(viewport);
    }
    
    // Check for variety in viewport sizes
    let mut unique_widths = HashSet::new();
    let mut unique_heights = HashSet::new();
    
    for viewport in &viewports {
        unique_widths.insert(viewport.width);
        unique_heights.insert(viewport.height);
        
        // Verify reasonable viewport dimensions
        assert!(viewport.width >= 800 && viewport.width <= 2560);
        assert!(viewport.height >= 600 && viewport.height <= 1440);
        assert!(viewport.device_pixel_ratio >= 1.0 && viewport.device_pixel_ratio <= 3.0);
    }
    
    // Should have some variety in dimensions
    assert!(unique_widths.len() >= 3, "Should have variety in viewport widths");
    assert!(unique_heights.len() >= 3, "Should have variety in viewport heights");
}

#[tokio::test]
async fn test_user_agent_spoofing() {
    let manager = TestUtils::create_test_fingerprint_manager().await;
    
    // Test different platform user agents
    let platforms = vec!["chrome", "firefox", "safari", "edge"];
    let mut user_agents = Vec::new();
    
    for platform in platforms {
        let ua = manager.generate_user_agent(platform).await.unwrap();
        user_agents.push(ua);
    }
    
    // Verify all user agents are different
    for i in 0..user_agents.len() {
        for j in i+1..user_agents.len() {
            assert_ne!(user_agents[i], user_agents[j], 
                      "User agents should be unique per platform");
        }
    }
    
    // Verify user agents contain expected browser identifiers
    assert!(user_agents[0].contains("Chrome"));
    assert!(user_agents[1].contains("Firefox"));
    assert!(user_agents[2].contains("Safari"));
    assert!(user_agents[3].contains("Edge"));
}

#[tokio::test]
async fn test_fingerprint_consistency() {
    let manager = TestUtils::create_test_fingerprint_manager().await;
    
    // Generate a fingerprint profile
    let profile = manager.generate_fingerprint_profile("test_session").await.unwrap();
    
    // Verify profile contains all required components
    assert!(!profile.canvas_fingerprint.is_empty());
    assert!(!profile.webgl_fingerprint.is_empty());
    assert!(!profile.audio_fingerprint.is_empty());
    assert!(!profile.user_agent.is_empty());
    assert!(profile.viewport.width > 0);
    assert!(profile.viewport.height > 0);
    
    // Generate the same profile again - should be consistent for the session
    let profile2 = manager.generate_fingerprint_profile("test_session").await.unwrap();
    
    assert_eq!(profile.canvas_fingerprint, profile2.canvas_fingerprint);
    assert_eq!(profile.webgl_fingerprint, profile2.webgl_fingerprint);
    assert_eq!(profile.audio_fingerprint, profile2.audio_fingerprint);
}

#[tokio::test]
async fn test_fingerprint_profile_isolation() {
    let manager = TestUtils::create_test_fingerprint_manager().await;
    
    // Generate profiles for different sessions
    let profile1 = manager.generate_fingerprint_profile("session_1").await.unwrap();
    let profile2 = manager.generate_fingerprint_profile("session_2").await.unwrap();
    
    // Profiles should be different between sessions
    assert_ne!(profile1.canvas_fingerprint, profile2.canvas_fingerprint);
    assert_ne!(profile1.webgl_fingerprint, profile2.webgl_fingerprint);
    assert_ne!(profile1.audio_fingerprint, profile2.audio_fingerprint);
}

#[tokio::test]
async fn test_canvas_noise_injection() {
    let manager = TestUtils::create_test_fingerprint_manager().await;
    
    // Test different noise patterns
    let noise_types = vec!["pixel_shift", "color_jitter", "gamma_adjust"];
    
    for noise_type in noise_types {
        let result = manager.apply_canvas_noise(noise_type).await;
        assert!(result.is_ok(), "Canvas noise injection should succeed for {}", noise_type);
    }
}

#[tokio::test]
async fn test_webgl_parameter_spoofing() {
    let manager = TestUtils::create_test_fingerprint_manager().await;
    
    // Test WebGL parameter randomization
    let params = manager.generate_webgl_parameters().await.unwrap();
    
    // Verify required parameters are present
    assert!(params.contains_key("renderer"));
    assert!(params.contains_key("vendor"));
    assert!(params.contains_key("version"));
    assert!(params.contains_key("shading_language_version"));
    
    // Verify parameters have realistic values
    let renderer = params.get("renderer").unwrap();
    assert!(!renderer.is_empty());
    assert!(renderer.contains("GPU") || renderer.contains("Graphics"));
}

#[test]
fn test_fingerprint_validation() {
    // Test fingerprint validation utility
    let original = "original_canvas_fingerprint_12345678";
    let spoofed = "spoofed_canvas_fingerprint_87654321";
    let invalid = "";
    let too_short = "short";
    
    assert!(TestUtils::validate_fingerprint_spoofing(original, spoofed));
    assert!(!TestUtils::validate_fingerprint_spoofing(original, invalid));
    assert!(!TestUtils::validate_fingerprint_spoofing(original, too_short));
    assert!(!TestUtils::validate_fingerprint_spoofing(original, original));
}
