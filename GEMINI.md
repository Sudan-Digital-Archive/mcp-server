# Sudan Digital Archive MCP Server

## Project Overview
This project is an experimental **Model Context Protocol (MCP) server** for the **Sudan Digital Archive (SDA)**. It is written in **Rust** and acts as a bridge between LLMs (like Gemini or Claude) and the SDA API. It allows AI agents to search the archive, retrieve metadata, and manage subjects directly through the MCP standard.

The server interacts with the SDA API defined in `openapi.json` and exposes these capabilities as "tools" that an LLM can invoke.

## Architecture

*   **Language**: Rust (Edition 2024)
*   **Core Library**: `rmcp` (for MCP server implementation)
*   **HTTP Client**: `reqwest`
*   **Serialization**: `serde`, `serde_json`

### Key Modules
*   **`src/main.rs`**: The application entry point. It handles CLI argument parsing (using `clap`), initializes the `SdaClient`, and starts the `SdaServer` using stdio transport.
*   **`src/server.rs`**: Contains the `SdaServer` struct and the tool definitions. It uses `rmcp` macros (`#[tool]`, `#[tool_router]`) to map Rust functions to MCP tools.
    *   **Tools**: `list_accessions`, `get_accession`, `update_accession`, `list_subjects`, `create_subject`, `delete_subject` (and private variants).
*   **`src/client.rs`**: (Internal) Encapsulates the HTTP logic for communicating with the Sudan Digital Archive API.
*   **`src/model.rs`**: Defines the Rust structs and enums for API request/response payloads, likely matching the schemas in `openapi.json`.

## Building and Running

### Prerequisites
*   Rust (latest stable)
*   SDA API Key

### Build
To build the release binary:
```bash
cargo build --release
```
The binary will be created at `target/release/sudan-digital-archive-mcp-server`.

### Run
The server communicates via `stdin` and `stdout`. It is typically run by an MCP client, but you can execute it manually for testing (though it expects JSON-RPC messages on stdin):
```bash
# Basic usage
cargo run -- --api-key "YOUR_KEY"

# With custom base URL
cargo run -- --api-key "YOUR_KEY" --base-url "https://api.example.com"
```

### Testing
Run standard Rust unit and integration tests:
```bash
cargo test
```

## Development Conventions

*   **Tool Arguments**: The codebase avoids using `Option<T>` for tool arguments (e.g., optional query parameters). Instead, it uses default values (like `-1` for integers or empty strings) to ensure better compatibility with current MCP client implementations that might struggle with complex JSON schemas (`anyOf: [null, ...]`).
*   **Error Handling**: Uses `anyhow` for rich error context. Errors from the SDA API are wrapped and returned as MCP `internal_error`s.
*   **Logging**: Uses the `tracing` crate. Logs are written to `stderr` to avoid interfering with the JSON-RPC communication on `stdout`.
*   **Formatting & Linting**:
    *   Format: `cargo fmt`
    *   Lint: `cargo clippy`
*   **CI/CD**: A GitHub Actions workflow (`.github/workflows/qa.yml`) runs formatting, linting, and compilation checks.
