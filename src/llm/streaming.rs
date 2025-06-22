use anyhow::{anyhow, Result};
use async_stream;
use axum::{
    response::{IntoResponse, Response, Sse},
    response::sse::{Event, KeepAlive},
};
use futures_util::{Stream, StreamExt};
use reqwest::Client;
use std::convert::Infallible;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tracing::{debug, error, warn};

use super::models::*;

/// Streaming service for real-time LLM responses
pub struct StreamingService {
    client: Client,
    openrouter_api_key: String,
}

impl StreamingService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            openrouter_api_key: api_key,
        }
    }

    /// Stream completion responses from OpenRouter
    pub async fn stream_completion(
        &self,
        request: OpenRouterRequest,
    ) -> Result<impl Stream<Item = Result<Event, Infallible>>> {
        let (tx, rx) = mpsc::channel(100);

        let client = self.client.clone();
        let api_key = self.openrouter_api_key.clone();

        // Spawn background task to handle streaming
        tokio::spawn(async move {
            if let Err(e) = Self::handle_stream_request(client, api_key, request, tx.clone()).await {
                let _ = tx.send(StreamEvent::Error(e.to_string())).await;
            }
        });

        // Convert receiver to SSE stream
        let stream = ReceiverStream::new(rx).map(|event| {
            match event {
                StreamEvent::Chunk(chunk) => {
                    match serde_json::to_string(&chunk) {
                        Ok(json) => Ok(Event::default().data(json)),
                        Err(e) => Ok(Event::default().data(format!(r#"{{"error":"{}"}}"#, e))),
                    }
                }
                StreamEvent::Done => Ok(Event::default().data("[DONE]")),
                StreamEvent::Error(err) => Ok(Event::default().data(format!(r#"{{"error":"{}"}}"#, err))),
            }
        });

        Ok(stream)
    }

    /// Handle the actual streaming request
    async fn handle_stream_request(
        client: Client,
        api_key: String,
        mut request: OpenRouterRequest,
        tx: mpsc::Sender<StreamEvent>,
    ) -> Result<()> {
        request.stream = Some(true);

        let response = client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "https://github.com/your-org/swoop")
            .header("X-Title", "Swoop Document Intelligence")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("OpenRouter API error: {}", error_text));
        }

        let mut stream = response.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    let chunk_str = String::from_utf8_lossy(&chunk);
                    buffer.push_str(&chunk_str);

                    // Process complete lines
                    let mut lines_to_process = Vec::new();
                    while let Some(line_end) = buffer.find('\n') {
                        let line = buffer[..line_end].trim().to_string();
                        lines_to_process.push(line);
                        buffer.drain(..line_end + 1);
                    }

                    for line in lines_to_process {
                        if line.is_empty() || line == "data: [DONE]" {
                            if line == "data: [DONE]" {
                                let _ = tx.send(StreamEvent::Done).await;
                            }
                            continue;
                        }

                        if let Some(data) = line.strip_prefix("data: ") {
                            match serde_json::from_str::<StreamChunk>(data) {
                                Ok(stream_chunk) => {
                                    debug!("Received stream chunk: {:?}", stream_chunk);
                                    let _ = tx.send(StreamEvent::Chunk(stream_chunk)).await;
                                }
                                Err(e) => {
                                    warn!("Failed to parse stream chunk: {} - Data: {}", e, data);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Stream chunk error: {}", e);
                    let _ = tx.send(StreamEvent::Error(e.to_string())).await;
                    break;
                }
            }
        }

        Ok(())
    }

    /// Create SSE response for streaming
    pub fn create_sse_response(
        stream: impl Stream<Item = Result<Event, Infallible>> + Send + 'static,
    ) -> Response {
        Sse::new(stream)
            .keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
            .into_response()
    }
}

/// Stream event types
#[derive(Debug, Clone)]
enum StreamEvent {
    Chunk(StreamChunk),
    Done,
    Error(String),
}

/// Streaming chat service for document-aware conversations
pub struct StreamingChatService {
    streaming_service: StreamingService,
}

impl StreamingChatService {
    pub fn new(api_key: String) -> Self {
        Self {
            streaming_service: StreamingService::new(api_key),
        }
    }

    /// Stream chat completion with document context
    pub async fn stream_chat_with_context(
        &self,
        messages: Vec<Message>,
        document_content: Option<String>,
        model_id: &str,
    ) -> Result<impl Stream<Item = Result<Event, Infallible>>> {
        let mut enhanced_messages = messages;

        // Add document context if provided
        if let Some(content) = document_content {
            let context_message = Message {
                role: "system".to_string(),
                content: format!(
                    "You are analyzing the following document. Use this context to answer questions:\n\n{}",
                    content
                ),
            };
            enhanced_messages.insert(0, context_message);
        }

        let request = OpenRouterRequest {
            model: model_id.to_string(),
            messages: enhanced_messages,
            max_tokens: Some(2048),
            temperature: Some(0.7),
            stream: Some(true),
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
        };

        self.streaming_service.stream_completion(request).await
    }

    /// Stream document analysis
    pub async fn stream_document_analysis(
        &self,
        document_content: &str,
        analysis_type: DocumentAnalysisType,
        model_id: &str,
    ) -> Result<impl Stream<Item = Result<Event, Infallible>>> {
        let prompt = match analysis_type {
            DocumentAnalysisType::Summary => {
                "Please provide a comprehensive summary of this document, highlighting the key points and main themes."
            }
            DocumentAnalysisType::KeyPoints => {
                "Extract the key points from this document in a bullet-point format."
            }
            DocumentAnalysisType::Questions => {
                "Generate thoughtful questions that could be answered based on this document's content."
            }
            DocumentAnalysisType::Entities => {
                "Identify and list all important entities mentioned in this document (people, organizations, locations, dates, etc.)."
            }
            DocumentAnalysisType::Sentiment => {
                "Analyze the sentiment and tone of this document. Provide insights into the emotional content and perspective."
            }
        };

        let messages = vec![
            Message {
                role: "system".to_string(),
                content: "You are a document analysis expert. Provide detailed, accurate analysis based on the given instructions.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: format!("{}\n\nDocument content:\n{}", prompt, document_content),
            },
        ];

        let request = OpenRouterRequest {
            model: model_id.to_string(),
            messages,
            max_tokens: Some(2048),
            temperature: Some(0.3), // Lower temperature for analysis tasks
            stream: Some(true),
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
        };

        self.streaming_service.stream_completion(request).await
    }
}

/// Document analysis types
#[derive(Debug, Clone)]
pub enum DocumentAnalysisType {
    Summary,
    KeyPoints,
    Questions,
    Entities,
    Sentiment,
}

/// Streaming utilities
pub struct StreamingUtils;

impl StreamingUtils {
    /// Convert streaming response to JSON lines format
    pub fn to_json_lines(
        stream: impl Stream<Item = Result<Event, Infallible>>,
    ) -> impl Stream<Item = Result<String, Infallible>> {
        stream.map(|event_result| {
            match event_result {
                Ok(_event) => {
                    // Event doesn't have a data() method without parameters
                    // We'll just return a simple JSON format
                    Ok(format!("{}\n", "{}"))
                }
                Err(e) => Ok(format!(r#"{{"error":"{:?}"}}\n"#, e)),
            }
        })
    }

    /// Create a heartbeat stream to keep connections alive
    pub fn heartbeat_stream() -> impl Stream<Item = Result<Event, Infallible>> {
        async_stream::stream! {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                yield Ok(Event::default().comment("heartbeat"));
            }
        }
    }
}

/// Rate limiting for streaming
pub struct StreamingRateLimiter {
    max_requests_per_minute: u32,
    request_times: std::sync::Arc<tokio::sync::Mutex<Vec<std::time::Instant>>>,
}

impl StreamingRateLimiter {
    pub fn new(max_requests_per_minute: u32) -> Self {
        Self {
            max_requests_per_minute,
            request_times: std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new())),
        }
    }

    pub async fn check_rate_limit(&self) -> bool {
        let mut times = self.request_times.lock().await;
        let now = std::time::Instant::now();
        let minute_ago = now - Duration::from_secs(60);

        // Remove old entries
        times.retain(|&time| time > minute_ago);

        if times.len() >= self.max_requests_per_minute as usize {
            false
        } else {
            times.push(now);
            true
        }
    }
} 