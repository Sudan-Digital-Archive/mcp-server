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
            .context("Failed to send create accession crawl request")?
            .error_for_status()
            .context("Server returned error for create accession crawl")?;
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
            .context("Failed to send list accessions request")?
            .error_for_status()
            .context("Server returned error for list accessions")?;

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
            .context("Failed to send list private accessions request")?
            .error_for_status()
            .context("Server returned error for list private accessions")?;
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
            .context("Failed to send get accession request")?
            .error_for_status()
            .context("Server returned error for get accession")?;
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
            .context("Failed to send get private accession request")?
            .error_for_status()
            .context("Server returned error for get private accession")?;
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
            .context("Failed to send update accession request")?
            .error_for_status()
            .context("Server returned error for update accession")?;

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
    ) -> Result<ListSubjectsResponse> {
        let url = format!("{}/api/v1/metadata-subjects", self.base_url);
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

        let response = self
            .client
            .get(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .query(&query)
            .send()
            .await
            .context("Failed to send list subjects request")?
            .error_for_status()
            .context("Server returned error for list subjects")?;
        response
            .json()
            .await
            .context("Failed to parse list subjects response")
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
            .await
            .context("Failed to send create subject request")?
            .error_for_status()
            .context("Server returned error for create subject")?;
        response
            .text()
            .await
            .context("Failed to parse create subject response text")
    }

    /// Deletes a metadata subject by its ID.
    pub async fn delete_subject(&self, id: i32, request: DeleteSubjectRequest) -> Result<()> {
        let url = format!("{}/api/v1/metadata-subjects/{}", self.base_url, id);
        self.client
            .delete(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context("Failed to send delete subject request")?
            .error_for_status()
            .context("Server returned error for delete subject")?;
        Ok(())
    }

    /// Updates a metadata subject by its ID.
    pub async fn update_subject(
        &self,
        id: i32,
        request: UpdateSubjectRequest,
    ) -> Result<DublinMetadataSubjectResponse> {
        let url = format!("{}/api/v1/metadata-subjects/{}", self.base_url, id);
        let response = self
            .client
            .put(&url)
            .header(self.auth_header().0, self.auth_header().1)
            .json(&request)
            .send()
            .await
            .context("Failed to send update subject request")?
            .error_for_status()
            .context("Server returned error for update subject")?;

        response
            .json()
            .await
            .context("Failed to parse update subject response")
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
        let expected_url = "https://api.example.com/api/v1/metadata-subjects/42";

        // Test URL construction by checking format string
        let constructed_url = format!("{}/api/v1/metadata-subjects/{}", client.base_url, id);
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
}
