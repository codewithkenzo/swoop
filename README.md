# Swoop 🚁 – AI-Powered Document Intelligence Platform

> **Status**: Production Ready ✅ | **Frontend**: Clean Build 🎯 | **Deploy**: One-click to Vercel 🌐

Transform your documents into intelligent, searchable insights with real-time AI processing. Built with Rust for blazing speed and deployed to the edge for global performance.

## ✨ What Makes Swoop Special?

### 🚀 **Real-time Everything**
- **Live Progress Tracking**: Watch your documents get processed in real-time with Server-Sent Events (SSE)
- **Instant AI Responses**: Streaming responses from 200+ AI models
- **Real-time Web Crawling**: See pages get discovered and processed as it happens
- **Operations Monitor**: Professional dashboard for tracking all processing activities

### 🧠 **Smart Document Processing**

**What Intelligence Do We Extract?**

- 📊 **Document Metrics** — word, character & line counts, reading-time and language detection.
- 🏷️ **AI Categorization** — classifies docs into technical, legal, business, academic and more.
- 🔍 **NER Extraction** — pulls people, orgs, places, dates and tech terms from text.
- 📝 **Structure Parsing** — detects headings, summaries and key points for fast skimming.
- 🔗 **Vector Embeddings** — 384-dim semantic vectors enable instant similarity search.
- 💡 **Smart Insights** — quality score, duplicate hash, processing time, confidence levels.
- 📄 **Multi-Format I/O** — PDF, Markdown, HTML, plain-text; Word & ePub on the way.
- 🔊 **Voice Streaming** — ElevenLabs TTS returns WAV chunks alongside chat tokens.
- 🧬 **Hybrid RAG Engine** — combines keyword BM25 with embedding search for accurate answers.
- 🌍 **Edge Deployment** — Rust API + Vercel Edge functions within 5 global regions.

### 🕸️ **Intelligent Web Crawling**
- Follows `robots.txt`, polite rate-limiting and retry back-off.
- Streams live URL discovery & status via SSE.
- Saves HTML, text and link graph into libSQL for analysis.

### 🔐 **Enterprise-Grade Auth**
- Magic-link email plus OAuth (GitHub, Google) via BetterAuth.
- JWT cookies: HTTP-only, SameSite, 7-day sliding window.
- Role-based ACL (Admin · Member · Viewer) on every route.

### ⚡ **Performance Beast**
- Rust async pipelines & zero-copy buffers keep memory low.
- Edge compute in 5 regions; P95 latency < 50 ms worldwide.
- Built-in Prometheus / tracing for real-time observability.

## 🏗️ Architecture

```txt
React SPA → Vercel Edge API → Rust Engine
             BetterAuth  PlanetScale  libSQL
```

## 🚀 Quick Start

_Prereqs: Rust 1.70+, Node 18+_

```bash
# Backend
cargo run --bin swoop_server
# Frontend
cd frontend && npm i && npm run dev
```

## 📋 Features (Phases 1-4)
- Auth & DB
- Backend Intelligence
- Frontend Polish
- Production Deploy

## 🔧 API Endpoints — quick curls

> Replace `<HOST>` with your Vercel domain or local `localhost:3000`.

### Health check

```bash
curl https://<HOST>/health
```

### Upload document (PDF/MD/HTML)

```bash
curl -F "file=@sample.pdf" \
     -H "Authorization: Bearer $SWOOP_API_KEY" \
     https://<HOST>/api/documents/upload
```

### Get document JSON

```bash
curl https://<HOST>/api/documents/{id}
```

### Stream processing progress (SSE)

```bash
curl -N https://<HOST>/api/documents/{id}/stream
```

### Chat with a document (sync)

```bash
curl -X POST https://<HOST>/api/chat \
     -H 'Content-Type: application/json' \
     -d '{"document_id":"{id}","message":"Summarise the key points"}'
```

### Chat with streaming tokens (SSE)

```bash
curl -N https://<HOST>/api/chat/stream?document_id={id}&q=What+is+the+goal
```

### Synthesised audio (TTS)

```bash
curl -L https://<HOST>/api/audio/{id}?voice=Rachel -o out.wav
```

### Start a crawl job

```bash
curl -X POST https://<HOST>/api/crawl \
     -H 'Content-Type: application/json' \
     -d '{"url":"https://rust-lang.org","depth":1}'
```

### Stream crawl progress

```bash
curl -N https://<HOST>/api/crawl/{job_id}/stream
```

## 🎨 Tech Stack
Rust · Axum · libSQL · React 18 · Tailwind · Vercel Edge

## 🤝 Contributing
Fork → branch → PR. Help welcome on formats, AI, UX.

## 📝 License
MIT
