//! MCP Server implementation for the Sudan Digital Archive.
//!
//! This module defines the `SdaServer` struct which implements the MCP server logic,
//! including tool registration and handling.

use crate::client::SdaClient;
use crate::model::*;
use anyhow::Result;
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    service::RequestContext,
    tool, tool_handler, tool_router,
};

/// The Sudan Digital Archive MCP Server.
///
/// It wraps an `SdaClient` and provides tools to interact with the SDA API
/// according to the Model Context Protocol.
#[derive(Clone)]
pub struct SdaServer {
    /// Client for interacting with the SDA API.
    client: SdaClient,
    /// Router for MCP tools.
    tool_router: ToolRouter<SdaServer>,
}

#[tool_router]
impl SdaServer {
    /// Creates a new instance of the `SdaServer`.
    pub fn new(client: SdaClient) -> Self {
        Self {
            client,
            tool_router: Self::tool_router(),
        }
    }

    /// Lists accessions from the Sudan Digital Archive.
    #[tool(description = "List accessions")]
    async fn list_accessions(
        &self,
        Parameters(args): Parameters<ListAccessionsArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .list_accessions(args)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    /// Lists private accessions from the Sudan Digital Archive.
    #[tool(description = "List private accessions")]
    async fn list_private_accessions(
        &self,
        Parameters(args): Parameters<ListAccessionsArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .list_private_accessions(args)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    /// Retrieves a single accession by its ID.
    #[tool(description = "Get a single accession")]
    async fn get_accession(
        &self,
        Parameters(args): Parameters<IdArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .get_accession(args.id)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    /// Retrieves a single private accession by its ID.
    #[tool(description = "Get a single private accession")]
    async fn get_private_accession(
        &self,
        Parameters(args): Parameters<IdArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .get_private_accession(args.id)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    /// Updates an existing accession.
    #[tool(description = "Update an accession")]
    async fn update_accession(
        &self,
        Parameters(args): Parameters<UpdateAccessionArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .update_accession(args.id, args.request)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    /// Lists metadata subjects available in the archive.
    #[tool(description = "List subjects")]
    async fn list_subjects(
        &self,
        Parameters(args): Parameters<ListSubjectsArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .list_subjects(
                if args.page != -1 {
                    Some(args.page)
                } else {
                    None
                },
                if args.per_page != -1 {
                    Some(args.per_page)
                } else {
                    None
                },
            )
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    /// Creates a new metadata subject.
    #[tool(description = "Create a subject")]
    async fn create_subject(
        &self,
        Parameters(args): Parameters<CreateSubjectArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .create_subject(args.request)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// Deletes an existing metadata subject.
    #[tool(description = "Delete a subject")]
    async fn delete_subject(
        &self,
        Parameters(args): Parameters<DeleteSubjectArgs>,
    ) -> Result<CallToolResult, McpError> {
        let request = DeleteSubjectRequest { lang: args.lang };
        self.client
            .delete_subject(args.id, request)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            "Subject deleted successfully".to_string(),
        )]))
    }
}

#[tool_handler]
impl ServerHandler for SdaServer {
    /// Provides information about the server and its capabilities.
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "This server provides tools to interact with the Sudan Digital Archive API."
                    .to_string(),
            ),
        }
    }

    /// Initializes the server connection.
    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(self.get_info())
    }
}
