# Sudan Digital Archive MCP Server - Agent Guidelines

This file contains guidelines for agentic coding agents working on the Sudan Digital Archive MCP Server codebase.

## Build & Development Commands

### Core Commands
- **Build**: `cargo build --release`
- **Run**: `cargo run -- --api-key "YOUR_KEY"`
- **Test**: `cargo test`
- **Format**: `cargo fmt`
- **Lint**: `cargo clippy`

### Running Single Tests
- Run specific test: `cargo test test_name`
- Run tests in module: `cargo test module_name`
- Run with output: `cargo test -- --nocapture`

## Code Style Guidelines

### Rust Edition & Standards
- Uses Rust Edition 2024
- Follows standard Rust naming conventions
- All modules have module-level documentation (`//!`)

### Import Organization
```rust
// Standard library imports first
use std::...;

// External crates next (alphabetical)
use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

// Local modules last
use crate::client::SdaClient;
use crate::model::*;
```

### Error Handling
- Use `anyhow::Result` for application errors
- Use `anyhow::Context` to add context to errors
- Convert API errors to MCP errors using `McpError::internal_error`
- Never use `unwrap()` in production code - use proper error handling

### API Client Patterns
- All HTTP methods use `reqwest` client
- Authentication via `x-api-key` header
- Use `error_for_status()` to handle HTTP errors
- Always add context with `anyhow::Context`

### MCP Server Patterns
- Use `#[tool]` macro for tool functions
- Use `#[tool_router]` for server implementation
- Tool functions return `Result<CallToolResult, McpError>`
- Use `Parameters()` wrapper for tool arguments
- Always wrap errors with context

### Data Models
- All structs derive `Debug`, `Serialize`, `Deserialize`, `JsonSchema`
- Use `#[serde(rename_all = "camelCase")]` for JSON camelCase
- Enums use `#[serde(rename_all = "snake_case")]`
- Optional fields: prefer `Option<T>` for internal models, but avoid in tool arguments for MCP compatibility

### MCP Compatibility Notes
- **Critical**: Avoid `Option<T>` in tool arguments to maximize MCP client compatibility
- Use default values (like `-1` for pagination, empty strings) instead of optional parameters
- This prevents JSON schema `anyOf` issues that some MCP clients struggle with

### Pagination & Query Parameters
- Default pagination value: `-1` (means "not specified")
- Build query vectors dynamically, skip `-1` values
- Use helper functions for complex query construction

### Logging
- Use `tracing` crate for structured logging
- Log to `stderr` to avoid interfering with JSON-RPC on `stdout`
- Use appropriate log levels: `error!`, `warn!`, `info!`, `debug!`

### Code Organization
```
src/
├── main.rs          # Entry point, CLI args, logging init
├── server.rs        # MCP server implementation, tool definitions  
├── client.rs        # HTTP client for SDA API
└── model.rs         # Data structures and enums
```

### Documentation
- All public functions have doc comments
- Include parameter descriptions and return value info
- Document important implementation details
- Keep examples minimal but helpful

### Testing
- Write unit tests for core logic in client methods
- Test error cases, not just success paths
- Use descriptive test names
- Mock external dependencies when possible

### Async Patterns
- All HTTP methods are `async fn`
- Use `.await` properly for all async operations
- Error handling must account for async context

### Security
- Never log API keys or sensitive data
- Use `#[doc(hidden)]` for internal-only functions if needed
- Validate input parameters before API calls
- Handle all potential API errors gracefully

## CI/CD Integration
- GitHub workflow runs: formatting, linting, compilation
- Automated releases when `Cargo.toml` version is updated on main
- Ensure all tests pass before submitting PRs

<!-- headroom:rtk-instructions -->
# RTK (Rust Token Killer) - Token-Optimized Commands

When running shell commands, **always prefix with `rtk`**. This reduces context
usage by 60-90% with zero behavior change. If rtk has no filter for a command,
it passes through unchanged — so it is always safe to use.

## Key Commands
```bash
# Git (59-80% savings)
rtk git status          rtk git diff            rtk git log

# Files & Search (60-75% savings)
rtk ls <path>           rtk read <file>         rtk grep <pattern>
rtk find <pattern>      rtk diff <file>

# Test (90-99% savings) — shows failures only
rtk pytest tests/       rtk cargo test          rtk test <cmd>

# Build & Lint (80-90% savings) — shows errors only
rtk tsc                 rtk lint                rtk cargo build
rtk prettier --check    rtk mypy                rtk ruff check

# Analysis (70-90% savings)
rtk err <cmd>           rtk log <file>          rtk json <file>
rtk summary <cmd>       rtk deps                rtk env

# GitHub (26-87% savings)
rtk gh pr view <n>      rtk gh run list         rtk gh issue list

# Infrastructure (85% savings)
rtk docker ps           rtk kubectl get         rtk docker logs <c>

# Package managers (70-90% savings)
rtk pip list            rtk pnpm install        rtk npm run <script>
```

## Rules
- In command chains, prefix each segment: `rtk git add . && rtk git commit -m "msg"`
- For debugging, use raw command without rtk prefix
- `rtk proxy <cmd>` runs command without filtering but tracks usage
<!-- /headroom:rtk-instructions -->
