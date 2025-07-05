# Contributing to Swoop

Thank you for your interest in contributing to Swoop! This guide provides everything you need to know to contribute effectively to our AI-powered document intelligence platform.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Development Setup](#development-setup)
4. [Contribution Workflow](#contribution-workflow)
5. [Code Standards](#code-standards)
6. [Testing Guidelines](#testing-guidelines)
7. [Documentation](#documentation)
8. [Community](#community)

## Code of Conduct

We are committed to providing a welcoming and inclusive environment for all contributors. Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

### Our Standards

- **Be respectful**: Treat everyone with respect and kindness
- **Be collaborative**: Work together to improve Swoop
- **Be constructive**: Provide helpful feedback and suggestions
- **Be patient**: Remember that everyone has different skill levels
- **Be inclusive**: Welcome newcomers and diverse perspectives

## Getting Started

### Types of Contributions

We welcome all types of contributions:

- **🐛 Bug Reports**: Help us identify and fix issues
- **✨ Feature Requests**: Suggest new features and improvements
- **🔧 Code Contributions**: Fix bugs, implement features, improve performance
- **📚 Documentation**: Improve guides, examples, and API docs
- **🎨 UI/UX**: Enhance the frontend experience
- **🧪 Testing**: Add tests, improve test coverage
- **🌐 Translation**: Help us support more languages

### Before You Start

1. **Check existing issues**: Look for similar bug reports or feature requests
2. **Read the docs**: Familiarize yourself with Swoop's architecture and features
3. **Join our community**: Connect with other contributors on Discord
4. **Start small**: Begin with good first issues to understand the codebase

## Development Setup

### Prerequisites

- **Rust**: Latest stable version (1.70+)
- **Node.js**: 18+ for frontend development
- **Git**: Version control
- **Docker**: Optional, for containerized development

### Quick Setup

```bash
# 1. Fork and clone the repository
git clone https://github.com/YOUR_USERNAME/swoop.git
cd swoop

# 2. Set up the backend
cargo build
cargo test

# 3. Set up the frontend
cd frontend
npm install
npm run dev

# 4. Set up documentation site
cd ../docs-site
npm install
npm run dev
```

### Environment Configuration

Create a `.env` file for development:

```bash
# AI Services (optional for basic development)
OPENROUTER_API_KEY=your_key_here
ELEVENLABS_API_KEY=your_key_here

# Database
DATABASE_URL=sqlite:./swoop_dev.db

# Development settings
RUST_LOG=debug
LOG_LEVEL=debug
WORKER_THREADS=4
```

### Development Tools

**Recommended IDE Setup:**
- **VS Code** with extensions:
  - rust-analyzer
  - CodeLLDB (debugging)
  - Error Lens
  - GitLens
  - ES7+ React/Redux/React-Native snippets
  - Prettier
  - ESLint

**Useful Commands:**
```bash
# Backend development
cargo watch -x 'run --bin swoop_server'  # Auto-reload server
cargo clippy                              # Linting
cargo fmt                                 # Formatting
cargo test                                # Run tests

# Frontend development
npm run dev                               # Development server
npm run lint                              # ESLint
npm run type-check                        # TypeScript checking
npm test                                  # Run tests

# Documentation
cd docs-site && npm run dev               # Documentation site
```

## Contribution Workflow

### 1. Create an Issue

Before starting work, create or find an issue:

```markdown
## Bug Report Template
**Describe the bug**
A clear description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. See error

**Expected behavior**
What you expected to happen.

**Environment**
- OS: [e.g. macOS, Linux, Windows]
- Swoop version: [e.g. 0.2.0]
- Browser: [if applicable]

**Additional context**
Any other context about the problem.
```

```markdown
## Feature Request Template
**Is your feature request related to a problem?**
A clear description of what the problem is.

**Describe the solution you'd like**
A clear description of what you want to happen.

**Describe alternatives you've considered**
Other solutions you've considered.

**Additional context**
Any other context or screenshots about the feature request.
```

### 2. Fork and Branch

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/swoop.git
cd swoop

# Create a feature branch
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-number-description
```

### Branch Naming Convention

- **Features**: `feature/description-of-feature`
- **Bug fixes**: `fix/issue-number-short-description`
- **Documentation**: `docs/description-of-changes`
- **Performance**: `perf/description-of-optimization`
- **Refactoring**: `refactor/description-of-changes`

### 3. Make Changes

#### Code Changes

Follow our coding standards (see below) and ensure:

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] New features have tests
- [ ] Documentation is updated
- [ ] Commit messages follow convention

#### Commit Message Convention

We use [Conventional Commits](https://conventionalcommits.org/):

```bash
# Format: type(scope): description
feat(api): add document categorization endpoint
fix(frontend): resolve upload progress indicator issue
docs(readme): update installation instructions
test(crawler): add robots.txt compliance tests
perf(search): optimize vector similarity calculation
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### 4. Testing

Ensure all tests pass before submitting:

```bash
# Backend tests
cargo test
cargo test --release  # Production build tests

# Frontend tests
cd frontend
npm test
npm run test:coverage

# Integration tests
cargo test --test integration_tests

# End-to-end tests (if applicable)
npm run test:e2e
```

### 5. Documentation

Update documentation for any changes:

- **API changes**: Update OpenAPI spec and API docs
- **New features**: Add to user documentation
- **Code changes**: Update inline documentation
- **Breaking changes**: Update migration guide

### 6. Submit Pull Request

```bash
# Push your changes
git push origin feature/your-feature-name

# Create pull request on GitHub
```

**Pull Request Template:**
```markdown
## Description
Brief description of changes made.

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Code refactoring

## Testing
- [ ] Tests pass locally
- [ ] New tests added for new functionality
- [ ] Manual testing completed

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Changes are backward compatible (or marked as breaking)

## Related Issues
Fixes #(issue number)
```

## Code Standards

### Rust Code Style

#### General Principles

```rust
// Use descriptive names
fn process_document_content(content: &str) -> Result<ProcessedDocument> {
    // Implementation
}

// Prefer explicit types for public APIs
pub struct DocumentAnalysis {
    pub category: DocumentCategory,
    pub confidence: f64,
    pub entities: Vec<Entity>,
}

// Use Result for error handling
pub enum ProcessingError {
    InvalidFormat(String),
    AIServiceUnavailable,
    DatabaseError(sqlx::Error),
}

// Document public APIs
/// Processes a document and returns analysis results.
/// 
/// # Arguments
/// * `content` - The document content to analyze
/// 
/// # Returns
/// * `Ok(ProcessedDocument)` - Analysis results
/// * `Err(ProcessingError)` - Processing failure
pub async fn analyze_document(content: &str) -> Result<ProcessedDocument, ProcessingError> {
    // Implementation
}
```

#### Error Handling

```rust
// Use thiserror for error types
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Invalid input: {message}")]
    InvalidInput { message: String },
    
    #[error("External service unavailable")]
    ServiceUnavailable,
}

// Implement proper error conversion
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            ApiError::InvalidInput { message } => (StatusCode::BAD_REQUEST, message),
            ApiError::ServiceUnavailable => (StatusCode::SERVICE_UNAVAILABLE, "Service unavailable"),
        };
        
        let body = Json(json!({
            "error": error_message,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }));
        
        (status, body).into_response()
    }
}
```

#### Async Code

```rust
// Prefer async/await over manual futures
pub async fn process_multiple_documents(
    documents: Vec<Document>,
) -> Result<Vec<ProcessedDocument>, ProcessingError> {
    let futures = documents
        .into_iter()
        .map(|doc| process_single_document(doc));
    
    futures::future::try_join_all(futures).await
}

// Use bounded concurrency for resource management
pub async fn process_with_concurrency(
    documents: Vec<Document>,
    max_concurrency: usize,
) -> Result<Vec<ProcessedDocument>, ProcessingError> {
    let semaphore = Arc::new(Semaphore::new(max_concurrency));
    
    let futures = documents.into_iter().map(|doc| {
        let semaphore = semaphore.clone();
        async move {
            let _permit = semaphore.acquire().await.unwrap();
            process_single_document(doc).await
        }
    });
    
    futures::future::try_join_all(futures).await
}
```

### TypeScript/React Code Style

#### General Principles

```typescript
// Use descriptive interfaces
interface DocumentAnalysis {
  category: DocumentCategory;
  confidence: number;
  entities: Entity[];
  summary: string;
}

// Prefer function components with hooks
interface DocumentViewerProps {
  document: Document;
  onAnalysisComplete?: (analysis: DocumentAnalysis) => void;
}

export const DocumentViewer: React.FC<DocumentViewerProps> = ({
  document,
  onAnalysisComplete,
}) => {
  const [analysis, setAnalysis] = useState<DocumentAnalysis | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Implementation
  return (
    <div className="document-viewer">
      {/* Component JSX */}
    </div>
  );
};
```

#### Custom Hooks

```typescript
// Extract reusable logic into custom hooks
export const useDocumentAnalysis = (documentId: string) => {
  const [analysis, setAnalysis] = useState<DocumentAnalysis | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!documentId) return;

    const analyzeDocument = async () => {
      setIsLoading(true);
      setError(null);

      try {
        const result = await api.analyzeDocument(documentId);
        setAnalysis(result);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Analysis failed');
      } finally {
        setIsLoading(false);
      }
    };

    analyzeDocument();
  }, [documentId]);

  return { analysis, isLoading, error };
};
```

#### API Client

```typescript
// Centralized API client with proper typing
class SwoopAPI {
  private baseURL: string;
  private apiKey: string;

  constructor(baseURL: string, apiKey: string) {
    this.baseURL = baseURL;
    this.apiKey = apiKey;
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<T> {
    const url = `${this.baseURL}${endpoint}`;
    const response = await fetch(url, {
      ...options,
      headers: {
        'Authorization': `Bearer ${this.apiKey}`,
        'Content-Type': 'application/json',
        ...options.headers,
      },
    });

    if (!response.ok) {
      throw new Error(`API error: ${response.statusText}`);
    }

    return response.json();
  }

  async uploadDocument(file: File, metadata?: DocumentMetadata): Promise<Document> {
    const formData = new FormData();
    formData.append('file', file);
    if (metadata) {
      formData.append('metadata', JSON.stringify(metadata));
    }

    return this.request('/api/documents/upload', {
      method: 'POST',
      body: formData,
      headers: {}, // Don't set Content-Type for FormData
    });
  }
}
```

### Code Formatting

**Rust:**
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Clippy linting
cargo clippy -- -D warnings
```

**TypeScript/React:**
```bash
# Format code
npm run format

# Lint code
npm run lint

# Fix linting issues
npm run lint:fix
```

## Testing Guidelines

### Backend Testing

#### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_document_processing() {
        let content = "Sample document content";
        let result = process_document_content(content).await;
        
        assert!(result.is_ok());
        let processed = result.unwrap();
        assert_eq!(processed.word_count, 3);
        assert!(processed.quality_score > 0.0);
    }

    #[test]
    fn test_entity_extraction() {
        let entities = extract_entities("John Doe works at Microsoft");
        
        assert_eq!(entities.len(), 2);
        assert_eq!(entities[0].text, "John Doe");
        assert_eq!(entities[0].entity_type, EntityType::Person);
        assert_eq!(entities[1].text, "Microsoft");
        assert_eq!(entities[1].entity_type, EntityType::Organization);
    }
}
```

#### Integration Tests

```rust
// tests/integration_tests.rs
use swoop::test_utils::TestServer;
use serde_json::json;

#[tokio::test]
async fn test_document_upload_and_analysis() {
    let server = TestServer::new().await;
    
    // Upload document
    let response = server
        .client()
        .post("/api/documents/upload")
        .multipart_form(&[("file", "test.pdf", b"PDF content")])
        .send()
        .await;
    
    assert_eq!(response.status(), 200);
    let document: Document = response.json().await;
    
    // Wait for processing
    server.wait_for_processing(&document.id).await;
    
    // Verify analysis
    let analysis = server.get_document_analysis(&document.id).await;
    assert!(analysis.is_some());
}
```

### Frontend Testing

#### Component Tests

```typescript
// src/components/__tests__/DocumentViewer.test.tsx
import { render, screen, waitFor } from '@testing-library/react';
import { DocumentViewer } from '../DocumentViewer';
import { mockDocument } from '../../test-utils/mocks';

describe('DocumentViewer', () => {
  it('renders document information', () => {
    render(<DocumentViewer document={mockDocument} />);
    
    expect(screen.getByText(mockDocument.title)).toBeInTheDocument();
    expect(screen.getByText(/Word count:/)).toBeInTheDocument();
  });

  it('handles analysis completion', async () => {
    const onAnalysisComplete = jest.fn();
    
    render(
      <DocumentViewer 
        document={mockDocument} 
        onAnalysisComplete={onAnalysisComplete}
      />
    );
    
    await waitFor(() => {
      expect(onAnalysisComplete).toHaveBeenCalledWith(
        expect.objectContaining({
          category: expect.any(String),
          confidence: expect.any(Number),
        })
      );
    });
  });
});
```

#### Hook Tests

```typescript
// src/hooks/__tests__/useDocumentAnalysis.test.ts
import { renderHook, waitFor } from '@testing-library/react';
import { useDocumentAnalysis } from '../useDocumentAnalysis';
import { mockApi } from '../../test-utils/mocks';

jest.mock('../../lib/api', () => mockApi);

describe('useDocumentAnalysis', () => {
  it('fetches analysis on mount', async () => {
    const { result } = renderHook(() => useDocumentAnalysis('doc-123'));
    
    expect(result.current.isLoading).toBe(true);
    
    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
      expect(result.current.analysis).toBeDefined();
    });
  });
});
```

### Test Coverage

Maintain high test coverage:

- **Backend**: Target 80%+ code coverage
- **Frontend**: Target 75%+ component coverage
- **Critical paths**: 100% coverage for security and data handling

```bash
# Check coverage
cargo tarpaulin --out Html
npm run test:coverage
```

## Documentation

### Code Documentation

```rust
/// Processes a document and extracts content and metadata.
///
/// This function handles multiple document formats including PDF, HTML,
/// and plain text. It performs content extraction, quality analysis,
/// and generates embeddings for semantic search.
///
/// # Arguments
///
/// * `content` - Raw document content as bytes
/// * `content_type` - MIME type of the document
/// * `options` - Processing options and preferences
///
/// # Returns
///
/// Returns a `Result` containing:
/// * `Ok(ProcessedDocument)` - Successfully processed document with analysis
/// * `Err(ProcessingError)` - Processing failure with error details
///
/// # Example
///
/// ```rust
/// use swoop::document_processor::{process_document, ProcessingOptions};
///
/// let content = include_bytes!("../test_data/sample.pdf");
/// let options = ProcessingOptions::default();
/// 
/// let result = process_document(content, "application/pdf", options).await?;
/// println!("Document category: {}", result.analysis.category);
/// ```
///
/// # Errors
///
/// This function can return errors in the following cases:
/// * Unsupported document format
/// * Corrupted or invalid file content
/// * AI service unavailability
/// * Database connection issues
pub async fn process_document(
    content: &[u8],
    content_type: &str,
    options: ProcessingOptions,
) -> Result<ProcessedDocument, ProcessingError> {
    // Implementation
}
```

### API Documentation

Keep OpenAPI spec updated with code changes:

```yaml
# docs/openapi.yaml
paths:
  /api/documents/upload:
    post:
      summary: Upload document for processing
      description: |
        Uploads a document file and immediately begins processing for content
        extraction, AI analysis, and vector embedding generation.
      requestBody:
        required: true
        content:
          multipart/form-data:
            schema:
              type: object
              properties:
                file:
                  type: string
                  format: binary
                  description: Document file to upload
                metadata:
                  type: object
                  description: Optional metadata for the document
```

### User Documentation

Write clear, helpful user documentation:

- **Step-by-step guides**: Include screenshots and examples
- **Code examples**: Provide working, copy-pasteable code
- **Troubleshooting**: Address common issues and solutions
- **API reference**: Keep in sync with OpenAPI spec

## Community

### Getting Help

- **Discord**: [Join our community](https://discord.gg/swoop)
- **GitHub Discussions**: [Ask questions and share ideas](https://github.com/your-org/swoop/discussions)
- **GitHub Issues**: [Report bugs and request features](https://github.com/your-org/swoop/issues)

### Mentorship

New contributors can request mentorship:

1. Comment on a "good first issue"
2. Tag `@maintainers` for guidance
3. Join our Discord #contributors channel
4. Attend monthly contributor calls

### Recognition

We recognize contributors through:

- **Contributor spotlight**: Monthly featured contributors
- **All Contributors**: Recognition in README
- **Swag**: Stickers and sweatshirts for regular contributors
- **Conference speaking**: Opportunities to present Swoop

### Maintainer Responsibilities

Core maintainers commit to:

- **Responsive reviews**: PR feedback within 72 hours
- **Inclusive community**: Welcoming environment for all
- **Clear communication**: Transparent roadmap and decisions
- **Quality standards**: Maintaining code and documentation quality

## Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, backward compatible

### Release Checklist

- [ ] All tests pass
- [ ] Documentation updated
- [ ] Changelog updated
- [ ] Version bumped
- [ ] Tagged release
- [ ] Release notes published

---

Thank you for contributing to Swoop! Your contributions help make document intelligence accessible to everyone. 🚀