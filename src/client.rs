//! API Client for the Sudan Digital Archive.
//!
//! This module provides a client for making HTTP requests to the SDA API.

use crate::model::*;
use anyhow::{Context, Result};
use reqwest::Client;

/// Client for interacting with the Sudan Digital Archive API.
#[derive(Clone)]
pub struct SdaClient {
    /// Internal HTTP client.
    client: Client,
    /// Base URL of the SDA API.
    base_url: String,
    /// API key for authentication.
    api_key: String,
}

impl SdaClient {
    /// Creates a new `SdaClient` with the given base URL and API key.
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            api_key,
        }
    }

    /// Returns the authentication header as a key-value tuple.
    fn auth_header(&self) -> (&str, &str) {
        ("x-api-key", &self.api_key)
    }

    /// Helper function to handle HTTP responses and capture error bodies.
    ///
    /// This is preferred over `error_for_status()` because it captures
    /// the response body (e.g., validation error details) and includes it
    /// in the error message, making debugging much easier.
    async fn handle_response(
        response: reqwest::Response,
        context: &str,
    ) -> Result<reqwest::Response> {
        if !response.status().is_success() {
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<failed to read error body>".to_string());
            let msg = if body.is_empty() {
                format!("{}: HTTP {}", context, status)
            } else {
                format!("{}: HTTP {} - {}", context, status, body)
            };
            return Err(anyhow::anyhow!(msg));
        }
        Ok(response)
    }

    /// Builds a query vector for accession-related requests.
    fn build_accession_query(
        &self,
        args: ListAccessionsArgs,
    ) -> Result<Vec<(&'static str, String)>> {
        let mut query = vec![];
        if args.page != -1 {
            query.push(("page", args.page.to_string()));
        }
        if args.per_page != -1 {
            query.push(("per_page", args.per_page.to_string()));
        }
        match args.lang {
            MetadataLanguage::English => query.push(("lang", "english".to_string())),
            MetadataLanguage::Arabic => query.push(("lang", "arabic".to_string())),
            MetadataLanguage::None => {}
        }
        if !args.metadata_subjects.is_empty() {
            for s in args.metadata_subjects {
                query.push(("metadata_subjects", s.to_string()));
            }
        }
        if args.metadata_subjects_inclusive_filter {
            query.push(("metadata_subjects_inclusive_filter", "true".to_string()));
        }
        if !args.metadata_locations.is_empty() {
            for s in args.metadata_locations {
                query.push(("metadata_locations", s.to_string()));
            }
        }
        if !args.metadata_creators.is_empty() {
            for s in args.metadata_creators {
                query.push(("metadata_creators", s.to_string()));
            }
        }
        if !args.metadata_contributors.is_empty() {
            for s in args.metadata_contributors {
                query.push(("metadata_contributors", s.to_string()));
            }
        }
        if args.metadata_contributors_inclusive_filter {
            query.push(("metadata_contributors_inclusive_filter", "true".to_string()));
        }
        if !args.metadata_contributor_roles.is_empty() {
            for s in args.metadata_contributor_roles {
                query.push(("metadata_contributor_roles", s.to_string()));
            }
        }
        if args.metadata_contributor_roles_inclusive_filter {
            query.push((
                "metadata_contributor_roles_inclusive_filter",
                "true".to_string(),
            ));
        }
        if !args.query_term.is_empty() {
            query.push(("query_term", args.query_term));
        }
        if !args.url_filter.is_empty() {
            query.push(("url_filter", args.url_filter));
        }
        if !args.date_from.is_empty() {
            query.push(("date_from", args.date_from));
        }
        if !args.date_to.is_empty() {
            query.push(("date_to", args.date_to));
        }
        if !args.location.is_empty() {
            query.push(("location", args.location));
        }
        if args.is_private {
            query.push(("is_private", "true".to_string()));
        }
        Ok(query)
    }

    /// Creates a new accession (starts a crawl).
    pub async fn create_accession_crawl(
        &self,
        request: CreateAccessionCrawlRequest,
    ) -> Result<String> {
        let url = format!("{}/api/v1/accessions/crawl", self.base_url);
        let response = self
            .client
            .post(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context("Failed to send create accession crawl request")?;

        let response =
            Self::handle_response(response, "Server returned error for create accession crawl")
                .await?;

        response
            .text()
            .await
            .context("Failed to parse create accession crawl response text")
    }

    /// Fetches a list of public accessions.
    pub async fn list_accessions(
        &self,
        args: ListAccessionsArgs,
    ) -> Result<ListAccessionsResponse> {
        let url = format!("{}/api/v1/accessions", self.base_url);
        let query = self.build_accession_query(args)?;

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context("Failed to send list accessions request")?;

        let response =
            Self::handle_response(response, "Server returned error for list accessions").await?;

        response
            .json()
            .await
            .context("Failed to parse list accessions response")
    }

    /// Fetches a list of private accessions.
    pub async fn list_private_accessions(
        &self,
        args: ListAccessionsArgs,
    ) -> Result<ListAccessionsResponse> {
        let url = format!("{}/api/v1/accessions/private", self.base_url);
        let query = self.build_accession_query(args)?;

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context("Failed to send list private accessions request")?;

        let response = Self::handle_response(
            response,
            "Server returned error for list private accessions",
        )
        .await?;

        response
            .json()
            .await
            .context("Failed to parse list private accessions response")
    }

    /// Retrieves a single public accession by its ID.
    pub async fn get_accession(&self, id: i32) -> Result<GetOneAccessionResponse> {
        let url = format!("{}/api/v1/accessions/{}", self.base_url, id);
        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .send()
            .await
            .context("Failed to send get accession request")?;

        let response =
            Self::handle_response(response, "Server returned error for get accession").await?;

        response
            .json()
            .await
            .context("Failed to parse get accession response")
    }

    /// Retrieves a single private accession by its ID.
    pub async fn get_private_accession(&self, id: i32) -> Result<GetOneAccessionResponse> {
        let url = format!("{}/api/v1/accessions/private/{}", self.base_url, id);
        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .send()
            .await
            .context("Failed to send get private accession request")?;

        let response =
            Self::handle_response(response, "Server returned error for get private accession")
                .await?;

        response
            .json()
            .await
            .context("Failed to parse get private accession response")
    }

    /// Updates an existing accession.
    pub async fn update_accession(
        &self,
        id: i32,
        request: UpdateAccessionRequest,
    ) -> Result<GetOneAccessionResponse> {
        let url = format!("{}/api/v1/accessions/{}", self.base_url, id);
        let response = self
            .client
            .put(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context("Failed to send update accession request")?;

        let response =
            Self::handle_response(response, "Server returned error for update accession").await?;

        response
            .json()
            .await
            .context("Failed to parse update accession response")
    }

    /// Lists metadata subjects with language parameter and optional pagination.
    pub async fn list_subjects(
        &self,
        lang: MetadataLanguage,
        page: Option<i64>,
        per_page: Option<i64>,
        in_collection_id: Option<i32>,
    ) -> Result<ListSubjectsResponse> {
        let url = format!("{}/api/v1/subjects", self.base_url);
        let mut query = vec![];

        // Add language parameter
        match lang {
            MetadataLanguage::English => query.push(("lang", "english".to_string())),
            MetadataLanguage::Arabic => query.push(("lang", "arabic".to_string())),
            MetadataLanguage::None => query.push(("lang", "english".to_string())), // fallback
        }

        if let Some(p) = page {
            query.push(("page", p.to_string()));
        }
        if let Some(pp) = per_page {
            query.push(("per_page", pp.to_string()));
        }
        if let Some(cid) = in_collection_id
            && cid != -1
        {
            query.push(("in_collection_id", cid.to_string()));
        }

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context("Failed to send list subjects request")?;

        let response =
            Self::handle_response(response, "Server returned error for list subjects").await?;

        response
            .json()
            .await
            .context("Failed to parse list subjects response")
    }

    /// Creates a new metadata subject.
    pub async fn create_subject(&self, request: CreateSubjectRequest) -> Result<String> {
        let url = format!("{}/api/v1/subjects", self.base_url);
        let response = self
            .client
            .post(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context("Failed to send create subject request")?;

        let response =
            Self::handle_response(response, "Server returned error for create subject").await?;

        response
            .text()
            .await
            .context("Failed to parse create subject response text")
    }

    /// Deletes a metadata subject by its ID.
    pub async fn delete_subject(&self, id: i32, request: DeleteSubjectRequest) -> Result<()> {
        let url = format!("{}/api/v1/subjects/{}", self.base_url, id);
        let response = self
            .client
            .delete(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context("Failed to send delete subject request")?;

        Self::handle_response(response, "Server returned error for delete subject").await?;
        Ok(())
    }

    /// Updates a metadata subject by its ID.
    pub async fn update_subject(
        &self,
        id: i32,
        request: UpdateSubjectRequest,
    ) -> Result<DublinMetadataSubjectResponse> {
        let url = format!("{}/api/v1/subjects/{}", self.base_url, id);
        let response = self
            .client
            .put(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context("Failed to send update subject request")?;

        let response =
            Self::handle_response(response, "Server returned error for update subject").await?;

        response
            .json()
            .await
            .context("Failed to parse update subject response")
    }

    /// Retrieves a single metadata subject by its ID.
    pub async fn get_subject(
        &self,
        id: i32,
        lang: MetadataLanguage,
    ) -> Result<DublinMetadataSubjectResponse> {
        let url = format!("{}/api/v1/subjects/{}", self.base_url, id);
        let mut query = vec![];

        match lang {
            MetadataLanguage::English => query.push(("lang", "english".to_string())),
            MetadataLanguage::Arabic => query.push(("lang", "arabic".to_string())),
            MetadataLanguage::None => {}
        }

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context(format!("Failed to send get subject request for ID {}", id))?;

        let response = Self::handle_response(
            response,
            &format!("Server returned error for get subject {}", id),
        )
        .await?;

        response
            .json()
            .await
            .context("Failed to parse get subject response")
    }

    /// Lists public collections.
    pub async fn list_collections(
        &self,
        args: ListCollectionsArgs,
    ) -> Result<ListCollectionsResponse> {
        let url = format!("{}/api/v1/collections", self.base_url);
        let mut query = vec![];

        if args.page != -1 {
            query.push(("page", args.page.to_string()));
        }
        if args.per_page != -1 {
            query.push(("per_page", args.per_page.to_string()));
        }
        match args.lang {
            MetadataLanguage::English => query.push(("lang", "english".to_string())),
            MetadataLanguage::Arabic => query.push(("lang", "arabic".to_string())),
            MetadataLanguage::None => {}
        }

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context("Failed to send list collections request")?;

        let response =
            Self::handle_response(response, "Server returned error for list collections").await?;

        response
            .json()
            .await
            .context("Failed to parse list collections response")
    }

    /// Lists private collections.
    pub async fn list_private_collections(
        &self,
        args: ListPrivateCollectionsArgs,
    ) -> Result<ListCollectionsResponse> {
        let url = format!("{}/api/v1/collections/private", self.base_url);
        let mut query = vec![];

        if args.page != -1 {
            query.push(("page", args.page.to_string()));
        }
        if args.per_page != -1 {
            query.push(("per_page", args.per_page.to_string()));
        }
        match args.lang {
            MetadataLanguage::English => query.push(("lang", "english".to_string())),
            MetadataLanguage::Arabic => query.push(("lang", "arabic".to_string())),
            MetadataLanguage::None => {}
        }
        query.push(("is_private", args.is_private.to_string()));

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context("Failed to send list private collections request")?;

        let response = Self::handle_response(
            response,
            "Server returned error for list private collections",
        )
        .await?;

        response
            .json()
            .await
            .context("Failed to parse list private collections response")
    }

    /// Retrieves a single collection by its ID.
    pub async fn get_collection(
        &self,
        id: i32,
        lang: MetadataLanguage,
    ) -> Result<CollectionResponse> {
        let url = format!("{}/api/v1/collections/{}", self.base_url, id);
        let mut query = vec![];

        match lang {
            MetadataLanguage::English => query.push(("lang", "english".to_string())),
            MetadataLanguage::Arabic => query.push(("lang", "arabic".to_string())),
            MetadataLanguage::None => {}
        }

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context(format!(
                "Failed to send get collection request for ID {}",
                id
            ))?;

        let response = Self::handle_response(
            response,
            &format!("Server returned error for get collection {}", id),
        )
        .await?;

        response
            .json()
            .await
            .context("Failed to parse get collection response")
    }

    /// Creates a new collection.
    pub async fn create_collection(&self, request: CreateCollectionRequest) -> Result<String> {
        let url = format!("{}/api/v1/collections", self.base_url);
        let response = self
            .client
            .post(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context("Failed to send create collection request")?;

        let response =
            Self::handle_response(response, "Server returned error for create collection").await?;

        response
            .text()
            .await
            .context("Failed to parse create collection response text")
    }

    /// Updates an existing collection.
    pub async fn update_collection(
        &self,
        id: i32,
        request: UpdateCollectionRequest,
    ) -> Result<CollectionResponse> {
        let url = format!("{}/api/v1/collections/{}", self.base_url, id);
        let response = self
            .client
            .put(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context(format!(
                "Failed to send update collection request for ID {}",
                id
            ))?;

        let response = Self::handle_response(
            response,
            &format!("Server returned error for update collection {}", id),
        )
        .await?;

        response
            .json()
            .await
            .context("Failed to parse update collection response")
    }

    /// Lists contributors.
    pub async fn list_contributors(
        &self,
        lang: MetadataLanguage,
        page: Option<i64>,
        per_page: Option<i64>,
        query_term: String,
    ) -> Result<ListContributorsResponse> {
        let url = format!("{}/api/v1/contributors", self.base_url);
        let mut query = vec![];

        match lang {
            MetadataLanguage::English => query.push(("lang", "english".to_string())),
            MetadataLanguage::Arabic => query.push(("lang", "arabic".to_string())),
            MetadataLanguage::None => {}
        }
        if let Some(p) = page {
            query.push(("page", p.to_string()));
        }
        if let Some(pp) = per_page {
            query.push(("per_page", pp.to_string()));
        }
        if !query_term.is_empty() {
            query.push(("query_term", query_term));
        }

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context("Failed to send list contributors request")?;

        let response =
            Self::handle_response(response, "Server returned error for list contributors").await?;

        response
            .json()
            .await
            .context("Failed to parse list contributors response")
    }

    /// Gets a single contributor by ID.
    pub async fn get_contributor(
        &self,
        id: i32,
        lang: MetadataLanguage,
    ) -> Result<ContributorResponse> {
        let url = format!("{}/api/v1/contributors/{}", self.base_url, id);
        let mut query = vec![];

        match lang {
            MetadataLanguage::English => query.push(("lang", "english".to_string())),
            MetadataLanguage::Arabic => query.push(("lang", "arabic".to_string())),
            MetadataLanguage::None => {}
        }

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context(format!(
                "Failed to send get contributor request for ID {}",
                id
            ))?;

        let response = Self::handle_response(
            response,
            &format!("Server returned error for get contributor {}", id),
        )
        .await?;

        response
            .json()
            .await
            .context("Failed to parse get contributor response")
    }

    /// Creates a new contributor.
    pub async fn create_contributor(&self, request: CreateContributorRequest) -> Result<String> {
        let url = format!("{}/api/v1/contributors", self.base_url);
        let response = self
            .client
            .post(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context("Failed to send create contributor request")?;

        let response =
            Self::handle_response(response, "Server returned error for create contributor").await?;

        response
            .text()
            .await
            .context("Failed to parse create contributor response text")
    }

    /// Updates a contributor.
    pub async fn update_contributor(
        &self,
        id: i32,
        request: UpdateContributorRequest,
    ) -> Result<ContributorResponse> {
        let url = format!("{}/api/v1/contributors/{}", self.base_url, id);
        let response = self
            .client
            .put(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context(format!(
                "Failed to send update contributor request for ID {}",
                id
            ))?;

        let response = Self::handle_response(
            response,
            &format!("Server returned error for update contributor {}", id),
        )
        .await?;

        response
            .json()
            .await
            .context("Failed to parse update contributor response")
    }

    /// Deletes a contributor.
    pub async fn delete_contributor(
        &self,
        id: i32,
        request: DeleteContributorRequest,
    ) -> Result<()> {
        let url = format!("{}/api/v1/contributors/{}", self.base_url, id);
        let response = self
            .client
            .delete(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context(format!(
                "Failed to send delete contributor request for ID {}",
                id
            ))?;

        Self::handle_response(response, "Server returned error for delete contributor").await?;
        Ok(())
    }

    /// Lists contributor roles.
    pub async fn list_contributor_roles(
        &self,
        lang: MetadataLanguage,
        page: Option<i64>,
        per_page: Option<i64>,
        query_term: String,
    ) -> Result<ListContributorRolesResponse> {
        let url = format!("{}/api/v1/contributors/roles", self.base_url);
        let mut query = vec![];

        match lang {
            MetadataLanguage::English => query.push(("lang", "english".to_string())),
            MetadataLanguage::Arabic => query.push(("lang", "arabic".to_string())),
            MetadataLanguage::None => {}
        }
        if let Some(p) = page {
            query.push(("page", p.to_string()));
        }
        if let Some(pp) = per_page {
            query.push(("per_page", pp.to_string()));
        }
        if !query_term.is_empty() {
            query.push(("query_term", query_term));
        }

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context("Failed to send list contributor roles request")?;

        let response =
            Self::handle_response(response, "Server returned error for list contributor roles")
                .await?;

        response
            .json()
            .await
            .context("Failed to parse list contributor roles response")
    }

    /// Gets a single contributor role by ID.
    pub async fn get_contributor_role(
        &self,
        id: i32,
        lang: MetadataLanguage,
    ) -> Result<ContributorRoleResponse> {
        let url = format!("{}/api/v1/contributors/roles/{}", self.base_url, id);
        let mut query = vec![];

        match lang {
            MetadataLanguage::English => query.push(("lang", "english".to_string())),
            MetadataLanguage::Arabic => query.push(("lang", "arabic".to_string())),
            MetadataLanguage::None => {}
        }

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context(format!(
                "Failed to send get contributor role request for ID {}",
                id
            ))?;

        let response = Self::handle_response(
            response,
            &format!("Server returned error for get contributor role {}", id),
        )
        .await?;

        response
            .json()
            .await
            .context("Failed to parse get contributor role response")
    }

    /// Creates a new contributor role.
    pub async fn create_contributor_role(
        &self,
        request: CreateContributorRoleRequest,
    ) -> Result<String> {
        let url = format!("{}/api/v1/contributors/roles", self.base_url);
        let response = self
            .client
            .post(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context("Failed to send create contributor role request")?;

        let response = Self::handle_response(
            response,
            "Server returned error for create contributor role",
        )
        .await?;

        response
            .text()
            .await
            .context("Failed to parse create contributor role response text")
    }

    /// Updates a contributor role.
    pub async fn update_contributor_role(
        &self,
        id: i32,
        request: UpdateContributorRoleRequest,
    ) -> Result<ContributorRoleResponse> {
        let url = format!("{}/api/v1/contributors/roles/{}", self.base_url, id);
        let response = self
            .client
            .put(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context(format!(
                "Failed to send update contributor role request for ID {}",
                id
            ))?;

        let response = Self::handle_response(
            response,
            &format!("Server returned error for update contributor role {}", id),
        )
        .await?;

        response
            .json()
            .await
            .context("Failed to parse update contributor role response")
    }

    /// Deletes a contributor role.
    pub async fn delete_contributor_role(
        &self,
        id: i32,
        request: DeleteContributorRoleRequest,
    ) -> Result<()> {
        let url = format!("{}/api/v1/contributors/roles/{}", self.base_url, id);
        let response = self
            .client
            .delete(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context(format!(
                "Failed to send delete contributor role request for ID {}",
                id
            ))?;

        Self::handle_response(
            response,
            "Server returned error for delete contributor role",
        )
        .await?;
        Ok(())
    }

    /// Lists creators.
    pub async fn list_creators(
        &self,
        lang: MetadataLanguage,
        page: Option<i64>,
        per_page: Option<i64>,
        query_term: String,
    ) -> Result<ListCreatorsResponse> {
        let url = format!("{}/api/v1/creators", self.base_url);
        let mut query = vec![];

        match lang {
            MetadataLanguage::English => query.push(("lang", "english".to_string())),
            MetadataLanguage::Arabic => query.push(("lang", "arabic".to_string())),
            MetadataLanguage::None => {}
        }
        if let Some(p) = page {
            query.push(("page", p.to_string()));
        }
        if let Some(pp) = per_page {
            query.push(("per_page", pp.to_string()));
        }
        if !query_term.is_empty() {
            query.push(("query_term", query_term));
        }

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context("Failed to send list creators request")?;

        let response =
            Self::handle_response(response, "Server returned error for list creators").await?;

        response
            .json()
            .await
            .context("Failed to parse list creators response")
    }

    /// Gets a single creator by ID.
    pub async fn get_creator(&self, id: i32, lang: MetadataLanguage) -> Result<CreatorResponse> {
        let url = format!("{}/api/v1/creators/{}", self.base_url, id);
        let mut query = vec![];

        match lang {
            MetadataLanguage::English => query.push(("lang", "english".to_string())),
            MetadataLanguage::Arabic => query.push(("lang", "arabic".to_string())),
            MetadataLanguage::None => {}
        }

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context(format!("Failed to send get creator request for ID {}", id))?;

        let response = Self::handle_response(
            response,
            &format!("Server returned error for get creator {}", id),
        )
        .await?;

        response
            .json()
            .await
            .context("Failed to parse get creator response")
    }

    /// Creates a new creator.
    pub async fn create_creator(&self, request: CreateCreatorRequest) -> Result<String> {
        let url = format!("{}/api/v1/creators", self.base_url);
        let response = self
            .client
            .post(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context("Failed to send create creator request")?;

        let response =
            Self::handle_response(response, "Server returned error for create creator").await?;

        response
            .text()
            .await
            .context("Failed to parse create creator response text")
    }

    /// Updates a creator.
    pub async fn update_creator(
        &self,
        id: i32,
        request: UpdateCreatorRequest,
    ) -> Result<CreatorResponse> {
        let url = format!("{}/api/v1/creators/{}", self.base_url, id);
        let response = self
            .client
            .put(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context(format!(
                "Failed to send update creator request for ID {}",
                id
            ))?;

        let response = Self::handle_response(
            response,
            &format!("Server returned error for update creator {}", id),
        )
        .await?;

        response
            .json()
            .await
            .context("Failed to parse update creator response")
    }

    /// Deletes a creator.
    pub async fn delete_creator(&self, id: i32, request: DeleteCreatorRequest) -> Result<()> {
        let url = format!("{}/api/v1/creators/{}", self.base_url, id);
        let response = self
            .client
            .delete(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context(format!(
                "Failed to send delete creator request for ID {}",
                id
            ))?;

        Self::handle_response(response, "Server returned error for delete creator").await?;
        Ok(())
    }

    /// Lists locations.
    pub async fn list_locations(
        &self,
        lang: MetadataLanguage,
        page: Option<i64>,
        per_page: Option<i64>,
        query_term: String,
    ) -> Result<ListLocationsResponse> {
        let url = format!("{}/api/v1/locations", self.base_url);
        let mut query = vec![];

        match lang {
            MetadataLanguage::English => query.push(("lang", "english".to_string())),
            MetadataLanguage::Arabic => query.push(("lang", "arabic".to_string())),
            MetadataLanguage::None => {}
        }
        if let Some(p) = page {
            query.push(("page", p.to_string()));
        }
        if let Some(pp) = per_page {
            query.push(("per_page", pp.to_string()));
        }
        if !query_term.is_empty() {
            query.push(("query_term", query_term));
        }

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context("Failed to send list locations request")?;

        let response =
            Self::handle_response(response, "Server returned error for list locations").await?;

        response
            .json()
            .await
            .context("Failed to parse list locations response")
    }

    /// Gets a single location by ID.
    pub async fn get_location(&self, id: i32, lang: MetadataLanguage) -> Result<LocationResponse> {
        let url = format!("{}/api/v1/locations/{}", self.base_url, id);
        let mut query = vec![];

        match lang {
            MetadataLanguage::English => query.push(("lang", "english".to_string())),
            MetadataLanguage::Arabic => query.push(("lang", "arabic".to_string())),
            MetadataLanguage::None => {}
        }

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context(format!("Failed to send get location request for ID {}", id))?;

        let response = Self::handle_response(
            response,
            &format!("Server returned error for get location {}", id),
        )
        .await?;

        response
            .json()
            .await
            .context("Failed to parse get location response")
    }

    /// Creates a new location.
    pub async fn create_location(&self, request: CreateLocationRequest) -> Result<String> {
        let url = format!("{}/api/v1/locations", self.base_url);
        let response = self
            .client
            .post(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context("Failed to send create location request")?;

        let response =
            Self::handle_response(response, "Server returned error for create location").await?;

        response
            .text()
            .await
            .context("Failed to parse create location response text")
    }

    /// Updates a location.
    pub async fn update_location(
        &self,
        id: i32,
        request: UpdateLocationRequest,
    ) -> Result<LocationResponse> {
        let url = format!("{}/api/v1/locations/{}", self.base_url, id);
        let response = self
            .client
            .put(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context(format!(
                "Failed to send update location request for ID {}",
                id
            ))?;

        let response = Self::handle_response(
            response,
            &format!("Server returned error for update location {}", id),
        )
        .await?;

        response
            .json()
            .await
            .context("Failed to parse update location response")
    }

    /// Deletes a location.
    pub async fn delete_location(&self, id: i32, request: DeleteLocationRequest) -> Result<()> {
        let url = format!("{}/api/v1/locations/{}", self.base_url, id);
        let response = self
            .client
            .delete(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context(format!(
                "Failed to send delete location request for ID {}",
                id
            ))?;

        Self::handle_response(response, "Server returned error for delete location").await?;
        Ok(())
    }

    /// Lists relations for an accession.
    pub async fn list_relations(
        &self,
        accession_id: i32,
        lang: MetadataLanguage,
    ) -> Result<ListRelationsResponse> {
        let url = format!(
            "{}/api/v1/accessions/{}/relation",
            self.base_url, accession_id
        );
        let mut query = vec![];

        match lang {
            MetadataLanguage::English => query.push(("lang", "english".to_string())),
            MetadataLanguage::Arabic => query.push(("lang", "arabic".to_string())),
            MetadataLanguage::None => {}
        }

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context(format!(
                "Failed to send list relations request for accession {}",
                accession_id
            ))?;

        let response = Self::handle_response(
            response,
            &format!("Server returned error for list relations {}", accession_id),
        )
        .await?;

        response
            .json()
            .await
            .context("Failed to parse list relations response")
    }

    /// Gets a single relation by ID.
    pub async fn get_relation(
        &self,
        accession_id: i32,
        relation_id: i32,
        lang: MetadataLanguage,
    ) -> Result<RelationResponse> {
        let url = format!(
            "{}/api/v1/accessions/{}/relation/{}",
            self.base_url, accession_id, relation_id
        );
        let mut query = vec![];

        match lang {
            MetadataLanguage::English => query.push(("lang", "english".to_string())),
            MetadataLanguage::Arabic => query.push(("lang", "arabic".to_string())),
            MetadataLanguage::None => {}
        }

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context(format!(
                "Failed to send get relation request for accession {} relation {}",
                accession_id, relation_id
            ))?;

        let response = Self::handle_response(
            response,
            &format!(
                "Server returned error for get relation {} from accession {}",
                relation_id, accession_id
            ),
        )
        .await?;

        response
            .json()
            .await
            .context("Failed to parse get relation response")
    }

    /// Creates a new relation.
    pub async fn create_relation(
        &self,
        accession_id: i32,
        request: CreateRelationRequest,
    ) -> Result<String> {
        let url = format!(
            "{}/api/v1/accessions/{}/relation",
            self.base_url, accession_id
        );
        let response = self
            .client
            .post(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context(format!(
                "Failed to send create relation request for accession {}",
                accession_id
            ))?;

        let response =
            Self::handle_response(response, "Server returned error for create relation").await?;

        response
            .text()
            .await
            .context("Failed to parse create relation response text")
    }

    /// Deletes a relation.
    pub async fn delete_relation(
        &self,
        accession_id: i32,
        relation_id: i32,
        lang: MetadataLanguage,
    ) -> Result<()> {
        let url = format!(
            "{}/api/v1/accessions/{}/relation/{}",
            self.base_url, accession_id, relation_id
        );
        let query = vec![(
            "lang",
            match lang {
                MetadataLanguage::English => "english",
                MetadataLanguage::Arabic => "arabic",
                MetadataLanguage::None => "english",
            },
        )];

        let response = self
            .client
            .delete(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context(format!(
                "Failed to send delete relation request for accession {} relation {}",
                accession_id, relation_id
            ))?;

        Self::handle_response(
            response,
            &format!(
                "Server returned error for delete relation {} from accession {}",
                relation_id, accession_id
            ),
        )
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_subject_url_construction() {
        let client = SdaClient::new(
            "https://api.example.com".to_string(),
            "test-key".to_string(),
        );
        let id = 42;
        let expected_url = "https://api.example.com/api/v1/subjects/42";

        // Test URL construction by checking format string
        let constructed_url = format!("{}/api/v1/subjects/{}", client.base_url, id);
        assert_eq!(constructed_url, expected_url);
    }

    #[test]
    fn test_auth_header_format() {
        let client = SdaClient::new(
            "https://api.example.com".to_string(),
            "my-api-key".to_string(),
        );
        let (header_name, header_value) = client.auth_header();

        assert_eq!(header_name, "x-api-key");
        assert_eq!(header_value, "my-api-key");
    }

    #[test]
    fn test_build_accession_query_with_empty_args() {
        let client = SdaClient::new(
            "https://api.example.com".to_string(),
            "test-key".to_string(),
        );
        let args = ListAccessionsArgs::default();

        let result = client.build_accession_query(args).unwrap();
        assert!(
            result.is_empty(),
            "Empty args should produce empty query vector"
        );
    }

    #[test]
    fn test_build_accession_query_with_pagination() {
        let client = SdaClient::new(
            "https://api.example.com".to_string(),
            "test-key".to_string(),
        );
        let mut args = ListAccessionsArgs::default();
        args.page = 2;
        args.per_page = 25;

        let result = client.build_accession_query(args).unwrap();
        assert_eq!(result.len(), 2);

        // Check that pagination parameters are included
        let page_param = result.iter().find(|(key, _)| *key == "page");
        let per_page_param = result.iter().find(|(key, _)| *key == "per_page");

        assert!(page_param.is_some());
        assert_eq!(page_param.unwrap().1, "2");
        assert!(per_page_param.is_some());
        assert_eq!(per_page_param.unwrap().1, "25");
    }

    #[test]
    fn test_build_accession_query_with_language_filter() {
        let client = SdaClient::new(
            "https://api.example.com".to_string(),
            "test-key".to_string(),
        );
        let mut args = ListAccessionsArgs::default();
        args.lang = MetadataLanguage::Arabic;

        let result = client.build_accession_query(args).unwrap();
        assert_eq!(result.len(), 1);

        let lang_param = result.iter().find(|(key, _)| *key == "lang");
        assert!(lang_param.is_some());
        assert_eq!(lang_param.unwrap().1, "arabic");
    }

    #[test]
    fn test_build_accession_query_ignores_default_pagination() {
        let client = SdaClient::new(
            "https://api.example.com".to_string(),
            "test-key".to_string(),
        );
        let mut args = ListAccessionsArgs::default();
        args.page = -1; // Default value
        args.per_page = -1; // Default value
        args.lang = MetadataLanguage::English;

        let result = client.build_accession_query(args).unwrap();
        assert_eq!(result.len(), 1); // Only language should be included

        let page_param = result.iter().find(|(key, _)| *key == "page");
        let per_page_param = result.iter().find(|(key, _)| *key == "per_page");

        assert!(page_param.is_none());
        assert!(per_page_param.is_none());
    }

    #[test]
    fn test_collection_url_construction() {
        let client = SdaClient::new(
            "https://api.example.com".to_string(),
            "test-key".to_string(),
        );
        let id = 123;
        let expected_url = "https://api.example.com/api/v1/collections/123";

        // Test URL construction by checking format string
        let constructed_url = format!("{}/api/v1/collections/{}", client.base_url, id);
        assert_eq!(constructed_url, expected_url);
    }

    #[test]
    fn test_list_collections_args_default() {
        let args = ListCollectionsArgs::default();

        assert_eq!(args.page, -1);
        assert_eq!(args.per_page, -1);
        assert_eq!(args.lang, MetadataLanguage::None);
    }

    #[test]
    fn test_list_private_collections_args_default() {
        let args = ListPrivateCollectionsArgs::default();

        assert_eq!(args.page, -1);
        assert_eq!(args.per_page, -1);
        assert_eq!(args.lang, MetadataLanguage::None);
        assert_eq!(args.is_private, true);
    }

    #[test]
    fn test_create_collection_request_serialization() {
        let request = CreateCollectionRequest {
            lang: MetadataLanguage::English,
            title: "Test Collection".to_string(),
            is_private: true,
            subject_ids: vec![1, 2, 3],
            description: "A test description".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("Test Collection"));
        assert!(json.contains("A test description"));
        assert!(json.contains("true"));
    }
}
