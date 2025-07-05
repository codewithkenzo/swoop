-- Initial Swoop Database Schema
-- This migration creates the core tables for the Swoop platform

-- Create documents table
CREATE TABLE IF NOT EXISTS documents (
    id VARCHAR PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    summary TEXT,
    quality_score REAL,
    content_hash VARCHAR,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    source_url TEXT,
    document_type VARCHAR,
    language VARCHAR,
    word_count INTEGER,
    size_bytes BIGINT,
    content_type VARCHAR,
    file_size BIGINT,
    extracted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    metadata JSONB DEFAULT '{}'::jsonb
);

-- Create document_batches table
CREATE TABLE IF NOT EXISTS document_batches (
    id VARCHAR PRIMARY KEY,
    document_ids JSONB NOT NULL,
    total_documents INTEGER NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create crawl_jobs table
CREATE TABLE IF NOT EXISTS crawl_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    url TEXT NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'pending',
    max_pages INTEGER DEFAULT 100,
    crawl_depth INTEGER DEFAULT 2,
    delay_ms INTEGER DEFAULT 1000,
    respect_robots_txt BOOLEAN DEFAULT true,
    include_patterns TEXT[],
    exclude_patterns TEXT[],
    pages_crawled INTEGER DEFAULT 0,
    pages_total INTEGER DEFAULT 0,
    success_rate REAL DEFAULT 0.0,
    avg_response_time_ms REAL DEFAULT 0.0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    error_message TEXT
);

-- Create crawl_pages table
CREATE TABLE IF NOT EXISTS crawl_pages (
    id VARCHAR PRIMARY KEY,
    job_id VARCHAR NOT NULL,
    url TEXT NOT NULL,
    status_code SMALLINT NOT NULL,
    text_length INTEGER NOT NULL,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create chat_conversations table
CREATE TABLE IF NOT EXISTS chat_conversations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id VARCHAR,
    user_message TEXT NOT NULL,
    assistant_response TEXT,
    context JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    response_time_ms INTEGER,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE SET NULL
);

-- Create vector_records table (for AI embeddings)
CREATE TABLE IF NOT EXISTS vector_records (
    id VARCHAR PRIMARY KEY,
    document_id VARCHAR NOT NULL,
    vector REAL[] NOT NULL,
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

-- Create settings table
CREATE TABLE IF NOT EXISTS settings (
    key VARCHAR PRIMARY KEY,
    value JSONB NOT NULL,
    description TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_documents_created_at ON documents(created_at);
CREATE INDEX IF NOT EXISTS idx_documents_document_type ON documents(document_type);
CREATE INDEX IF NOT EXISTS idx_documents_source_url ON documents(source_url);
CREATE INDEX IF NOT EXISTS idx_documents_content_hash ON documents(content_hash);

CREATE INDEX IF NOT EXISTS idx_crawl_jobs_status ON crawl_jobs(status);
CREATE INDEX IF NOT EXISTS idx_crawl_jobs_created_at ON crawl_jobs(created_at);

CREATE INDEX IF NOT EXISTS idx_crawl_pages_job_id ON crawl_pages(job_id);
CREATE INDEX IF NOT EXISTS idx_crawl_pages_fetched_at ON crawl_pages(fetched_at);
CREATE INDEX IF NOT EXISTS idx_crawl_pages_status_code ON crawl_pages(status_code);

CREATE INDEX IF NOT EXISTS idx_chat_conversations_document_id ON chat_conversations(document_id);
CREATE INDEX IF NOT EXISTS idx_chat_conversations_created_at ON chat_conversations(created_at);

CREATE INDEX IF NOT EXISTS idx_vector_records_document_id ON vector_records(document_id);

CREATE INDEX IF NOT EXISTS idx_settings_key ON settings(key);

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers for updated_at columns
CREATE TRIGGER update_documents_updated_at BEFORE UPDATE ON documents
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_settings_updated_at BEFORE UPDATE ON settings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert default settings
INSERT INTO settings (key, value, description) VALUES
    ('theme', '"system"', 'UI theme preference: system, light, or dark'),
    ('ai_features_enabled', 'true', 'Enable AI-powered document analysis'),
    ('auto_categorization', 'true', 'Automatically categorize uploaded documents'),
    ('generate_embeddings', 'true', 'Generate vector embeddings for semantic search'),
    ('crawler_max_concurrent', '10', 'Maximum concurrent crawl requests'),
    ('crawler_default_delay', '1000', 'Default delay between crawl requests (ms)')
ON CONFLICT (key) DO NOTHING;