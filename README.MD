# Sudan Digital Archive MCP Server

**Experimental MCP Server**

This is an experimental Model Context Protocol (MCP) server for the Sudan Digital Archive (SDA). It provides tools to interact with the SDA API, allowing for automations and enhanced search capabilities within MCP-compatible environments like Gemini or Claude.

## Features

- List and search public and private accessions.
- Retrieve detailed metadata for specific accessions.
- Manage metadata subjects (list, create, delete).
- Update accession details.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- An API Key for the Sudan Digital Archive.

### Building from Source

You are expected to build the binary yourself for your specific platform:

```bash
cargo build --release
```

The resulting binary will be located at `target/release/sudan-digital-archive-mcp-server`.

### Configuration

You can configure the server using command-line arguments or environment variables.

- `--api-key` or `API_KEY`: Your SDA API key.
- `--base-url`: (Optional) The base URL for the SDA API.

### Integration Example (Gemini/Claude)

To use this server with an MCP client, add it to your configuration. Here is an example for Gemini:

```json
{
  "mcpServers": {
    "sudan-digital-archive": {
      "command": "/path/to/your/sudan-digital-archive-mcp-server",
      "args": [
        "--api-key", "YOUR_API_KEY_HERE"
      ],
      "env": {
        "API_KEY": "YOUR_API_KEY_HERE"
      }
    }
  }
}
```

## Note on MCP Compatibility

Model Context Protocol is a developing standard. During development, I observed that some MCP clients have issues consuming JSON schemas that use complex optional types (e.g., `Option<T>` in Rust translating to `anyOf: [..., { "type": "null" }]`).

To maximize compatibility with current MCP clients, we have intentionally avoided using optional types for tool arguments, preferring default values (like `-1` for pagination or empty strings) where appropriate. This area may be updated in the future as MCP client implementations mature.

## Development

The project includes a GitHub workflow for quality assurance:
- **Format**: Checks code formatting with `cargo fmt`.
- **Lint**: Checks for common issues with `cargo clippy`.
- **Check**: Ensures the project compiles successfully.

## License

This project is licensed under the terms of the license included in the repository.
