# PhotoPrism MCP Server - Deployment Guide

This guide covers two deployment methods for the PhotoPrism MCP server.

## Overview

The PhotoPrism MCP server provides Model Context Protocol (MCP) access to PhotoPrism photo libraries. It supports both Docker and systemd (native) deployment.

**Current Status:**
- ✅ Docker container: Running on port 3005 (Primary deployment)
- ⏸️ Systemd service: Available but disabled (can be used as alternative)

## Prerequisites

- PhotoPrism instance running and accessible
- Rust toolchain (for building from source)
- Docker and Docker Compose (for container deployment)
- Linux system with systemd (for systemd deployment)

## Method 1: Docker Deployment (Currently Active)

### Advantages
- Isolated environment
- Easy to update
- Portable across systems
- Consistent dependencies
- No need to manage system services

### Installation

1. **Build and start container:**
   ```bash
   cd /opt/stacks/photoprism-mcp
   docker compose up -d
   ```

2. **Verify:**
   ```bash
   docker compose ps
   docker compose logs -f
   curl -X POST http://localhost:3005/mcp \
     -H "Content-Type: application/json" \
     -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}'
   ```

### Management Commands

```bash
# View logs
docker compose logs -f

# Restart container
docker compose restart

# Stop container
docker compose down

# Rebuild after code changes
docker compose build --no-cache
docker compose up -d
```

### Docker Configuration

The `docker-compose.yaml` file:
```yaml
services:
  photoprism-mcp:
    build:
      context: ./mcp-server
      dockerfile: Dockerfile
    container_name: photoprism-mcp
    ports:
      - "3005:3005"  # Host:Container
    environment:
      - TRANSPORT=http
      - HTTP_PORT=3005
      - PHOTOPRISM_URL=${PHOTOPRISM_URL:-http://192.168.50.90:2342}
      - PHOTOPRISM_USERNAME=${PHOTOPRISM_USERNAME:-admin}
      - PHOTOPRISM_PASSWORD=${PHOTOPRISM_PASSWORD:-photoprism}
      - RUST_LOG=info
    restart: unless-stopped
    networks:
      - photoprism-mcp

networks:
  photoprism-mcp:
    driver: bridge
```

## Method 2: Systemd Deployment (Alternative)

### Advantages
- Direct binary execution (better performance)
- Native systemd integration
- Simpler troubleshooting
- Automatic restart on failure

### Installation

1. **Build the binary:**
   ```bash
   cd /opt/stacks/photoprism-mcp/mcp-server
   cargo build --release
   ```

2. **Create systemd service:**
   ```bash
   sudo tee /etc/systemd/system/photoprism-mcp.service << 'SERVICE'
   [Unit]
   Description=PhotoPrism MCP Server
   After=network.target

   [Service]
   Type=simple
   User=root
   WorkingDirectory=/opt/stacks/photoprism-mcp/mcp-server
   ExecStart=/opt/stacks/photoprism-mcp/mcp-server/target/release/photoprism-mcp
   Environment="TRANSPORT=http"
   Environment="HTTP_PORT=3005"
   Environment="PHOTOPRISM_URL=http://192.168.50.90:2342"
   Environment="PHOTOPRISM_USERNAME=admin"
   Environment="PHOTOPRISM_PASSWORD=photoprism"
   Environment="RUST_LOG=info"
   Restart=on-failure
   RestartSec=5s

   [Install]
   WantedBy=multi-user.target
   SERVICE
   ```

3. **Stop Docker container first** (to avoid port conflict):
   ```bash
   cd /opt/stacks/photoprism-mcp
   docker compose down
   ```

4. **Enable and start systemd service:**
   ```bash
   sudo systemctl daemon-reload
   sudo systemctl enable photoprism-mcp.service
   sudo systemctl start photoprism-mcp.service
   ```

5. **Verify:**
   ```bash
   systemctl status photoprism-mcp.service
   curl -X POST http://localhost:3005/mcp \
     -H "Content-Type: application/json" \
     -d '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}'
   ```

### Management Commands

```bash
# View logs
journalctl -u photoprism-mcp.service -f

# Restart service
sudo systemctl restart photoprism-mcp.service

# Stop service
sudo systemctl stop photoprism-mcp.service

# Disable service
sudo systemctl disable photoprism-mcp.service
```

## Environment Variables

Both deployment methods support the same environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `TRANSPORT` | Transport protocol (`http` or `stdio`) | `stdio` |
| `HTTP_PORT` | HTTP server port | `3005` |
| `PHOTOPRISM_URL` | PhotoPrism instance URL | `http://localhost:2342` |
| `PHOTOPRISM_USERNAME` | PhotoPrism username | `admin` |
| `PHOTOPRISM_PASSWORD` | PhotoPrism password | Required |
| `RUST_LOG` | Log level (`error`, `warn`, `info`, `debug`, `trace`) | `info` |

## Troubleshooting

### Common Issues

1. **Port already in use:**
   - Both systemd and Docker use port 3005
   - Only run one at a time
   - Stop systemd: `sudo systemctl stop photoprism-mcp.service`
   - Stop Docker: `docker compose down`
   - Check port: `ss -tulpn | grep 3005`

2. **Container exits immediately:**
   - Check logs: `docker compose logs`
   - Verify environment variables
   - Ensure PhotoPrism is accessible

3. **Connection refused:**
   - Verify service is running
   - Check firewall rules
   - Test PhotoPrism connectivity

### Fixed Issues

**Tracing Subscriber Panic in Docker:**
- **Problem:** Container exited with code 0, no error output
- **Cause:** `tracing_subscriber::fmt().init()` panics when stdout/stderr aren't properly attached
- **Solution:** Changed to `try_init()` with explicit stderr writer in `src/main.rs`

**Docker Build Cache Issues:**
- **Problem:** Dummy `lib.rs` from dependency caching interfered with actual source
- **Solution:** Simplified Dockerfile to copy all source at once instead of caching dependencies

## Technical Details

### Tracing Fix

The main.rs file includes this fix for Docker compatibility:

```rust
// Initialize tracing/logging with explicit stderr and try_init to prevent Docker panics
let _ = tracing_subscriber::fmt()
    .with_writer(std::io::stderr)
    .with_env_filter(
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"))
    )
    .try_init();

eprintln!("Starting PhotoPrism MCP Server");
```

This prevents silent failures in Docker environments where stdout/stderr may not be properly configured.

### MCP Protocol

The server implements MCP protocol version 2025-06-18 with:
- Tools support
- Resources support
- Prompts support
- Elicitation support
- SSE streaming
- Message replay
- Session management

## Testing

Test the MCP server with:

```bash
# Test MCP server (port 3005)
curl -X POST http://localhost:3005/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc":"2.0",
    "id":1,
    "method":"initialize",
    "params":{
      "protocolVersion":"2024-11-05",
      "capabilities":{},
      "clientInfo":{"name":"test","version":"1.0"}
    }
  }' | jq .
```

Expected response:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "protocolVersion": "2025-06-18",
    "serverInfo": {
      "name": "photoprism",
      "version": "0.1.0"
    },
    "capabilities": {
      "tools": {},
      "resources": {},
      "prompts": {},
      "elicitation": {}
    }
  },
  "id": 1
}
```

## Support

For issues or questions:
- Check logs first (`journalctl` or `docker compose logs`)
- Verify PhotoPrism connectivity
- Ensure environment variables are correct
- Review this deployment guide

## License

Same license as the PhotoPrism MCP server project.
