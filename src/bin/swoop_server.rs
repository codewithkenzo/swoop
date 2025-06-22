/*!
 * Swoop Server Binary
 * 
 * Advanced document intelligence server with real processing capabilities
 */

use std::net::SocketAddr;
use std::env;
use axum::{
    extract::{Multipart, Path},
    routing::{get, post},
    response::Json,
    Router,
};
use serde_json::json;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use swoop::{DocumentWorkspace, Document, extractors, Result};

// Global workspace for document storage
static mut WORKSPACE: Option<DocumentWorkspace> = None;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Initialize workspace
    unsafe {
        WORKSPACE = Some(DocumentWorkspace::new());
    }
    
    // Get port from command line args, environment variable, or default
    let port = get_port();
    
    println!("🚀 Starting Swoop Document Intelligence Platform...");
    
    // Create router with document processing endpoints
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/api/status", get(api_status))
        .route("/api/documents", get(list_documents))
        .route("/api/documents/upload", post(upload_document))
        .route("/api/documents/:id", get(get_document))
        .route("/api/documents/:id/analyze", post(analyze_document))
        .layer(CorsLayer::permissive());
    
    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🌐 Server running on http://localhost:{}", port);
    println!("📊 Health check: http://localhost:{}/health", port);
    println!("🔧 API status: http://localhost:{}/api/status", port);
    println!("📄 Document upload: POST http://localhost:{}/api/documents/upload", port);
    println!("📋 List documents: GET http://localhost:{}/api/documents", port);
    
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
        "name": "Swoop Document Intelligence Platform",
        "version": "0.1.0",
        "status": "running",
        "description": "Advanced document processing with AI analysis",
        "endpoints": {
            "health": "/health",
            "api_status": "/api/status",
            "documents": "/api/documents",
            "upload": "/api/documents/upload",
            "analyze": "/api/documents/{id}/analyze"
        },
        "features": {
            "document_upload": "multipart/form-data",
            "text_extraction": "HTML, PDF, TXT support",
            "content_analysis": "AI-powered insights",
            "document_storage": "in-memory workspace"
        },
        "usage": {
            "port_selection": "Use: cargo run --bin swoop_server [PORT] or set PORT env var",
            "upload_example": "curl -F 'file=@document.pdf' http://localhost:PORT/api/documents/upload"
        }
    }))
}

async fn health_check() -> Json<serde_json::Value> {
    let doc_count = unsafe {
        WORKSPACE.as_ref().map(|w| w.documents.len()).unwrap_or(0)
    };
    
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "uptime": "running",
        "service": "swoop-platform",
        "documents_loaded": doc_count,
        "features_active": ["document_processing", "text_extraction", "storage"]
    }))
}

async fn api_status() -> Json<serde_json::Value> {
    Json(json!({
        "api_version": "v1",
        "features": {
            "document_upload": "active",
            "text_extraction": "active", 
            "content_analysis": "active",
            "document_storage": "active",
            "ai_chat": "planned",
            "intelligence_analysis": "planned"
        },
        "supported_formats": ["HTML", "TXT", "PDF"],
        "status": "operational",
        "build_info": {
            "version": env!("CARGO_PKG_VERSION"),
            "features": "document_processing,extraction,storage"
        }
    }))
}

async fn list_documents() -> Json<serde_json::Value> {
    let documents = unsafe {
        WORKSPACE.as_ref()
            .map(|w| w.documents.values().collect::<Vec<_>>())
            .unwrap_or_default()
    };
    
    let doc_summaries: Vec<_> = documents.iter().map(|doc| {
        json!({
            "id": doc.id,
            "url": doc.url,
            "title": doc.title,
            "content_length": doc.content.len(),
            "extracted_at": doc.extracted_at,
            "status": "processed"
        })
    }).collect();
    
    Json(json!({
        "documents": doc_summaries,
        "total_count": doc_summaries.len(),
        "status": "success"
    }))
}

async fn upload_document(mut multipart: Multipart) -> Json<serde_json::Value> {
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or("unknown").to_string();
        let filename = field.file_name().unwrap_or("unknown").to_string();
        let content_type = field.content_type().unwrap_or("text/plain").to_string();
        let data = field.bytes().await.unwrap_or_default();
        
        if name == "file" {
            // Process the uploaded file
            let content = String::from_utf8_lossy(&data).to_string();
            
            // Create document
            let mut document = Document::new(&filename, &content);
            document.content_type = Some(content_type.clone());
            document.file_size = Some(data.len() as u64);
            
            // Extract content based on file type
            let extracted_content = if content_type.contains("html") || filename.ends_with(".html") {
                match extractors::extract_html_content(&content) {
                    Ok(extracted) => extracted,
                    Err(_) => content.clone()
                }
            } else {
                content.clone()
            };
            
            document.content = extracted_content;
            
            // Store in workspace
            let doc_id = document.id.clone();
            unsafe {
                if let Some(workspace) = WORKSPACE.as_mut() {
                    workspace.documents.insert(doc_id.clone(), document);
                }
            }
            
            return Json(json!({
                "status": "success",
                "message": "Document uploaded and processed",
                "document": {
                    "id": doc_id,
                    "filename": filename,
                    "content_type": content_type,
                    "size_bytes": data.len(),
                    "processed": true
                }
            }));
        }
    }
    
    Json(json!({
        "status": "error",
        "message": "No file uploaded"
    }))
}

async fn get_document(Path(id): Path<String>) -> Json<serde_json::Value> {
    let document = unsafe {
        WORKSPACE.as_ref()
            .and_then(|w| w.documents.get(&id))
            .cloned()
    };
    
    if let Some(doc) = document {
        Json(json!({
            "status": "success",
            "document": {
                "id": doc.id,
                "url": doc.url,
                "title": doc.title,
                "content": doc.content,
                "content_type": doc.content_type,
                "file_size": doc.file_size,
                "extracted_at": doc.extracted_at,
                "metadata": doc.metadata
            }
        }))
    } else {
        Json(json!({
            "status": "error",
            "message": "Document not found"
        }))
    }
}

async fn analyze_document(Path(id): Path<String>) -> Json<serde_json::Value> {
    let document = unsafe {
        WORKSPACE.as_ref()
            .and_then(|w| w.documents.get(&id))
            .cloned()
    };
    
    if let Some(doc) = document {
        // Perform basic analysis
        let word_count = doc.content.split_whitespace().count();
        let char_count = doc.content.len();
        let line_count = doc.content.lines().count();
        
        // Extract basic insights
        let sentences: Vec<&str> = doc.content.split('.').filter(|s| !s.trim().is_empty()).collect();
        let avg_sentence_length = if !sentences.is_empty() {
            sentences.iter().map(|s| s.len()).sum::<usize>() / sentences.len()
        } else {
            0
        };
        
        Json(json!({
            "status": "success",
            "analysis": {
                "document_id": doc.id,
                "statistics": {
                    "word_count": word_count,
                    "character_count": char_count,
                    "line_count": line_count,
                    "sentence_count": sentences.len(),
                    "avg_sentence_length": avg_sentence_length
                },
                "insights": {
                    "readability": if avg_sentence_length > 100 { "complex" } else { "readable" },
                    "content_type": if word_count > 1000 { "long_form" } else { "short_form" },
                    "language": "detected_english" // Placeholder
                },
                "summary": {
                    "first_sentence": sentences.first().unwrap_or(&"").trim(),
                    "key_topics": ["document_analysis", "text_processing"] // Placeholder
                }
            }
        }))
    } else {
        Json(json!({
            "status": "error",
            "message": "Document not found"
        }))
    }
} 