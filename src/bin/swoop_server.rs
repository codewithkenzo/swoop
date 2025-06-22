/*!
 * Swoop Server Binary
 * 
 * A minimal HTTP server for the Swoop platform
 */

use std::net::SocketAddr;
use std::env;
use axum::{
    routing::get,
    response::Json,
    Router,
};
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Get port from command line args, environment variable, or default
    let port = get_port();
    
    println!("🚀 Starting Swoop Server...");
    
    // Create a simple router
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/api/status", get(api_status))
        .layer(CorsLayer::permissive());
    
    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🌐 Server running on http://localhost:{}", port);
    println!("📊 Health check: http://localhost:{}/health", port);
    println!("🔧 API status: http://localhost:{}/api/status", port);
    
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

fn get_port() -> u16 {
    // Check command line arguments first
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if let Ok(port) = args[1].parse::<u16>() {
            if port > 1024 && port < 65535 {
                return port;
            } else {
                eprintln!("⚠️  Port {} is invalid. Using default port.", port);
            }
        } else {
            eprintln!("⚠️  Invalid port argument '{}'. Using default port.", args[1]);
        }
    }
    
    // Check environment variable
    if let Ok(port_str) = env::var("PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            if port > 1024 && port < 65535 {
                return port;
            }
        }
    }
    
    // Default port
    3001
}

async fn root() -> Json<serde_json::Value> {
    Json(json!({
        "name": "Swoop Platform",
        "version": "0.1.0",
        "status": "running",
        "description": "Advanced Document Intelligence Platform",
        "endpoints": {
            "health": "/health",
            "api_status": "/api/status"
        },
        "usage": {
            "port_selection": "Use: cargo run --bin swoop_server [PORT] or set PORT env var"
        }
    }))
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "uptime": "running",
        "service": "swoop-platform"
    }))
}

async fn api_status() -> Json<serde_json::Value> {
    Json(json!({
        "api_version": "v1",
        "features": {
            "document_processing": "available",
            "ai_chat": "available", 
            "intelligence_analysis": "available",
            "fuzzy_search": "available",
            "personality_system": "available"
        },
        "status": "operational",
        "build_info": {
            "version": env!("CARGO_PKG_VERSION"),
            "build_time": "minimal"
        }
    }))
} 