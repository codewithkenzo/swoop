#!/bin/bash

# Clean build script for Swoop - removes Cursor AppImage from PATH
# This prevents the "unknown proxy name: 'cursor-bin'" error

echo "🧹 Setting up clean build environment..."

# Create clean PATH without Cursor AppImage paths
CLEAN_PATH="/usr/local/sbin:/usr/local/bin:/usr/bin:/bin:/sbin:/usr/sbin"
if [ -d "$HOME/.local/share/cargo/bin" ]; then
    CLEAN_PATH="$HOME/.local/share/cargo/bin:$CLEAN_PATH"
fi

echo "📦 Building Rust backend with AI features..."

# Build with clean environment
env -i \
    HOME="$HOME" \
    RUSTUP_HOME="$HOME/.local/share/rustup" \
    CARGO_HOME="$HOME/.local/share/cargo" \
    PATH="$CLEAN_PATH" \
    bash -c "cd $(pwd) && cargo build --release --features ai"

if [ $? -eq 0 ]; then
    echo "✅ Backend build successful!"
    echo "📁 Binary location: target/release/"
    ls -la target/release/ | grep -E "(swoop|demo)" || echo "   No main binaries found, but build completed"
else
    echo "❌ Backend build failed"
    exit 1
fi

echo ""
echo "🚀 Build complete! You can now run:"
echo "   ./target/release/swoop_demo  # Demo application"
echo "   cargo run --features ai --bin swoop_demo  # Or with cargo" 