# Swoop — Document Intelligence at Edge Speed 🚀

```txt
           ┌─────────────┐     SSE / WebSockets      ┌────────────┐
  PDF      │  Rust API   │  <----------------------  │  React 18  │
, HTML ──► │  (Axum)     │                           │   + SWC    │
  Docx     │  + RAG/TTS  │  ----------------------►  │   Tailwind │
           └─────────────┘     Chunked Audio & JSON  └────────────┘
```

Swoop turns ordinary documents & web pages into a **live knowledge base** you can chat with.  
Built for 2025 stacks: Rust-on-the-edge, hybrid Retrieval-Augmented Generation, and neural text-to-speech streams.

---

## 🍃 Why Swoop?

* **Instant answers** – semantic search + embeddings served from Qdrant, ~60 ms global.
* **Voice everywhere** – sentences TTS-synthesised on the fly via ElevenLabs.
* **Deploy anywhere** – Vercel Edge, Fly.io, bare metal; single command.
* **Privacy first** – no vendor lock-in, optional on-prem vector DB.

---

## Quick start

```bash
# 1. Backend (Rust)
cargo run --features "tts semantic-rag" --bin swoop_server

# 2. Frontend
cd frontend && npm i && npm run dev
```
Open http://localhost:5173 and drop a PDF.

---

## Folder tour (condensed)

```
/src              Rust core (API, RAG, TTS, storages)
/frontend         React 18 + shadcn/ui dashboard
/vercel-edge      Edge runtime (Turso + Auth)
/tests            Integration tests (SSE, audio)
```

---

## Dev notes

* 🦀  Minimum Rust 1.78-nightly (for async trait impls).  
* Node 18+, pnpm ≥8 recommended.
* `.env.example` shows **all** vars – keep secrets in `.env.local`.

---

## Roadmap

- [x] SSE streaming for crawl / chat
- [x] ElevenLabs TTS integration
- [ ] Whisper voice-input endpoint
- [ ] Multi-tenant "Spaces" with RBAC
- [ ] One-click deploy to edge-functions

---

## License
MIT — see `LICENSE` (feel free to fork, just don't sell it back to us 😉)
