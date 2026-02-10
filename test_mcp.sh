#!/bin/bash
# Quick test of PhotoPrism MCP tools

echo "=== Testing list_albums (should return max 20) ==="
curl -s -X POST http://localhost:3005/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
      "name": "list_albums",
      "arguments": {}
    }
  }' | jq -r '.result.content[0].text' | jq '.count, .note' 2>/dev/null || echo "Failed"

echo ""
echo "=== Testing search_photos (should default to 10) ==="
curl -s -X POST http://localhost:3005/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 2,
    "method": "tools/call",
    "params": {
      "name": "search_photos",
      "arguments": {
        "query": "photo",
        "count": null
      }
    }
  }' | jq -r '.result.content[0].text' | jq '.count, .note' 2>/dev/null || echo "Failed"
