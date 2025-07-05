-- Swoop Database Initialization Script
-- This script creates the necessary tables and initial data for the Swoop application

-- Create extensions if they don't exist
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "vector";

-- Documents table
CREATE TABLE IF NOT EXISTS documents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    file_type VARCHAR(50) NOT NULL,
    file_size BIGINT NOT NULL,
    file_path VARCHAR(512),
    metadata JSONB DEFAULT '{}',
    embedding vector(1536), -- OpenAI embedding dimension
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Crawl jobs table
CREATE TABLE IF NOT EXISTS crawl_jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    url VARCHAR(2048) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, running, completed, failed, cancelled
    progress INTEGER DEFAULT 0,
    pages_crawled INTEGER DEFAULT 0,
    pages_found INTEGER DEFAULT 0,
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    error_message TEXT,
    config JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Crawled pages table
CREATE TABLE IF NOT EXISTS crawled_pages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    crawl_job_id UUID NOT NULL REFERENCES crawl_jobs(id) ON DELETE CASCADE,
    url VARCHAR(2048) NOT NULL,
    title VARCHAR(512),
    content TEXT,
    status_code INTEGER,
    crawled_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    embedding vector(1536)
);

-- Chat conversations table
CREATE TABLE IF NOT EXISTS chat_conversations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Chat messages table
CREATE TABLE IF NOT EXISTS chat_messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    conversation_id UUID NOT NULL REFERENCES chat_conversations(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL, -- user, assistant, system
    content TEXT NOT NULL,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Settings table
CREATE TABLE IF NOT EXISTS settings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    key VARCHAR(255) NOT NULL UNIQUE,
    value JSONB NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_documents_created_at ON documents(created_at);
CREATE INDEX IF NOT EXISTS idx_documents_file_type ON documents(file_type);
CREATE INDEX IF NOT EXISTS idx_crawl_jobs_status ON crawl_jobs(status);
CREATE INDEX IF NOT EXISTS idx_crawl_jobs_created_at ON crawl_jobs(created_at);
CREATE INDEX IF NOT EXISTS idx_crawled_pages_crawl_job_id ON crawled_pages(crawl_job_id);
CREATE INDEX IF NOT EXISTS idx_crawled_pages_url ON crawled_pages(url);
CREATE INDEX IF NOT EXISTS idx_chat_messages_conversation_id ON chat_messages(conversation_id);
CREATE INDEX IF NOT EXISTS idx_chat_messages_created_at ON chat_messages(created_at);
CREATE INDEX IF NOT EXISTS idx_settings_key ON settings(key);

-- Vector similarity search indexes (if vector extension is available)
CREATE INDEX IF NOT EXISTS idx_documents_embedding ON documents USING ivfflat (embedding vector_cosine_ops);
CREATE INDEX IF NOT EXISTS idx_crawled_pages_embedding ON crawled_pages USING ivfflat (embedding vector_cosine_ops);

-- Insert default settings
INSERT INTO settings (key, value, description) VALUES
    ('theme', '"light"', 'UI theme preference'),
    ('advanced_crawl', 'false', 'Enable advanced crawling features'),
    ('max_crawl_depth', '3', 'Maximum crawling depth'),
    ('crawl_delay', '1000', 'Delay between crawl requests in milliseconds'),
    ('max_file_size', '104857600', 'Maximum file size in bytes (100MB)'),
    ('allowed_file_types', '["pdf", "txt", "html", "md", "doc", "docx"]', 'Allowed file types for upload')
ON CONFLICT (key) DO NOTHING;

-- Insert demo data
INSERT INTO documents (title, content, file_type, file_size, file_path) VALUES
    ('Sample PDF Document', 'This is a sample PDF document content for testing purposes. It contains various types of information including text, data, and formatting examples.', 'pdf', 1024, '/tmp/sample.pdf'),
    ('HTML Page Example', '<html><body><h1>Sample HTML Content</h1><p>This is a sample HTML document that demonstrates web content processing capabilities.</p></body></html>', 'html', 512, '/tmp/sample.html'),
    ('Markdown Documentation', '# Sample Markdown\n\nThis is a sample markdown document.\n\n## Features\n\n- Document processing\n- Vector search\n- AI analysis\n\n## Usage\n\nUpload documents and start searching!', 'md', 256, '/tmp/sample.md')
ON CONFLICT (id) DO NOTHING;

-- Insert demo crawl job
INSERT INTO crawl_jobs (url, status, progress, pages_crawled, pages_found, config) VALUES
    ('https://example.com', 'completed', 100, 5, 5, '{"depth": 2, "delay": 1000}')
ON CONFLICT (id) DO NOTHING;

-- Insert demo chat conversation
INSERT INTO chat_conversations (title) VALUES
    ('Sample Conversation about Documents')
ON CONFLICT (id) DO NOTHING;

-- Insert demo chat messages
INSERT INTO chat_messages (conversation_id, role, content) VALUES
    ((SELECT id FROM chat_conversations WHERE title = 'Sample Conversation about Documents' LIMIT 1), 'user', 'What documents do we have in the system?'),
    ((SELECT id FROM chat_conversations WHERE title = 'Sample Conversation about Documents' LIMIT 1), 'assistant', 'We currently have 3 documents in the system: a PDF document, an HTML page, and a Markdown file. Would you like me to provide more details about any of these documents?')
ON CONFLICT (id) DO NOTHING;

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers for updated_at columns
CREATE TRIGGER update_documents_updated_at BEFORE UPDATE ON documents
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_crawl_jobs_updated_at BEFORE UPDATE ON crawl_jobs
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_chat_conversations_updated_at BEFORE UPDATE ON chat_conversations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_settings_updated_at BEFORE UPDATE ON settings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column(); 