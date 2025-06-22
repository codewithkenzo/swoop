// Core Document Types
export interface Document {
  id: string;
  title: string;
  content: string;
  summary?: string;
  metadata: DocumentMetadata;
  quality_score: number;
  content_hash: string;
  created_at: string;
  updated_at: string;
  word_count: number;
  character_count: number;
  language?: string;
  tags: string[];
}

export interface DocumentMetadata {
  source_url?: string;
  author?: string;
  created_at?: string;
  modified_at?: string;
  file_size?: number;
  mime_type?: string;
  encoding?: string;
  page_count?: number;
  processed_at: string;
}

export interface DocumentBatch {
  id: string;
  document_ids: string[];
  total_documents: number;
  status: 'pending' | 'processing' | 'completed' | 'failed';
  created_at: string;
}

// Extraction Types
export interface ExtractionResult {
  emails: string[];
  phones: string[];
  links: string[];
  metadata: Record<string, any>;
  sensitive_data: SensitiveData[];
  quality_score: number;
  classification: string[];
  validation_issues: string[];
}

export interface SensitiveData {
  data_type: string;
  original_text: string;
  redacted_text: string;
}

// API Request/Response Types
export interface UploadRequest {
  file: File;
  options?: {
    extract_metadata?: boolean;
    analyze_content?: boolean;
    detect_language?: boolean;
  };
}

export interface UploadResponse {
  document_id: string;
  status: 'uploaded' | 'processing' | 'completed' | 'failed';
  message: string;
  analysis?: ExtractionResult;
}

export interface AnalyzeRequest {
  document_id: string;
  analysis_type: 'basic' | 'comprehensive' | 'ai_powered';
  options?: {
    include_summary?: boolean;
    include_entities?: boolean;
    include_sentiment?: boolean;
    include_topics?: boolean;
  };
}

export interface AnalyzeResponse {
  document_id: string;
  analysis: {
    summary?: string;
    entities?: NamedEntity[];
    sentiment?: SentimentAnalysis;
    topics?: string[];
    extraction: ExtractionResult;
  };
  processing_time_ms: number;
}

// LLM Integration Types
export interface ChatRequest {
  messages: ChatMessage[];
  document_ids?: string[];
  model?: string;
  stream?: boolean;
  temperature?: number;
  max_tokens?: number;
}

export interface ChatMessage {
  role: 'user' | 'assistant' | 'system';
  content: string;
}

export interface ChatResponse {
  id: string;
  choices: {
    message: ChatMessage;
    finish_reason: string;
  }[];
  usage: {
    prompt_tokens: number;
    completion_tokens: number;
    total_tokens: number;
  };
  model: string;
}

// AI Analysis Types
export interface NamedEntity {
  text: string;
  label: string;
  confidence: number;
  start: number;
  end: number;
}

export interface SentimentAnalysis {
  overall: 'positive' | 'negative' | 'neutral';
  confidence: number;
  emotions?: Record<string, number>;
}

// Authentication Types
export interface AuthRequest {
  api_key?: string;
  token?: string;
}

export interface User {
  id: string;
  email: string;
  tier: 'free' | 'basic' | 'premium' | 'enterprise';
  usage: {
    requests_today: number;
    requests_limit: number;
    cost_today: number;
  };
}

// Error Types
export interface APIError {
  error: string;
  message: string;
  code: number;
  details?: Record<string, any>;
}

// Utility Types
export interface PaginatedResponse<T> {
  data: T[];
  pagination: {
    page: number;
    limit: number;
    total: number;
    has_more: boolean;
  };
}

export interface HealthCheck {
  status: 'healthy' | 'degraded' | 'unhealthy';
  version: string;
  uptime_seconds: number;
  checks: {
    database: boolean;
    llm_service: boolean;
    storage: boolean;
  };
}

// Environment Variables
export interface EdgeEnv {
  OPENROUTER_API_KEY: string;
  TURSO_DATABASE_URL: string;
  TURSO_AUTH_TOKEN: string;
  SWOOP_API_KEY: string;
}

// Edge Runtime Specific Types
export interface EdgeRequest extends Request {
  geo?: {
    city?: string;
    country?: string;
    region?: string;
    latitude?: string;
    longitude?: string;
  };
}

export interface EdgeResponse extends Response {
  headers: Headers;
} 