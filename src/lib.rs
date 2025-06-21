//! # Swoop
//! 
//! Intelligent document analysis and management platform with advanced web crawling capabilities.
//! 
//! Swoop provides a comprehensive solution for document processing, content extraction, and 
//! workspace management. It combines a high-performance Rust backend with a modern desktop 
//! application built using Tauri, React, and TypeScript.
//! 
//! ## Core Capabilities
//! 
//! - **Document Workspace**: Full-featured document management with file system integration
//! - **Advanced Extraction**: Sophisticated content parsing using CSS, XPath, and JSONPath
//! - **Production Monitoring**: Enterprise-grade observability and health monitoring
//! - **Concurrent Processing**: Thread-safe architecture with intelligent resource management
//! - **Multi-Platform**: Cross-platform desktop application and web interface
//! 
//! ## Architecture
//! 
//! The platform consists of three main components:
//! - **Core Engine** (Rust): High-performance document processing and web crawling
//! - **Desktop Application** (Tauri + React + TypeScript): Cross-platform workspace interface
//! - **Web Interface**: Browser-based monitoring and configuration dashboard
//! 
//! ## Quick Start
//! 
//! ```rust
//! use swoop::{DocumentWorkspace, CrawlConfig};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let workspace = DocumentWorkspace::new("./documents").await?;
//!     let config = CrawlConfig::default();
//!     let results = workspace.process_url("https://example.com", config).await?;
//!     println!("Processed document: {}", results.title.unwrap_or_default());
//!     Ok(())
//! }
//! ``` 