// API Response Types
export interface Document {
  id: string
  title: string
  content: string
  url: string
  file_path?: string
  mime_type: string
  size: number
  created_at: string
  updated_at: string
  metadata: DocumentMetadata
  analysis?: AnalysisResults
}

export interface DocumentMetadata {
  title?: string
  description?: string
  keywords?: string[]
  author?: string
  language?: string
  [key: string]: any
}

export interface AnalysisResults {
  category?: DocumentCategory
  entities: ExtractedEntity[]
  embeddings?: string
  tags: string[]
  language?: string
  confidence: AnalysisConfidence
}

export interface AnalysisConfidence {
  categorization?: number
  entity_extraction?: number
  embeddings?: number
  auto_tagging?: number
  language_detection?: number
}

export interface ExtractedEntity {
  text: string
  entity_type: EntityType
  confidence: number
  start_pos: number
  end_pos: number
}

export enum DocumentCategory {
  Legal = 'Legal',
  Academic = 'Academic',
  Technical = 'Technical',
  Business = 'Business',
  News = 'News',
  Personal = 'Personal',
  Unknown = 'Unknown'
}

export enum EntityType {
  Person = 'Person',
  Organization = 'Organization',
  Location = 'Location',
  Date = 'Date',
  Money = 'Money'
}

// UI State Types
export interface DocumentFilter {
  category?: DocumentCategory
  tags?: string[]
  dateRange?: {
    start: Date
    end: Date
  }
  searchQuery?: string
}

export interface UploadProgress {
  file: File
  progress: number
  status: 'pending' | 'uploading' | 'processing' | 'completed' | 'error'
  error?: string
  documentId?: string
}

export interface CrawlJob {
  id: string
  url: string
  status: 'pending' | 'running' | 'completed' | 'failed'
  progress: number
  documents_found: number
  created_at: string
  completed_at?: string
  error?: string
}

// Analysis Configuration
export interface AnalysisConfig {
  categorization: boolean
  entity_extraction: boolean
  embeddings: boolean
  auto_tagging: boolean
  language_detection: boolean
}

// API Endpoints
export interface ApiResponse<T> {
  data: T
  success: boolean
  message?: string
}

export interface PaginatedResponse<T> {
  data: T[]
  total: number
  page: number
  per_page: number
  has_more: boolean
} 