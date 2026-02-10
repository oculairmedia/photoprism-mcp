#!/bin/bash
set -e

echo "Starting PhotoPrism MCP server..."
echo "PHOTOPRISM_URL: ${PHOTOPRISM_URL}"
echo "TRANSPORT: ${TRANSPORT}"
echo "HTTP_PORT: ${HTTP_PORT}"

# Run photoprism-mcp
exec /usr/local/bin/photoprism-mcp
