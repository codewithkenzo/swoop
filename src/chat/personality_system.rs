/*!
 * AI Personality System
 * 
 * Custom personalities for different use cases, languages, and user needs
 */

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Personality system manager
pub struct PersonalitySystem {
    personalities: HashMap<String, Personality>,
    default_personalities: Vec<Personality>,
}

/// AI Personality definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personality {
    pub id: String,
    pub name: String,
    pub description: String,
    pub language: String,
    pub personality_type: PersonalityType,
    pub traits: PersonalityTraits,
    pub behavior: BehaviorSettings,
    pub prompts: PersonalityPrompts,
    pub context_settings: ContextSettings,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Types of personalities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersonalityType {
    /// Professional assistant for business
    Professional,
    /// Academic researcher and analyst
    Academic,
    /// Creative writing assistant
    Creative,
    /// Technical expert and developer
    Technical,
    /// Casual conversational partner
    Casual,
    /// Educational tutor
    Educational,
    /// Legal assistant
    Legal,
    /// Medical information assistant
    Medical,
    /// Customer service representative
    CustomerService,
    /// Custom personality type
    Custom(String),
}

/// Personality traits configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTraits {
    /// Formality level (0.0 = very casual, 1.0 = very formal)
    pub formality: f32,
    /// Enthusiasm level (0.0 = subdued, 1.0 = very enthusiastic)
    pub enthusiasm: f32,
    /// Detail level (0.0 = brief, 1.0 = very detailed)
    pub detail_level: f32,
    /// Empathy level (0.0 = analytical, 1.0 = very empathetic)
    pub empathy: f32,
    /// Confidence level (0.0 = uncertain, 1.0 = very confident)
    pub confidence: f32,
    /// Humor level (0.0 = serious, 1.0 = very humorous)
    pub humor: f32,
    /// Patience level (0.0 = direct, 1.0 = very patient)
    pub patience: f32,
}

/// Behavior settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorSettings {
    /// Always ask for clarification when uncertain
    pub ask_for_clarification: bool,
    /// Provide sources and citations
    pub cite_sources: bool,
    /// Use examples in explanations
    pub use_examples: bool,
    /// Offer alternative perspectives
    pub offer_alternatives: bool,
    /// Follow up with related questions
    pub suggest_followups: bool,
    /// Use technical terminology
    pub use_technical_terms: bool,
    /// Provide step-by-step instructions
    pub step_by_step: bool,
    /// Include warnings and caveats
    pub include_warnings: bool,
}

/// Personality-specific prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityPrompts {
    /// System prompt for the personality
    pub system_prompt: String,
    /// Greeting message
    pub greeting: String,
    /// How to handle uncertainty
    pub uncertainty_response: String,
    /// How to conclude conversations
    pub conclusion_style: String,
    /// Response when no documents are found
    pub no_documents_response: String,
    /// Custom instructions for document analysis
    pub document_analysis_style: String,
}

/// Context handling settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSettings {
    /// Maximum context length to use
    pub max_context_length: usize,
    /// Prioritize recent messages
    pub prioritize_recent: bool,
    /// Include document metadata in responses
    pub include_metadata: bool,
    /// Summarize long documents
    pub summarize_long_docs: bool,
    /// Focus on specific document sections
    pub focus_sections: Vec<String>,
}

impl PersonalitySystem {
    /// Create new personality system
    pub fn new() -> Result<Self> {
        let mut system = Self {
            personalities: HashMap::new(),
            default_personalities: Vec::new(),
        };
        
        system.load_default_personalities()?;
        Ok(system)
    }
    
    /// Load default personalities
    fn load_default_personalities(&mut self) -> Result<()> {
        let defaults = vec![
            self.create_professional_personality(),
            self.create_technical_personality(),
            self.create_casual_personality(),
        ];
        
        for personality in defaults {
            self.personalities.insert(personality.id.clone(), personality.clone());
            self.default_personalities.push(personality);
        }
        
        Ok(())
    }
    
    /// Get personality by ID
    pub async fn get_personality(&self, personality_id: &str) -> Result<Personality> {
        self.personalities
            .get(personality_id)
            .cloned()
            .ok_or_else(|| crate::error::Error::Other(format!("Personality not found: {}", personality_id)))
    }
    
    /// List all available personalities
    pub fn list_personalities(&self) -> Vec<&Personality> {
        self.personalities.values().collect()
    }
    
    /// Create custom personality
    pub fn create_custom_personality(&mut self, personality: Personality) -> Result<()> {
        self.personalities.insert(personality.id.clone(), personality);
        Ok(())
    }
    
    /// Get personality for language
    pub fn get_personality_for_language(&self, language: &str, personality_type: PersonalityType) -> Option<&Personality> {
        self.personalities.values().find(|p| {
            p.language == language && 
            std::mem::discriminant(&p.personality_type) == std::mem::discriminant(&personality_type)
        })
    }
    
    // Default personality creators
    fn create_professional_personality(&self) -> Personality {
        Personality {
            id: "professional_en".to_string(),
            name: "Professional Assistant".to_string(),
            description: "Formal, efficient business assistant".to_string(),
            language: "en".to_string(),
            personality_type: PersonalityType::Professional,
            traits: PersonalityTraits {
                formality: 0.8,
                enthusiasm: 0.5,
                detail_level: 0.7,
                empathy: 0.6,
                confidence: 0.8,
                humor: 0.2,
                patience: 0.7,
            },
            behavior: BehaviorSettings {
                ask_for_clarification: true,
                cite_sources: true,
                use_examples: true,
                offer_alternatives: true,
                suggest_followups: true,
                use_technical_terms: true,
                step_by_step: true,
                include_warnings: true,
            },
            prompts: PersonalityPrompts {
                system_prompt: "You are a professional business assistant.".to_string(),
                greeting: "Good day! How may I assist you?".to_string(),
                uncertainty_response: "Let me clarify the details.".to_string(),
                conclusion_style: "Is there anything else I can help with?".to_string(),
                no_documents_response: "I don't have specific documents for this query.".to_string(),
                document_analysis_style: "I'll analyze the key points systematically.".to_string(),
            },
            context_settings: ContextSettings {
                max_context_length: 8000,
                prioritize_recent: true,
                include_metadata: true,
                summarize_long_docs: true,
                focus_sections: vec!["executive_summary".to_string()],
            },
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    
    fn create_technical_personality(&self) -> Personality {
        Personality {
            id: "technical_en".to_string(),
            name: "Technical Expert".to_string(),
            description: "Precise technical specialist".to_string(),
            language: "en".to_string(),
            personality_type: PersonalityType::Technical,
            traits: PersonalityTraits {
                formality: 0.6,
                enthusiasm: 0.5,
                detail_level: 0.9,
                empathy: 0.4,
                confidence: 0.9,
                humor: 0.3,
                patience: 0.6,
            },
            behavior: BehaviorSettings {
                ask_for_clarification: true,
                cite_sources: true,
                use_examples: true,
                offer_alternatives: true,
                suggest_followups: true,
                use_technical_terms: true,
                step_by_step: true,
                include_warnings: true,
            },
            prompts: PersonalityPrompts {
                system_prompt: "You are a technical expert and developer assistant.".to_string(),
                greeting: "Hello! I'm here to help with technical challenges.".to_string(),
                uncertainty_response: "Let me understand the technical requirements.".to_string(),
                conclusion_style: "Need help with implementation details?".to_string(),
                no_documents_response: "I can help based on best practices.".to_string(),
                document_analysis_style: "I'll analyze the technical specifications.".to_string(),
            },
            context_settings: ContextSettings {
                max_context_length: 10000,
                prioritize_recent: true,
                include_metadata: true,
                summarize_long_docs: false,
                focus_sections: vec!["implementation".to_string(), "examples".to_string()],
            },
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    
    fn create_casual_personality(&self) -> Personality {
        Personality {
            id: "casual_en".to_string(),
            name: "Friendly Companion".to_string(),
            description: "Relaxed, conversational chat partner".to_string(),
            language: "en".to_string(),
            personality_type: PersonalityType::Casual,
            traits: PersonalityTraits {
                formality: 0.2,
                enthusiasm: 0.7,
                detail_level: 0.5,
                empathy: 0.8,
                confidence: 0.6,
                humor: 0.7,
                patience: 0.8,
            },
            behavior: BehaviorSettings {
                ask_for_clarification: true,
                cite_sources: false,
                use_examples: true,
                offer_alternatives: false,
                suggest_followups: true,
                use_technical_terms: false,
                step_by_step: false,
                include_warnings: false,
            },
            prompts: PersonalityPrompts {
                system_prompt: "You are a friendly, casual conversation partner.".to_string(),
                greeting: "Hey there! What's on your mind?".to_string(),
                uncertainty_response: "Hmm, want to give me more context?".to_string(),
                conclusion_style: "Hope that helps! 😊".to_string(),
                no_documents_response: "I don't have those docs handy.".to_string(),
                document_analysis_style: "Let me check out what you've got here.".to_string(),
            },
            context_settings: ContextSettings {
                max_context_length: 4000,
                prioritize_recent: true,
                include_metadata: false,
                summarize_long_docs: true,
                focus_sections: vec!["summary".to_string()],
            },
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
} 