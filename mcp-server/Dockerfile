# Multi-stage build for PhotoPrism MCP Server

# Build arguments
ARG BUILDTIME
ARG VERSION
ARG REVISION

# Build stage
# Using latest stable Rust to support Cargo.lock version 4 and edition2024
FROM rust:latest as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "" > src/lib.rs

# Build dependencies (cached layer)
RUN cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src
COPY tests ./tests

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Pass build args to runtime
ARG BUILDTIME
ARG VERSION
ARG REVISION

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/photoprism-mcp /usr/local/bin/

# Add metadata labels
LABEL org.opencontainers.image.title="PhotoPrism MCP Server"
LABEL org.opencontainers.image.description="Model Context Protocol server for PhotoPrism API integration"
LABEL org.opencontainers.image.version="${VERSION}"
LABEL org.opencontainers.image.created="${BUILDTIME}"
LABEL org.opencontainers.image.revision="${REVISION}"
LABEL org.opencontainers.image.source="https://github.com/oculairmedia/photoprism-mcp"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.vendor="OculairMedia"

# Set environment variables with defaults
ENV PHOTOPRISM_URL=http://localhost:2342
ENV PHOTOPRISM_USERNAME=admin
ENV TRANSPORT=stdio

# Create non-root user
RUN useradd -m -u 1000 photoprism-mcp

USER photoprism-mcp

ENTRYPOINT ["/usr/local/bin/photoprism-mcp"]
