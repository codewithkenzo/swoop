/*!
 * Example usage of the Crawl4AI high-performance server
 * 
 * This example demonstrates how to start the server with real-time
 * WebSocket connections, Server-Sent Events, and a modern web interface.
 */

use swoop::server::{CrawlServer, ServerConfig};
use swoop::Result;
use std::net::SocketAddr;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    // Configure the server
    let config = ServerConfig {
        bind_addr: "0.0.0.0:8080".parse().unwrap(),
        static_dir: Some(PathBuf::from("static")),
        enable_compression: true,
        request_timeout: 30,
        websocket_ping_interval: 30,
        max_connections: 1000,
        enable_cors: true,
        enable_logging: true,
        server_name: "Crawl4AI/1.0".to_string(),
    };
    
    println!("🚀 Starting swoop Server...");
    println!("📊 Dashboard: http://{}/", config.bind_addr);
    println!("🔌 WebSocket: ws://{}/ws", config.bind_addr);
    println!("📡 Events: http://{}/events", config.bind_addr);
    println!("🔍 Health: http://{}/api/v1/health", config.bind_addr);
    println!("📈 Stats: http://{}/api/v1/stats", config.bind_addr);
    
    // Create and start the server
    let server = CrawlServer::new(config);
    
    // This will run until Ctrl+C is pressed
    server.start().await?;
    
    Ok(())
}

/*
 * Usage Instructions:
 * 
 * 1. Build and run:
 *    cargo run --example server_example
 * 
 * 2. Open your browser to:
 *    - http://localhost:8080/ - Beautiful landing page with live stats
 *    - http://localhost:8080/dashboard - Interactive dashboard
 * 
 * 3. Test the API:
 *    curl http://localhost:8080/api/v1/health
 *    curl http://localhost:8080/api/v1/stats
 * 
 * 4. Real-time features:
 *    - Open browser dev tools and watch the WebSocket/SSE connections
 *    - Stats update automatically every 5 seconds
 *    - Smooth animations and modern UI
 * 
 * 5. API endpoints available:
 *    POST /api/v1/crawl - Start a new crawl job
 *    GET  /api/v1/crawl/:id - Get crawl status
 *    DELETE /api/v1/crawl/:id - Stop crawl job
 *    GET  /api/v1/documents - Search documents
 *    GET  /api/v1/documents/:id - Get specific document
 *    GET  /api/v1/stats - Server statistics
 *    GET  /api/v1/health - Health check
 * 
 * Features demonstrated:
 * - Modern async/await Rust patterns
 * - High-performance connection handling
 * - Real-time WebSocket connections
 * - Server-Sent Events for browser compatibility
 * - Beautiful, animated web interface
 * - Comprehensive middleware pipeline
 * - Graceful shutdown handling
 * - Production-ready error handling
 * - Automatic compression and CORS
 * - Built-in security headers
 * - Request logging and metrics
 */ 