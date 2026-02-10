# Tokio Async Runtime Exit Issue in Docker - Analysis & Solutions

## Problem Summary

**Symptom**: Rust binary using Tokio 1.47 with `#[tokio::main]` exits immediately with code 0 in Docker containers but works perfectly on host.

**Key Observations**:
- No stdout/stderr output at all
- strace shows `exit_group(0)` immediately after initialization
- Same binary works on host via systemd
- Mounting binary into plain debian:bookworm-slim container also fails
- Letta MCP server (also Rust + Tokio) works fine with same base image

## Root Cause

The issue is caused by **`tracing_subscriber` initialization failing silently in Docker containers** when stdout/stderr are not properly attached or are in a non-standard state.

### Why This Happens

1. **tracing_subscriber::fmt()** tries to initialize a writer to stdout/stderr
2. In Docker containers, especially when run without proper terminal attachment (`-t` flag), stdout may be in a buffered or non-blocking state
3. The initialization can fail silently, causing a panic that's caught by Tokio's panic handler
4. The `#[tokio::main]` macro's default panic handler calls `std::process::exit(0)` instead of bubbling up the error

### Evidence from Research

From [Tokio Issue #3956](https://github.com/tokio-rs/tokio/issues/3956):
- Same symptoms: immediate exit with code 0 in Kubernetes/Docker
- Same strace pattern: `exit_group(0)` right after initialization
- **Solution that worked**: Remove `#[tokio::main]` and manually create runtime with custom panic handler

## Solutions (Ordered by Effectiveness)

### Solution 1: Make Tracing Initialization Non-Fatal (RECOMMENDED)

**Change `tracing_subscriber::fmt().init()` to handle initialization failures gracefully:**

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing/logging with fallback
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .try_init();  // Use try_init() instead of init()
    
    // Alternative: wrap in result handling
    // if let Err(e) = tracing_subscriber::fmt()
    //     .with_env_filter(...)
    //     .try_init() {
    //     eprintln!("Failed to initialize tracing: {}", e);
    //     // Continue anyway - tracing is not critical
    // }

    eprintln!("Starting PhotoPrism MCP Server");  // Use eprintln for critical messages
    
    // ... rest of code
}
```

**Why this works**: 
- `try_init()` returns a `Result` instead of panicking
- Application continues even if tracing fails
- Critical startup messages use `eprintln!()` which is more reliable

### Solution 2: Use stdout/stderr Writer Explicitly

**Configure the writer to use unbuffered stderr:**

```rust
#[tokio::main]
async fn main() -> Result<()> {
    use tracing_subscriber::fmt::writer::MakeWriterExt;
    
    // Use stderr explicitly with error fallback
    let _ = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .try_init();
    
    eprintln!("Starting PhotoPrism MCP Server");
    
    // ... rest of code
}
```

### Solution 3: Manual Runtime Creation (MOST ROBUST)

**Replace `#[tokio::main]` with explicit runtime builder:**

```rust
fn main() -> Result<()> {
    // Initialize logging before runtime
    let _ = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .try_init();
    
    eprintln!("Starting PhotoPrism MCP Server");
    
    // Create runtime explicitly
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    
    // Run async code
    runtime.block_on(async_main())
}

async fn async_main() -> Result<()> {
    // Load configuration
    let config = match Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            // ... rest of error handling
            std::process::exit(1);
        }
    };
    
    // ... rest of original main() code
}
```

**Why this is most robust**:
- Separates tracing initialization from runtime creation
- Allows proper error handling before Tokio takes over
- Gives you control over panic behavior
- Same pattern used in the workaround from Tokio issue #3956

### Solution 4: Docker-Specific Fixes

**Update Dockerfile to ensure proper stdout/stderr handling:**

```dockerfile
# In Dockerfile
FROM debian:bookworm-slim

# ... existing setup ...

# Ensure stdout/stderr are unbuffered
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Use exec form of CMD to avoid shell buffering
CMD ["photoprism-mcp"]

# OR use docker-entrypoint.sh with explicit unbuffering:
# CMD ["/usr/local/bin/docker-entrypoint.sh"]
```

**Update docker-entrypoint.sh:**

```bash
#!/bin/bash
set -e

# Force unbuffered output
export RUST_LOG="${RUST_LOG:-info}"
export RUST_BACKTRACE="${RUST_BACKTRACE:-1}"

echo "Starting PhotoPrism MCP server..." >&2
echo "PHOTOPRISM_URL: ${PHOTOPRISM_URL}" >&2
echo "TRANSPORT: ${TRANSPORT}" >&2
echo "HTTP_PORT: ${HTTP_PORT}" >&2

# Use exec to replace shell process
exec /usr/local/bin/photoprism-mcp "$@"
```

**Run Docker with proper flags:**

```bash
# For interactive testing
docker run -it photoprism-mcp

# For daemon mode with logs
docker run -a stdout -a stderr photoprism-mcp

# With docker-compose
docker-compose up --abort-on-container-exit
```

## Implementation Plan

### Immediate Fix (Quick Deploy)

1. Change `init()` to `try_init()` in `mcp-server/src/main.rs`
2. Replace `tracing::info!()` startup messages with `eprintln!()`
3. Test in container

### Long-term Fix (Production Ready)

1. Implement Solution 3 (manual runtime creation)
2. Add proper error handling and logging initialization
3. Update docker-entrypoint.sh with unbuffered output
4. Add health check endpoint
5. Test thoroughly

## Code Changes Required

### File: `mcp-server/src/main.rs`

**Minimal change (Solution 1):**
```diff
- #[tokio::main]
- async fn main() -> Result<()> {
+ #[tokio::main]
+ async fn main() -> Result<()> {
      // Initialize tracing/logging
-     tracing_subscriber::fmt()
+     let _ = tracing_subscriber::fmt()
+         .with_writer(std::io::stderr)
          .with_env_filter(
              EnvFilter::try_from_default_env()
                  .unwrap_or_else(|_| EnvFilter::new("info"))
          )
-         .init();
+         .try_init();

-     tracing::info!("Starting PhotoPrism MCP Server");
+     eprintln!("Starting PhotoPrism MCP Server");
```

**Recommended change (Solution 3):**
```diff
- #[tokio::main]
- async fn main() -> Result<()> {
+ fn main() -> Result<()> {
      // Initialize tracing/logging
-     tracing_subscriber::fmt()
+     let _ = tracing_subscriber::fmt()
+         .with_writer(std::io::stderr)
          .with_env_filter(
              EnvFilter::try_from_default_env()
                  .unwrap_or_else(|_| EnvFilter::new("info"))
          )
-         .init();
+         .try_init();

-     tracing::info!("Starting PhotoPrism MCP Server");
+     eprintln!("Starting PhotoPrism MCP Server");
+
+     // Create runtime
+     let runtime = tokio::runtime::Builder::new_multi_thread()
+         .enable_all()
+         .build()?;
+
+     runtime.block_on(async_main())
+ }
+
+ async fn async_main() -> Result<()> {
      // Load configuration
      let config = match Config::load() {
          Ok(cfg) => cfg,
          Err(e) => {
              eprintln!("Failed to load configuration: {}", e);
```

## Testing Strategy

1. **Test minimal fix first**: Apply Solution 1, rebuild, test in container
2. **Verify output**: Check that error messages appear in `docker logs`
3. **Test different transports**: Try both stdio and HTTP modes
4. **Compare with Letta**: Examine Letta's initialization to see differences
5. **Test on host**: Ensure fix doesn't break host execution

## Additional Debugging

If issues persist after Solution 1:

```rust
fn main() -> Result<()> {
    // Debug: Print to stderr BEFORE any Tokio/tracing code
    eprintln!("Process starting, PID: {}", std::process::id());
    
    // Try to initialize tracing
    match tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .try_init() {
        Ok(_) => eprintln!("Tracing initialized successfully"),
        Err(e) => eprintln!("Tracing initialization failed: {}", e),
    }
    
    eprintln!("Creating Tokio runtime...");
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    
    eprintln!("Starting async main...");
    runtime.block_on(async_main())
}
```

## Related Issues & References

1. **Tokio Issue #3956**: #[tokio::main] macro crashes and exits 0 in Kubernetes
   - https://github.com/tokio-rs/tokio/issues/3956
   - Exact same symptoms and solution pattern

2. **tracing-subscriber stdout/stderr issues**: 
   - Initialization can fail silently in non-TTY environments
   - Using `try_init()` is recommended for production

3. **Docker stdout buffering**:
   - Docker can buffer stdout differently than stderr
   - Using stderr for critical messages is more reliable

## Why Letta Works

Letta MCP server likely:
1. Uses `try_init()` instead of `init()`
2. Has proper error handling around tracing initialization
3. May not use tracing at all, or initializes it differently
4. Could be using a different logger (env_logger, etc.)

## Conclusion

**Primary cause**: `tracing_subscriber::fmt().init()` panicking silently when stdout/stderr are not properly initialized in Docker containers.

**Primary solution**: Use `try_init()` and handle initialization errors gracefully, plus use `eprintln!()` for critical startup messages.

**Production recommendation**: Implement Solution 3 for complete control over initialization order and error handling.
