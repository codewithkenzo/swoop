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

📊 **Document Metrics** – word, character & line counts, reading-time and language detection.
🏷️ **AI Categorization** – classifies docs into technical, legal, business, academic and more.
🔍 **NER Extraction** – pulls people, orgs, places, dates and tech terms from text.
📝 **Structure Parsing** – detects headings, summaries and key points for fast skimming.
🔗 **Vector Embeddings** – 384-dim semantic vectors enable instant similarity search.
💡 **Smart Insights** – quality score, duplicate hash, processing time, confidence levels.
📄 **Multi-Format I/O** – PDF, Markdown, HTML, plain-text; Word & ePub on the way.
🔊 **Voice Streaming** – ElevenLabs TTS returns WAV chunks alongside chat tokens.
🧬 **Hybrid RAG Engine** – combines keyword BM25 with embedding search for accurate answers.
🌍 **Edge Deployment** – Rust API + Vercel Edge functions within 5 global regions.

### 🕸️ **Intelligent Web Crawling**
### 🔐 **Enterprise-Grade Auth**
### ⚡ **Performance Beast**

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

## 🔧 API Endpoints
(See previous docs for full list)

## 🎨 Tech Stack
Rust · Axum · libSQL · React 18 · Tailwind · Vercel Edge

## 🤝 Contributing
Fork → branch → PR. Help welcome on formats, AI, UX.

## 📝 License
MIT
