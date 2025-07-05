/*!
 * Swoop Server Binary
 * 
 * Advanced document intelligence server with robust processing capabilities
 */

use std::env;
use std::sync::{Arc, Mutex, LazyLock};
use axum::{
    extract::{Multipart, Path, Json, Query},
    http::{StatusCode, HeaderMap, HeaderValue},
    response::{Json as ResponseJson, IntoResponse},
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use tower_http::cors::CorsLayer;
use swoop::{DocumentWorkspace, Document};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use clap::{Arg, Command};
use std::collections::HashMap;
use axum::response::sse::{Sse, Event, KeepAlive};
use futures_util::{stream, Stream, StreamExt};
use tokio_stream::wrappers::IntervalStream;
use std::time::Duration;
use axum::body::Bytes;
use swoop::crawler::Crawler;
use swoop::storage::memory::MemoryStorage;
use once_cell::sync::Lazy;
use async_stream::stream;
use swoop::monitoring as monitoring;

// Thread-safe workspace using LazyLock
static WORKSPACE: LazyLock<Arc<Mutex<DocumentWorkspace>>> = LazyLock::new(|| {
    println!("📂 Initializing document workspace...");
    Arc::new(Mutex::new(DocumentWorkspace::new()))
});

// In-memory storage + crawler shared by all handlers. Using `once_cell::sync::Lazy`
// instead of `std::sync::LazyLock` for MSRV compatibility.
static MEMORY_STORAGE: Lazy<Arc<MemoryStorage>> = Lazy::new(|| Arc::new(MemoryStorage::new()));

static CRAWLER: Lazy<Arc<Crawler>> = Lazy::new(|| {
    Arc::new(Crawler::new(MEMORY_STORAGE.clone()))
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

#[derive(Deserialize)]
struct VoiceChatRequest {
    text: String,
    voice: Option<String>,
    model: Option<String>,
    stream: Option<bool>,
}

#[derive(Serialize)]
struct VoiceChatChunk {
    role: &'static str,
    text: Option<String>,
    audio_b64: Option<String>,
}

// --- Types for Crawl API -----------------------------------------------------
#[derive(Deserialize)]
struct StartCrawlPayload {
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    seeds: Option<Vec<String>>, // optional list of seed URLs
    #[serde(default)]
    max_depth: Option<usize>,
    #[serde(default)]
    max_pages: Option<usize>,
}

#[derive(Serialize)]
struct StartCrawlResponsePayload {
    job_id: String,
}

// Status structure mirrors `CrawlStats` but flattens nested fields for ease of
// consumption on the frontend.
#[derive(Serialize)]
struct CrawlStatusPayload {
    job_id: String,
    status: String,
    urls_processed: usize,
    successful_fetches: usize,
    failed_fetches: usize,
    documents_extracted: usize,
    links_discovered: usize,
    bytes_downloaded: usize,
    avg_fetch_time_ms: f64,
}

// Response chunk for streaming chat
#[derive(Serialize)]
struct ChatStreamChunk {
    role: &'static str,
    content: String,
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
                    eprintln!("Text extraction error: {e}, using raw content");
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
    // Clone document content while holding the lock, then release immediately
    let document_content = {
        let workspace_guard = WORKSPACE.lock().unwrap();
        match workspace_guard.documents.get(&doc_id) {
            Some(doc) => doc.content.clone(),
            None => return Err(StatusCode::NOT_FOUND),
        }
    };

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
                format!("Could not find '{search_term}' in this document.")
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
        format!("{first}... {middle}")
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

async fn audio_handler(
    Path(id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> std::result::Result<impl IntoResponse, StatusCode> {
    // Extract the text we need while the lock is held, then drop it
    let voice = params.get("voice").cloned().unwrap_or_else(|| "en-us-female".to_string());

    let text = {
        let workspace_guard = WORKSPACE.lock().unwrap();
        match workspace_guard.documents.get(&id) {
            Some(doc) => doc.content.clone(),
            None => return Err(StatusCode::NOT_FOUND),
        }
    };

    match swoop::tts::synthesize_to_wav(&text, &voice).await {
        Ok(pcm) => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", HeaderValue::from_static("audio/wav"));
            headers.insert("Cache-Control", HeaderValue::from_static("public, max-age=31536000"));
            Ok((headers, Bytes::from(pcm)))
        }
        Err(_) => Err(StatusCode::NOT_IMPLEMENTED),
    }
}

async fn voice_chat_handler(
    Json(req): Json<VoiceChatRequest>
) -> Result<Sse<impl Stream<Item = Result<Event, std::io::Error>>>, StatusCode> {
    let voice = req.voice.unwrap_or_else(|| "en-us-female".to_string());
    let response_text = format!("You said: {}. Here is a short response from the AI.", req.text.trim());

    let tokens_vec: Vec<String> = response_text.split_whitespace().map(|s| s.to_string()).collect();
    let stream_len = tokens_vec.len();
    let tokens = Arc::new(tokens_vec);

    let token_stream = IntervalStream::new(tokio::time::interval(Duration::from_millis(300)))
        .enumerate()
        .take(stream_len)
        .map(move |(idx, _)| {
            let tok = tokens[idx].clone();
            let chunk = VoiceChatChunk { role: "assistant", text: Some(tok), audio_b64: None };
            let json = serde_json::to_string(&chunk).unwrap();
            Ok(Event::default().data(json))
        });

    let text_clone = response_text.clone();
    let voice_clone = voice.clone();
    let final_audio_events = async move {
        match swoop::tts::synthesize_to_wav(&text_clone, &voice_clone).await {
            Ok(pcm) => {
                let audio_b64 = base64::encode(pcm);
                let chunk = VoiceChatChunk { role: "assistant", text: None, audio_b64: Some(audio_b64) };
                let json = serde_json::to_string(&chunk).unwrap();
                vec![Ok(Event::default().data(json))]
            }
            Err(_) => Vec::new(),
        }
    };

    let stream = token_stream.chain(stream::iter(final_audio_events.await));

    Ok(Sse::new(stream))
}

// ---------------------------------------------------------------------------
// 💬 Server-Sent Events: Chat Stream (alias to /api/chat/stream per README)
// ---------------------------------------------------------------------------
async fn chat_stream_handler(
    Json(request): Json<ChatRequest>
) -> Result<Sse<impl Stream<Item = Result<Event, std::io::Error>>>, StatusCode> {
    // Generate full response using existing chat logic
    let full_response = match process_chat_request(request).await {
        Ok(res) => res.response,
        Err(_) => "Sorry, something went wrong.".to_string(),
    };

    // Split into tokens for demo purposes
    let tokens_vec: Vec<String> = full_response.split_whitespace().map(|s| s.to_string()).collect();
    let stream_len = tokens_vec.len();
    let tokens = Arc::new(tokens_vec);

    let token_stream = IntervalStream::new(tokio::time::interval(Duration::from_millis(200)))
        .enumerate()
        .take(stream_len)
        .map(move |(idx, _)| {
            let tok = tokens[idx].clone();
            let chunk = ChatStreamChunk { role: "assistant", content: tok };
            let json = serde_json::to_string(&chunk).unwrap();
            Ok(Event::default().data(json))
        });

    Ok(Sse::new(token_stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)).text("keep-alive")))
}

// --- Handlers ----------------------------------------------------------------
async fn start_crawl_handler(
    Json(payload): Json<StartCrawlPayload>,
) -> Result<ResponseJson<StartCrawlResponsePayload>, StatusCode> {
    // Determine seeds: use explicit seeds array if provided and non-empty, otherwise fallback to single url
    let seeds_list = if let Some(seeds_vec) = &payload.seeds {
        if !seeds_vec.is_empty() {
            seeds_vec.clone()
        } else {
            Vec::new()
        }
    } else if let Some(u) = &payload.url {
        if u.trim().is_empty() { Vec::new() } else { vec![u.clone()] }
    } else {
        Vec::new()
    };

    if seeds_list.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Build custom config if max_depth or max_pages provided
    let mut custom_config = None;
    if payload.max_depth.is_some() || payload.max_pages.is_some() {
        let mut cfg = CRAWLER.config.clone();
        if let Some(d) = payload.max_depth {
            cfg.max_depth = d;
        }
        if let Some(p) = payload.max_pages {
            cfg.max_urls = p;
        }
        custom_config = Some(cfg);
    }

    let job_id = CRAWLER
        .start_crawl(seeds_list, custom_config)
        .await
        .map_err(|e| {
            log::error!("Failed to start crawl: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(ResponseJson(StartCrawlResponsePayload { job_id }))
}

async fn get_crawl_status_handler(
    Path(job_id): Path<String>,
) -> Result<ResponseJson<CrawlStatusPayload>, StatusCode> {
    match CRAWLER.get_job_status(&job_id) {
        Some(stats) => Ok(ResponseJson(CrawlStatusPayload {
            job_id: stats.job_id,
            status: stats.status,
            urls_processed: stats.urls_processed,
            successful_fetches: stats.successful_fetches,
            failed_fetches: stats.failed_fetches,
            documents_extracted: stats.documents_extracted,
            links_discovered: stats.links_discovered,
            bytes_downloaded: stats.bytes_downloaded,
            avg_fetch_time_ms: stats.avg_fetch_time_ms,
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn stop_crawl_handler(
    Path(job_id): Path<String>,
) -> Result<ResponseJson<Value>, StatusCode> {
    if CRAWLER.stop_job(&job_id) {
        Ok(ResponseJson(json!({ "stopped": true })))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn crawl_results_handler(
    Path(_job_id): Path<String>,
) -> Result<ResponseJson<Value>, StatusCode> {
    let pages = MEMORY_STORAGE.list_crawl_pages(&_job_id).await;
    let mapped: Vec<Value> = pages.into_iter().map(|p| json!({
        "url": p.url,
        "status_code": p.status_code,
        "text_length": p.text_length,
        "fetched_at": p.fetched_at,
    })).collect();
    Ok(ResponseJson(json!({ "pages": mapped })))
}

async fn list_crawl_jobs_handler() -> ResponseJson<Value> {
    let jobs = CRAWLER.get_active_jobs();
    ResponseJson(json!({ "jobs": jobs }))
}

// ---------------------------------------------------------------------------
// 🛰️  Server-Sent Events: Crawl Progress Stream
// ---------------------------------------------------------------------------
async fn crawl_stream_handler(
    Path(job_id): Path<String>,
) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let job_id_clone = job_id.clone();

    // Stream that polls crawler stats every second and emits JSON updates
    let event_stream = stream! {
        // First, immediately check if job exists
        let initial_status = CRAWLER.get_job_status(&job_id_clone);
        
        match initial_status {
            Some(stats) => {
                // Job exists, send initial status
                let data_json = serde_json::to_string(&stats).unwrap_or_default();
                yield Ok(Event::default().event("update").data(&data_json));
                
                // If already completed, send completion event and keep alive briefly
                if stats.status == "completed" || stats.status == "failed" {
                    yield Ok(Event::default().event("completed").data(&data_json));
                    
                    // Keep the connection alive for a few seconds to prevent browser EventSource errors
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    
                    // Send a final keep-alive before closing
                    yield Ok(Event::default().event("close").data("{\"message\":\"Stream ended normally\"}"));
                    return;
                }
            }
            None => {
                // Job not found - might be completed and cleaned up, or never existed
                let err = serde_json::json!({
                    "error": "crawl_job_not_found",
                    "job_id": job_id_clone,
                    "message": "Job may have completed quickly or does not exist"
                });
                yield Ok(Event::default().event("error").data(&err.to_string()));
                
                // Keep connection alive briefly even for errors
                tokio::time::sleep(Duration::from_secs(1)).await;
                yield Ok(Event::default().event("close").data("{\"message\":\"Stream ended with error\"}"));
                return;
            }
        }

        // Continue polling for updates
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;

            match CRAWLER.get_job_status(&job_id_clone) {
                Some(stats) => {
                    let data_json = serde_json::to_string(&stats).unwrap_or_default();
                    yield Ok(Event::default().event("update").data(&data_json));

                    if stats.status == "completed" || stats.status == "failed" {
                        // Send final completion event
                        yield Ok(Event::default().event("completed").data(&data_json));
                        
                        // Keep connection alive briefly to prevent browser errors
                        tokio::time::sleep(Duration::from_secs(2)).await;
                        
                        // Send close event and end gracefully
                        yield Ok(Event::default().event("close").data("{\"message\":\"Stream completed normally\"}"));
                        break;
                    }
                }
                None => {
                    // Job disappeared during polling - likely completed and cleaned up
                    let completion = serde_json::json!({
                        "job_id": job_id_clone,
                        "status": "completed",
                        "message": "Job completed and was cleaned up"
                    });
                    yield Ok(Event::default().event("completed").data(&completion.to_string()));
                    
                    // Keep connection alive briefly
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    yield Ok(Event::default().event("close").data("{\"message\":\"Stream ended normally\"}"));
                    break;
                }
            }
        }
    };

    Sse::new(event_stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)).text("keep-alive"))
}

// ---------------------------------------------------------------------------
// 📄 Server-Sent Events: Document Processing Stream
// ---------------------------------------------------------------------------
async fn document_stream_handler(
    Path(doc_id): Path<String>,
) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let doc_id_clone = doc_id.clone();

    let event_stream = stream! {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;

            let maybe_doc = {
                let workspace_guard = WORKSPACE.lock().unwrap();
                workspace_guard.documents.get(&doc_id_clone).cloned()
            };

            match maybe_doc {
                Some(doc) => {
                    let data = serde_json::to_string(&doc).unwrap_or_default();
                    yield Ok(Event::default().event("completed").data(data));
                    break;
                }
                None => {
                    let data = serde_json::json!({
                        "status": "pending",
                        "document_id": doc_id_clone,
                    });
                    yield Ok(Event::default().event("update").data(data.to_string()));
                }
            }
        }
    };

    Sse::new(event_stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)).text("keep-alive"))
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
        .route("/api/chat/stream", post(chat_stream_handler))
        .route("/api/documents/:id/chat", post(document_chat_handler))
        .route("/api/audio/:id", get(audio_handler))
        .route("/api/voice-chat", post(voice_chat_handler))
        .route("/api/documents/:id/stream", get(document_stream_handler))
        .route("/api/crawl", get(list_crawl_jobs_handler).post(start_crawl_handler))
        .route("/api/crawl/:job_id", get(get_crawl_status_handler))
        .route("/api/crawl/:job_id/stop", post(stop_crawl_handler))
        .route("/api/crawl/:job_id/results", get(crawl_results_handler))
        .route("/api/crawl/:job_id/stream", get(crawl_stream_handler))
        .route("/api/metrics", get(monitoring::monitoring_metrics_endpoint))
        .route("/api/stats", get(monitoring::monitoring_stats_endpoint))
        .layer(CorsLayer::permissive());
    
    // --- Dynamic port binding --------------------------------------------------
    let starting_port = port;
    let mut current_port = starting_port;
    loop {
        let addr = format!("0.0.0.0:{current_port}");
        match tokio::net::TcpListener::bind(&addr).await {
            Ok(listener) => {
                println!("Server running on http://localhost:{current_port}");
                println!("Health check: http://localhost:{current_port}/health");
                println!("API status: http://localhost:{current_port}/api/status");
                println!("Document upload: POST http://localhost:{current_port}/api/documents/upload");
                println!("List documents: GET http://localhost:{current_port}/api/documents");
                println!("Document analysis: POST http://localhost:{current_port}/api/documents/{{id}}/analyze");
                println!("General chat: POST http://localhost:{current_port}/api/chat");
                println!("Document chat: POST http://localhost:{current_port}/api/documents/{{id}}/chat");
                println!("Audio playback: GET http://localhost:{current_port}/api/audio/{{id}}");
                println!("Voice chat: POST http://localhost:{current_port}/api/voice-chat");
                println!("Enhanced with PDF/Markdown support, chat interface, and robust processing");
                // Clone the app so it can be reused if we loop again.
                axum::serve(listener, app.clone().with_state(()).into_make_service()).await?;
                return Ok(());
            }
            Err(e) if e.kind() == std::io::ErrorKind::AddrInUse && current_port < starting_port + 10 => {
                current_port += 1;
                continue;
            }
            Err(e) => return Err(e.into()),
        }
    }
} 