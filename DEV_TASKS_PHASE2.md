# Phase 2 Development Checklist – Backend Features

> This file tracks in-progress engineering tasks. Remove items as they land in code. **Do not ship with production artifacts.**

## Document Processing Pipeline

- [ ] Detect MIME type → choose loader (PDF, HTML, Markdown, Plain-text)
- [ ] Extract plain text via `extractors::extract_text`
- [ ] Run AI analysis modules
  - [ ] `ai::ner::EntityExtractor` – capture entities + confidence
  - [ ] `ai::categorization::DocumentCategorizer` – assign `DocumentCategory`
  - [ ] `ai::embeddings::DocumentEmbedder` – generate 384-dim sentence embedding
- [ ] Persist `Document`, `DocumentVector`, and `entities` to `storage::LibSqlStorage`
- [ ] Broadcast `DocumentProcessingStatus` updates over `tokio::sync::broadcast::Sender<AppState>`

## Crawler Results & Endpoints

- [ ] Extend crawler to write `{url, status_code, text_len}` rows
- [ ] Expose `GET /api/crawl/:id/results` (edge-friendly, paginated)

## Streaming (SSE)

- [ ] Implement `/api/documents/:id/stream` to push processing progress
- [ ] Implement `/api/crawl/:id/stream` to push fetch progress

## Misc

- [ ] Unit tests for categorizer & embedder integration
- [ ] Benchmarks for large batch embedding throughput
- [ ] Docs update once stable 