use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::extractors;
use crate::llm::{LLMService, models::*};

/// Advanced document processor with LLM integration
pub struct DocumentProcessor {
    llm_service: Option<LLMService>,
}

impl DocumentProcessor {
    pub fn new(llm_service: Option<LLMService>) -> Self {
        Self { llm_service }
    }

    /// Process document with comprehensive analysis
    pub async fn process_document(&self, file_path: &Path, content: &[u8]) -> Result<ProcessedDocument> {
        let start_time = std::time::Instant::now();
        
        // Extract basic content
        let extracted_content = self.extract_content(file_path, content).await?;
        
        // Perform LLM-powered analysis if available
        let analysis = if let Some(llm_service) = &self.llm_service {
            Some(self.analyze_with_llm(llm_service, &extracted_content).await?)
        } else {
            None
        };
        
        let processing_time = start_time.elapsed();
        
        Ok(ProcessedDocument {
            id: uuid::Uuid::new_v4().to_string(),
            filename: file_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            content: extracted_content,
            analysis,
            metadata: DocumentMetadata {
                file_size: content.len(),
                processing_time_ms: processing_time.as_millis() as u64,
                created_at: chrono::Utc::now(),
                content_hash: self.calculate_content_hash(content),
            },
        })
    }

    /// Extract content using existing extractors
    async fn extract_content(&self, file_path: &Path, content: &[u8]) -> Result<ExtractedContent> {
        let extension = file_path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        let document_format = match extension.as_str() {
            "pdf" => DocumentFormat::PDF,
            "md" | "markdown" => DocumentFormat::Markdown,
            "html" | "htm" => DocumentFormat::HTML,
            "txt" => DocumentFormat::PlainText,
            _ => DocumentFormat::PlainText,
        };

        // Use existing extractors - the main extract_text function
        let filename = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        
        let content_type = match document_format {
            DocumentFormat::PDF => "application/pdf",
            DocumentFormat::HTML => "text/html",
            DocumentFormat::Markdown => "text/markdown",
            _ => "text/plain",
        };

        let extracted = extractors::extract_text(content, content_type, filename)
            .map_err(|e| anyhow!("Content extraction failed: {}", e))?;

        let text_clone = extracted.text.clone();
        Ok(ExtractedContent {
            text: extracted.text,
            format: document_format,
            word_count: extracted.word_count,
            char_count: extracted.character_count,
            line_count: text_clone.lines().count(),
            language: self.detect_language(&text_clone),
            quality_score: self.calculate_quality_score(extracted.word_count, extracted.character_count, text_clone.lines().count()),
        })
    }

    /// Analyze document using LLM
    async fn analyze_with_llm(&self, llm_service: &LLMService, content: &ExtractedContent) -> Result<DocumentAnalysis> {
        // Execute analysis tasks sequentially to avoid type issues
        let summary = self.generate_summary(llm_service, &content.text).await?;
        let key_points_str = self.extract_key_points(llm_service, &content.text).await?;
        let entities_str = self.extract_entities(llm_service, &content.text).await?;
        let sentiment_str = self.analyze_sentiment(llm_service, &content.text).await?;
        let topics_str = self.extract_topics(llm_service, &content.text).await?;

        let results = vec![summary, key_points_str, entities_str, sentiment_str, topics_str];
        
        Ok(DocumentAnalysis {
            summary: results[0].clone(),
            key_points: serde_json::from_str(&results[1])
                .unwrap_or_else(|_| vec!["Failed to extract key points".to_string()]),
            entities: serde_json::from_str(&results[2])
                .unwrap_or_else(|_| HashMap::new()),
            sentiment: self.parse_sentiment(&results[3]),
            topics: serde_json::from_str(&results[4])
                .unwrap_or_else(|_| vec!["general".to_string()]),
            confidence_score: self.calculate_analysis_confidence(&results),
        })
    }

    /// Generate document summary
    async fn generate_summary(&self, llm_service: &LLMService, text: &str) -> Result<String> {
        let request = CompletionRequest {
            user_id: "system".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "You are a document analysis expert. Provide a clear, concise summary of the given text. Focus on the main points and key information.".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: format!("Please summarize this document:\n\n{}", text.chars().take(4000).collect::<String>()),
                },
            ],
            model_preference: None,
            max_tokens: Some(256),
            temperature: Some(0.3),
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            stream: false,
            document_context: vec![],
            task_category: TaskCategory::Summarization,
            priority: RequestPriority::Normal,
        };

        let response = llm_service.complete(request).await?;
        Ok(response.choices.first()
            .map(|choice| choice.message.content.clone())
            .unwrap_or_else(|| "Summary generation failed".to_string()))
    }

    /// Extract key points
    async fn extract_key_points(&self, llm_service: &LLMService, text: &str) -> Result<String> {
        let request = CompletionRequest {
            user_id: "system".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "Extract the key points from the given text. Return as a JSON array of strings.".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: format!("Extract key points from:\n\n{}", text.chars().take(4000).collect::<String>()),
                },
            ],
            model_preference: None,
            max_tokens: Some(512),
            temperature: Some(0.2),
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            stream: false,
            document_context: vec![],
            task_category: TaskCategory::DocumentAnalysis,
            priority: RequestPriority::Normal,
        };

        let response = llm_service.complete(request).await?;
        Ok(response.choices.first()
            .map(|choice| choice.message.content.clone())
            .unwrap_or_else(|| "[]".to_string()))
    }

    /// Extract entities
    async fn extract_entities(&self, llm_service: &LLMService, text: &str) -> Result<String> {
        let request = CompletionRequest {
            user_id: "system".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "Extract named entities from the text. Return as JSON object with categories: people, organizations, locations, dates, other.".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: format!("Extract entities from:\n\n{}", text.chars().take(4000).collect::<String>()),
                },
            ],
            model_preference: None,
            max_tokens: Some(512),
            temperature: Some(0.1),
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            stream: false,
            document_context: vec![],
            task_category: TaskCategory::DocumentAnalysis,
            priority: RequestPriority::Normal,
        };

        let response = llm_service.complete(request).await?;
        Ok(response.choices.first()
            .map(|choice| choice.message.content.clone())
            .unwrap_or_else(|| "{}".to_string()))
    }

    /// Analyze sentiment
    async fn analyze_sentiment(&self, llm_service: &LLMService, text: &str) -> Result<String> {
        let request = CompletionRequest {
            user_id: "system".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "Analyze the sentiment of the text. Return: positive, negative, or neutral.".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: format!("Analyze sentiment of:\n\n{}", text.chars().take(2000).collect::<String>()),
                },
            ],
            model_preference: None,
            max_tokens: Some(50),
            temperature: Some(0.1),
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            stream: false,
            document_context: vec![],
            task_category: TaskCategory::DocumentAnalysis,
            priority: RequestPriority::Normal,
        };

        let response = llm_service.complete(request).await?;
        Ok(response.choices.first()
            .map(|choice| choice.message.content.clone())
            .unwrap_or_else(|| "neutral".to_string()))
    }

    /// Extract topics
    async fn extract_topics(&self, llm_service: &LLMService, text: &str) -> Result<String> {
        let request = CompletionRequest {
            user_id: "system".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "Identify the main topics discussed in the text. Return as JSON array of topic strings.".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: format!("Identify topics in:\n\n{}", text.chars().take(3000).collect::<String>()),
                },
            ],
            model_preference: None,
            max_tokens: Some(256),
            temperature: Some(0.2),
            top_p: None,
            frequency_penalty: None,
            presence_penalty: None,
            stream: false,
            document_context: vec![],
            task_category: TaskCategory::DocumentAnalysis,
            priority: RequestPriority::Normal,
        };

        let response = llm_service.complete(request).await?;
        Ok(response.choices.first()
            .map(|choice| choice.message.content.clone())
            .unwrap_or_else(|| r#"["general"]"#.to_string()))
    }

    /// Extract document structure
    fn extract_structure(&self, text: &str, format: &DocumentFormat) -> DocumentStructure {
        match format {
            DocumentFormat::Markdown => self.extract_markdown_structure(text),
            DocumentFormat::HTML => self.extract_html_structure(text),
            _ => self.extract_generic_structure(text),
        }
    }

    /// Extract markdown structure
    fn extract_markdown_structure(&self, text: &str) -> DocumentStructure {
        let mut headings = Vec::new();
        let mut sections = Vec::new();
        let mut current_section = String::new();
        let mut current_heading: Option<String> = None;

        for line in text.lines() {
            if line.starts_with('#') {
                // Save previous section
                if !current_section.is_empty() {
                    sections.push(DocumentSection {
                        title: current_heading.clone().unwrap_or_else(|| "Untitled".to_string()),
                        content: current_section.trim().to_string(),
                        level: 1,
                    });
                    current_section.clear();
                }

                // Extract heading
                let level = line.chars().take_while(|&c| c == '#').count();
                let title = line.trim_start_matches('#').trim().to_string();
                
                headings.push(DocumentHeading {
                    title: title.clone(),
                    level,
                });
                
                current_heading = Some(title);
            } else {
                current_section.push_str(line);
                current_section.push('\n');
            }
        }

        // Save last section
        if !current_section.is_empty() {
            sections.push(DocumentSection {
                title: current_heading.unwrap_or_else(|| "Untitled".to_string()),
                content: current_section.trim().to_string(),
                level: 1,
            });
        }

        DocumentStructure {
            headings: headings.clone(),
            sections,
            has_table_of_contents: headings.len() > 2,
            estimated_reading_time_minutes: (text.split_whitespace().count() / 200) as u32,
        }
    }

    /// Extract HTML structure
    fn extract_html_structure(&self, text: &str) -> DocumentStructure {
        // Simple HTML structure extraction
        let mut headings = Vec::new();
        let heading_regex = regex::Regex::new(r"<h([1-6])[^>]*>(.*?)</h[1-6]>").unwrap();
        
        for cap in heading_regex.captures_iter(text) {
            if let (Some(level_str), Some(title)) = (cap.get(1), cap.get(2)) {
                if let Ok(level) = level_str.as_str().parse::<usize>() {
                    headings.push(DocumentHeading {
                        title: title.as_str().to_string(),
                        level,
                    });
                }
            }
        }

        DocumentStructure {
            headings,
            sections: vec![], // Would need more complex parsing
            has_table_of_contents: false,
            estimated_reading_time_minutes: (text.split_whitespace().count() / 200) as u32,
        }
    }

    /// Extract generic structure
    fn extract_generic_structure(&self, text: &str) -> DocumentStructure {
        let paragraphs: Vec<&str> = text.split("\n\n").filter(|p| !p.trim().is_empty()).collect();
        
        DocumentStructure {
            headings: vec![],
            sections: vec![DocumentSection {
                title: "Content".to_string(),
                content: text.to_string(),
                level: 1,
            }],
            has_table_of_contents: false,
            estimated_reading_time_minutes: (text.split_whitespace().count() / 200) as u32,
        }
    }

    /// Detect document language
    fn detect_language(&self, text: &str) -> String {
        let sample = text.chars().take(1000).collect::<String>().to_lowercase();
        
        if sample.contains("the ") && sample.contains(" and ") {
            "en".to_string()
        } else {
            "unknown".to_string()
        }
    }

    /// Calculate content quality score
    fn calculate_quality_score(&self, word_count: usize, char_count: usize, line_count: usize) -> f32 {
        let mut score: f32 = 0.5; // Base score
        
        // Word count factor
        if word_count > 100 {
            score += 0.2;
        }
        if word_count > 500 {
            score += 0.1;
        }
        
        // Character to word ratio (average word length)
        let avg_word_length = if word_count > 0 {
            char_count as f32 / word_count as f32
        } else {
            0.0
        };
        
        if avg_word_length >= 4.0 && avg_word_length <= 8.0 {
            score += 0.1; // Good average word length
        }
        
        // Line density
        let words_per_line = if line_count > 0 {
            word_count as f32 / line_count as f32
        } else {
            0.0
        };
        
        if words_per_line >= 8.0 && words_per_line <= 20.0 {
            score += 0.1; // Good line density
        }
        
        score.min(1.0)
    }

    /// Parse sentiment result
    fn parse_sentiment(&self, sentiment_text: &str) -> DocumentSentiment {
        let text = sentiment_text.to_lowercase();
        if text.contains("positive") {
            DocumentSentiment::Positive
        } else if text.contains("negative") {
            DocumentSentiment::Negative
        } else {
            DocumentSentiment::Neutral
        }
    }

    /// Calculate analysis confidence
    fn calculate_analysis_confidence(&self, results: &[String]) -> f32 {
        let mut confidence: f32 = 1.0;
        
        // Reduce confidence for each empty or error result
        for result in results {
            if result.is_empty() || result.contains("error") || result.contains("failed") {
                confidence -= 0.2;
            }
        }
        
        confidence.max(0.0)
    }

    /// Calculate content hash
    fn calculate_content_hash(&self, content: &[u8]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

/// Document formats supported
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentFormat {
    PDF,
    Markdown,
    HTML,
    PlainText,
    Word,
    RTF,
}

/// Processed document result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedDocument {
    pub id: String,
    pub filename: String,
    pub content: ExtractedContent,
    pub analysis: Option<DocumentAnalysis>,
    pub metadata: DocumentMetadata,
}

/// Extracted content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedContent {
    pub text: String,
    pub format: DocumentFormat,
    pub word_count: usize,
    pub char_count: usize,
    pub line_count: usize,
    pub language: String,
    pub quality_score: f32,
}

/// Document analysis from LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentAnalysis {
    pub summary: String,
    pub key_points: Vec<String>,
    pub entities: HashMap<String, Vec<String>>,
    pub sentiment: DocumentSentiment,
    pub topics: Vec<String>,
    pub confidence_score: f32,
}

/// Document structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentStructure {
    pub headings: Vec<DocumentHeading>,
    pub sections: Vec<DocumentSection>,
    pub has_table_of_contents: bool,
    pub estimated_reading_time_minutes: u32,
}

/// Document heading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentHeading {
    pub title: String,
    pub level: usize,
}

/// Document section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSection {
    pub title: String,
    pub content: String,
    pub level: usize,
}

/// Document sentiment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentSentiment {
    Positive,
    Negative,
    Neutral,
}

/// Document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub file_size: usize,
    pub processing_time_ms: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub content_hash: String,
} 