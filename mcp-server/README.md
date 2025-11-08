# PhotoPrism MCP Server

A [Model Context Protocol](https://modelcontextprotocol.io) (MCP) server for [PhotoPrism](https://photoprism.app), enabling AI assistants like Claude to interact with your PhotoPrism photo library.

## Features

- **Photo Management**: Search, view, and update photo metadata
- **Album Operations**: Create, manage, and organize albums
- **Label & Subject Search**: Find photos by labels and people
- **Resource Access**: Read-only access to photos, albums, and system status via URI templates
- **Multiple Transports**: Supports STDIO (for Claude Desktop) and HTTP/SSE (for remote access)

## Installation

### Prerequisites

- Rust 1.75 or later
- A running PhotoPrism instance
- PhotoPrism admin credentials

### Build from Source

```bash
cd mcp-server
cargo build --release
```

The compiled binary will be at `target/release/photoprism-mcp`.

### Docker

```bash
docker build -t photoprism-mcp .
docker run -e PHOTOPRISM_URL=http://your-photoprism:2342 \
           -e PHOTOPRISM_USERNAME=admin \
           -e PHOTOPRISM_PASSWORD=your-password \
           photoprism-mcp
```

## Configuration

### Environment Variables

- `PHOTOPRISM_URL` - PhotoPrism server URL (default: `http://localhost:2342`)
- `PHOTOPRISM_USERNAME` - PhotoPrism username (default: `admin`)
- `PHOTOPRISM_PASSWORD` - PhotoPrism password (required)
- `TRANSPORT` - Transport mode: `stdio` or `http` (default: `stdio`)
- `HTTP_ADDR` - HTTP server address when using HTTP transport (default: `0.0.0.0:3000`)

### Configuration File

Create `~/.config/photoprism-mcp/config.yaml`:

```yaml
base_url: http://localhost:2342
username: admin
# Password is loaded from PHOTOPRISM_PASSWORD env var
```

## Usage with Claude Desktop

Add to your Claude Desktop configuration (`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS):

```json
{
  "mcpServers": {
    "photoprism": {
      "command": "/path/to/photoprism-mcp",
      "env": {
        "PHOTOPRISM_URL": "http://localhost:2342",
        "PHOTOPRISM_USERNAME": "admin",
        "PHOTOPRISM_PASSWORD": "your-password"
      }
    }
  }
}
```

## Available Tools

### Photo Tools

- `get_photo` - Get photo details by UID
- `update_photo` - Update photo metadata (title, description, favorite, private)
- `search_photos` - Search photos by text query

### Album Tools

- `list_albums` - List all albums
- `get_album` - Get album details by UID
- `create_album` - Create a new album
- `add_photos_to_album` - Add photos to an album

### Label Tools

- `list_labels` - List all labels/tags
- `get_photos_by_label` - Get photos with a specific label

### Subject Tools

- `list_subjects` - List all subjects/people
- `get_subject_photos` - Get photos of a specific person

### Library Tools

- `get_status` - Get PhotoPrism library status

## Available Resources

Resources provide read-only access to data via URIs:

- `photoprism://photos/recent` - 20 most recent photos
- `photoprism://photos/favorites` - Favorite photos
- `photoprism://photos/{uid}` - Specific photo by UID
- `photoprism://albums/list` - All albums
- `photoprism://albums/{uid}` - Specific album by UID
- `photoprism://system/status` - System status and statistics

## Example Usage

Once configured with Claude Desktop, you can ask Claude to:

- "Show me my recent photos"
- "Create an album called 'Summer 2024'"
- "Find all photos labeled 'sunset'"
- "What people are in my photo library?"
- "Add photos to my vacation album"

## Development

### Running Tests

```bash
cargo test
```

### Running Locally

```bash
# With environment variables
export PHOTOPRISM_URL=http://localhost:2342
export PHOTOPRISM_USERNAME=admin
export PHOTOPRISM_PASSWORD=your-password
cargo run

# With HTTP transport
TRANSPORT=http HTTP_ADDR=127.0.0.1:3000 cargo run
```

### Project Structure

```
mcp-server/
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library exports
│   ├── config.rs            # Configuration handling
│   ├── error.rs             # Error types
│   ├── client/              # PhotoPrism API client
│   │   └── mod.rs
│   ├── types/               # Data types
│   │   ├── mod.rs
│   │   ├── photo.rs
│   │   ├── album.rs
│   │   └── common.rs
│   ├── tools/               # MCP tools
│   │   ├── mod.rs
│   │   ├── photos.rs
│   │   ├── albums.rs
│   │   ├── search.rs
│   │   ├── labels.rs
│   │   ├── subjects.rs
│   │   └── library.rs
│   ├── resources/           # MCP resources
│   │   ├── mod.rs
│   │   ├── photos.rs
│   │   ├── albums.rs
│   │   └── system.rs
│   └── server/              # MCP server
│       ├── mod.rs
│       └── photoprism.rs
└── tests/                   # Tests
    ├── client_test.rs
    └── integration_test.rs
```

## Architecture

The server follows a layered architecture:

1. **MCP Server Layer** - Handles MCP protocol and transport
2. **Tools & Resources Layer** - Implements MCP tools and resources
3. **API Client Layer** - Communicates with PhotoPrism REST API
4. **Types Layer** - Shared data structures

## Security Considerations

- Never commit credentials to version control
- Use environment variables or secure config files for passwords
- Consider using read-only PhotoPrism accounts for safety
- Review tool operations before granting write access

## License

AGPL-3.0 - Same as PhotoPrism

## Contributing

Contributions welcome! Please ensure:

- All tests pass (`cargo test`)
- Code is formatted (`cargo fmt`)
- No clippy warnings (`cargo clippy`)

## Troubleshooting

### Authentication Failed

- Verify PhotoPrism URL is correct and accessible
- Check username and password
- Ensure PhotoPrism is running and responding

### Connection Refused

- Verify PhotoPrism is running
- Check firewall settings
- Ensure correct port (default: 2342)

### No Photos Found

- Run indexing in PhotoPrism first
- Check PhotoPrism logs for errors
- Verify API access is enabled

## Resources

- [PhotoPrism Documentation](https://docs.photoprism.app/)
- [Model Context Protocol](https://modelcontextprotocol.io)
- [TurboMCP Framework](https://github.com/QuantGeekDev/turbomcp)

## Support

For issues and questions:

- PhotoPrism: https://github.com/photoprism/photoprism
- MCP Server: Create an issue in this repository
