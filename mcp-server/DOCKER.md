# PhotoPrism MCP Server - Docker Guide

This guide covers building, running, and deploying the PhotoPrism MCP Server using Docker.

## Quick Start

### Using Docker Compose (Recommended)

1. **Copy the environment file**:
   ```bash
   cp .env.example .env
   ```

2. **Edit `.env` with your PhotoPrism credentials**:
   ```bash
   PHOTOPRISM_URL=http://your-photoprism-server:2342
   PHOTOPRISM_USERNAME=admin
   PHOTOPRISM_PASSWORD=your-password
   ```

3. **Start the MCP server**:
   ```bash
   docker-compose up -d
   ```

4. **View logs**:
   ```bash
   docker-compose logs -f photoprism-mcp
   ```

### Using Docker (Manual)

1. **Build the image**:
   ```bash
   docker build -t photoprism-mcp .
   ```

2. **Run the container**:
   ```bash
   docker run -it --rm \
     -e PHOTOPRISM_URL=http://your-photoprism:2342 \
     -e PHOTOPRISM_USERNAME=admin \
     -e PHOTOPRISM_PASSWORD=your-password \
     photoprism-mcp
   ```

### Using Pre-built Images from GitHub Container Registry

Pull the latest image:
```bash
docker pull ghcr.io/oculairmedia/photoprism-mcp/mcp-server:latest
```

Run with environment variables:
```bash
docker run -it --rm \
  -e PHOTOPRISM_URL=http://your-photoprism:2342 \
  -e PHOTOPRISM_USERNAME=admin \
  -e PHOTOPRISM_PASSWORD=your-password \
  ghcr.io/oculairmedia/photoprism-mcp/mcp-server:latest
```

## Available Tags

- `latest` - Latest stable release from main branch
- `develop` - Latest development version
- `vX.Y.Z` - Specific version tags
- `main-sha-<git-sha>` - Specific commit from main branch
- `claude/**` - Development branches from Claude

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PHOTOPRISM_URL` | `http://localhost:2342` | PhotoPrism server URL |
| `PHOTOPRISM_USERNAME` | `admin` | PhotoPrism username |
| `PHOTOPRISM_PASSWORD` | - | PhotoPrism password (required) |
| `TRANSPORT` | `stdio` | Transport mode: `stdio`, `http`, `tcp`, `websocket` |
| `RUST_LOG` | `info` | Logging level: `error`, `warn`, `info`, `debug`, `trace` |

### Volume Mounts (Optional)

For persistent configuration:
```bash
docker run -it --rm \
  -v ~/.config/photoprism-mcp:/home/photoprism-mcp/.config/photoprism-mcp \
  -e PHOTOPRISM_PASSWORD=your-password \
  ghcr.io/oculairmedia/photoprism-mcp/mcp-server:latest
```

## Using with Claude Desktop

Add to your Claude Desktop MCP configuration (`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS):

```json
{
  "mcpServers": {
    "photoprism": {
      "command": "docker",
      "args": [
        "run",
        "-i",
        "--rm",
        "-e", "PHOTOPRISM_URL=http://your-photoprism:2342",
        "-e", "PHOTOPRISM_USERNAME=admin",
        "-e", "PHOTOPRISM_PASSWORD=your-password",
        "ghcr.io/oculairmedia/photoprism-mcp/mcp-server:latest"
      ]
    }
  }
}
```

## Testing with PhotoPrism

To test with a local PhotoPrism instance:

1. **Start both services**:
   ```bash
   docker-compose --profile testing up -d
   ```

2. **Access PhotoPrism**:
   - URL: http://localhost:2342
   - Username: admin
   - Password: (from PHOTOPRISM_PASSWORD env var)

3. **Test the MCP server**:
   ```bash
   docker-compose exec photoprism-mcp photoprism-mcp --help
   ```

## Multi-Architecture Support

The published images support multiple architectures:
- `linux/amd64` (x86_64)
- `linux/arm64` (ARM64/Apple Silicon)

Docker will automatically pull the correct architecture for your system.

## Building Custom Images

### Build for your architecture:
```bash
docker build -t photoprism-mcp:custom .
```

### Build for multiple architectures:
```bash
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t photoprism-mcp:multi-arch \
  --push \
  .
```

### Build with version information:
```bash
docker build \
  --build-arg VERSION=1.0.0 \
  --build-arg BUILDTIME=$(date -u +'%Y-%m-%dT%H:%M:%SZ') \
  --build-arg REVISION=$(git rev-parse HEAD) \
  -t photoprism-mcp:1.0.0 \
  .
```

## GitHub Actions Workflows

This repository includes automated workflows:

### On Pull Requests (`mcp-docker-build-pr.yml`)
- Runs tests
- Builds Docker image (doesn't push)
- Validates build succeeds

### On Push to Main/Develop (`mcp-docker-publish.yml`)
- Runs tests
- Builds multi-arch images
- Pushes to GitHub Container Registry
- Tags with branch name and commit SHA

### On Version Tags (`mcp-release.yml`)
- Creates GitHub release
- Builds binaries for multiple platforms
- Builds and pushes Docker images with version tags
- Tags as `latest`

## Troubleshooting

### Container exits immediately
Check logs:
```bash
docker-compose logs photoprism-mcp
```

Ensure PHOTOPRISM_PASSWORD is set:
```bash
docker-compose config | grep PHOTOPRISM_PASSWORD
```

### Cannot connect to PhotoPrism
Verify network connectivity:
```bash
docker-compose exec photoprism-mcp ping photoprism
```

Check PhotoPrism URL is correct:
```bash
docker-compose exec photoprism-mcp env | grep PHOTOPRISM_URL
```

### Permission issues
The container runs as non-root user (UID 1000). If mounting volumes, ensure proper permissions:
```bash
chown -R 1000:1000 /path/to/config
```

## Development

### Live development with mounted source:
```bash
docker run -it --rm \
  -v $(pwd):/app \
  -w /app \
  rust:1.75-slim \
  bash -c "cargo build && cargo test"
```

### Build and test locally:
```bash
# Build
docker-compose build

# Run tests
docker-compose run --rm photoprism-mcp cargo test

# Interactive shell
docker-compose run --rm photoprism-mcp bash
```

## Security Considerations

1. **Never commit `.env` file** - It contains sensitive credentials
2. **Use secrets management** - For production, use Docker secrets or environment variable injection
3. **Keep images updated** - Regularly pull latest images for security updates
4. **Network isolation** - Use Docker networks to isolate PhotoPrism MCP server

## Performance

### Resource Limits
The docker-compose.yml includes resource limits:
- CPU: 0.5-1.0 cores
- Memory: 256MB-512MB

Adjust based on your workload in `docker-compose.yml`:
```yaml
deploy:
  resources:
    limits:
      cpus: '2'
      memory: 1G
```

## Support

- GitHub Issues: https://github.com/oculairmedia/photoprism-mcp/issues
- Documentation: See main README.md
- PhotoPrism Docs: https://docs.photoprism.app/
