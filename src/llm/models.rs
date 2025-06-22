use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// OpenRouter API request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenRouterRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

/// Message structure for chat completions
#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Internal completion request with additional metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequest {
    pub user_id: String,
    pub messages: Vec<Message>,
    pub model_preference: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub stream: bool,
    pub document_context: Vec<String>,
    pub task_category: TaskCategory,
    pub priority: RequestPriority,
}

/// Task categories for model routing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TaskCategory {
    DocumentAnalysis,
    Summarization,
    QuestionAnswering,
    CodeGeneration,
    Translation,
    CreativeWriting,
    DataExtraction,
    Classification,
    General,
}

/// Request priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequestPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// User tier for model access and cost management
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserTier {
    Free {
        daily_limit: u32,
        models: Vec<String>,
    },
    Basic {
        daily_limit: u32,
        cost_limit: f64,
        models: Vec<String>,
    },
    Premium {
        daily_limit: u32,
        cost_limit: f64,
        models: Vec<String>,
        priority_access: bool,
    },
    Enterprise {
        cost_limit: f64,
        models: Vec<String>,
        dedicated_resources: bool,
        custom_models: Vec<String>,
    },
}

impl UserTier {
    pub fn can_access_model(&self, model_id: &str) -> bool {
        match self {
            UserTier::Free { models, .. } => models.contains(&model_id.to_string()),
            UserTier::Basic { models, .. } => models.contains(&model_id.to_string()),
            UserTier::Premium { models, .. } => models.contains(&model_id.to_string()),
            UserTier::Enterprise { models, custom_models, .. } => {
                models.contains(&model_id.to_string()) || custom_models.contains(&model_id.to_string())
            }
        }
    }

    pub fn get_cost_limit(&self) -> f64 {
        match self {
            UserTier::Free { .. } => 0.0,
            UserTier::Basic { cost_limit, .. } => *cost_limit,
            UserTier::Premium { cost_limit, .. } => *cost_limit,
            UserTier::Enterprise { cost_limit, .. } => *cost_limit,
        }
    }

    pub fn get_daily_limit(&self) -> Option<u32> {
        match self {
            UserTier::Free { daily_limit, .. } => Some(*daily_limit),
            UserTier::Basic { daily_limit, .. } => Some(*daily_limit),
            UserTier::Premium { daily_limit, .. } => Some(*daily_limit),
            UserTier::Enterprise { .. } => None, // Unlimited
        }
    }
}

/// OpenRouter completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub id: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
    pub model: String,
    pub created: u64,
}

/// Choice in completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
    pub finish_reason: Option<String>,
    pub index: u32,
}

/// Usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
}

/// Streaming chunk response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    pub id: String,
    pub choices: Vec<StreamChoice>,
    pub usage: Option<Usage>,
    pub model: String,
    pub created: u64,
}

/// Streaming choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChoice {
    pub delta: StreamDelta,
    pub finish_reason: Option<String>,
    pub index: u32,
}

/// Streaming delta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamDelta {
    pub role: Option<String>,
    pub content: Option<String>,
}

/// OpenRouter models response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsResponse {
    pub data: Vec<ModelInfo>,
}

/// Model information from OpenRouter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub context_length: u32,
    pub architecture: Option<ModelArchitecture>,
    pub pricing: Option<ModelPricing>,
    pub top_provider: Option<ProviderInfo>,
    pub per_request_limits: Option<RequestLimits>,
}

/// Model architecture information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelArchitecture {
    pub modality: String,
    pub tokenizer: String,
    pub instruct_type: Option<String>,
}

/// Model pricing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricing {
    pub prompt: String,
    pub completion: String,
    pub image: Option<String>,
    pub request: Option<String>,
}

/// Provider information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    pub context_length: u32,
    pub max_completion_tokens: Option<u32>,
    pub is_moderated: bool,
}

/// Request limits per model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLimits {
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
}

/// Model registry for managing available models
#[derive(Debug, Clone)]
pub struct ModelRegistry {
    pub models: HashMap<String, ModelInfo>,
    pub models_by_category: HashMap<TaskCategory, Vec<String>>,
    pub free_models: Vec<String>,
    pub premium_models: Vec<String>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
            models_by_category: HashMap::new(),
            free_models: Vec::new(),
            premium_models: Vec::new(),
        }
    }

    pub fn add_model(&mut self, model: ModelInfo) {
        let is_free = model.pricing.as_ref()
            .map(|p| p.prompt == "0" && p.completion == "0")
            .unwrap_or(false);

        if is_free {
            self.free_models.push(model.id.clone());
        } else {
            self.premium_models.push(model.id.clone());
        }

        // Categorize models based on their capabilities
        self.categorize_model(&model);
        
        self.models.insert(model.id.clone(), model);
    }

    fn categorize_model(&mut self, model: &ModelInfo) {
        let model_id = model.id.clone();
        let name_lower = model.name.to_lowercase();
        let desc_lower = model.description.as_ref()
            .map(|d| d.to_lowercase())
            .unwrap_or_default();

        // Categorize based on model name and description
        if name_lower.contains("code") || desc_lower.contains("code") {
            self.add_to_category(TaskCategory::CodeGeneration, model_id.clone());
        }
        
        if name_lower.contains("vision") || name_lower.contains("vl") {
            self.add_to_category(TaskCategory::DocumentAnalysis, model_id.clone());
        }
        
        if desc_lower.contains("summarization") || desc_lower.contains("summary") {
            self.add_to_category(TaskCategory::Summarization, model_id.clone());
        }
        
        if desc_lower.contains("translation") || name_lower.contains("translate") {
            self.add_to_category(TaskCategory::Translation, model_id.clone());
        }
        
        if desc_lower.contains("creative") || desc_lower.contains("writing") {
            self.add_to_category(TaskCategory::CreativeWriting, model_id.clone());
        }
        
        // Default to general category
        self.add_to_category(TaskCategory::General, model_id);
    }

    fn add_to_category(&mut self, category: TaskCategory, model_id: String) {
        self.models_by_category
            .entry(category)
            .or_insert_with(Vec::new)
            .push(model_id);
    }

    pub fn get_models_for_category(&self, category: &TaskCategory) -> Vec<&ModelInfo> {
        self.models_by_category
            .get(category)
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|id| self.models.get(id))
            .collect()
    }

    pub fn get_model(&self, model_id: &str) -> Option<&ModelInfo> {
        self.models.get(model_id)
    }

    pub fn get_free_models(&self) -> Vec<&ModelInfo> {
        self.free_models
            .iter()
            .filter_map(|id| self.models.get(id))
            .collect()
    }

    pub fn get_premium_models(&self) -> Vec<&ModelInfo> {
        self.premium_models
            .iter()
            .filter_map(|id| self.models.get(id))
            .collect()
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
} 