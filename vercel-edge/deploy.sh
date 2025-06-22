#!/bin/bash

# Swoop Vercel Edge Deployment Script
# This script deploys the Swoop edge solution to Vercel

set -e

echo "🚀 Deploying Swoop to Vercel Edge..."

# Check if Vercel CLI is installed
if ! command -v vercel &> /dev/null; then
    echo "❌ Vercel CLI not found. Installing..."
    npm install -g vercel
fi

# Check if we're in the right directory
if [ ! -f "vercel.json" ]; then
    echo "❌ vercel.json not found. Make sure you're in the vercel-edge directory."
    exit 1
fi

# Install dependencies
echo "📦 Installing dependencies..."
npm install

# Run TypeScript compilation check
echo "🔍 Checking TypeScript compilation..."
npx tsc --noEmit

# Set up environment variables (if .env.local doesn't exist)
if [ ! -f ".env.local" ]; then
    echo "⚙️  Creating .env.local template..."
    cat > .env.local << EOF
# Turso Database Configuration
TURSO_DATABASE_URL=libsql://your-database.turso.io
TURSO_AUTH_TOKEN=your-auth-token

# OpenRouter API Configuration
OPENROUTER_API_KEY=sk-or-v1-your-api-key

# Swoop Configuration
SWOOP_API_KEY=your-swoop-api-key
SWOOP_VERSION=1.0.0

# Optional: Analytics
ANALYTICS_ENABLED=true
DEBUG_MODE=false
EOF
    echo "📝 Please edit .env.local with your actual credentials before deploying."
    echo "   You can get a Turso database at: https://turso.tech"
    echo "   You can get an OpenRouter API key at: https://openrouter.ai"
    exit 0
fi

# Check for required environment variables
source .env.local
if [ -z "$TURSO_DATABASE_URL" ] || [ -z "$TURSO_AUTH_TOKEN" ] || [ -z "$OPENROUTER_API_KEY" ]; then
    echo "❌ Missing required environment variables in .env.local"
    echo "   Required: TURSO_DATABASE_URL, TURSO_AUTH_TOKEN, OPENROUTER_API_KEY"
    exit 1
fi

# Deploy to Vercel
echo "🌐 Deploying to Vercel..."

# Check if this is production deployment
if [ "$1" = "--production" ] || [ "$1" = "--prod" ]; then
    echo "🎯 Production deployment..."
    vercel --prod
else
    echo "🧪 Preview deployment..."
    vercel
fi

echo "✅ Deployment complete!"
echo ""
echo "🔗 Your Swoop edge deployment is ready!"
echo "📊 Monitor your deployment at: https://vercel.com/dashboard"
echo "📖 API Documentation: https://github.com/codewithkenzo/swoop"
echo ""
echo "🧪 Test your deployment:"
echo "   curl https://your-deployment-url/api/edge/health"
echo ""
echo "🎉 Swoop is now running globally on Vercel Edge!" 