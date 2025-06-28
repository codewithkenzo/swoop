# Swoop Deployment Guide (Edge-first)

> This walk-through gets you from zero → fully deployed on **Vercel Edge** + **Turso** in ~10 minutes.
>
> We assume you already forked the repo and pushed it to GitHub. If you're self-hosting or using Fly/GCP feel free to adapt – the env layout is identical.

---

## 1. Prerequisites

| Tool | Version | Why |
| ---- | -------- | --- |
| Vercel CLI | ≥ 33 | Project creation / CI |
| Turso CLI  | ≥ 0.76 | Spin up libSQL DB |
| Node       | ≥ 18   | Builds edge / UI |
| pnpm (optional) | ≥ 8 | faster installs |

---

## 2. Create the database (Turso)

```bash
# Authenticate (opens browser once)
turso auth login

# Create a free DB in the closest region
turso db create swoop-db --group starter

# Grab credentials
export TURSO_DATABASE_URL=$(turso db show swoop-db --url)
export TURSO_AUTH_TOKEN=$(turso db tokens create swoop-db --scope admin)
```

> libSQL is just SQLite + replication – perfect for edge latency.

### 2.1  Run migrations  
The Rust backend ships SQLx migrations under `/migrations`.

```bash
cd swoop
cargo install sqlx-cli --no-default-features --features rustls
sqlx migrate run \ 
  --database-url "$TURSO_DATABASE_URL?authToken=$TURSO_AUTH_TOKEN"
```

---

## 3. Configure environment vars

In Vercel dashboard → *Settings → Environment Variables* add:

| Key | Value |
| --- | ----- |
| `TURSO_DATABASE_URL` | `$TURSO_DATABASE_URL` |
| `TURSO_AUTH_TOKEN`   | `$TURSO_AUTH_TOKEN`   |
| `OPENROUTER_API_KEY` | your-openrouter-key |
| `SWOOP_API_KEY`      | generate-something-long |
| `ELEVENLABS_API_KEY` | _(optional)_ enable TTS |

The keys match `vercel-edge/vercel.json`.

---

## 4. Link & deploy

```bash
# at repo root
pnpm install   # or npm i – installs UI deps
vercel link    # follow prompts
vercel --prod  # first deploy, creates edge + serverless functions
```

The build runs two pipelines:

1. **Edge Functions** under `api/edge/**` (runtime = `edge`).
2. **Serverless** under `api/serverless/**` (runtime = `nodejs18.x`).

The `vercel.json` routes section already wires paths → correct runtime.

---

## 5. Verify health

```bash
curl https://<your-vercel-domain>/health            # JSON OK
curl -N https://<your-vercel-domain>/api/edge/health  # edge check
```

Upload a doc from the UI, watch SSE stream & wave-audio.

---

## 6. (Optional) Deploy standalone Rust API

If you want Rust API outside Vercel (e.g. on Fly):

```bash
cargo run --release --features "tts semantic-rag" \
  -- --port 8080 \
  --database-url "$TURSO_DATABASE_URL?authToken=$TURSO_AUTH_TOKEN"
```

Point the React frontend env `VITE_SWOOOP_API` to that host.

---

## 7. Troubleshooting

| Symptom | Fix |
| ------- | ---- |
| 500 on `/api/documents/upload` | Confirm Turso vars + migrations |
| CORS blocked | `vercel.json` already adds `Access-Control-Allow-*`; purge cache |
| TTS 501 | Compile Rust with `--features tts` **and** set `ELEVENLABS_API_KEY` |

---

**You're live!**  
Hit `https://<domain>` – unauthenticated users land on the animated hero, auth users on the dashboard.

Happy swooping 🦅 