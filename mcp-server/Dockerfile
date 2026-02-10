# Multi-stage build for PhotoPrism MCP Server
FROM rust:latest as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy everything at once
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 curl && \
    rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/photoprism-mcp /usr/local/bin/photoprism-mcp

# Copy entrypoint script
COPY docker-entrypoint.sh /usr/local/bin/

# Make executables
RUN chmod +x /usr/local/bin/photoprism-mcp && \
    chmod +x /usr/local/bin/docker-entrypoint.sh

# Set environment defaults
ENV PHOTOPRISM_URL=http://localhost:2342 \
    PHOTOPRISM_USERNAME=admin \
    TRANSPORT=stdio

# Run via entrypoint (uses exec like letta)
CMD ["/usr/local/bin/docker-entrypoint.sh"]
