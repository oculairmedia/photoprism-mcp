#!/bin/bash
set -e

echo "=== PhotoPrism MCP Server Starting ==="
echo "PHOTOPRISM_URL: $PHOTOPRISM_URL"
echo "PHOTOPRISM_USERNAME: $PHOTOPRISM_USERNAME"
echo "TRANSPORT: $TRANSPORT"
echo "HTTP_PORT: $HTTP_PORT"
echo "RUST_LOG: $RUST_LOG"
echo "========================================"

echo "Testing binary execution..."
if /usr/local/bin/photoprism-mcp & then
    PID=$!
    echo "Binary started with PID: $PID"
    sleep 2
    if kill -0 $PID 2>/dev/null; then
        echo "Process still running!"
        wait $PID
    else
        echo "Process exited immediately!"
        exit 1
    fi
else
    echo "Failed to start binary!"
    exit 1
fi
