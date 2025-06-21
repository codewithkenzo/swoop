#!/bin/bash

# Swoop Frontend Setup Script
# This script sets up the React TypeScript frontend for the Swoop platform

set -e

echo "🚀 Setting up Swoop Frontend..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the Swoop project root directory"
    exit 1
fi

# Create frontend directory if it doesn't exist
if [ ! -d "frontend" ]; then
    echo "📁 Creating frontend directory..."
    mkdir -p frontend
fi

cd frontend

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js 18+ and try again."
    exit 1
fi

# Check Node.js version
NODE_VERSION=$(node --version | cut -d'v' -f2 | cut -d'.' -f1)
if [ "$NODE_VERSION" -lt 18 ]; then
    echo "❌ Node.js version 18+ is required. Current version: $(node --version)"
    exit 1
fi

echo "✅ Node.js version: $(node --version)"

# Install dependencies
if [ -f "package.json" ]; then
    echo "📦 Installing dependencies..."
    npm install
    
    # Add missing dependencies if needed
    echo "🔧 Adding additional dependencies..."
    npm install --save-dev tailwindcss-animate @radix-ui/react-slot
    
    echo "✅ Dependencies installed successfully!"
else
    echo "❌ package.json not found. Please ensure the frontend files are properly created."
    exit 1
fi

# Create .env file if it doesn't exist
if [ ! -f ".env" ]; then
    echo "⚙️ Creating .env file..."
    cat > .env << EOF
VITE_API_BASE_URL=http://localhost:8080
VITE_APP_NAME=Swoop
VITE_ENABLE_DEVTOOLS=true
EOF
    echo "✅ .env file created"
fi

# Build the project to verify everything works
echo "🏗️ Building project to verify setup..."
npm run build

echo ""
echo "🎉 Frontend setup complete!"
echo ""
echo "📋 Next steps:"
echo "   1. cd frontend"
echo "   2. npm run dev          # Start development server"
echo "   3. Open http://localhost:3000"
echo ""
echo "🔧 Available commands:"
echo "   npm run dev             # Development server"
echo "   npm run build           # Production build"
echo "   npm run preview         # Preview production build"
echo "   npm run lint            # Run ESLint"
echo "   npm run type-check      # TypeScript type checking"
echo ""
echo "📖 See frontend/README.md for detailed documentation" 