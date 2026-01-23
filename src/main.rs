//! Main entry point for the Sudan Digital Archive MCP Server.
//!
//! This module handles command-line argument parsing, logging initialization,
//! and starts the MCP server using the stdio transport.

use anyhow::Result;
use clap::Parser;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{self, EnvFilter};

mod client;
mod model;
mod server;

use client::SdaClient;
use server::SdaServer;

/// Command-line arguments for the Sudan Digital Archive MCP Server.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// API Key for Sudan Digital Archive.
    /// Can also be set via the `API_KEY` environment variable.
    #[arg(long, env = "API_KEY")]
    api_key: String,

    /// Base URL for the Sudan Digital Archive API.
    #[arg(long, default_value = "https://api.sudandigitalarchive.com/sda-api")]
    base_url: String,
}

/// Main function to initialize and run the MCP server.
///
/// It parses arguments, sets up tracing for logging, and starts the server
/// listening on stdin/stdout.
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting SDA MCP server");

    let client = SdaClient::new(args.base_url, args.api_key);
    let server = SdaServer::new(client);

    let service = server.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}
