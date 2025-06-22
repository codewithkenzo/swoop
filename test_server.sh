#!/bin/bash
# Swoop Server Test Script

echo "🧪 Testing Swoop Server Functionality"
echo "======================================"

# Function to test endpoint
test_endpoint() {
    local url=$1
    local description=$2
    echo "Testing $description..."
    
    response=$(curl -s -w "%{http_code}" "$url")
    http_code="${response: -3}"
    body="${response%???}"
    
    if [ "$http_code" = "200" ]; then
        echo "✅ $description - HTTP $http_code"
        echo "   Response: $(echo "$body" | jq -r '.status // .name // "OK"')"
    else
        echo "❌ $description - HTTP $http_code"
        echo "   Error: $body"
    fi
    echo ""
}

# Function to start server and wait
start_server() {
    local port=$1
    local method=$2
    echo "🚀 Starting server on port $port ($method)..."
    
    if [ "$method" = "env" ]; then
        PORT=$port cargo run --bin swoop_server > /dev/null 2>&1 &
    else
        cargo run --bin swoop_server $port > /dev/null 2>&1 &
    fi
    
    SERVER_PID=$!
    sleep 3  # Wait for server to start
    
    # Check if server is running
    if kill -0 $SERVER_PID 2>/dev/null; then
        echo "✅ Server started successfully (PID: $SERVER_PID)"
        return 0
    else
        echo "❌ Server failed to start"
        return 1
    fi
}

# Function to stop server
stop_server() {
    if [ ! -z "$SERVER_PID" ]; then
        echo "🛑 Stopping server (PID: $SERVER_PID)..."
        kill $SERVER_PID 2>/dev/null
        wait $SERVER_PID 2>/dev/null
        echo "✅ Server stopped"
    fi
    pkill -f swoop_server 2>/dev/null  # Cleanup any remaining processes
}

# Test 1: Command line argument
echo "Test 1: Command Line Port Argument"
echo "-----------------------------------"
if start_server 3004 "arg"; then
    test_endpoint "http://localhost:3004/" "Root endpoint"
    test_endpoint "http://localhost:3004/health" "Health check"
    test_endpoint "http://localhost:3004/api/status" "API status"
    stop_server
fi

echo ""

# Test 2: Environment variable
echo "Test 2: Environment Variable Port"
echo "---------------------------------"
if start_server 3005 "env"; then
    test_endpoint "http://localhost:3005/" "Root endpoint"
    test_endpoint "http://localhost:3005/health" "Health check"
    test_endpoint "http://localhost:3005/api/status" "API status"
    stop_server
fi

echo ""

# Test 3: Default port (if available)
echo "Test 3: Default Port (3001)"
echo "---------------------------"
if ! ss -tlnp | grep -q :3001; then
    if start_server "" "default"; then
        test_endpoint "http://localhost:3001/" "Root endpoint"
        test_endpoint "http://localhost:3001/health" "Health check"
        test_endpoint "http://localhost:3001/api/status" "API status"
        stop_server
    fi
else
    echo "⚠️  Port 3001 is already in use, skipping default port test"
fi

echo ""

# Test 4: Invalid port handling
echo "Test 4: Invalid Port Handling"
echo "-----------------------------"
echo "Testing invalid port argument..."
cargo run --bin swoop_server 99999 2>&1 | head -3

echo ""
echo "🎉 Server testing complete!"
echo ""
echo "Usage Examples:"
echo "  cargo run --bin swoop_server 3006          # Use port 3006"
echo "  PORT=3007 cargo run --bin swoop_server     # Use environment variable"
echo "  cargo run --bin swoop_server               # Use default port 3001" 