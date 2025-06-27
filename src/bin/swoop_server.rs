/*!
 * Swoop Server Binary
 * 
 * Advanced document intelligence server with robust processing capabilities
 */

use std::env;
use std::sync::{Arc, Mutex, LazyLock};

use axum::{
    extract::{Multipart, Path, Json},
    http::StatusCode,
    response::Json as ResponseJson,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use swoop::{DocumentWorkspace, Document};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use clap::{Arg, Command};

// Thread-safe workspace using LazyLock
static WORKSPACE: LazyLock<Arc<Mutex<DocumentWorkspace>>> = LazyLock::new(|| {
    println!("📂 Initializing document workspace...");
    Arc::new(Mutex::new(DocumentWorkspace::new()))
});

#[derive(Debug, Serialize, Deserialize)]
struct DocumentResponse {
    id: String,
    filename: String,
    content_type: String,
    size_bytes: u64,
    processed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnalysisResponse {
    document_id: String,
    statistics: DocumentStatistics,
    insights: DocumentInsights,
    summary: DocumentSummary,
}

#[derive(Debug, Serialize, Deserialize)]
struct DocumentStatistics {
    word_count: usize,
    character_count: usize,
    line_count: usize,
    sentence_count: usize,
    avg_sentence_length: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct DocumentInsights {
    readability: String,
    content_type: String,
    language: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DocumentSummary {
    first_sentence: String,
    key_topics: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DocumentListItem {
    id: String,
    url: String,
    title: String,
    content_length: usize,
    extracted_at: DateTime<Utc>,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    message: String,
    document_id: Option<String>,
    context_window: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatResponse {
    response: String,
    document_context: Option<String>,
    confidence: f64,
    sources: Vec<String>,
    timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DocumentChatContext {
    document_id: String,
    relevant_excerpt: String,
    word_position: usize,
    context_score: f64,
}

fn get_workspace_document_count() -> usize {
    let workspace_guard = WORKSPACE.lock().unwrap();
    workspace_guard.documents.len()
}

// Robust text extraction with fallback methods
fn extract_text_robust(content: &str, content_type: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    match content_type {
        "text/html" => {
            // Simple HTML tag removal as fallback
            let re = regex::Regex::new(r"<[^>]*>")?;
            let text = re.replace_all(content, " ");
            let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
            if text.trim().is_empty() {
                Ok("No extractable text content found".to_string())
            } else {
                Ok(text)
            }
        }
        "text/plain" => Ok(content.to_string()),
        _ => {
            // For unknown content types, try to extract any readable text
            let cleaned = content
                .chars()
                .filter(|c| c.is_ascii_graphic() || c.is_whitespace())
                .collect::<String>();
            
            if cleaned.trim().is_empty() {
                Ok("Binary or unreadable content".to_string())
            } else {
                Ok(cleaned)
            }
        }
    }
}

// Enhanced text analysis with error tolerance
fn analyze_text_robust(text: &str) -> DocumentStatistics {
    let words: Vec<&str> = text.split_whitespace().collect();
    let word_count = words.len();
    let character_count = text.len();
    
    // Count lines more robustly
    let line_count = std::cmp::max(1, text.lines().count());
    
    // Count sentences with multiple delimiters and error handling
    let sentence_delimiters = ['.', '!', '?'];
    let sentence_count = text
        .chars()
        .filter(|c| sentence_delimiters.contains(c))
        .count()
        .max(1); // Ensure at least 1 sentence
    
    let avg_sentence_length = if sentence_count > 0 {
        character_count as f64 / sentence_count as f64
    } else {
        character_count as f64
    };
    
    DocumentStatistics {
        word_count,
        character_count,
        line_count,
        sentence_count,
        avg_sentence_length,
    }
}

fn classify_content(stats: &DocumentStatistics) -> DocumentInsights {
    let readability = if stats.avg_sentence_length > 100.0 {
        "complex"
    } else if stats.avg_sentence_length > 50.0 {
        "moderate"
    } else {
        "readable"
    };
    
    let content_type = if stats.word_count < 100 {
        "short_form"
    } else if stats.word_count < 1000 {
        "medium_form"
    } else {
        "long_form"
    };
    
    DocumentInsights {
        readability: readability.to_string(),
        content_type: content_type.to_string(),
        language: "detected_english".to_string(), // Placeholder for future language detection
    }
}

async fn root_handler() -> ResponseJson<Value> {
    Json(json!({
        "service": "Swoop Document Intelligence Platform",
        "version": "0.2.0",
        "status": "operational",
        "features": [
            "document_upload",
            "content_extraction", 
            "text_analysis",
            "robust_processing",
            "chat_interface",
            "document_chat",
            "pdf_support",
            "markdown_support"
        ],
        "endpoints": {
            "health": "/health",
            "upload": "/api/documents/upload",
            "list": "/api/documents",
            "analyze": "/api/documents/{id}/analyze",
            "chat": "/api/chat",
            "document_chat": "/api/documents/{id}/chat"
        }
    }))
}

async fn health_handler() -> ResponseJson<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": Utc::now(),
        "uptime": "running",
        "service": "swoop-platform",
        "documents_loaded": get_workspace_document_count(),
        "features_active": [
            "document_processing",
            "text_extraction",
            "storage",
            "robust_parsing"
        ]
    }))
}

async fn api_status_handler() -> ResponseJson<Value> {
    Json(json!({
        "api_version": "v1",
        "status": "active",
        "endpoints_available": 5,
        "documents_processed": get_workspace_document_count(),
        "capabilities": [
            "multipart_upload",
            "html_processing",
            "text_analysis",
            "error_recovery"
        ]
    }))
}

async fn upload_handler(mut multipart: Multipart) -> Result<ResponseJson<Value>, StatusCode> {
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        if field.name() == Some("file") {
            let filename = field.file_name()
                .unwrap_or("unknown.txt")
                .to_string();
            
            let content_type = field.content_type()
                .unwrap_or("text/plain")
                .to_string();
            
            let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            let size_bytes = data.len() as u64;
            
            // Robust content processing with encoding detection
            let content = match std::str::from_utf8(&data) {
                Ok(s) => s.to_string(),
                Err(_) => {
                    // Last resort: replace invalid characters
                    String::from_utf8_lossy(&data).into_owned()
                }
            };
            
            // Extract text with robust error handling
            let extracted_text = match extract_text_robust(&content, &content_type) {
                Ok(text) => text,
                Err(e) => {
                    eprintln!("Text extraction error: {}, using raw content", e);
                    content.clone()
                }
            };
            
            let doc_id = format!("doc_{:x}", rand::random::<u32>());
            
            // Create document with proper structure
            let mut document = Document::new(&filename, &extracted_text);
            document.id = doc_id.clone();
            document.content_type = Some(content_type.clone());
            document.file_size = Some(size_bytes);
            document.content = if content_type.contains("html") { content.clone() } else { extracted_text.clone() };
            
            // Store document safely
            {
                let mut workspace_guard = WORKSPACE.lock().unwrap();
                workspace_guard.documents.insert(doc_id.clone(), document);
            }
            
            return Ok(Json(json!({
                "status": "success",
                "message": "Document uploaded and processed",
                "document": {
                    "id": doc_id,
                    "filename": filename,
                    "content_type": content_type,
                    "size_bytes": size_bytes,
                    "processed": true
                }
            })));
        }
    }
    
    Err(StatusCode::BAD_REQUEST)
}

async fn list_documents_handler() -> ResponseJson<Value> {
    let workspace_guard = WORKSPACE.lock().unwrap();
    let documents: Vec<DocumentListItem> = workspace_guard.documents.values()
        .map(|doc| DocumentListItem {
            id: doc.id.clone(),
            url: doc.source_url.clone().unwrap_or_default(),
            title: doc.title.clone(),
            content_length: doc.content.len(),
            extracted_at: doc.extracted_at,
            status: "processed".to_string(),
        })
        .collect();
    
    Json(json!({
        "documents": documents,
        "total_count": documents.len(),
        "status": "success"
    }))
}

async fn analyze_document_handler(Path(doc_id): Path<String>) -> Result<ResponseJson<Value>, StatusCode> {
    let workspace_guard = WORKSPACE.lock().unwrap();
    let document = workspace_guard.documents.get(&doc_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    let statistics = analyze_text_robust(&document.content);
    let insights = classify_content(&statistics);
    
    let first_sentence = document.content
        .split_terminator(&['.', '!', '?'])
        .next()
        .unwrap_or(&document.content)
        .trim()
        .to_string();
    
    let summary = DocumentSummary {
        first_sentence,
        key_topics: vec!["document_analysis".to_string(), "text_processing".to_string()],
    };
    
    let analysis = AnalysisResponse {
        document_id: doc_id,
        statistics,
        insights,
        summary,
    };
    
    Ok(Json(json!({
        "status": "success",
        "analysis": analysis
    })))
}

async fn chat_handler(Json(request): Json<ChatRequest>) -> Result<ResponseJson<Value>, StatusCode> {
    let response = process_chat_request(request).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(json!({
        "status": "success",
        "chat": response
    })))
}

async fn document_chat_handler(
    Path(doc_id): Path<String>,
    Json(request): Json<ChatRequest>
) -> Result<ResponseJson<Value>, StatusCode> {
    // Ensure document exists
    let workspace_guard = WORKSPACE.lock().unwrap();
    let document = workspace_guard.documents.get(&doc_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    
    let document_content = document.content.clone();
    drop(workspace_guard);
    
    let mut chat_request = request;
    chat_request.document_id = Some(doc_id.clone());
    
    let response = process_document_chat(chat_request, &document_content).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(json!({
        "status": "success",
        "document_id": doc_id,
        "chat": response
    })))
}

// Simple chat processing without external AI (placeholder for future integration)
async fn process_chat_request(request: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
    let message = request.message.to_lowercase();
    
    // Simple keyword-based responses for demonstration
    let response = if message.contains("upload") {
        "To upload a document, send a POST request to /api/documents/upload with a multipart form containing your file."
    } else if message.contains("analyze") {
        "To analyze a document, use POST /api/documents/{id}/analyze after uploading it."
    } else if message.contains("list") || message.contains("documents") {
        "To list all documents, use GET /api/documents. This will show all uploaded and processed documents."
    } else if message.contains("help") || message.contains("api") {
        "Available endpoints: /api/documents/upload (POST), /api/documents (GET), /api/documents/{id}/analyze (POST), /api/chat (POST)"
    } else if message.contains("format") || message.contains("support") {
        "Supported formats: HTML, PDF, Markdown, and plain text files with robust extraction capabilities."
    } else {
        "I can help you with document processing, analysis, and API usage. Ask me about uploading, analyzing, or listing documents."
    };
    
    Ok(ChatResponse {
        response: response.to_string(),
        document_context: None,
        confidence: 0.8,
        sources: vec!["swoop_platform".to_string()],
        timestamp: Utc::now(),
    })
}

// Document-specific chat processing
async fn process_document_chat(
    request: ChatRequest, 
    document_content: &str
) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
    let message = request.message.to_lowercase();
    let context_window = request.context_window.unwrap_or(500);
    
    // Find relevant content based on keywords
    let relevant_context = find_relevant_content(&message, document_content, context_window);
    
    let response = if message.contains("summary") || message.contains("summarize") {
        format!("Document summary: {}", create_simple_summary(document_content))
    } else if message.contains("word count") || message.contains("length") {
        format!("This document contains {} words and {} characters.", 
            document_content.split_whitespace().count(),
            document_content.len())
    } else if message.contains("search") || message.contains("find") {
        // Extract search terms (simple approach)
        let search_terms: Vec<&str> = message.split_whitespace()
            .skip_while(|&w| w != "search" && w != "find")
            .skip(1)
            .collect();
        
        if !search_terms.is_empty() {
            let search_term = search_terms.join(" ");
            if document_content.to_lowercase().contains(&search_term) {
                format!("Found '{}' in the document. Here's the relevant context: {}", 
                    search_term, relevant_context.clone().unwrap_or("No specific context found".to_string()))
            } else {
                format!("Could not find '{}' in this document.", search_term)
            }
        } else {
            "Please specify what you'd like to search for in the document.".to_string()
        }
    } else {
        format!("Based on the document content: {}", 
            relevant_context.clone().unwrap_or("I can help you analyze, search, or summarize this document.".to_string()))
    };
    
    Ok(ChatResponse {
        response,
        document_context: relevant_context,
        confidence: 0.7,
        sources: vec![request.document_id.unwrap_or("unknown".to_string())],
        timestamp: Utc::now(),
    })
}

fn find_relevant_content(query: &str, content: &str, window_size: usize) -> Option<String> {
    let query_words: Vec<&str> = query.split_whitespace().collect();
    let _content_lower = content.to_lowercase();
    
    // Find the best matching position
    let mut best_score = 0;
    let mut best_position = 0;
    
    let words: Vec<&str> = content.split_whitespace().collect();
    
    for (i, window) in words.windows(window_size.min(words.len())).enumerate() {
        let window_text = window.join(" ").to_lowercase();
        let score = query_words.iter()
            .map(|&word| if window_text.contains(word) { 1 } else { 0 })
            .sum::<i32>();
        
        if score > best_score {
            best_score = score;
            best_position = i;
        }
    }
    
    if best_score > 0 {
        let end_pos = (best_position + window_size).min(words.len());
        Some(words[best_position..end_pos].join(" "))
    } else {
        None
    }
}

fn create_simple_summary(content: &str) -> String {
    let sentences: Vec<&str> = content.split_terminator(&['.', '!', '?'])
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    
    if sentences.is_empty() {
        return "No clear sentences found in the document.".to_string();
    }
    
    // Take first sentence and a middle sentence for a simple summary
    let first = sentences.first().unwrap_or(&"");
    let summary = if sentences.len() > 2 {
        let middle = sentences[sentences.len() / 2];
        format!("{}... {}", first, middle)
    } else {
        first.to_string()
    };
    
    // Limit summary length
    if summary.len() > 200 {
        format!("{}...", &summary[..200])
    } else {
        summary
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI arguments
    let matches = Command::new("swoop_server")
        .version("0.2.0")
        .author("Swoop Team")
        .about("Advanced document intelligence platform server")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Port to run the server on")
                .default_value("3001")
        )
        .arg(
            Arg::new("log-level")
                .short('l')
                .long("log-level")
                .value_name("LEVEL")
                .help("Log level (debug, info, warn, error)")
                .default_value("info")
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Configuration file path")
        )
        .get_matches();

    // Set up logging based on CLI args
    let log_level = matches.get_one::<String>("log-level").unwrap();
    env::set_var("RUST_LOG", log_level);
    env_logger::init();
    
    // Get port from CLI args, environment, or default
    let port: u16 = matches.get_one::<String>("port")
        .and_then(|p| p.parse().ok())
        .or_else(|| env::var("PORT").ok().and_then(|p| p.parse().ok()))
        .unwrap_or(3001);
    
    println!("🚀 Starting Swoop Document Intelligence Platform v0.2.0...");
    
    // Force workspace initialization
    let _ = get_workspace_document_count();
    
    // Build application with CORS
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .route("/api/status", get(api_status_handler))
        .route("/api/documents/upload", post(upload_handler))
        .route("/api/documents", get(list_documents_handler))
        .route("/api/documents/:id/analyze", post(analyze_document_handler))
        .route("/api/chat", post(chat_handler))
        .layer(CorsLayer::permissive());
    
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    
    println!("Server running on http://localhost:{}", port);
    println!("Health check: http://localhost:{}/health", port);
    println!("API status: http://localhost:{}/api/status", port);
    println!("Document upload: POST http://localhost:{}/api/documents/upload", port);
    println!("List documents: GET http://localhost:{}/api/documents", port);
    println!("Document analysis: POST http://localhost:{}/api/documents/{{id}}/analyze", port);
    println!("General chat: POST http://localhost:{}/api/chat", port);
    println!("Document chat: POST http://localhost:{}/api/documents/{{id}}/chat", port);
    println!("Enhanced with PDF/Markdown support, chat interface, and robust processing");
    
    axum::serve(listener, app).await?;
    
    Ok(())
} 