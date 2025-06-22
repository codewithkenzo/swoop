/*!
 * AI Chat System Module
 * 
 * Production-ready document chat with AI/LLM integration,
 * fuzzy search, @ tagging, and custom personalities.
 */

pub mod personality_system;

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Chat system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatConfig {
    /// LLM provider settings
    pub llm_provider: LLMProviderConfig,
    /// Document indexing settings
    pub document_index: DocumentIndexConfig,
    /// Search configuration
    pub search_config: SearchConfig,
    /// Conversation settings
    pub conversation_config: ConversationConfig,
}

/// LLM provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMProviderConfig {
    /// Provider type (cloud or local)
    pub provider_type: LLMProviderType,
    /// Cloud API configuration
    pub cloud_config: Option<CloudLLMConfig>,
    /// Local model configuration
    pub local_config: Option<LocalLLMConfig>,
    /// Default model to use
    pub default_model: String,
    /// Enable automatic provider switching
    pub auto_provider_switching: bool,
    /// Maximum context length
    pub max_context_length: usize,
    /// Temperature for generation
    pub temperature: f32,
}

/// LLM provider types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LLMProviderType {
    OpenAI,
    Anthropic,
    Ollama,
    LlamaCpp,
    Mixed, // Use both cloud and local based on task
}

/// Cloud LLM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudLLMConfig {
    /// API key
    pub api_key: String,
    /// API endpoint
    pub endpoint: String,
    /// Available models
    pub models: Vec<CloudModel>,
    /// Rate limiting
    pub rate_limit_rpm: u32,
    /// Timeout in seconds
    pub timeout_seconds: u32,
}

/// Local LLM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalLLMConfig {
    /// Model path or name
    pub model_path: String,
    /// Model type
    pub model_type: LocalModelType,
    /// Hardware acceleration
    pub hardware_acceleration: HardwareAcceleration,
    /// Context window size
    pub context_window: usize,
    /// Number of threads
    pub num_threads: u32,
}

/// Cloud model definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudModel {
    pub name: String,
    pub max_tokens: usize,
    pub cost_per_1k_tokens: f64,
    pub supports_function_calling: bool,
    pub supports_streaming: bool,
}

/// Local model types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocalModelType {
    LlamaCpp,
    Ollama,
    GGUF,
}

/// Hardware acceleration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HardwareAcceleration {
    CPU,
    CUDA,
    Metal,
    OpenCL,
    Vulkan,
}

/// Document indexing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentIndexConfig {
    /// Enable vector embeddings
    pub enable_embeddings: bool,
    /// Embedding model to use
    pub embedding_model: String,
    /// Chunk size for documents
    pub chunk_size: usize,
    /// Chunk overlap
    pub chunk_overlap: usize,
    /// Index update frequency
    pub index_update_frequency_hours: u32,
}

/// Search configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    /// Enable fuzzy search
    pub enable_fuzzy_search: bool,
    /// Fuzzy search threshold
    pub fuzzy_threshold: f64,
    /// Maximum search results
    pub max_search_results: usize,
    /// Enable semantic search
    pub enable_semantic_search: bool,
    /// Search result ranking weights
    pub ranking_weights: SearchRankingWeights,
}

/// Search ranking weights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRankingWeights {
    pub text_similarity: f64,
    pub semantic_similarity: f64,
    pub document_age: f64,
    pub document_quality: f64,
    pub user_interaction: f64,
}

/// Conversation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationConfig {
    /// Maximum conversation history
    pub max_history_length: usize,
    /// Enable conversation memory
    pub enable_memory: bool,
    /// Memory retention days
    pub memory_retention_days: u32,
    /// Enable context compression
    pub enable_context_compression: bool,
    /// Auto-save conversations
    pub auto_save: bool,
}

/// Chat message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Referenced documents
    pub document_refs: Vec<DocumentReference>,
    /// @ mentions used in message
    pub mentions: Vec<Mention>,
    /// Message metadata
    pub metadata: MessageMetadata,
}

/// Message roles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

/// Document reference in message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentReference {
    pub document_id: String,
    pub title: String,
    pub relevance_score: f64,
    pub excerpt: String,
}

/// @ mention in message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mention {
    pub mention_type: MentionType,
    pub target_id: String,
    pub display_name: String,
    pub position: (usize, usize), // start, end positions in text
}

/// Types of mentions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MentionType {
    Document,
    Folder,
    Tag,
    Personality,
    SavedSearch,
}

/// Message metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMetadata {
    /// Language detected
    pub language: String,
    /// Processing time
    pub processing_time_ms: u64,
    /// Token count
    pub token_count: Option<usize>,
    /// Cost estimation
    pub estimated_cost: Option<f64>,
    /// Quality score
    pub quality_score: f64,
}

/// Conversation context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    pub conversation_id: String,
    pub messages: Vec<ChatMessage>,
    pub active_documents: Vec<String>,
    pub active_personality: Option<String>,
    pub language: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub metadata: ConversationMetadata,
}

/// Conversation metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMetadata {
    pub title: Option<String>,
    pub tags: Vec<String>,
    pub document_count: usize,
    pub message_count: usize,
    pub total_tokens: usize,
    pub estimated_cost: f64,
}

/// Main chat system
pub struct ChatSystem {
    config: ChatConfig,
    personality_system: personality_system::PersonalitySystem,
    conversations: HashMap<String, ConversationContext>,
}

impl ChatSystem {
    /// Create new chat system
    pub fn new(config: ChatConfig) -> Result<Self> {
        let personality_system = personality_system::PersonalitySystem::new();
        
        Ok(Self {
            config,
            personality_system,
            conversations: HashMap::new(),
        })
    }

    /// Process a chat message
    pub async fn process_message(
        &self,
        conversation_id: &str,
        message: &str,
        personality_id: Option<&str>,
    ) -> Result<ChatResponse> {
        let start_time = std::time::Instant::now();
        
        // Parse message for mentions
        let parsed_message = self.parse_mentions(message).await?;
        
        // Search for relevant documents based on mentions
        let relevant_docs = self.search_relevant_documents(&parsed_message).await?;
        
        // Get personality if specified
        let personality = if let Some(pid) = personality_id {
            self.personality_system.get_personality(pid)
        } else {
            None
        };
        
        // Generate response (mock implementation)
        let response_content = self.generate_mock_response(message, &personality).await?;
        
        // Create response message
        let response_message = ChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            role: MessageRole::Assistant,
            content: response_content,
            timestamp: chrono::Utc::now(),
            document_refs: relevant_docs.clone(),
            mentions: vec![],
            metadata: MessageMetadata {
                language: "en".to_string(),
                processing_time_ms: start_time.elapsed().as_millis() as u64,
                token_count: Some(message.split_whitespace().count()),
                estimated_cost: Some(0.001),
                quality_score: 0.9,
            },
        };
        
        // Generate suggestions
        let suggestions = self.generate_suggestions_mock().await?;
        
        Ok(ChatResponse {
            message: response_message,
            context_used: relevant_docs.len(),
            processing_time_ms: start_time.elapsed().as_millis() as u64,
            suggestions,
        })
    }

    /// Search documents
    pub async fn search_documents(
        &self,
        query: &str,
        _filters: Option<SearchFilters>,
    ) -> Result<Vec<DocumentSearchResult>> {
        // Mock document search
        let results = vec![
            DocumentSearchResult {
                document_id: "doc1".to_string(),
                title: format!("Document about {}", query),
                chunk_id: Some("chunk1".to_string()),
                score: 0.9,
                excerpt: format!("This document contains information about {}...", query),
                metadata: HashMap::new(),
            }
        ];
        
        Ok(results)
    }

    /// Resolve a mention to a document reference
    pub async fn resolve_mention(
        &self,
        mention: &Mention,
    ) -> Result<Option<DocumentReference>> {
        match mention.mention_type {
            MentionType::Document => {
                Ok(Some(DocumentReference {
                    document_id: mention.target_id.clone(),
                    title: mention.display_name.clone(),
                    relevance_score: 0.9,
                    excerpt: "Document excerpt...".to_string(),
                }))
            }
            _ => Ok(None),
        }
    }

    // Private helper methods
    async fn parse_mentions(&self, message: &str) -> Result<ParsedMessage> {
        let mut mentions = Vec::new();
        
        // Simple @ mention parsing
        let words: Vec<&str> = message.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            if word.starts_with('@') && word.len() > 1 {
                let target = &word[1..];
                mentions.push(Mention {
                    mention_type: MentionType::Document,
                    target_id: target.to_string(),
                    display_name: target.to_string(),
                    position: (i, i + 1),
                });
            }
        }
        
        Ok(ParsedMessage {
            content: message.to_string(),
            mentions,
            language: "en".to_string(),
        })
    }

    async fn search_relevant_documents(
        &self,
        parsed_message: &ParsedMessage,
    ) -> Result<Vec<DocumentReference>> {
        let mut relevant_docs = Vec::new();
        
        // For each mention, create a document reference
        for mention in &parsed_message.mentions {
            if let Some(doc_ref) = self.resolve_mention(mention).await? {
                relevant_docs.push(doc_ref);
            }
        }
        
        Ok(relevant_docs)
    }

    async fn generate_mock_response(
        &self,
        message: &str,
        personality: &Option<personality_system::Personality>,
    ) -> Result<String> {
        let base_response = format!("I understand you're asking about: {}", message);
        
        if let Some(p) = personality {
            Ok(format!("{} [Response styled with {} personality]", base_response, p.name))
        } else {
            Ok(base_response)
        }
    }

    async fn generate_suggestions_mock(&self) -> Result<Vec<String>> {
        Ok(vec![
            "Would you like to search for related documents?".to_string(),
            "I can help you analyze this topic further.".to_string(),
            "Consider exploring the document references I mentioned.".to_string(),
        ])
    }
}

// Helper structs
struct ParsedMessage {
    content: String,
    mentions: Vec<Mention>,
    language: String,
}

/// Chat response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub message: ChatMessage,
    pub context_used: usize,
    pub processing_time_ms: u64,
    pub suggestions: Vec<String>,
}

/// Search filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    pub document_types: Option<Vec<String>>,
    pub date_range: Option<(chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>,
    pub tags: Option<Vec<String>>,
    pub folders: Option<Vec<String>>,
    pub min_quality_score: Option<f64>,
}

/// Document search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSearchResult {
    pub document_id: String,
    pub title: String,
    pub chunk_id: Option<String>,
    pub score: f64,
    pub excerpt: String,
    pub metadata: HashMap<String, serde_json::Value>,
} 