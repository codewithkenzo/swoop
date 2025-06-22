# 🚀 Swoop Document Intelligence Platform

**Advanced document processing and AI analysis platform built with Rust and React**

[![Rust](https://img.shields.io/badge/rust-1.88+-orange.svg)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/react-18+-blue.svg)](https://reactjs.org/)
[![TypeScript](https://img.shields.io/badge/typescript-5+-blue.svg)](https://www.typescriptlang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## ✨ Features

### 🔥 Core Capabilities
- **📄 Document Upload & Processing**: Multipart file upload with real-time processing
- **🧠 Intelligent Content Extraction**: Advanced HTML parsing with script/style removal
- **📊 Document Analysis**: Word count, readability analysis, content classification
- **🗂️ Document Workspace**: In-memory document storage with UUID-based IDs
- **🌐 RESTful API**: JSON-based API with comprehensive endpoints
- **⚡ High Performance**: Async Rust backend with concurrent processing

### 🎯 Advanced Features
- **🔍 Content Intelligence**: Automatic text extraction from HTML documents
- **📈 Analytics Dashboard**: Real-time statistics and insights
- **🔧 Configurable Server**: CLI args, environment variables, and defaults
- **🌍 CORS-Enabled**: Frontend integration ready
- **📱 Modern Frontend**: React + TypeScript + shadcn/ui components

## 🚀 Quick Start

### Prerequisites
- **Rust 1.88+** (nightly recommended)
- **Node.js 18+** (for frontend)
- **curl** and **jq** (for testing)

### 🏃 Running the Backend

```bash
# Clone the repository
git clone https://github.com/codewithkenzo/swoop.git
cd swoop

# Build and run the server
cargo run --bin swoop_server

# Or specify a custom port
cargo run --bin swoop_server 3001

# Or use environment variable
PORT=3001 cargo run --bin swoop_server
```

### 🌐 Running the Frontend

```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev
```

## 📚 API Documentation

### Base URL
```
http://localhost:3001
```

### 🔗 Endpoints

#### 📋 System Information
```bash
# Get platform information
curl http://localhost:3001/

# Health check
curl http://localhost:3001/health

# API status
curl http://localhost:3001/api/status
```

#### 📄 Document Management
```bash
# Upload document
curl -F "file=@document.html" http://localhost:3001/api/documents/upload

# List all documents
curl http://localhost:3001/api/documents

# Get specific document
curl http://localhost:3001/api/documents/{id}

# Analyze document
curl -X POST http://localhost:3001/api/documents/{id}/analyze
```

### 📊 Example Responses

#### Document Upload Response
```json
{
  "status": "success",
  "message": "Document uploaded and processed",
  "document": {
    "id": "doc_98965e43",
    "filename": "example.html",
    "content_type": "text/html",
    "size_bytes": 1024,
    "processed": true
  }
}
```

#### Document Analysis Response
```json
{
  "status": "success",
  "analysis": {
    "document_id": "doc_98965e43",
    "statistics": {
      "word_count": 150,
      "character_count": 1024,
      "line_count": 15,
      "sentence_count": 8,
      "avg_sentence_length": 18
    },
    "insights": {
      "readability": "readable",
      "content_type": "short_form",
      "language": "detected_english"
    },
    "summary": {
      "first_sentence": "This is the first sentence...",
      "key_topics": ["document_analysis", "text_processing"]
    }
  }
}
```

## 🧪 Testing

### Manual Testing Commands

```bash
# Start server
cargo run --bin swoop_server 3001 &

# Test health endpoint
curl -s http://localhost:3001/health | jq .

# Upload a test document
curl -F "file=@test_data/small.html" http://localhost:3001/api/documents/upload | jq .

# Get document ID from upload response, then analyze
curl -X POST http://localhost:3001/api/documents/doc_12345678/analyze | jq .

# List all documents
curl -s http://localhost:3001/api/documents | jq .

# Stop server
pkill -f swoop_server
```

### Comprehensive Test Suite

```bash
# Run comprehensive tests (includes workflow routing, performance, accuracy)
./comprehensive_test.sh
```

## 🏗️ Architecture

### Backend Stack
- **🦀 Rust**: Core language for performance and safety
- **🌐 Axum**: Modern async web framework
- **📊 Serde**: JSON serialization/deserialization
- **🔧 Tokio**: Async runtime
- **📅 Chrono**: Date/time handling
- **🆔 UUID**: Unique document identifiers

### Frontend Stack
- **⚛️ React 18**: Modern React with hooks
- **📘 TypeScript**: Type-safe development
- **🎨 Tailwind CSS**: Utility-first styling
- **🧩 shadcn/ui**: Accessible component library
- **⚡ Vite**: Fast build tool
- **🔄 TanStack Query**: API state management

### 📁 Project Structure
```
swoop/
├── src/
│   ├── bin/
│   │   └── swoop_server.rs      # Main server binary
│   ├── config.rs                # Configuration management
│   ├── error.rs                 # Error handling
│   ├── extractors/
│   │   └── mod.rs              # Content extraction
│   ├── models.rs               # Data models
│   └── lib.rs                  # Library root
├── frontend/
│   ├── src/                    # React TypeScript frontend
│   └── v0_scaffold/            # v0.dev generated UI
├── test_data/                  # Sample documents
└── comprehensive_test.sh       # Test suite
```

## 🔧 Configuration

### Server Configuration
- **Port**: CLI arg, `PORT` env var, or default 3001
- **CORS**: Enabled for all origins (development)
- **Logging**: Environment-based (`RUST_LOG`)

### Environment Variables
```bash
export PORT=3001                    # Server port
export RUST_LOG=info               # Logging level
```

## 🚀 Development

### Building from Source
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Check code
cargo check
```

### Frontend Development
```bash
cd frontend

# Development with hot reload
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## 📈 Performance

- **⚡ Fast Processing**: Handles documents in milliseconds
- **🔄 Concurrent Operations**: Multiple document analysis simultaneously
- **💾 Memory Efficient**: In-memory workspace with controlled storage
- **🚀 Async Architecture**: Non-blocking I/O operations

## 🔮 Roadmap

### Phase 3: Advanced AI Integration
- [ ] **🤖 LLM Integration**: GPT/Claude API for advanced analysis
- [ ] **🏷️ Smart Tagging**: Automatic document categorization
- [ ] **🔍 Semantic Search**: Vector-based document search
- [ ] **💬 AI Chat**: Document Q&A capabilities

### Phase 4: Enterprise Features
- [ ] **🔐 Authentication**: User management and permissions
- [ ] **💾 Persistent Storage**: Database integration
- [ ] **📊 Advanced Analytics**: Usage metrics and insights
- [ ] **🖥️ Desktop App**: Tauri-based native application

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **v0.dev**: For the beautiful frontend scaffolding
- **Rust Community**: For the amazing ecosystem
- **React Team**: For the powerful frontend framework

---

**Built with ❤️ by the Swoop Team**

For more information, visit our [documentation](https://github.com/codewithkenzo/swoop) or join our [community](https://github.com/codewithkenzo/swoop/discussions).
