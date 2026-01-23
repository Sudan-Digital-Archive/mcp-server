//! API Client for the Sudan Digital Archive.
//!
//! This module provides a client for making HTTP requests to the SDA API.

use crate::model::*;
use anyhow::Result;
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
        if args.is_private {
            query.push(("is_private", "true".to_string()));
        }
        Ok(query)
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
            .await?
            .error_for_status()?;

        Ok(response.json().await?)
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
            .await?
            .error_for_status()?;
        Ok(response.json().await?)
    }

    /// Retrieves a single public accession by its ID.
    pub async fn get_accession(&self, id: i32) -> Result<GetOneAccessionResponse> {
        let url = format!("{}/api/v1/accessions/{}", self.base_url, id);
        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .send()
            .await?
            .error_for_status()?;
        Ok(response.json().await?)
    }

    /// Retrieves a single private accession by its ID.
    pub async fn get_private_accession(&self, id: i32) -> Result<GetOneAccessionResponse> {
        let url = format!("{}/api/v1/accessions/private/{}", self.base_url, id);
        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .send()
            .await?
            .error_for_status()?;
        Ok(response.json().await?)
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
            .await?
            .error_for_status()?;

        Ok(response.json().await?)
    }

    /// Lists metadata subjects with optional pagination.
    pub async fn list_subjects(
        &self,
        page: Option<i64>,
        per_page: Option<i64>,
    ) -> Result<ListSubjectsResponse> {
        let url = format!("{}/api/v1/metadata-subjects", self.base_url);
        let mut query = vec![];
        if let Some(p) = page {
            query.push(("page", p.to_string()));
        }
        if let Some(pp) = per_page {
            query.push(("per_page", pp.to_string()));
        }

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await?
            .error_for_status()?;
        Ok(response.json().await?)
    }

    /// Creates a new metadata subject.
    pub async fn create_subject(&self, request: CreateSubjectRequest) -> Result<String> {
        let url = format!("{}/api/v1/metadata-subjects", self.base_url);
        let response = self
            .client
            .post(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await?
            .error_for_status()?;
        Ok(response.text().await?)
    }

    /// Deletes a metadata subject by its ID.
    pub async fn delete_subject(&self, id: i32, request: DeleteSubjectRequest) -> Result<()> {
        let url = format!("{}/api/v1/metadata-subjects/{}", self.base_url, id);
        self.client
            .delete(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}
