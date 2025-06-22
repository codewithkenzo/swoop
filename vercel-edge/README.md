# Swoop Vercel Edge Solution

A globally distributed, serverless-first implementation of the Swoop document intelligence platform, optimized for edge computing with sub-50ms response times worldwide.

## 🌐 Architecture Overview

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Edge Runtime  │    │ Serverless Node  │    │ Turso Database  │
│                 │    │                  │    │                 │
│ • Health checks │    │ • File uploads   │    │ • Global sync   │
│ • Auth verify   │    │ • AI processing  │    │ • <50ms latency │
│ • Doc retrieval │    │ • Heavy compute  │    │ • Auto-scaling  │
│ • Fast queries  │    │ • LLM integration│    │ • Multi-region  │
└─────────────────┘    └──────────────────┘    └─────────────────┘
        │                        │                        │
        └────────────────────────┼────────────────────────┘
                                 │
                    ┌──────────────────┐
                    │ OpenRouter API   │
                    │                  │
                    │ • Multi-model    │
                    │ • Global access  │
                    │ • Pay-per-use    │
                    └──────────────────┘
```

## 🚀 Quick Start

### Prerequisites
- Node.js 18+ and npm
- Vercel CLI (`npm install -g vercel`)
- Turso account ([turso.tech](https://turso.tech))
- OpenRouter API key ([openrouter.ai](https://openrouter.ai))

### 1. Setup Database
```bash
# Install Turso CLI
curl -sSfL https://get.tur.so/install.sh | bash

# Create database
turso db create swoop-production
turso db tokens create swoop-production

# Get connection details
turso db show swoop-production
```

### 2. Deploy to Vercel
```bash
# Clone and navigate
git clone https://github.com/codewithkenzo/swoop.git
cd swoop/vercel-edge

# Run deployment script
./deploy.sh

# For production deployment
./deploy.sh --production
```

### 3. Configure Environment
The deployment script will create `.env.local` template:
```bash
# Edit with your credentials
nano .env.local
```

## 📁 Project Structure

```
vercel-edge/
├── api/
│   ├── edge/                    # Edge Runtime functions (<10ms)
│   │   ├── health.ts           # Health monitoring
│   │   ├── auth/
│   │   │   └── verify.ts       # API key validation
│   │   └── documents/
│   │       └── [id].ts         # Document retrieval/deletion
│   └── serverless/             # Node.js functions (heavy processing)
│       ├── documents/
│       │   └── upload.ts       # File upload & processing
│       └── llm/
│           └── chat.ts         # AI chat integration
├── lib/
│   └── database.ts             # Turso database client
├── types/
│   └── index.ts                # TypeScript definitions
├── vercel.json                 # Deployment configuration
├── package.json                # Dependencies
├── deploy.sh                   # Deployment script
└── README.md                   # This file
```

## 🔧 API Endpoints

### Edge Functions (Ultra-Fast)
```bash
# Health check - Global monitoring
GET /health
Response: {"status": "healthy", "edge_region": "iad1", "response_time_ms": 12}

# Document retrieval - Cached globally
GET /api/documents/doc_123
Response: {"document": {...}, "edge_region": "sfo1"}

# Authentication - Fast validation
POST /api/auth/verify
Body: {"api_key": "your-key"}
Response: {"valid": true, "user": {...}}

# Document deletion - Instant
DELETE /api/documents/doc_123
Headers: Authorization: Bearer your-key
Response: {"success": true, "deleted_at": "2024-01-15T10:30:00Z"}
```

### Serverless Functions (Heavy Processing)
```bash
# Document upload - Full processing pipeline
POST /api/documents/upload
Headers: 
  Authorization: Bearer your-key
  Content-Type: multipart/form-data
Body: file=@document.pdf
Response: {
  "document_id": "doc_abc123",
  "status": "completed",
  "analysis": {
    "emails": ["contact@example.com"],
    "phones": ["+1-555-123-4567"],
    "quality_score": 0.95
  }
}

# AI Chat - LLM integration
POST /api/llm/chat
Headers: 
  Authorization: Bearer your-key
  Content-Type: application/json
Body: {
  "messages": [{"role": "user", "content": "Summarize this document"}],
  "document_ids": ["doc_123"],
  "model": "openai/gpt-4o-mini"
}
Response: {
  "id": "chat_456",
  "choices": [{"message": {"content": "Document summary..."}}],
  "usage": {"total_tokens": 150}
}
```

## 🌍 Global Performance

### Edge Regions
- **IAD1** (Washington DC) - North America East
- **SFO1** (San Francisco) - North America West  
- **FRA1** (Frankfurt) - Europe
- **HND1** (Tokyo) - Asia Pacific
- **SYD1** (Sydney) - Oceania

### Performance Targets
- **Edge Functions**: <10ms response time
- **Database Queries**: <50ms globally via Turso
- **Document Retrieval**: <100ms with caching
- **File Upload**: <5s for documents up to 10MB
- **AI Processing**: <30s for complex analysis

## 🔐 Security Features

### Authentication
- Bearer token authentication for all protected endpoints
- API key validation with user tier management
- Rate limiting per user tier (Free: 100/day, Premium: 10K/day)

### Data Protection
- Automatic PII detection and redaction
- Secure document storage with encryption at rest
- CORS headers for browser security
- Environment variable protection

### Privacy Compliance
- Document retention policies
- User data deletion capabilities
- Audit logging for compliance
- GDPR-ready data handling

## 📊 Monitoring & Analytics

### Built-in Monitoring
```bash
# Health endpoint provides:
{
  "status": "healthy",
  "version": "1.0.0",
  "uptime_seconds": 3600,
  "response_time_ms": 15,
  "edge_region": "iad1",
  "checks": {
    "database": true,
    "llm_service": true,
    "storage": true
  }
}
```

### Vercel Analytics
- Automatic function performance monitoring
- Error tracking and alerting
- Usage analytics and billing
- Custom metrics via Vercel dashboard

## 🚧 Development

### Local Development
```bash
# Install dependencies
npm install

# Run TypeScript checks
npx tsc --noEmit

# Test functions locally (requires Vercel CLI)
vercel dev

# Access local endpoints
curl http://localhost:3000/health
```

### Testing
```bash
# Test edge functions
curl -X GET "https://your-deployment.vercel.app/health"

# Test document upload
curl -X POST \
  -H "Authorization: Bearer your-api-key" \
  -F "file=@test-document.pdf" \
  "https://your-deployment.vercel.app/api/documents/upload"

# Test AI chat
curl -X POST \
  -H "Authorization: Bearer your-api-key" \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"Hello!"}]}' \
  "https://your-deployment.vercel.app/api/llm/chat"
```

## 🔄 Deployment Options

### Preview Deployment (Testing)
```bash
./deploy.sh
# Creates a preview URL for testing
```

### Production Deployment
```bash
./deploy.sh --production
# Deploys to your production domain
```

### Custom Domain
```bash
# Add custom domain in Vercel dashboard
# Configure DNS records
# SSL certificates handled automatically
```

## 📈 Scaling & Costs

### Automatic Scaling
- **Edge Functions**: Scale to zero, unlimited concurrent executions
- **Serverless Functions**: Auto-scale based on demand
- **Database**: Turso handles global replication and scaling
- **CDN**: Vercel's global CDN for static assets

### Cost Optimization
- **Edge Runtime**: Extremely cost-effective for light operations
- **Serverless**: Pay-per-execution for heavy processing
- **Database**: Turso's pay-per-query model
- **LLM**: OpenRouter's competitive multi-model pricing

### Usage Tiers
- **Hobby**: Free tier with generous limits
- **Pro**: $20/month with higher limits
- **Enterprise**: Custom pricing for high-volume usage

## 🛠️ Customization

### Adding New Edge Functions
1. Create function in `api/edge/`
2. Export default handler with `runtime = 'edge'`
3. Update `vercel.json` routes if needed
4. Deploy with `./deploy.sh`

### Adding Serverless Functions
1. Create function in `api/serverless/`
2. Use Node.js runtime features
3. Configure `maxDuration` in `vercel.json`
4. Deploy with `./deploy.sh`

### Database Schema Changes
1. Update `lib/database.ts` methods
2. Update TypeScript types in `types/index.ts`
3. Run migrations on Turso database
4. Deploy updated functions

## 🔗 Integration

### Frontend Integration
```typescript
// React/Next.js integration
const api = new SwoopClient('https://your-deployment.vercel.app');

// Upload document
const result = await api.uploadDocument(file);

// Chat with AI
const response = await api.chat([
  { role: 'user', content: 'Analyze this document' }
], [result.document_id]);
```

### Backend Integration
```bash
# Webhook integration
curl -X POST \
  -H "Authorization: Bearer your-api-key" \
  -H "Content-Type: application/json" \
  -d '{"webhook_url": "https://your-app.com/webhook"}' \
  "https://your-deployment.vercel.app/api/webhooks/register"
```

## 📞 Support

- **Documentation**: [Swoop GitHub](https://github.com/codewithkenzo/swoop)
- **Vercel Support**: [vercel.com/support](https://vercel.com/support)
- **Turso Support**: [turso.tech/support](https://turso.tech/support)
- **OpenRouter Support**: [openrouter.ai/docs](https://openrouter.ai/docs)

---

**🎉 Your Swoop platform is now running globally on Vercel Edge!**

Experience sub-50ms response times worldwide with automatic scaling, built-in security, and comprehensive document intelligence capabilities. 