# Swoop – Deployment & Environment Configuration Guide

> Covers local development, environment variables, and production deployment to **Vercel Edge Functions** with BetterAuth, PlanetScale/Turso, and multi-LLM providers (OpenAI, Anthropic, Google Gemini, OpenRouter).

---

## 1. Repository Pre-Requisites

1. **Node 18+** with **pnpm** 8.x
2. **Rust 1.75+** with `wasm32-unknown-unknown` target (for edge bindings)
3. **Vercel CLI** `npm i -g vercel`
4. A **PlanetScale** (MySQL) _or_ **Turso** (libSQL) database
5. SMTP credentials or an email provider (Brevo, Resend, Postmark) for magic-link sign-in.

---

## 2. Environment Variables Reference

Copy `vercel-edge/env.example` → `.env` (local) **and** paste into Vercel → *Project ▷ Settings ▷ Environment Variables*.

| Key | Required? | Description |
|-----|-----------|-------------|
| `DATABASE_URL` | ✅ | PlanetScale _mysql://…_ **or** Turso _libsql://…_ URL (Data Proxy allowed) |
| `PRISMA_DB_PROVIDER` | ✅ | `mysql` or `libsql` (used by Prisma Data Proxy) |
| `BETTERAUTH_SECRET` | ✅ | 32+ byte random string (`openssl rand -hex 32`) |
| `EMAIL_SERVER` | ✅ | SMTP URI, e.g. `smtp://user:pass@mailserver:587` |
| `EMAIL_FROM` | ✅ | Friendly from header, e.g. `Swoop <noreply@swoop.ai>` |
| `GITHUB_CLIENT_ID / SECRET` | ⛔ (opt) | GitHub OAuth keys (https://github.com/settings/developers) |
| `GOOGLE_CLIENT_ID / SECRET` | ⛔ (opt) | Google OAuth keys (OAuth Consent Screen in GCP) |
| `OPENAI_API_KEY` | ⛔ | OpenAI completion & embedding models |
| `ANTHROPIC_API_KEY` | ⛔ | Claude models (via Anthropic API) |
| `GEMINI_API_KEY` | ⛔ | Google Gemini Pro models |
| `OPENROUTER_API_KEY` | ⛔ | OpenRouter LLM routing hub |
| `ELEVENLABS_API_KEY` | ⛔ | ElevenLabs TTS API key – enables real audio synthesis |
| `ELEVENLABS_VOICE_ID` | ⛔ | Override default voice (optional) |
| `TURSO_AUTH_TOKEN` | only for Turso | JWT from `turso db tokens` |
| `NEXTAUTH_URL` | prod only | Full canonical URL (e.g. `https://swoop.vercel.app`) |

💡 **Tip:** for local dev run `echo "BETTERAUTH_SECRET=$(openssl rand -hex 32)" >> .env`.

---

## 3. Local Development

```bash
# 1) Install JS deps & generate Prisma client
pnpm install

# 2) Start Rust API (dev mode)
cargo run --bin swoop_server

# 3) Start Vite React frontend
pnpm --filter frontend dev

# 4) Edge layer (auth + uploads) via Vercel dev
cd vercel-edge
vercel dev --listen 3003
```

Open http://localhost:5173 – the React app proxies API calls to Rust port (configurable) and auth to Vercel Edge port 3003.

---

## 4. Deploying to Vercel

1. **Link repo** → `vercel link` (choose/new project).
2. **Set env vars**: `vercel env pull` will fetch them locally. Use dashboard for secrets.
3. **Edge Functions memory/timeout**: set to **256 MB / 10 s** in *Project ▷ Settings ▷ Functions*.
4. **Build Command** (root):
   ```bash
   pnpm install
   pnpm --filter vercel-edge install
   pnpm --filter vercel-edge run build   # Next.js build for edge layer
   cargo build --release                 # compile Rust binaries
   ```
   The provided `vercel.json` routes `/api/*` to edge/serverless runtime automatically.
5. **CI Gates**: Vercel executes `pnpm run build` which in `vercel-edge/package.json` already runs `prisma generate`. Add optional step in **GitHub Actions** to run `cargo clippy -- -D warnings`.
6. **Preview Protection**: enable Password-Protected Previews or Vercel SSO to ensure non-prod URLs remain private.

---

## 5. Service Integrations

### BetterAuth (Edge)
* All auth actions are handled by `/api/auth/[...route].ts` in the edge runtime.
* Secure, HTTP-only cookies with 7-day TTL are configured in `vercel-edge/lib/auth.ts`.
* RBAC: user `role` field defaults to `USER`; assign `ADMIN` in DB as needed.

### PlanetScale
* Create a **branch**, promote to prod; copy **Proxy URL**.
* Add to `DATABASE_URL`. Prisma Data Proxy pooling keeps Edge cold-starts cheap.

### Turso (libSQL)
* `turso db create swoop` → `turso db tokens create swoop --scope all`
* Use `DATABASE_URL="libsql://swoop.turso.io"` and `TURSO_AUTH_TOKEN`.

### LLM Providers
* **OpenAI**: https://platform.openai.com/account/api-keys
* **Anthropic (Claude)**: https://console.anthropic.com/account/keys
* **Gemini Pro**: https://aistudio.google.com/app/apikey
* **OpenRouter**: https://openrouter.ai/keys – supports multi-model routing.

The current code will automatically pick up whichever keys are provided and route requests accordingly (see `llm/routing.rs`).

---

## 6. Operational Tips

| Area | Recommendation |
|------|----------------|
Observability (Optional) If you already have a **Sentry** account you can set `SENTRY_DSN` to capture runtime exceptions.  Otherwise leave it blank; the platform will still run just fine.  You can also rely on **Vercel Analytics** (free) or self-hosted OpenTelemetry + Grafana for metrics. |
| **Edge Cold-Start** | Keep Prisma Data Proxy on; ensure `prisma` version ≥ 5.12. |
| **Storage** | For >10 MB uploads consider S3/Cloudflare R2 and store pointer in DB. |
| **Scaling** | Edge functions scale automatically; heavy ML should be off-loaded to Rust API or background workers. |
| **Security** | Rotate JWT & database creds every 90 days; enforce OAuth only in production. |

---

## 7. Running Smoke Tests

```bash
# Lint + build (frontend)
pnpm run lint && pnpm run build
# Static typecheck (vercel-edge)
cd vercel-edge && pnpm type-check
# Rust clippy
task clippy   # or cargo clippy -- -D warnings
```

---

🎉  You're ready to ship Swoop to the world!  Push to your `main` branch and `vercel --prod` for a permanent production URL. 

> ℹ️  **Rust build flag:** To include TTS at runtime compile binaries with `--features tts`:

```bash
cargo build --release --features tts
```

When the feature is omitted, endpoints `/api/audio/{id}` and `/api/voice-chat` return **501 Not Implemented** but the remainder of the platform functions normally. 