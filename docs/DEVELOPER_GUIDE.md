# Swoop Developer Guide

## Quick Start

Get your local development environment up and running in minutes.

### Prerequisites

- **Rust** (latest stable) - [Install via rustup](https://rustup.rs/)
- **Node.js** 18+ - [Download](https://nodejs.org/)
- **Git** - [Download](https://git-scm.com/)

### One-Command Setup

```bash
# Clone and setup everything
git clone https://github.com/your-org/swoop.git
cd swoop
./scripts/dev-setup.sh
```

### Manual Setup

```bash
# 1. Backend dependencies
cargo build

# 2. Frontend dependencies  
cd frontend
npm install

# 3. Environment configuration
cp .env.example .env
# Edit .env with your API keys

# 4. Start development servers
cargo run --bin swoop_server &
cd frontend && npm run dev
```

## Development Workflow

### Project Structure
```
swoop/
├── src/                    # Rust backend
│   ├── lib.rs             # Core library
│   ├── models.rs          # Data structures
│   ├── document_processor.rs
│   ├── llm/               # AI integration
│   ├── storage/           # Database layers
│   ├── crawler.rs         # Web crawling
│   └── bin/
│       └── swoop_server.rs
├── frontend/              # React frontend
│   ├── src/
│   │   ├── components/
│   │   ├── pages/
│   │   ├── hooks/
│   │   └── lib/
│   ├── package.json
│   └── vite.config.ts
├── docs/                  # Documentation
├── tests/                 # Test suites
└── Cargo.toml
```

### Daily Development

**Backend Development:**
```bash
# Start server with hot reload
cargo watch -x 'run --bin swoop_server'

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt
```

**Frontend Development:**
```bash
cd frontend

# Start dev server
npm run dev

# Run tests
npm test

# Type checking
npm run type-check

# Build for production
npm run build
```

### Common Commands

```bash
# Full test suite
cargo test && cd frontend && npm test

# Lint everything
cargo clippy && cargo fmt && cd frontend && npm run lint

# Build everything
cargo build --release && cd frontend && npm run build

# Clean build artifacts
cargo clean && cd frontend && rm -rf dist node_modules
```

## Environment Configuration

### Required Environment Variables

```bash
# .env file
# API Keys
OPENROUTER_API_KEY=your-openrouter-key
ELEVENLABS_API_KEY=your-elevenlabs-key

# Database
DATABASE_URL=sqlite:./swoop.db
REDIS_URL=redis://localhost:6379

# Security
JWT_SECRET=your-jwt-secret
API_KEY_SALT=your-api-key-salt

# External Services
QDRANT_URL=http://localhost:6333
QDRANT_API_KEY=optional-qdrant-key

# Development
RUST_LOG=debug
LOG_LEVEL=debug
```

### Optional Configuration

```bash
# Performance
MAX_UPLOAD_SIZE=10485760  # 10MB
WORKER_THREADS=4
CONNECTION_POOL_SIZE=20

# Features
ENABLE_CRAWLING=true
ENABLE_TTS=true
ENABLE_CHAT=true

# Rate Limiting
RATE_LIMIT_REQUESTS=100
RATE_LIMIT_WINDOW=60
```

## API Development

### Adding New Endpoints

1. **Define the handler function:**
```rust
// src/bin/swoop_server.rs
async fn my_new_handler(
    Json(payload): Json<MyRequest>,
) -> Result<Json<MyResponse>, AppError> {
    // Your logic here
    Ok(Json(MyResponse { /* ... */ }))
}
```

2. **Add the route:**
```rust
let app = Router::new()
    .route("/api/my-endpoint", post(my_new_handler))
    // ... other routes
```

3. **Define request/response types:**
```rust
// src/models.rs
#[derive(Deserialize)]
pub struct MyRequest {
    pub field: String,
}

#[derive(Serialize)]
pub struct MyResponse {
    pub result: String,
}
```

4. **Add tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_my_endpoint() {
        // Test implementation
    }
}
```

### Error Handling

```rust
// Custom error types
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("External service error: {0}")]
    ExternalService(String),
}

// Error response implementation
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::ExternalService(_) => (StatusCode::SERVICE_UNAVAILABLE, "Service unavailable"),
        };
        
        let body = Json(json!({
            "error": error_message,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }));
        
        (status, body).into_response()
    }
}
```

## Frontend Development

### Component Structure

```typescript
// src/components/MyComponent.tsx
import React from 'react';
import { useQuery } from '@tanstack/react-query';
import { api } from '../lib/api';

interface MyComponentProps {
  id: string;
}

export const MyComponent: React.FC<MyComponentProps> = ({ id }) => {
  const { data, isLoading, error } = useQuery({
    queryKey: ['my-data', id],
    queryFn: () => api.getMyData(id),
  });

  if (isLoading) return <div>Loading...</div>;
  if (error) return <div>Error: {error.message}</div>;

  return (
    <div className="p-4 border rounded">
      <h2 className="text-xl font-bold">{data.title}</h2>
      <p>{data.description}</p>
    </div>
  );
};
```

### API Client

```typescript
// src/lib/api.ts
import axios from 'axios';

const client = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080',
  headers: {
    'Content-Type': 'application/json',
  },
});

// Request interceptor for auth
client.interceptors.request.use((config) => {
  const token = localStorage.getItem('auth-token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Response interceptor for errors
client.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // Handle unauthorized
      localStorage.removeItem('auth-token');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

export const api = {
  // Documents
  uploadDocument: (file: File) => {
    const formData = new FormData();
    formData.append('file', file);
    return client.post('/api/documents/upload', formData);
  },
  
  getDocuments: () => client.get('/api/documents'),
  
  // Chat
  sendMessage: (message: string, documentId?: string) => {
    return client.post('/api/chat', { message, documentId });
  },
  
  // ... other API methods
};
```

### Custom Hooks

```typescript
// src/hooks/useDocuments.ts
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { api } from '../lib/api';

export const useDocuments = () => {
  return useQuery({
    queryKey: ['documents'],
    queryFn: api.getDocuments,
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
};

export const useUploadDocument = () => {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: api.uploadDocument,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['documents'] });
    },
  });
};
```

## Database Development

### Schema Migrations

```sql
-- migrations/001_initial.sql
CREATE TABLE documents (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    content_type TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_documents_created_at ON documents(created_at);
CREATE INDEX idx_documents_content_type ON documents(content_type);
```

### Database Queries

```rust
// src/storage/mod.rs
use sqlx::{Pool, Sqlite};

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = sqlx::sqlite::SqlitePool::connect(database_url).await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(Self { pool })
    }
    
    pub async fn create_document(&self, doc: &Document) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO documents (id, title, content, content_type, file_size) 
             VALUES (?, ?, ?, ?, ?)",
            doc.id,
            doc.title,
            doc.content,
            doc.content_type,
            doc.file_size
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn get_documents(&self) -> Result<Vec<Document>, sqlx::Error> {
        let documents = sqlx::query_as!(
            Document,
            "SELECT * FROM documents ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(documents)
    }
}
```

## Testing

### Backend Tests

```rust
// tests/integration_tests.rs
use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;
use swoop::create_app;

#[tokio::test]
async fn test_upload_document() {
    let app = create_app().await;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/documents/upload")
                .header("content-type", "multipart/form-data")
                .body(Body::from("test content"))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}
```

### Frontend Tests

```typescript
// src/components/__tests__/MyComponent.test.tsx
import { render, screen } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { MyComponent } from '../MyComponent';

const createWrapper = () => {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: { retry: false },
    },
  });
  
  return ({ children }: { children: React.ReactNode }) => (
    <QueryClientProvider client={queryClient}>
      {children}
    </QueryClientProvider>
  );
};

test('renders component with data', async () => {
  render(<MyComponent id="test-id" />, { wrapper: createWrapper() });
  
  expect(screen.getByText('Loading...')).toBeInTheDocument();
  
  // Wait for data to load
  await screen.findByText('Test Title');
  expect(screen.getByText('Test Title')).toBeInTheDocument();
});
```

## Debugging

### Backend Debugging

```bash
# Enable debug logging
RUST_LOG=debug cargo run --bin swoop_server

# Use debugger
cargo install cargo-watch
cargo watch -x 'run --bin swoop_server'
```

### Frontend Debugging

```bash
# Enable dev tools
npm run dev

# Debug with source maps
npm run build -- --sourcemap

# Analyze bundle
npm run build-analyze
```

### Common Issues

**Database Connection Issues:**
```bash
# Check SQLite file permissions
ls -la swoop.db

# Reset database
rm swoop.db
cargo run --bin swoop_server
```

**API Connection Issues:**
```bash
# Check if server is running
curl http://localhost:8080/health

# Check CORS configuration
curl -H "Origin: http://localhost:3000" \
     -H "Access-Control-Request-Method: POST" \
     -H "Access-Control-Request-Headers: X-Requested-With" \
     -X OPTIONS http://localhost:8080/api/documents
```

## Performance Optimization

### Backend Optimization

```rust
// Connection pooling
let pool = sqlx::sqlite::SqlitePool::connect_with(
    SqliteConnectOptions::new()
        .filename("swoop.db")
        .create_if_missing(true)
        .synchronous(SqliteSynchronous::Normal)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(30)),
)
.await?;

// Batch processing
pub async fn process_documents_batch(docs: Vec<Document>) -> Result<Vec<ProcessedDocument>> {
    let futures = docs.into_iter()
        .map(|doc| process_document(doc))
        .collect::<Vec<_>>();
    
    let results = futures::future::join_all(futures).await;
    Ok(results.into_iter().collect::<Result<Vec<_>, _>>()?)
}
```

### Frontend Optimization

```typescript
// Lazy loading
const MyComponent = React.lazy(() => import('./MyComponent'));

// Memoization
const MyComponent = React.memo(({ data }: Props) => {
  const expensiveValue = useMemo(() => {
    return calculateExpensiveValue(data);
  }, [data]);
  
  return <div>{expensiveValue}</div>;
});

// Virtual scrolling for large lists
import { FixedSizeList as List } from 'react-window';

const VirtualizedList = ({ items }) => (
  <List
    height={600}
    itemCount={items.length}
    itemSize={50}
  >
    {({ index, style }) => (
      <div style={style}>
        {items[index].name}
      </div>
    )}
  </List>
);
```

## Deployment

### Local Production Build

```bash
# Build backend
cargo build --release

# Build frontend
cd frontend
npm run build

# Start production server
./target/release/swoop_server --port 8080
```

### Docker Deployment

```dockerfile
# Dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/swoop_server /usr/local/bin/

EXPOSE 8080
CMD ["swoop_server"]
```

```bash
# Build and run
docker build -t swoop .
docker run -p 8080:8080 swoop
```

### Environment-Specific Configs

```bash
# Development
NODE_ENV=development
RUST_LOG=debug

# Staging
NODE_ENV=staging
RUST_LOG=info

# Production
NODE_ENV=production
RUST_LOG=warn
DATABASE_URL=postgresql://user:pass@db:5432/swoop
```

## Contributing

### Code Style

```bash
# Rust formatting
cargo fmt

# TypeScript/JavaScript
npm run lint:fix
npm run format

# Git hooks (automatic)
npm run prepare
```

### Commit Messages

```bash
# Format: type(scope): description
feat(api): add document search endpoint
fix(frontend): resolve upload progress indicator
docs(readme): update installation instructions
test(crawler): add robots.txt compliance tests
```

### Pull Request Process

1. **Create feature branch:** `git checkout -b feature/my-feature`
2. **Write tests:** Ensure >90% code coverage
3. **Update documentation:** Add/update relevant docs
4. **Run full test suite:** `npm run test:all`
5. **Create PR:** Use provided PR template
6. **Code review:** Address all feedback
7. **Merge:** Squash and merge after approval

## Troubleshooting

### Common Development Issues

**"cargo build" fails:**
```bash
# Update Rust toolchain
rustup update

# Clear cache
cargo clean

# Check dependencies
cargo check
```

**"npm install" fails:**
```bash
# Clear cache
npm cache clean --force

# Delete node_modules
rm -rf node_modules package-lock.json
npm install
```

**Database migration errors:**
```bash
# Reset database
rm swoop.db
cargo run --bin swoop_server

# Manual migration
sqlx migrate run
```

### Getting Help

- **Documentation:** Check `docs/` directory
- **Issues:** Create GitHub issue with template
- **Discussions:** Use GitHub Discussions
- **Chat:** Join Discord server (link in README)

### Development Tools

**Recommended VS Code Extensions:**
- rust-analyzer
- ES7+ React/Redux/React-Native snippets
- Prettier
- SQLite Viewer
- Thunder Client (API testing)

**Useful CLI Tools:**
```bash
# Install development tools
cargo install cargo-watch cargo-expand sqlx-cli
npm install -g @vercel/ncc typescript
```

This guide covers the essential development workflow for Swoop. For specific implementation details, refer to the source code and additional documentation in the `docs/` directory.