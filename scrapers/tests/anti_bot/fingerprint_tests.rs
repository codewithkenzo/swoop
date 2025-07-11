//! Fingerprint manager tests
//! 
//! Tests for browser fingerprint spoofing and evasion techniques

use scrapers::anti_bot::fingerprint_manager::*;

#[tokio::test]
async fn test_fingerprint_manager_creation() {
    let manager = FingerprintManager::new().await.unwrap();
    
    // Test that manager can generate a profile
    let profile = manager.generate_fingerprint_profile().await;
    
    // Basic validation that profile contains expected fields
    assert!(!profile.canvas_signature.is_empty());
    assert!(!profile.webgl_signature.is_empty());
    assert!(!profile.audio_signature.is_empty());
    assert!(!profile.tls_signature.is_empty());
}

#[tokio::test]
async fn test_fingerprint_uniqueness() {
    let manager = FingerprintManager::new().await.unwrap();
    
    // Generate multiple profiles
    let profile1 = manager.generate_fingerprint_profile().await;
    let profile2 = manager.generate_fingerprint_profile().await;
    
    // Profiles should be different (spoofed)
    assert_ne!(profile1.canvas_signature, profile2.canvas_signature);
    assert_ne!(profile1.webgl_signature, profile2.webgl_signature);
    assert_ne!(profile1.audio_signature, profile2.audio_signature);
}

#[tokio::test]
async fn test_fingerprint_profile_completeness() {
    let manager = FingerprintManager::new().await.unwrap();
    let profile = manager.generate_fingerprint_profile().await;
    
    // All fingerprint components should be non-empty
    assert!(!profile.canvas_signature.is_empty());
    assert!(!profile.webgl_signature.is_empty());
    assert!(!profile.audio_signature.is_empty());
    assert!(!profile.tls_signature.is_empty());
    
    // Viewport should have reasonable dimensions
    assert!(profile.viewport_data.width > 0);
    assert!(profile.viewport_data.height > 0);
    assert!(profile.viewport_data.color_depth > 0);
}
