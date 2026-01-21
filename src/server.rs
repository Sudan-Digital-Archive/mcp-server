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

#[derive(Clone)]
pub struct SdaServer {
    client: SdaClient,
    tool_router: ToolRouter<SdaServer>,
}

#[tool_router]
impl SdaServer {
    pub fn new(client: SdaClient) -> Self {
        Self {
            client,
            tool_router: Self::tool_router(),
        }
    }

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

    #[tool(description = "List subjects")]
    async fn list_subjects(
        &self,
        Parameters(args): Parameters<ListSubjectsArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .list_subjects(
                if args.page != -1 { Some(args.page) } else { None },
                if args.per_page != -1 { Some(args.per_page) } else { None },
            )
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

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

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(self.get_info())
    }
}
