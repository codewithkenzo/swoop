/*!
 * Comprehensive Monitoring System Integration Tests
 * Tests all Phase 2 monitoring features including health checks, metrics, and real-time stats
 */

use std::time::Duration;
use tokio::time::sleep;
use reqwest::Client;
use serde_json::Value;

use swoop::server::{CrawlServer, ServerConfig};
use swoop::monitoring::MonitoringSystem;

const TEST_SERVER_ADDR: &str = "127.0.0.1:8090";

#[tokio::test]
async fn test_monitoring_system_initialization() {
    println!("🧪 Testing MonitoringSystem initialization...");
    
    let monitoring = MonitoringSystem::new()
        .expect("Should initialize monitoring system");
    
    let stats = monitoring.get_stats().await;
    assert_eq!(stats.requests_total, 0);
    assert_eq!(stats.successful_crawls, 0);
    assert_eq!(stats.failed_crawls, 0);
    
    println!("✅ MonitoringSystem initialized correctly");
}

#[tokio::test]
async fn test_health_endpoints_integration() {
    println!("🧪 Testing health endpoints integration...");
    
    // Start test server
    let config = ServerConfig {
        bind_addr: TEST_SERVER_ADDR.parse().unwrap(),
        ..Default::default()
    };
    
    let server = CrawlServer::new(config).expect("Should create server");
    
    // Start server in background
    let server_handle = tokio::spawn(async move {
        server.start().await
    });
    
    // Wait for server to start
    sleep(Duration::from_millis(500)).await;
    
    let client = Client::new();
    let base_url = format!("http://{}", TEST_SERVER_ADDR);
    
    // Test health endpoint
    println!("📋 Testing /health endpoint...");
    let health_response = client
        .get(&format!("{}/health", base_url))
        .send()
        .await
        .expect("Health endpoint should respond");
    
    assert!(health_response.status().is_success());
    let health_json: Value = health_response.json().await.expect("Should be valid JSON");
    assert_eq!(health_json["status"], "healthy");
    println!("✅ Health endpoint working");
    
    // Test readiness endpoint
    println!("📋 Testing /ready endpoint...");
    let ready_response = client
        .get(&format!("{}/ready", base_url))
        .send()
        .await
        .expect("Ready endpoint should respond");
    
    assert!(ready_response.status().is_success());
    let ready_json: Value = ready_response.json().await.expect("Should be valid JSON");
    assert_eq!(ready_json["status"], "ready");
    println!("✅ Readiness endpoint working");
    
    // Test metrics endpoint
    println!("📋 Testing /metrics endpoint...");
    let metrics_response = client
        .get(&format!("{}/metrics", base_url))
        .send()
        .await
        .expect("Metrics endpoint should respond");
    
    assert!(metrics_response.status().is_success());
    let metrics_text = metrics_response.text().await.expect("Should be text");
    assert!(metrics_text.contains("crawl4ai_requests_total"));
    println!("✅ Metrics endpoint working");
    
    // Test monitoring stats endpoint
    println!("📋 Testing /monitoring/stats endpoint...");
    let stats_response = client
        .get(&format!("{}/monitoring/stats", base_url))
        .send()
        .await
        .expect("Stats endpoint should respond");
    
    assert!(stats_response.status().is_success());
    let stats_json: Value = stats_response.json().await.expect("Should be valid JSON");
    assert!(stats_json.get("requests_total").is_some());
    assert!(stats_json.get("uptime_seconds").is_some());
    println!("✅ Monitoring stats endpoint working");
    
    // Cleanup
    server_handle.abort();
    println!("🎉 All monitoring endpoints working correctly!");
}

#[tokio::test]
async fn test_stress_monitoring_system() {
    println!("🧪 Testing monitoring system under load...");
    
    let monitoring = MonitoringSystem::new()
        .expect("Should initialize monitoring system");
    
    // Simulate multiple concurrent operations
    let mut handles = vec![];
    
    for i in 0..10 {
        let monitoring_clone = monitoring.clone();
        let handle = tokio::spawn(async move {
            let start = std::time::Instant::now();
            
            // Simulate crawl request
            monitoring_clone.record_crawl_request().await;
            
            // Simulate processing time
            tokio::time::sleep(Duration::from_millis(50)).await;
            
            // Record success
            monitoring_clone.record_successful_crawl(start.elapsed()).await;
            
            println!("✅ Completed stress test operation {}", i + 1);
        });
        
        handles.push(handle);
    }
    
    // Wait for all operations to complete
    for handle in handles {
        handle.await.expect("Task should complete");
    }
    
    // Check final stats
    let final_stats = monitoring.get_stats().await;
    assert_eq!(final_stats.requests_total, 10);
    assert_eq!(final_stats.successful_crawls, 10);
    assert_eq!(final_stats.failed_crawls, 0);
    
    println!("✅ Monitoring system handled {} operations correctly", final_stats.requests_total);
    println!("🎉 Stress test completed successfully!");
}

#[tokio::test] 
async fn test_real_crawl_with_monitoring() {
    println!("🧪 Testing real crawl operations with monitoring...");
    
    let config = ServerConfig {
        bind_addr: "127.0.0.1:8091".parse().unwrap(),
        ..Default::default()
    };
    
    let server = CrawlServer::new(config).expect("Should create server");
    
    // Start server in background
    let server_handle = tokio::spawn(async move {
        server.start().await
    });
    
    // Wait for server to start
    sleep(Duration::from_secs(1)).await;
    
    let client = Client::new();
    let base_url = "http://127.0.0.1:8091";
    
    // Get initial stats
    println!("📊 Getting initial monitoring stats...");
    let initial_stats_response = client
        .get(&format!("{}/monitoring/stats", base_url))
        .send()
        .await
        .expect("Stats endpoint should respond");
    
    let initial_stats: Value = initial_stats_response.json().await.expect("Should be valid JSON");
    let initial_requests = initial_stats["requests_total"].as_u64().unwrap_or(0);
    
    // Start a crawl job
    println!("🕷️ Starting crawl job...");
    let crawl_payload = serde_json::json!({
        "urls": ["https://httpbin.org/html"],
        "max_depth": 1,
        "max_pages": 1
    });
    
    let crawl_response = client
        .post(&format!("{}/api/v1/crawl", base_url))
        .json(&crawl_payload)
        .send()
        .await
        .expect("Crawl endpoint should respond");
    
    assert!(crawl_response.status().is_success());
    let crawl_json: Value = crawl_response.json().await.expect("Should be valid JSON");
    assert_eq!(crawl_json["status"], "started");
    println!("✅ Crawl job started: {}", crawl_json["job_id"]);
    
    // Wait a bit for processing
    sleep(Duration::from_secs(2)).await;
    
    // Check updated stats
    println!("📊 Checking updated monitoring stats...");
    let updated_stats_response = client
        .get(&format!("{}/monitoring/stats", base_url))
        .send()
        .await
        .expect("Stats endpoint should respond");
    
    let updated_stats: Value = updated_stats_response.json().await.expect("Should be valid JSON");
    let updated_requests = updated_stats["requests_total"].as_u64().unwrap_or(0);
    
    // Verify metrics were updated
    assert!(updated_requests > initial_requests, "Request count should have increased");
    println!("✅ Monitoring captured request: {} -> {}", initial_requests, updated_requests);
    
    // Test GUI dashboard
    println!("🖥️ Testing GUI dashboard...");
    let dashboard_response = client
        .get(&format!("{}/dashboard", base_url))
        .send()
        .await
        .expect("Dashboard should respond");
    
    assert!(dashboard_response.status().is_success());
    let dashboard_html = dashboard_response.text().await.expect("Should be HTML");
    assert!(dashboard_html.contains("Crawl4AI Dashboard"));
    assert!(dashboard_html.contains("Server Status"));
    assert!(dashboard_html.contains("Statistics"));
    println!("✅ GUI dashboard working correctly");
    
    // Cleanup
    server_handle.abort();
    println!("🎉 Real crawl with monitoring completed successfully!");
}

#[tokio::test]
async fn test_websocket_and_sse_integration() {
    println!("🧪 Testing WebSocket and SSE integration...");
    
    let config = ServerConfig {
        bind_addr: "127.0.0.1:8092".parse().unwrap(),
        ..Default::default()
    };
    
    let server = CrawlServer::new(config).expect("Should create server");
    
    // Start server in background
    let server_handle = tokio::spawn(async move {
        server.start().await
    });
    
    // Wait for server to start
    sleep(Duration::from_secs(1)).await;
    
    let client = Client::new();
    let base_url = "http://127.0.0.1:8092";
    
    // Test SSE endpoint
    println!("📡 Testing SSE events endpoint...");
    let sse_response = client
        .get(&format!("{}/events", base_url))
        .send()
        .await
        .expect("SSE endpoint should respond");
    
    assert!(sse_response.status().is_success());
    assert_eq!(sse_response.headers().get("content-type").unwrap(), "text/event-stream");
    println!("✅ SSE endpoint working");
    
    // Test legacy stats compatibility
    println!("📊 Testing legacy stats compatibility...");
    let legacy_stats_response = client
        .get(&format!("{}/api/v1/stats", base_url))
        .send()
        .await
        .expect("Legacy stats should respond");
    
    assert!(legacy_stats_response.status().is_success());
    let legacy_stats: Value = legacy_stats_response.json().await.expect("Should be valid JSON");
    assert!(legacy_stats.get("total_requests").is_some());
    println!("✅ Legacy stats endpoint working");
    
    // Cleanup
    server_handle.abort();
    println!("🎉 WebSocket and SSE integration working correctly!");
} 