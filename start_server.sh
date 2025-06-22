#!/bin/bash
# Swoop Platform Server Startup Script

echo "🚀 Starting Swoop Document Intelligence Platform..."

# Parse command line arguments
PORT=""
if [ $# -gt 0 ]; then
    PORT="$1"
fi

echo "📦 Building project..."

# Build the project
cargo build --bin swoop_server

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "🌐 Starting server..."
    
    # Run the server with port if specified
    if [ -n "$PORT" ]; then
        echo "🔧 Using port: $PORT"
        cargo run --bin swoop_server "$PORT"
    else
        echo "🔧 Using default port configuration"
        cargo run --bin swoop_server
    fi
else
    echo "❌ Build failed!"
    exit 1
fi 