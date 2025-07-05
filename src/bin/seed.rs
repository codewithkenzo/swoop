use std::env;
use tokio_postgres::{Client, Error, NoTls};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Get database URL from environment or use default
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://swoop_user:swoop_password@localhost:5432/swoop".to_string());

    // Connect to the database
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    // Spawn the connection task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    println!("🌱 Starting database seeding...");

    // Seed documents
    seed_documents(&client).await?;
    
    // Seed crawl jobs
    seed_crawl_jobs(&client).await?;
    
    // Seed chat conversations
    seed_chat_conversations(&client).await?;

    println!("✅ Database seeding completed successfully!");
    Ok(())
}

async fn seed_documents(client: &Client) -> Result<(), Error> {
    println!("📄 Seeding documents...");

    let documents = vec![
        (
            "Sample PDF Document",
            "This is a comprehensive PDF document containing technical specifications, user guidelines, and best practices for document management systems. It includes detailed information about file formats, processing capabilities, and integration methods.",
            "pdf",
            2048576i64, // 2MB
            "/tmp/sample_technical_spec.pdf"
        ),
        (
            "HTML Web Page Analysis",
            "<html><head><title>Web Content Analysis</title></head><body><h1>Advanced Web Scraping Techniques</h1><p>This document explores modern web scraping methodologies, including JavaScript rendering, dynamic content extraction, and rate limiting strategies.</p><section><h2>Key Technologies</h2><ul><li>Rust web scraping libraries</li><li>Headless browser automation</li><li>Anti-bot detection circumvention</li></ul></section></body></html>",
            "html",
            1536000i64, // 1.5MB
            "/tmp/web_analysis.html"
        ),
        (
            "Markdown Documentation Guide",
            "# Comprehensive Documentation Guide\n\n## Overview\n\nThis guide provides detailed instructions for creating, managing, and optimizing documentation systems.\n\n## Core Features\n\n- **Document Processing**: Advanced text extraction and analysis\n- **Vector Search**: Semantic similarity matching\n- **AI Integration**: Natural language processing capabilities\n- **Real-time Updates**: Live document synchronization\n\n## Implementation\n\n### Setup\n\n```bash\n# Install dependencies\ncargo install swoop-cli\n\n# Initialize project\nswoop init --template=docs\n```\n\n### Configuration\n\n```yaml\ndatabase:\n  type: postgresql\n  url: ${DATABASE_URL}\n  \nprocessing:\n  max_file_size: 100MB\n  supported_formats: [pdf, html, md, txt]\n```\n\n## Best Practices\n\n1. **Structure**: Organize documents hierarchically\n2. **Metadata**: Include comprehensive tagging\n3. **Indexing**: Optimize for search performance\n4. **Security**: Implement access controls\n\n## Troubleshooting\n\nCommon issues and solutions for document processing workflows.",
            "md",
            512000i64, // 512KB
            "/tmp/documentation_guide.md"
        ),
    ];

    for (title, content, file_type, file_size, file_path) in documents {
        let id = Uuid::new_v4();
        client.execute(
            "INSERT INTO documents (id, title, content, file_type, file_size, file_path, metadata) 
             VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (id) DO NOTHING",
            &[
                &id,
                &title,
                &content,
                &file_type,
                &file_size,
                &file_path,
                &serde_json::json!({
                    "language": "en",
                    "category": "demo",
                    "source": "seed_script",
                    "tags": ["demo", "sample", file_type],
                    "processing_time": 0.5,
                    "word_count": content.split_whitespace().count()
                })
            ],
        ).await?;
        
        println!("  ✓ Added document: {}", title);
    }

    Ok(())
}

async fn seed_crawl_jobs(client: &Client) -> Result<(), Error> {
    println!("🕷️  Seeding crawl jobs...");

    let crawl_jobs = vec![
        (
            "https://example.com",
            "completed",
            100,
            5,
            5,
            serde_json::json!({
                "depth": 2,
                "delay": 1000,
                "follow_links": true,
                "ignore_robots": false,
                "max_pages": 10
            })
        ),
        (
            "https://docs.rust-lang.org",
            "pending",
            0,
            0,
            0,
            serde_json::json!({
                "depth": 3,
                "delay": 500,
                "follow_links": true,
                "ignore_robots": false,
                "max_pages": 50
            })
        ),
        (
            "https://github.com/trending/rust",
            "running",
            45,
            12,
            15,
            serde_json::json!({
                "depth": 1,
                "delay": 2000,
                "follow_links": false,
                "ignore_robots": false,
                "max_pages": 20
            })
        ),
        (
            "https://news.ycombinator.com",
            "completed",
            100,
            25,
            25,
            serde_json::json!({
                "depth": 2,
                "delay": 1500,
                "follow_links": true,
                "ignore_robots": false,
                "max_pages": 30
            })
        ),
        (
            "https://stackoverflow.com/questions/tagged/rust",
            "failed",
            20,
            3,
            15,
            serde_json::json!({
                "depth": 1,
                "delay": 1000,
                "follow_links": false,
                "ignore_robots": false,
                "max_pages": 100,
                "error": "Rate limited by server"
            })
        ),
    ];

    for (url, status, progress, pages_crawled, pages_found, config) in crawl_jobs {
        let id = Uuid::new_v4();
        let (started_at, completed_at) = match status {
            "completed" => (Some(chrono::Utc::now() - chrono::Duration::hours(2)), Some(chrono::Utc::now())),
            "running" => (Some(chrono::Utc::now() - chrono::Duration::minutes(30)), None),
            "failed" => (Some(chrono::Utc::now() - chrono::Duration::minutes(15)), None),
            _ => (None, None),
        };

        client.execute(
            "INSERT INTO crawl_jobs (id, url, status, progress, pages_crawled, pages_found, started_at, completed_at, config) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) ON CONFLICT (id) DO NOTHING",
            &[
                &id,
                &url,
                &status,
                &progress,
                &pages_crawled,
                &pages_found,
                &started_at,
                &completed_at,
                &config,
            ],
        ).await?;
        
        println!("  ✓ Added crawl job: {} ({})", url, status);
    }

    Ok(())
}

async fn seed_chat_conversations(client: &Client) -> Result<(), Error> {
    println!("💬 Seeding chat conversations...");

    // Create conversation
    let conversation_id = Uuid::new_v4();
    client.execute(
        "INSERT INTO chat_conversations (id, title) VALUES ($1, $2) ON CONFLICT (id) DO NOTHING",
        &[&conversation_id, &"Advanced Document Processing Discussion"],
    ).await?;

    println!("  ✓ Created conversation: Advanced Document Processing Discussion");

    // Add messages to conversation
    let messages = vec![
        (
            "user",
            "Can you explain how the document processing pipeline works in this system?"
        ),
        (
            "assistant",
            "Certainly! The document processing pipeline in Swoop consists of several key stages:\n\n1. **Upload & Validation**: Documents are uploaded and validated for size, format, and content safety\n2. **Content Extraction**: Text is extracted from various formats (PDF, HTML, Markdown, etc.)\n3. **Text Processing**: The content is cleaned, normalized, and prepared for analysis\n4. **Vector Embedding**: Text is converted to high-dimensional vectors using AI models\n5. **Indexing**: Documents are indexed for fast retrieval and semantic search\n6. **Storage**: Both raw content and processed data are stored in the database\n\nThe system supports real-time processing and can handle multiple document formats simultaneously."
        ),
        (
            "user",
            "What about the search capabilities? How does semantic search work?"
        ),
        (
            "assistant",
            "Great question! The semantic search in Swoop uses vector similarity matching:\n\n**How it works:**\n- Your query is converted to a vector using the same AI model as the documents\n- We calculate cosine similarity between your query vector and all document vectors\n- Results are ranked by similarity score and returned\n\n**Advantages:**\n- Finds relevant content even if exact keywords don't match\n- Understands context and meaning, not just word matching\n- Can find documents about similar topics using different terminology\n\n**Example:**\nSearching for 'machine learning' might also return documents about 'artificial intelligence', 'neural networks', or 'deep learning' based on semantic similarity.\n\nThe system currently indexes embeddings with 1536 dimensions and uses PostgreSQL's vector extension for efficient similarity searches."
        ),
        (
            "user",
            "How does the web crawling feature integrate with the document processing?"
        ),
        (
            "assistant",
            "The web crawling feature seamlessly integrates with our document processing pipeline:\n\n**Crawling Process:**\n1. **URL Discovery**: Starting from seed URLs, the crawler discovers new pages\n2. **Content Extraction**: HTML content is extracted and cleaned\n3. **Rate Limiting**: Respectful crawling with configurable delays\n4. **Deduplication**: Prevents processing the same content multiple times\n\n**Integration Points:**\n- Crawled content flows directly into the document processing pipeline\n- Each crawled page becomes a document in the system\n- Vector embeddings are generated for semantic search\n- Crawl jobs are tracked with progress monitoring\n\n**Configuration Options:**\n- Maximum crawl depth (default: 3 levels)\n- Crawl delay (default: 1 second between requests)\n- Maximum pages per job\n- Robots.txt compliance\n- Custom user agent strings\n\nThe crawler runs asynchronously and provides real-time progress updates through WebSocket connections."
        )
    ];

    for (role, content) in messages {
        let message_id = Uuid::new_v4();
        client.execute(
            "INSERT INTO chat_messages (id, conversation_id, role, content, metadata) 
             VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO NOTHING",
            &[
                &message_id,
                &conversation_id,
                &role,
                &content,
                &serde_json::json!({
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "source": "seed_script",
                    "word_count": content.split_whitespace().count()
                })
            ],
        ).await?;
        
        println!("  ✓ Added message from: {}", role);
    }

    Ok(())
} 