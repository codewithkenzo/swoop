/*!
 * Integration Tests for Swoop API Server
 * 
 * Tests the HTTP API endpoints and core functionality
 */

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use serial_test::serial;
use tokio::time::timeout;

// Import from the actual codebase structure
use swoop::api_server::{AppState, create_router};
use swoop::test_utils::build_test_router;

#[allow(dead_code)]
const TEST_TIMEOUT: Duration = Duration::from_secs(30);

#[tokio::test]
#[serial]
async fn test_health_endpoint() {
    println!("🔍 Testing health endpoint...");
    
    let app = build_test_router().await;
    let response = warp::test::request()
        .method("GET")
        .path("/health")
        .reply(&app)
        .await;
    
    assert_eq!(response.status(), 200);
    println!("  ✅ Health endpoint test passed");
}

#[tokio::test]
#[serial]
async fn test_document_upload() {
    println!("📄 Testing document upload...");
    
    let app = build_test_router().await;
    
    // Test document upload
    let test_content = "This is test document content";
    let response = warp::test::request()
        .method("POST")
        .path("/api/documents")
        .header("content-type", "text/plain")
        .body(test_content)
        .reply(&app)
        .await;
    
    assert_eq!(response.status(), 200);
    
    let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
    assert!(body["id"].is_string());
    assert!(body["status"].is_string());
    
    println!("  ✅ Document upload test passed - ID: {}", body["id"]);
}

#[tokio::test]
#[serial]
async fn test_document_status() {
    println!("📋 Testing document status...");
    
    let app = build_test_router().await;
    
    // First upload a document
    let test_content = "Test content for status check";
    let upload_response = warp::test::request()
        .method("POST")
        .path("/api/documents")
        .header("content-type", "text/plain")
        .body(test_content)
        .reply(&app)
        .await;
    
    let upload_body: serde_json::Value = serde_json::from_slice(upload_response.body()).unwrap();
    let doc_id = upload_body["id"].as_str().unwrap();
    
    // Check status
    let status_response = warp::test::request()
        .method("GET")
        .path(&format!("/api/documents/{}/status", doc_id))
        .reply(&app)
        .await;
    
    assert_eq!(status_response.status(), 200);
    
    let status_body: serde_json::Value = serde_json::from_slice(status_response.body()).unwrap();
    assert_eq!(status_body["id"], doc_id);
    assert!(status_body["status"].is_string());
    
    println!("  ✅ Document status test passed - Status: {}", status_body["status"]);
}

#[tokio::test]
#[serial]
async fn test_crawl_start() {
    println!("🕷️ Testing crawl start...");
    
    let app = build_test_router().await;
    
    let crawl_request = serde_json::json!({
        "url": "https://example.com",
        "max_depth": 2,
        "max_pages": 10
    });
    
    let response = warp::test::request()
        .method("POST")
        .path("/api/crawl")
        .header("content-type", "application/json")
        .body(crawl_request.to_string())
        .reply(&app)
        .await;
    
    assert_eq!(response.status(), 200);
    
    let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
    assert!(body["job_id"].is_string());
    assert!(body["status"].is_string());
    
    println!("  ✅ Crawl start test passed - Job ID: {}", body["job_id"]);
}

#[tokio::test]
#[serial]
async fn test_crawl_status() {
    println!("📊 Testing crawl status...");
    
    let app = build_test_router().await;
    
    // Start a crawl first
    let crawl_request = serde_json::json!({
        "url": "https://example.com",
        "max_depth": 1,
        "max_pages": 5
    });
    
    let start_response = warp::test::request()
        .method("POST")
        .path("/api/crawl")
        .header("content-type", "application/json")
        .body(crawl_request.to_string())
        .reply(&app)
        .await;
    
    let start_body: serde_json::Value = serde_json::from_slice(start_response.body()).unwrap();
    let job_id = start_body["job_id"].as_str().unwrap();
    
    // Check status
    let status_response = warp::test::request()
        .method("GET")
        .path(&format!("/api/crawl/{}/status", job_id))
        .reply(&app)
        .await;
    
    assert_eq!(status_response.status(), 200);
    
    let status_body: serde_json::Value = serde_json::from_slice(status_response.body()).unwrap();
    assert_eq!(status_body["job_id"], job_id);
    assert!(status_body["status"].is_string());
    
    println!("  ✅ Crawl status test passed - Status: {}", status_body["status"]);
}

#[tokio::test]
#[serial]
async fn test_chat_endpoint() {
    println!("💬 Testing chat endpoint...");
    
    let app = build_test_router().await;
    
    let chat_request = serde_json::json!({
        "message": "Hello, how are you?",
        "context": "test context"
    });
    
    let response = warp::test::request()
        .method("POST")
        .path("/api/chat")
        .header("content-type", "application/json")
        .body(chat_request.to_string())
        .reply(&app)
        .await;
    
    assert_eq!(response.status(), 200);
    
    let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
    assert!(body["response"].is_string());
    
    println!("  ✅ Chat endpoint test passed - Response: {}", body["response"]);
}

#[tokio::test]
#[serial]
async fn test_system_stats() {
    println!("📈 Testing system stats...");
    
    let app = build_test_router().await;
    
    let response = warp::test::request()
        .method("GET")
        .path("/api/system/stats")
        .reply(&app)
        .await;
    
    assert_eq!(response.status(), 200);
    
    let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
    assert!(body["uptime"].is_number());
    assert!(body["total_documents"].is_number());
    assert!(body["active_crawls"].is_number());
    
    println!("  ✅ System stats test passed - Uptime: {}", body["uptime"]);
}

#[tokio::test]
#[serial]
async fn test_error_handling() {
    println!("⚠️ Testing error handling...");
    
    let app = build_test_router().await;
    
    // Test 404 for non-existent document
    let response = warp::test::request()
        .method("GET")
        .path("/api/documents/nonexistent/status")
        .reply(&app)
        .await;
    
    assert_eq!(response.status(), 404);
    
    // Test invalid JSON
    let response = warp::test::request()
        .method("POST")
        .path("/api/chat")
        .header("content-type", "application/json")
        .body("{invalid json")
        .reply(&app)
        .await;
    
    assert_eq!(response.status(), 400);
    
    println!("  ✅ Error handling test passed");
} 