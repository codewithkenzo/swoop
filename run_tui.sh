#!/bin/bash

# Swoop TUI Dashboard Runner
# This script runs the enhanced TUI dashboard for monitoring web scraping operations

echo "Starting Swoop TUI Dashboard..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎯 Interactive Controls:"
echo "   • Press 'q' to quit"
echo "   • Press 'Space' to pause/resume scraping"
echo "   • Press '+/-' to adjust rate limit"
echo "   • Press '1-4' to switch between tabs"
echo "   • Press 'Tab' to cycle through tabs"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

cargo run --bin swoop-tui
