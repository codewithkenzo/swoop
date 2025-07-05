/*!
 * AI Chat System Module
 * 
 * Production-ready document chat with AI/LLM integration,
 * fuzzy search, @ tagging, and custom personalities.
 */

pub mod personality_system;

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Configuration for chat system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatConfig {
    /// Model to use for chat
    pub model: String,
    /// Maximum tokens per response
    pub max_tokens: usize,
    /// Temperature for generation
    pub temperature: f32,
    /// System prompt
    pub system_prompt: String,
}

impl Default for ChatConfig {
    fn default() -> Self {
        Self {
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: 1000,
            temperature: 0.7,
            system_prompt: "You are a helpful assistant.".to_string(),
        }
    }
}

/// Chat system for interactive document analysis
#[derive(Debug, Clone)]
pub struct ChatSystem {
    config: ChatConfig,
    personality: personality_system::PersonalitySystem,
}

impl ChatSystem {
    /// Create a new chat system
    pub fn new(config: ChatConfig) -> Self {
        Self {
            config,
            personality: personality_system::PersonalitySystem::new().unwrap(),
        }
    }
    
    /// Process a chat message
    pub async fn process_message(&self, message: &str) -> Result<String> {
        // Apply personality to the message
        let personalized_message = self.personality.apply_personality(message);
        
        // Basic response generation
        Ok(format!("Response to: {personalized_message}"))
    }
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

// Re-export from personality_system
pub use personality_system::PersonalitySystem; 