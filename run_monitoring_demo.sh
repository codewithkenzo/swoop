#!/bin/bash

# Advanced Monitoring Demo Runner
# Showcases production-grade monitoring capabilities of Swoop

set -e

echo "🎯 === SWOOP 🚀 ADVANCED MONITORING DEMO ==="
echo

# Colors for output
RED='\\033[0;31m'
GREEN='\\033[0;32m'
BLUE='\\033[0;34m'
YELLOW='\\033[1;33m'
NC='\\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    print_error "Cargo not found. Please install Rust: https://rustup.rs/"
    exit 1
fi

print_status "Building monitoring demo..."
cargo build --release --example monitoring_demo

print_status "Starting monitoring demo server..."
print_info "The demo will:"
print_info "  • Initialize advanced monitoring system"
print_info "  • Simulate crawl workload with metrics"
print_info "  • Start HTTP server with monitoring endpoints"
print_info "  • Run continuous background metrics simulation"
echo

print_info "🌐 Monitoring endpoints available:"
echo -e "  ${YELLOW}📊 Health Check:${NC}    http://localhost:8080/health"
echo -e "  ${YELLOW}🔥 Readiness:${NC}       http://localhost:8080/ready"
echo -e "  ${YELLOW}📈 Prometheus:${NC}      http://localhost:8080/metrics"
echo -e "  ${YELLOW}📋 Statistics:${NC}      http://localhost:8080/stats"
echo -e "  ${YELLOW}🎛️  Dashboard:${NC}       http://localhost:8080/dashboard"
echo

print_info "🧪 You can test the endpoints with:"
echo "  curl http://localhost:8080/health"
echo "  curl http://localhost:8080/ready"
echo "  curl http://localhost:8080/metrics"
echo "  curl http://localhost:8080/stats"
echo

print_warning "Press Ctrl+C to stop the demo"
echo

# Set environment for better logging
export RUST_LOG=info

# Run the monitoring demo
cargo run --release --example monitoring_demo 