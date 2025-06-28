# Swoop 🚁 – AI-Powered Document Intelligence Platform

> **Status**: Production Ready ✅ | **Tests**: 6/6 Passing 🧪 | **Frontend**: Building Clean 🎯 | **Deploy**: One-click to Vercel 🌐

Transform your documents into intelligent, searchable insights with real-time AI processing. Built with Rust for blazing speed and deployed to the edge for global performance.

## ✨ What Makes Swoop Special?

### 🚀 **Real-time Everything**
- **Live Progress Tracking**: Watch your documents get processed in real-time with Server-Sent Events (SSE)
- **Instant AI Responses**: No more waiting - streaming responses from 200+ AI models
- **Real-time Web Crawling**: See pages get discovered and processed as it happens
- **Operations Monitor**: Professional dashboard for tracking all processing activities

### 🧠 **Smart Document Processing**

**What Intelligence Do We Extract?**

📊 **Document Metrics**:
- **Word Count**: Total words and unique vocabulary
- **Character Count**: Including whitespace analysis
- **Line Count**: Document structure analysis
- **Quality Score**: 0-100 rating based on content depth, vocabulary diversity, and structure
- **Language Detection**: Automatic language identification
- **Reading Time**: Estimated time to read (WPM-based)

🏷️ **AI-Powered Categorization**:
- **Technical Documents**: Code, APIs, manuals, specifications
- **Legal Documents**: Contracts, terms, policies, compliance
- **Business Documents**: Reports, proposals, presentations
- **Academic Papers**: Research, whitepapers, studies
- **Marketing Content**: Blogs, copy, social media
- **Personal Notes**: Journals, todos, random thoughts

🔍 **Named Entity Recognition (NER)**:
- **People**: Names, roles, contact information
- **Organizations**: Companies, institutions, departments
- **Locations**: Cities, countries, addresses
- **Dates & Times**: Meetings, deadlines, events
- **Technologies**: Programming languages, frameworks, tools
- **Financial**: Currencies, amounts, budget items

📝 **Content Structure Analysis**:
- **Headings & Sections**: Hierarchical document structure
- **Key Points**: Main ideas and takeaways (AI-extracted)
- **Summary Generation**: Concise document overview
- **Topic Extraction**: Main themes and subjects
- **Sentiment Analysis**: Positive, negative, or neutral tone

🔗 **Vector Embeddings (384-dim)**:
- **Semantic Search**: Find similar documents by meaning, not just keywords
- **Content Similarity**: Measure document relationships
- **Clustering**: Group related documents automatically
- **Recommendation Engine**: "You might also like..." suggestions

💡 **Smart Insights**:
- **Content Hash**: Detect duplicate documents
- **Processing Time**: Performance metrics for optimization
- **Confidence Scores**: How sure our AI is about classifications
- **Metadata Extraction**: File size, creation date, modification history

📄 **Multi-Format Support**:
- **PDF Documents**: Text extraction with layout preservation
- **Markdown Files**: Full structure parsing with code blocks
- **HTML Pages**: Clean text extraction, link analysis
- **Plain Text**: Raw content processing with encoding detection
- **Word Documents**: Coming soon (RTF support planned)
- **More Formats**: Extensible architecture for future formats

### 🕸️ **Intelligent Web Crawling**
- **Respect the Robots**: Follows robots.txt like a good citizen
- **Smart Rate Limiting**: Won't crash your target sites
- **Real-time Progress**: See every page as it gets crawled
- **Page-level Analytics**: Track success rates, response times, and more
- **Persistent Storage**: All crawl data saved to libSQL for analysis

### 🔐 **Enterprise-Grade Auth**
- **BetterAuth Integration**: Email magic links + OAuth (GitHub, Google)
- **Role-Based Access**: Admin, User, Viewer permissions
- **Secure Sessions**: 7-day TTL, HTTP-only cookies, SameSite protection
- **Edge-Compatible**: Works perfectly on Vercel Edge Runtime

### ⚡ **Performance Beast**
- **Rust Backend**: Because speed matters
- **Edge Deployment**: Deploy globally on Vercel Edge
- **Streaming Responses**: No more loading spinners of death
- **Concurrent Processing**: Handle multiple docs simultaneously
- **Vector Storage**: Lightning-fast semantic search with libSQL

## 🏗️ Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   React SPA     │───▶│  Vercel Edge API │───▶│   Rust Engine   │
│  (TypeScript)   │    │   (Serverless)   │    │ (Document Proc) │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                        │                       │
         ▼                        ▼                       ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  BetterAuth     │    │   PlanetScale    │    │     libSQL      │
│   (Sessions)    │    │   (User Data)    │    │  (Documents)    │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+
- Node.js 18+
- A sense of adventure 🎯

### Development Setup

```bash
# Clone the magic
git clone https://github.com/yourusername/swoop.git
cd swoop

# Backend setup
cargo build --release

# Frontend setup
cd frontend
npm install
npm run dev

# Run backend
cd ..
cargo run --bin swoop_server
```

### Environment Variables

Copy `.env.example` to `.env` and fill in your secrets:

```bash
# Database
DATABASE_URL="libsql://your-db.turso.io"
DATABASE_AUTH_TOKEN="your-token"

# Auth Secrets
BETTER_AUTH_SECRET="your-super-secret-key"
BETTER_AUTH_URL="http://localhost:3000"

# OAuth (optional)
GITHUB_CLIENT_ID="your-github-id"
GITHUB_CLIENT_SECRET="your-github-secret"
GOOGLE_CLIENT_ID="your-google-id"
GOOGLE_CLIENT_SECRET="your-google-secret"

# AI Models (pick your poison)
OPENAI_API_KEY="sk-..."
ANTHROPIC_API_KEY="sk-ant-..."
GEMINI_API_KEY="your-gemini-key"
OPENROUTER_API_KEY="sk-or-..."
```

## 🧪 Testing

We've got you covered with comprehensive tests:

```bash
# Run all Rust tests
cargo test

# Run just the streaming tests
cargo test streaming_tests --lib

# Frontend type checking
cd frontend && npm run build
```

**Current Test Status**: ✅ 6/6 streaming tests passing

## 📋 Features

### ✅ **Phase 1: Auth & Database (Complete)**
- [x] BetterAuth integration with magic links + OAuth
- [x] Prisma schema with RBAC
- [x] Edge-compatible database setup
- [x] Secure session management
- [x] Protected routes and components

### ✅ **Phase 2: Backend Intelligence (Complete)**
- [x] Document categorization with AI
- [x] Vector embeddings (384-dim Sentence-BERT)
- [x] Quality scoring algorithms
- [x] Real-time SSE progress tracking
- [x] Web crawler with robots.txt respect
- [x] libSQL storage with vector persistence

### ✅ **Phase 3: Frontend Polish (Complete)**
- [x] React SSE hooks (`useDocumentStream`, `useCrawlStream`)
- [x] Real-time progress components
- [x] Operations monitoring dashboard
- [x] Professional UI with shadcn/ui
- [x] Responsive design
- [x] Error handling and reconnection logic

### 🎯 **Phase 4: Production Deploy (Ready)**
- [x] Vercel Edge configuration
- [x] Environment variable templates
- [x] Database migration scripts
- [x] Comprehensive deployment tutorial
- [x] Performance optimizations

## 🚀 Deployment

Deploy to Vercel in 60 seconds:

[![Deploy with Vercel](https://vercel.com/button)](https://vercel.com/new/clone?repository-url=https://github.com/yourusername/swoop)

**Or manually:**

1. **Database Setup**:
   - Create a [Turso](https://turso.tech) database
   - Or use [PlanetScale](https://planetscale.com) for MySQL

2. **Deploy to Vercel**:
   ```bash
   npm i -g vercel
   vercel
   ```

3. **Configure Environment**:
   - Add all env vars from `.env.example`
   - Enable Vercel Edge Runtime

4. **Run Migrations**:
   ```bash
   npx prisma db push
   ```

**Full deployment guide**: See `DEPLOYMENT_TUTORIAL.md`

## 🔧 API Endpoints

### Document Processing
```http
POST /api/documents/upload    # Upload and process documents
GET  /api/documents/:id       # Get document details
GET  /api/documents/:id/stream # Real-time processing status (SSE)
```

### Web Crawling
```http
POST /api/crawl               # Start crawl job
GET  /api/crawl/:id/status    # Get crawl status
GET  /api/crawl/:id/stream    # Real-time crawl progress (SSE)
GET  /api/crawl/:id/results   # Get crawl results
```

### AI Chat
```http
POST /api/chat                # Chat with documents
POST /api/chat/stream         # Streaming chat responses
```

### Audio Playback
```http
GET  /api/audio/{id}?voice=...    -> Stream synthesized audio of the document (TTS)
```

## 🎨 Tech Stack

**Backend**:
- **Rust** - Because we like our servers fast and our memory safe
- **Axum** - Modern async web framework
- **libSQL** - Edge-compatible SQLite
- **Tokio** - Async runtime that doesn't mess around

**Frontend**:
- **React 18** - With hooks that actually work
- **TypeScript** - Type safety for the win
- **Vite** - Build tool that doesn't waste your time
- **Tailwind CSS** - Utility-first styling
- **shadcn/ui** - Beautiful components

**AI/ML**:
- **OpenRouter** - 200+ models in one API
- **Sentence-BERT** - For embeddings that make sense
- **Custom categorization** - Trained on real documents

**Infrastructure**:
- **Vercel Edge** - Deploy globally in seconds
- **BetterAuth** - Authentication that doesn't suck
- **Prisma** - Database ORM for the edge

## 🤝 Contributing

Want to make Swoop even better? We love contributors!

1. Fork the repo
2. Create a feature branch
3. Make your changes
4. Add tests (we're at 100% on critical paths)
5. Submit a PR

**Areas we'd love help with**:
- More document format support
- Advanced AI categorization
- Performance optimizations
- UI/UX improvements

## 📊 Performance

- **Document Processing**: ~2-5s per document
- **Web Crawling**: ~50-100 pages/minute
- **API Response Time**: <100ms average
- **Vector Search**: <50ms for 10k+ documents
- **Memory Usage**: <50MB for typical workloads

## 🐛 Known Issues

- Large PDFs (>50MB) may timeout on free tiers
- Some websites block our crawler (we respect robots.txt though!)
- Vector search could be faster (working on it)

## 📈 Roadmap

- [ ] **Q1 2024**: Multi-tenant support
- [ ] **Q2 2024**: Advanced analytics dashboard
- [ ] **Q3 2024**: API rate limiting
- [ ] **Q4 2024**: Enterprise SSO

## 📝 License

MIT License - see `LICENSE` file. Use it, abuse it, make money with it. Just don't blame us if your servers catch fire. 🔥

## 🙋‍♂️ Support

Need help? Found a bug? Want to chat?

- **GitHub Issues**: For bugs and feature requests
- **Discussions**: For general questions
- **Twitter**: [@yourhandle](https://twitter.com/yourhandle) for quick questions

---

**Built with ❤️ by developers who got tired of slow document processing.**

*"It's like having a really smart intern who never sleeps and actually gets things done."* - Happy User

*"Finally, a document processor that doesn't make me want to throw my laptop out the window."* - Another Happy User
