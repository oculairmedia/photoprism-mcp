# Quick Fix for Tokio Docker Exit Issue

## TL;DR

Your binary exits immediately in Docker because `tracing_subscriber::fmt().init()` panics silently when stdout/stderr aren't properly attached.

## Fastest Fix (2 minutes)

Edit `mcp-server/src/main.rs`:

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // OLD (causes silent exit):
    // tracing_subscriber::fmt()
    //     .with_env_filter(...)
    //     .init();
    
    // NEW (handles failures):
    let _ = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)  // Use stderr explicitly
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .try_init();  // try_init() instead of init()
    
    // Use eprintln for critical messages
    eprintln!("Starting PhotoPrism MCP Server");
    
    // ... rest of code
}
```

## Test

```bash
cd /opt/stacks/photoprism-mcp/mcp-server
cargo build --release
docker build -t photoprism-mcp .
docker run --rm photoprism-mcp
```

You should now see "Starting PhotoPrism MCP Server" in the output.

## If That Doesn't Work

See `TOKIO_DOCKER_EXIT_ANALYSIS.md` for complete solutions including manual runtime creation.

## Why This Happens

1. Docker containers often don't have proper TTY attachment
2. `tracing_subscriber::fmt().init()` panics if it can't write to stdout
3. The panic is caught by Tokio's default handler which calls `exit(0)`
4. No error message is shown because stdout isn't working

## Production Fix

For production, use manual runtime creation (see analysis doc Solution 3) for better error handling.
