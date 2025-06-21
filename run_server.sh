#!/bin/bash

# Script to run Crawl4AI server with direct toolchain path to bypass proxy issues

echo "🚀 Setting up Crawl4AI Server..."

# Set the correct toolchain path
CARGO_PATH="/home/kenzo/.local/share/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/cargo"
RUSTC_PATH="/home/kenzo/.local/share/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc"

# Set environment variables
export RUSTUP_TOOLCHAIN=stable
export PATH="/home/kenzo/.local/share/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:$PATH"

echo "📦 Cargo version: $($CARGO_PATH --version)"
echo "🦀 Rustc version: $($RUSTC_PATH --version)"

echo "🔧 Checking project..."
$CARGO_PATH check

if [ $? -eq 0 ]; then
    echo "✅ Project compiles successfully!"
    echo "🚀 Starting server..."
    echo "📊 Dashboard will be available at: http://localhost:8080/"
    echo "🔌 WebSocket endpoint: ws://localhost:8080/ws"
    echo "📡 Server-sent events: http://localhost:8080/events"
    echo "Press Ctrl+C to stop the server"
    echo ""
    
    $CARGO_PATH run --example server_example
else
    echo "❌ Compilation failed. Trying to fix missing dependencies..."
    
    # Try to update and build
    $CARGO_PATH update
    $CARGO_PATH build
fi 