use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting Simple Test Server...");
    
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("✅ Server running at http://127.0.0.1:8080");
    println!("📊 Visit http://127.0.0.1:8080 for hello world");
    println!("🔌 Visit http://127.0.0.1:8080/health for health check");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn hello_world() -> Html<&'static str> {
    Html("<h1>🦀 Rust Server Working!</h1><p>QuantumScribe Crawl4AI Core is ready!</p>")
}

async fn health_check() -> &'static str {
    "OK"
} 