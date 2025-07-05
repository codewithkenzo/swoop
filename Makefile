# Swoop Development Makefile

# Variables
RUST_LOG ?= info
DATABASE_URL ?= postgresql://swoop_user:swoop_password@localhost:5432/swoop

# Development Commands
.PHONY: help dev build test clean docker-up docker-down seed

# Default target
help:
	@echo "Swoop Development Commands:"
	@echo "  dev           - Start development server"
	@echo "  build         - Build the project"
	@echo "  test          - Run tests"
	@echo "  clean         - Clean build artifacts"
	@echo "  docker-up     - Start Docker services"
	@echo "  docker-down   - Stop Docker services"
	@echo "  seed          - Seed the database with demo data"
	@echo "  setup         - Full development setup"

# Development server
dev:
	@echo "🚀 Starting development server..."
	RUST_LOG=$(RUST_LOG) cargo run --bin swoop_server

# Build project
build:
	@echo "🔨 Building project..."
	cargo build --release

# Run tests
test:
	@echo "🧪 Running tests..."
	cargo test

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

# Start Docker services
docker-up:
	@echo "🐳 Starting Docker services..."
	docker-compose up -d
	@echo "⏳ Waiting for database to be ready..."
	@sleep 10
	@echo "✅ Docker services started!"
	@echo "📊 pgAdmin: http://localhost:5050 (admin@swoop.local / admin)"
	@echo "🗄️  PostgreSQL: localhost:5432 (swoop_user / swoop_password)"

# Stop Docker services
docker-down:
	@echo "🛑 Stopping Docker services..."
	docker-compose down

# Seed database with demo data
seed:
	@echo "🌱 Seeding database with demo data..."
	@echo "📡 Checking database connection..."
	@if ! pg_isready -h localhost -p 5432 -U swoop_user -d swoop -q; then \
		echo "❌ Database not ready. Please run 'make docker-up' first."; \
		exit 1; \
	fi
	@echo "✅ Database is ready!"
	@echo "🌱 Running seed script..."
	DATABASE_URL=$(DATABASE_URL) cargo run --bin seed
	@echo "✅ Database seeding completed!"

# Full development setup
setup: docker-up seed
	@echo "🎉 Development environment setup complete!"
	@echo ""
	@echo "📋 Next steps:"
	@echo "  1. Copy config.template to .env and customize"
	@echo "  2. Run 'make dev' to start the development server"
	@echo "  3. Visit http://localhost:3001 for the API"
	@echo "  4. Frontend runs on http://localhost:5173"
	@echo ""
	@echo "🔧 Available commands:"
	@echo "  make dev      - Start development server"
	@echo "  make test     - Run tests"
	@echo "  make seed     - Re-seed database"
	@echo "  make clean    - Clean build artifacts"

# Additional utilities
check-deps:
	@echo "🔍 Checking dependencies..."
	@command -v docker >/dev/null 2>&1 || { echo "❌ Docker is required but not installed."; exit 1; }
	@command -v docker-compose >/dev/null 2>&1 || { echo "❌ Docker Compose is required but not installed."; exit 1; }
	@command -v cargo >/dev/null 2>&1 || { echo "❌ Rust/Cargo is required but not installed."; exit 1; }
	@command -v pg_isready >/dev/null 2>&1 || { echo "❌ PostgreSQL client tools are required but not installed."; exit 1; }
	@echo "✅ All dependencies are installed!"

# Database utilities
db-reset: docker-down docker-up seed
	@echo "🔄 Database reset complete!"

db-backup:
	@echo "💾 Creating database backup..."
	@mkdir -p backups
	@pg_dump $(DATABASE_URL) > backups/swoop_backup_$(shell date +%Y%m%d_%H%M%S).sql
	@echo "✅ Backup created in backups/ directory"

db-restore:
	@echo "📥 Restoring database from backup..."
	@read -p "Enter backup filename: " backup_file; \
	if [ -f "backups/$$backup_file" ]; then \
		psql $(DATABASE_URL) < backups/$$backup_file; \
		echo "✅ Database restored from $$backup_file"; \
	else \
		echo "❌ Backup file not found: backups/$$backup_file"; \
	fi

# Development helpers
fmt:
	@echo "🎨 Formatting code..."
	cargo fmt

lint:
	@echo "🔍 Linting code..."
	cargo clippy -- -D warnings

watch:
	@echo "👀 Watching for changes..."
	cargo watch -x run

# Performance testing
bench:
	@echo "⚡ Running performance benchmarks..."
	cargo run --bin performance_crawler_demo --release 