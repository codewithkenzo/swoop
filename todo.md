# Swoop Platform - Cursor Development Tasks

## Meta Guidelines
- Mark ONE task in_progress at a time in todo.md before starting
- After finishing, mark it completed, push, and ping Claude for review
- Each task should be atomic and testable with clear deliverables

## 0. Meta
- [x] Create todo.md with all granular tasks
- [ ] Maintain single task in_progress workflow

## 1. Environment & Database - Phase 0

### 1.1 (cursor-env-docker) - COMPLETED
- [x] Add docker-compose.yml with services:
  - postgres:14 (expose 5432)
  - pgadmin (optional)
- [x] Create backend/.env template with DATABASE_URL and other config (created as config.template)
- [x] Add scripts/dev/db-init.sql to create db/user/demo tables

### 1.2 (cursor-env-seed) - COMPLETED
- [x] Write scripts/dev/seed.rs (or SQL) to insert:
  - 3 demo documents (PDF + HTML)
  - 5 crawl seed URLs
  - 1 sample chat conversation
- [x] Add make seed entry in Makefile

### 1.3 (cursor-env-ci) - COMPLETED
- [x] Update .github/workflows/ci.yml to spin up Postgres container
- [x] Run cargo test + frontend npm test in CI

## 2. Backend + DB Integration

### 2.1 (cursor-api-config) - COMPLETED
- [x] Inject DATABASE_URL via config.rs
- [x] Make storage::postgres implementation behind feature postgres
- [x] Add graceful fallback to in-memory SQLite if env var missing

### 2.2 (cursor-endpoint-metrics) - PENDING
- [ ] Expose /api/metrics JSON + /metrics Prometheus with no AppState requirement
- [ ] Use src/monitoring.rs helpers → add thin wrappers in swoop_server.rs
- [ ] Update frontend getMetrics() if path changes

### 2.3 (cursor-endpoint-settings) - PENDING
- [ ] POST /api/settings (JSON) → persist to settings table
- [ ] GET /api/settings → return merged defaults + overrides

### 2.4 (cursor-tests-integration) - PENDING
- [ ] Add tests/db_integration.rs
- [ ] Ensure upload inserts row, crawl creates job row, chat writes conversation row
- [ ] Use sqlx::query! assertions

## 3. E2E Automated Tests (browsermcp)

### 3.1 (cursor-e2e-upload) - PENDING
- [ ] Happy-path: upload PDF → appears in list in ≤5s
- [ ] Error: upload >100MB → toast "File too large"

### 3.2 (cursor-e2e-crawl) - PENDING
- [ ] Start crawl, stream reaches completed, progress bar hits 100%
- [ ] Cancel button stops job, status → cancelled

### 3.3 (cursor-e2e-search) - PENDING
- [ ] Search "example" returns ≥1 doc, clicking opens detail page

### 3.4 (cursor-e2e-chat) - PENDING
- [ ] Ask question, receive response, SSE stream closes gracefully

### 3.5 (cursor-e2e-settings) - PENDING
- [ ] Toggle Dark-mode + Advanced-crawl; reload page → settings persist

### 3.6 (cursor-e2e-metrics) - PENDING
- [ ] Navigate to Monitoring page → charts populated, 200 OK for /api/metrics

## 4. Frontend Polish & Accessibility

### 4.1 (cursor-ui-buttons) - PENDING
- [ ] Disable buttons while loading; show spinner inside button (use @radix-ui/react-spinner)

### 4.2 (cursor-ui-toasts) - PENDING
- [ ] Global toast provider; success/error messages from hooks

### 4.3 (cursor-ui-skeletons) - PENDING
- [ ] Add skeleton components for document list & crawl progress

### 4.4 (cursor-a11y) - PENDING
- [ ] Ensure all inputs have labels, color contrast ≥ 4.5, keyboard navigation on main flows

## 5. Monitoring & Alerts

### 5.1 (cursor-frontend-charts) - PENDING
- [ ] Use recharts to show: requests/min, active connections, memory

### 5.2 (cursor-alert-errors) - PENDING
- [ ] Global Axios/Fetch interceptor → toast on network error

### 5.3 (cursor-backend-exporter) - PENDING
- [ ] Add optional --prometheus-port CL flag to export metrics on :9090 for external scraping

## 6. Documentation & Handoff

### 6.1 (cursor-docs-update) - PENDING
- [ ] Update README.md (local setup, docker-compose, seeding)
- [ ] Add DEV_GUIDE.md (running tests, lint, fmt, watch)

### 6.2 (cursor-todo-sync) - PENDING
- [ ] After each PR, update todo.md task status and link PR/commit SHA
- [ ] Tag Claude in PR for async review (@Claude-Architect please review acceptance criteria X-Y)

## 7. Architect/QA Review Items (from SPEC.md)

### 7.1 (architect-accessibility) - PENDING
- [ ] Add comprehensive tooltips for all metrics and AI features
- [ ] Implement skip-to-content links for keyboard navigation
- [ ] Add ARIA labels to all interactive components
- [ ] Improve keyboard focus indicators throughout the application
- [ ] Add role attributes to custom components
- [ ] Test and fix tab order across all pages

### 7.2 (architect-mobile) - PENDING
- [ ] Fix table display on mobile devices (horizontal scroll or card layout)
- [ ] Test landscape orientation on tablets
- [ ] Optimize touch target sizes for mobile
- [ ] Test all features on mobile devices

### 7.3 (architect-api-docs) - PENDING
- [ ] Complete OpenAPI/Swagger documentation for all endpoints
- [ ] Add code examples for common API usage patterns
- [ ] Document authentication and authorization requirements
- [ ] Add error response documentation with examples
- [ ] Implement interactive API documentation (Swagger UI)

### 7.4 (architect-demo-scripts) - PENDING
- [ ] Prepare demo data sets (1000+ documents)
- [ ] Set up demo websites for crawling
- [ ] Rehearse and time all demo scripts
- [ ] Test demo scenarios in various environments
- [ ] Create demo reset and cleanup scripts

### 7.5 (architect-performance) - PENDING
- [ ] Fix remaining 2 test failures (95.3% → 100%)
- [ ] Add performance benchmark tests
- [ ] Implement connection pooling for database operations
- [ ] Add caching layer for frequently accessed data
- [ ] Optimize bundle size and implement code splitting

### 7.6 (architect-security) - PENDING
- [ ] Implement Content Security Policy (CSP)
- [ ] Add input validation and sanitization
- [ ] Implement rate limiting for API endpoints
- [ ] Add security headers to all responses
- [ ] Add vulnerability scanning automation

## Task Status Legend
- [ ] PENDING - Not started
- [x] IN_PROGRESS - Currently working on
- [x] COMPLETED - Finished and ready for review
- [x] REVIEWED - Claude has reviewed and approved

## Feature Readiness Matrix (from SPEC.md)
| Feature | Status | Demo Ready | Production Ready | Notes |
|---------|--------|------------|------------------|-------|
| Document Upload | ✅ Complete | ✅ Yes | ✅ Yes | 100% acceptance criteria met |
| Web Crawler | ✅ Complete | ✅ Yes | ✅ Yes | 77.9 pages/sec performance |
| AI Analysis | ✅ Complete | ✅ Yes | ✅ Yes | 95% accuracy, needs polish |
| Real-time Streaming | ✅ Complete | ✅ Yes | ✅ Yes | SSE architecture working |
| Search Engine | ✅ Complete | ✅ Yes | ✅ Yes | Semantic search functional |
| Chat Interface | ✅ Complete | ⚠️ Partial | ⚠️ Partial | Needs UI polish |
| Settings | ✅ Complete | ✅ Yes | ✅ Yes | Theme/preferences working |
| Monitoring | ✅ Complete | ✅ Yes | ✅ Yes | Real-time metrics |
| Authentication | ✅ Complete | ✅ Yes | ✅ Yes | Session-based auth |
| Database Integration | ✅ Complete | ✅ Yes | ✅ Yes | SQLite/LibSQL working |
| Mobile Experience | ⚠️ Partial | ⚠️ Partial | ❌ No | Needs responsive fixes |
| Accessibility | ⚠️ Partial | ❌ No | ❌ No | Needs ARIA, keyboard nav |
| API Documentation | ⚠️ Partial | ❌ No | ❌ No | Needs OpenAPI completion |

## Claude-Code Async Tasks (do not overlap Cursor tasks)

###  A. Landing Page & Marketing

#### A.1 (claude-ui-landing-design) - PENDING
- [ ] Draft a `frontend/src/pages/Landing.tsx` redesign proposal in `docs/landing_design.md` including:
  - Wireframe sketches (Markdown diagrams)
  - Color palette (dark + vibrant accent)
  - Font & spacing guidelines
  - Animation concepts (Framer Motion)

#### A.2 (claude-ui-landing-hero) - PENDING
- [ ] Implement Hero section (`components/marketing/Hero.tsx`) per design with:
  - Responsive layout
  - Background gradient animation
  - CTA button group ("Get Started" / "View Demo")
  - Dark-mode aware styles

#### A.3 (claude-ui-landing-pricing) - PENDING
- [ ] Create Pricing tables component (`components/marketing/Pricing.tsx`) supporting three tiers with monthly/yearly toggle.

#### A.4 (claude-ui-landing-assets) - PENDING
- [ ] Add SVG wave divider, logo variants, and favicon assets under `frontend/public/`.

###  B. Documentation & Planning

#### B.1 (claude-docs-update-cleanup) - PENDING
- [ ] Refresh `CLEANUP_AND_PRIORITIZATION_ANALYSIS.md` to reflect latest compile status & removed errors (use green ✅ for resolved items).

#### B.2 (claude-docs-demo-plan) - PENDING
- [ ] Update `AGGRESSIVE_CRAWLER_DEMO_PLAN.md` with new benchmark targets (aim 150 URLs/s) and integration checkpoints.

###  C. DevOps & Deployment

#### C.1 (claude-devops-vercel-domain) - PENDING
- [ ] Research available `.ai` or `.app` domains via Namecheap API; propose top 3 names in `docs/domain_options.md`.
- [ ] Draft Vercel configuration (`vercel.json`) with edge functions routing.

###  D. Code Quality Upgrades

#### D.1 (claude-refactor-base64) - PENDING
- [ ] Replace deprecated `base64::encode/decode` with `engine.encode/engine.decode` across codebase; submit PR with zero warnings.

---
> Note: Claude-Code should mark ONE task as `IN_PROGRESS` at a time under this section and open a dedicated branch `claude/<task-id>` for each PR. 