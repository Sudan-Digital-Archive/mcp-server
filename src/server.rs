//! MCP Server implementation for the Sudan Digital Archive.
//!
//! This module defines the `SdaServer` struct which implements the MCP server logic,
//! including tool registration and handling.

use crate::client::SdaClient;
use crate::model::{
    CreateAccessionCrawlArgs, CreateAccessionCrawlRequest, CreateCollectionArgs,
    CreateCollectionRequest, CreateContributorArgs, CreateContributorRequest,
    CreateContributorRoleArgs, CreateContributorRoleRequest, CreateCreatorArgs,
    CreateCreatorRequest, CreateLocationArgs, CreateLocationRequest, CreateRelationArgs,
    CreateRelationRequest, CreateSubjectArgs, CreateSubjectRequest, DeleteContributorArgs,
    DeleteContributorRequest, DeleteContributorRoleArgs, DeleteContributorRoleRequest,
    DeleteCreatorArgs, DeleteCreatorRequest, DeleteLocationArgs, DeleteLocationRequest,
    DeleteRelationArgs, DeleteSubjectArgs, DeleteSubjectRequest, GetCollectionArgs,
    GetContributorArgs, GetContributorRoleArgs, GetCreatorArgs, GetLocationArgs, GetRelationArgs,
    GetSubjectArgs, IdArgs, ListAccessionsArgs, ListCollectionsArgs, ListContributorRolesArgs,
    ListContributorsArgs, ListCreatorsArgs, ListLocationsArgs, ListPrivateCollectionsArgs,
    ListRelationsArgs, ListSubjectsArgs, UpdateAccessionArgs, UpdateAccessionRequest,
    UpdateCollectionArgs, UpdateCollectionRequest, UpdateContributorArgs, UpdateContributorRequest,
    UpdateContributorRoleArgs, UpdateContributorRoleRequest, UpdateCreatorArgs,
    UpdateCreatorRequest, UpdateLocationArgs, UpdateLocationRequest, UpdateSubjectArgs,
    UpdateSubjectRequest,
};
use anyhow::{Context, Result};
use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, Content, Implementation, InitializeRequestParam, InitializeResult,
        ProtocolVersion, ServerCapabilities, ServerInfo,
    },
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

/// Converts a default ID value (-1) to None for API requests.
/// MCP clients pass -1 to indicate "not set", but the API expects null.
#[allow(dead_code)]
pub(crate) fn opt_id(id: i64) -> Option<i64> {
    if id == -1 { None } else { Some(id) }
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
            .context("Failed to list accessions")
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

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
            .context("Failed to list private accessions")
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

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
            .context(format!("Failed to get accession with ID {}", args.id))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

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
            .context(format!(
                "Failed to get private accession with ID {}",
                args.id
            ))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    /// Updates an existing accession.
    ///
    /// **Important Language Convention:**
    /// - When `metadata_language` is `"english"`: provide English text in `metadata_title` and `metadata_description`
    /// - When `metadata_language` is `"arabic"`: provide Arabic text in `metadata_title` and `metadata_description`
    ///
    /// The API uses `metadata_language` to determine which language's metadata you're updating.
    #[tool(
        description = "Update an accession. Note: contributor_role_ids must be 1:1 with contributor_ids (same length). **Important:** The metadata_language field determines which language's title and description are being updated - when set to english, provide English text in metadata_title/metadata_description; when set to arabic, provide Arabic text in those fields."
    )]
    async fn update_accession(
        &self,
        Parameters(args): Parameters<UpdateAccessionArgs>,
    ) -> Result<CallToolResult, McpError> {
        let opt_id = |id: i64| if id == -1 { None } else { Some(id) };
        let request = UpdateAccessionRequest {
            is_private: args.is_private,
            metadata_description: args.metadata_description,
            metadata_language: args.metadata_language,
            metadata_subjects: args.metadata_subjects,
            metadata_time: args.metadata_time,
            metadata_title: args.metadata_title,
            metadata_contributor_ids: args.metadata_contributor_ids,
            metadata_contributor_role_ids: args.metadata_contributor_role_ids,
            metadata_creator_id: opt_id(args.metadata_creator_id),
            metadata_location_id: opt_id(args.metadata_location_id),
        };
        let response = self
            .client
            .update_accession(args.id, request)
            .await
            .context(format!("Failed to update accession with ID {}", args.id))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    /// Creates a new accession by crawling a URL.
    ///
    /// **Important Language Convention:**
    /// - When `metadata_language` is `"english"`: provide English text in `metadata_title` and `metadata_description`
    /// - When `metadata_language` is `"arabic"`: provide Arabic text in `metadata_title` and `metadata_description`
    #[tool(
        description = "Create a new accession (crawl). Note: metadata_time must be in ISO 8601 format without timezone (e.g., '2026-02-01T00:00:00', not '2026-02-01T00:00:00Z'). Contributor role IDs must be 1:1 with contributor IDs (same length). **Important:** The metadata_language field determines which language's title and description are being created - when set to english, provide English text; when set to arabic, provide Arabic text."
    )]
    async fn create_accession_crawl(
        &self,
        Parameters(args): Parameters<CreateAccessionCrawlArgs>,
    ) -> Result<CallToolResult, McpError> {
        let opt_id = |id: i64| if id == -1 { None } else { Some(id) };
        let request = CreateAccessionCrawlRequest {
            url: args.url,
            metadata_language: args.metadata_language,
            metadata_title: args.metadata_title,
            metadata_time: args.metadata_time,
            metadata_subjects: args.metadata_subjects,
            is_private: args.is_private,
            metadata_format: args.metadata_format,
            browser_profile: args.browser_profile,
            metadata_description: args.metadata_description,
            s3_filename: args.s3_filename,
            metadata_contributor_ids: args.metadata_contributor_ids,
            metadata_contributor_role_ids: args.metadata_contributor_role_ids,
            metadata_creator_id: opt_id(args.metadata_creator_id),
            metadata_location_id: opt_id(args.metadata_location_id),
            send_email_notification: args.send_email_notification,
        };
        let response = self
            .client
            .create_accession_crawl(request)
            .await
            .context("Failed to create accession crawl")
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// Lists metadata subjects available in the archive.
    ///
    /// **Important:** Use the `lang` parameter to specify which language's subjects to retrieve:
    /// - `lang: "english"` returns English subjects
    /// - `lang: "arabic"` returns Arabic subjects
    #[tool(
        description = "List subjects. Use the lang parameter to specify 'english' or 'arabic' to get subjects in that language."
    )]
    async fn list_subjects(
        &self,
        Parameters(args): Parameters<ListSubjectsArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .list_subjects(
                args.lang,
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
                if args.in_collection_id != -1 {
                    Some(args.in_collection_id)
                } else {
                    None
                },
            )
            .await
            .context("Failed to list subjects")
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    /// Retrieves a single subject by its ID.
    #[tool(description = "Get a single subject")]
    async fn get_subject(
        &self,
        Parameters(args): Parameters<GetSubjectArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .get_subject(args.id, args.lang)
            .await
            .context(format!("Failed to get subject with ID {}", args.id))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

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
        let request = CreateSubjectRequest {
            lang: args.lang,
            metadata_subject: args.metadata_subject,
        };
        let response = self
            .client
            .create_subject(request)
            .await
            .context("Failed to create subject")
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

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
            .context(format!("Failed to delete subject with ID {}", args.id))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            "Subject deleted successfully".to_string(),
        )]))
    }

    /// Updates an existing metadata subject.
    #[tool(description = "Update a subject")]
    async fn update_subject(
        &self,
        Parameters(args): Parameters<UpdateSubjectArgs>,
    ) -> Result<CallToolResult, McpError> {
        let request = UpdateSubjectRequest {
            lang: args.lang,
            metadata_subject: args.metadata_subject,
        };
        let response = self
            .client
            .update_subject(args.id, request)
            .await
            .context(format!("Failed to update subject with ID {}", args.id))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    /// Lists public collections from the Sudan Digital Archive.
    #[tool(description = "List public collections")]
    async fn list_collections(
        &self,
        Parameters(args): Parameters<ListCollectionsArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .list_collections(args)
            .await
            .context("Failed to list collections")
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    /// Lists private collections from the Sudan Digital Archive.
    #[tool(description = "List private collections")]
    async fn list_private_collections(
        &self,
        Parameters(args): Parameters<ListPrivateCollectionsArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .list_private_collections(args)
            .await
            .context("Failed to list private collections")
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    /// Retrieves a single collection by its ID.
    #[tool(description = "Get a single collection")]
    async fn get_collection(
        &self,
        Parameters(args): Parameters<GetCollectionArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .get_collection(args.id, args.lang)
            .await
            .context(format!("Failed to get collection with ID {}", args.id))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    /// Creates a new collection.
    #[tool(description = "Create a collection")]
    async fn create_collection(
        &self,
        Parameters(args): Parameters<CreateCollectionArgs>,
    ) -> Result<CallToolResult, McpError> {
        let request = CreateCollectionRequest {
            lang: args.lang,
            title: args.title,
            is_private: args.is_private,
            subject_ids: args.subject_ids,
            description: args.description,
        };
        let response = self
            .client
            .create_collection(request)
            .await
            .context("Failed to create collection")
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// Updates an existing collection.
    #[tool(description = "Update a collection")]
    async fn update_collection(
        &self,
        Parameters(args): Parameters<UpdateCollectionArgs>,
    ) -> Result<CallToolResult, McpError> {
        let request = UpdateCollectionRequest {
            lang: args.lang,
            title: args.title,
            is_private: args.is_private,
            subject_ids: args.subject_ids,
            description: args.description,
        };
        let response = self
            .client
            .update_collection(args.id, request)
            .await
            .context(format!("Failed to update collection with ID {}", args.id))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(description = "List contributors")]
    async fn list_contributors(
        &self,
        Parameters(args): Parameters<ListContributorsArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .list_contributors(
                args.lang,
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
                args.query_term,
            )
            .await
            .context("Failed to list contributors")
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(description = "Get a contributor")]
    async fn get_contributor(
        &self,
        Parameters(args): Parameters<GetContributorArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .get_contributor(args.id, args.lang)
            .await
            .context(format!("Failed to get contributor with ID {}", args.id))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(description = "Create a contributor")]
    async fn create_contributor(
        &self,
        Parameters(args): Parameters<CreateContributorArgs>,
    ) -> Result<CallToolResult, McpError> {
        let request = CreateContributorRequest {
            lang: args.lang,
            contributor: args.contributor,
        };
        let response = self
            .client
            .create_contributor(request)
            .await
            .context("Failed to create contributor")
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    #[tool(description = "Update a contributor")]
    async fn update_contributor(
        &self,
        Parameters(args): Parameters<UpdateContributorArgs>,
    ) -> Result<CallToolResult, McpError> {
        let request = UpdateContributorRequest {
            lang: args.lang,
            contributor: args.contributor,
        };
        let response = self
            .client
            .update_contributor(args.id, request)
            .await
            .context(format!("Failed to update contributor with ID {}", args.id))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(description = "Delete a contributor")]
    async fn delete_contributor(
        &self,
        Parameters(args): Parameters<DeleteContributorArgs>,
    ) -> Result<CallToolResult, McpError> {
        let request = DeleteContributorRequest { lang: args.lang };
        self.client
            .delete_contributor(args.id, request)
            .await
            .context(format!("Failed to delete contributor with ID {}", args.id))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            "Contributor deleted successfully".to_string(),
        )]))
    }

    #[tool(description = "List contributor roles")]
    async fn list_contributor_roles(
        &self,
        Parameters(args): Parameters<ListContributorRolesArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .list_contributor_roles(
                args.lang,
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
                args.query_term,
            )
            .await
            .context("Failed to list contributor roles")
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(description = "Get a contributor role")]
    async fn get_contributor_role(
        &self,
        Parameters(args): Parameters<GetContributorRoleArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .get_contributor_role(args.id, args.lang)
            .await
            .context(format!(
                "Failed to get contributor role with ID {}",
                args.id
            ))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(description = "Create a contributor role")]
    async fn create_contributor_role(
        &self,
        Parameters(args): Parameters<CreateContributorRoleArgs>,
    ) -> Result<CallToolResult, McpError> {
        let request = CreateContributorRoleRequest {
            lang: args.lang,
            role: args.role,
        };
        let response = self
            .client
            .create_contributor_role(request)
            .await
            .context("Failed to create contributor role")
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    #[tool(description = "Update a contributor role")]
    async fn update_contributor_role(
        &self,
        Parameters(args): Parameters<UpdateContributorRoleArgs>,
    ) -> Result<CallToolResult, McpError> {
        let request = UpdateContributorRoleRequest {
            lang: args.lang,
            role: args.role,
        };
        let response = self
            .client
            .update_contributor_role(args.id, request)
            .await
            .context(format!(
                "Failed to update contributor role with ID {}",
                args.id
            ))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(description = "Delete a contributor role")]
    async fn delete_contributor_role(
        &self,
        Parameters(args): Parameters<DeleteContributorRoleArgs>,
    ) -> Result<CallToolResult, McpError> {
        let request = DeleteContributorRoleRequest { lang: args.lang };
        self.client
            .delete_contributor_role(args.id, request)
            .await
            .context(format!(
                "Failed to delete contributor role with ID {}",
                args.id
            ))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            "Contributor role deleted successfully".to_string(),
        )]))
    }

    #[tool(description = "List creators")]
    async fn list_creators(
        &self,
        Parameters(args): Parameters<ListCreatorsArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .list_creators(
                args.lang,
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
                args.query_term,
            )
            .await
            .context("Failed to list creators")
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(description = "Get a creator")]
    async fn get_creator(
        &self,
        Parameters(args): Parameters<GetCreatorArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .get_creator(args.id, args.lang)
            .await
            .context(format!("Failed to get creator with ID {}", args.id))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(description = "Create a creator")]
    async fn create_creator(
        &self,
        Parameters(args): Parameters<CreateCreatorArgs>,
    ) -> Result<CallToolResult, McpError> {
        let request = CreateCreatorRequest {
            lang: args.lang,
            creator: args.creator,
        };
        let response = self
            .client
            .create_creator(request)
            .await
            .context("Failed to create creator")
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    #[tool(description = "Update a creator")]
    async fn update_creator(
        &self,
        Parameters(args): Parameters<UpdateCreatorArgs>,
    ) -> Result<CallToolResult, McpError> {
        let request = UpdateCreatorRequest {
            lang: args.lang,
            creator: args.creator,
        };
        let response = self
            .client
            .update_creator(args.id, request)
            .await
            .context(format!("Failed to update creator with ID {}", args.id))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(description = "Delete a creator")]
    async fn delete_creator(
        &self,
        Parameters(args): Parameters<DeleteCreatorArgs>,
    ) -> Result<CallToolResult, McpError> {
        let request = DeleteCreatorRequest { lang: args.lang };
        self.client
            .delete_creator(args.id, request)
            .await
            .context(format!("Failed to delete creator with ID {}", args.id))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            "Creator deleted successfully".to_string(),
        )]))
    }

    #[tool(description = "List locations")]
    async fn list_locations(
        &self,
        Parameters(args): Parameters<ListLocationsArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .list_locations(
                args.lang,
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
                args.query_term,
            )
            .await
            .context("Failed to list locations")
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(description = "Get a location")]
    async fn get_location(
        &self,
        Parameters(args): Parameters<GetLocationArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .get_location(args.id, args.lang)
            .await
            .context(format!("Failed to get location with ID {}", args.id))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(description = "Create a location")]
    async fn create_location(
        &self,
        Parameters(args): Parameters<CreateLocationArgs>,
    ) -> Result<CallToolResult, McpError> {
        let request = CreateLocationRequest {
            lang: args.lang,
            location: args.location,
        };
        let response = self
            .client
            .create_location(request)
            .await
            .context("Failed to create location")
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    #[tool(description = "Update a location")]
    async fn update_location(
        &self,
        Parameters(args): Parameters<UpdateLocationArgs>,
    ) -> Result<CallToolResult, McpError> {
        let request = UpdateLocationRequest {
            lang: args.lang,
            location: args.location,
        };
        let response = self
            .client
            .update_location(args.id, request)
            .await
            .context(format!("Failed to update location with ID {}", args.id))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(description = "Delete a location")]
    async fn delete_location(
        &self,
        Parameters(args): Parameters<DeleteLocationArgs>,
    ) -> Result<CallToolResult, McpError> {
        let request = DeleteLocationRequest { lang: args.lang };
        self.client
            .delete_location(args.id, request)
            .await
            .context(format!("Failed to delete location with ID {}", args.id))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            "Location deleted successfully".to_string(),
        )]))
    }

    #[tool(description = "List relations for an accession")]
    async fn list_relations(
        &self,
        Parameters(args): Parameters<ListRelationsArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .list_relations(args.accession_id, args.lang)
            .await
            .context(format!(
                "Failed to list relations for accession {}",
                args.accession_id
            ))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(description = "Get a relation")]
    async fn get_relation(
        &self,
        Parameters(args): Parameters<GetRelationArgs>,
    ) -> Result<CallToolResult, McpError> {
        let response = self
            .client
            .get_relation(args.accession_id, args.relation_id, args.lang)
            .await
            .context(format!(
                "Failed to get relation {} for accession {}",
                args.relation_id, args.accession_id
            ))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&response).unwrap(),
        )]))
    }

    #[tool(description = "Create a relation")]
    async fn create_relation(
        &self,
        Parameters(args): Parameters<CreateRelationArgs>,
    ) -> Result<CallToolResult, McpError> {
        let request = CreateRelationRequest {
            related_accession_id: args.related_accession_id,
            relation_type: args.relation_type,
        };
        let response = self
            .client
            .create_relation(args.accession_id, request)
            .await
            .context(format!(
                "Failed to create relation for accession {}",
                args.accession_id
            ))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    #[tool(description = "Delete a relation")]
    async fn delete_relation(
        &self,
        Parameters(args): Parameters<DeleteRelationArgs>,
    ) -> Result<CallToolResult, McpError> {
        self.client
            .delete_relation(args.accession_id, args.relation_id, args.lang)
            .await
            .context(format!(
                "Failed to delete relation {} for accession {}",
                args.relation_id, args.accession_id
            ))
            .map_err(|e| McpError::internal_error(format!("{:#}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            "Relation deleted successfully".to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opt_id_returns_none_for_minus_one() {
        let result = opt_id(-1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_opt_id_returns_some_for_positive_id() {
        let result = opt_id(123);
        assert_eq!(result, Some(123));
    }

    #[test]
    fn test_opt_id_returns_some_for_zero() {
        let result = opt_id(0);
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_opt_id_returns_some_for_large_id() {
        let result = opt_id(1073741824);
        assert_eq!(result, Some(1073741824));
    }
}
