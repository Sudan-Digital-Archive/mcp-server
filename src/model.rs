//! Data models for the Sudan Digital Archive.
//!
//! This module defines the structs and enums used for communication
//! between the MCP server and the SDA API.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Supported languages for metadata.
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MetadataLanguage {
    /// No language specified.
    #[default]
    None,
    /// English language.
    English,
    /// Arabic language.
    Arabic,
}

/// Supported metadata formats.
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub enum DublinMetadataFormat {
    /// Web ARChive (Zipped) format.
    #[serde(rename = "wacz")]
    Wacz,
}

/// Supported browser profiles for hard to archive sites.
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BrowserProfile {
    /// Profile for crawling Facebook.
    Facebook,
}

/// Default value for pagination fields.
fn default_pagination() -> i64 {
    -1
}

/// Arguments for creating a new accession (crawl).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateAccessionCrawlArgs {
    /// The parameters for creating the accession crawl.
    pub request: CreateAccessionCrawlRequest,
}

/// Arguments for listing accessions.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListAccessionsArgs {
    /// Page number for pagination.
    #[serde(default = "default_pagination")]
    pub page: i64,
    /// Number of items per page.
    #[serde(default = "default_pagination", alias = "per_page")]
    pub per_page: i64,
    /// Language filter for metadata.
    #[serde(default)]
    pub lang: MetadataLanguage,
    /// Filter by specific metadata subject IDs.
    #[serde(default)]
    pub metadata_subjects: Vec<i32>,
    /// Whether the subject filter should be inclusive.
    #[serde(default)]
    pub metadata_subjects_inclusive_filter: bool,
    /// General query term to search for.
    #[serde(default)]
    pub query_term: String,
    /// Filter by URL.
    #[serde(default)]
    pub url_filter: String,
    /// Start date filter.
    #[serde(default)]
    pub date_from: String,
    /// End date filter.
    #[serde(default)]
    pub date_to: String,
    /// Whether to include private accessions.
    #[serde(default)]
    pub is_private: bool,
}

/// Arguments for listing metadata subjects.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListSubjectsArgs {
    /// Page number for pagination.
    #[serde(default = "default_pagination")]
    pub page: i64,
    /// Number of items per page.
    #[serde(default = "default_pagination", alias = "per_page")]
    pub per_page: i64,
}

/// Simple arguments containing only an ID.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IdArgs {
    /// The unique identifier.
    pub id: i32,
}

/// Arguments for updating an accession.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateAccessionArgs {
    /// The ID of the accession to update.
    pub id: i32,
    /// The updated data.
    pub request: UpdateAccessionRequest,
}

/// Arguments for creating a metadata subject.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateSubjectArgs {
    /// The subject data to create.
    pub request: CreateSubjectRequest,
}

/// Arguments for deleting a metadata subject.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteSubjectArgs {
    /// The ID of the subject to delete.
    pub id: i32,
    /// The language of the subject.
    pub lang: MetadataLanguage,
}

/// Request body for updating an accession.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateAccessionRequest {
    /// Privacy status.
    pub is_private: bool,
    /// Description of the accession.
    #[serde(default)]
    pub metadata_description: String,
    /// Language of the metadata.
    pub metadata_language: MetadataLanguage,
    /// List of subject IDs.
    pub metadata_subjects: Vec<i32>,
    /// Time period related to the accession.
    pub metadata_time: String,
    /// Title of the accession.
    pub metadata_title: String,
}

/// Request body for creating a metadata subject.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateSubjectRequest {
    /// Language of the subject.
    pub lang: MetadataLanguage,
    /// The subject name/term.
    pub metadata_subject: String,
}

/// Request body for deleting a metadata subject.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteSubjectRequest {
    /// Language of the subject.
    pub lang: MetadataLanguage,
}

/// Request body for creating a new accession crawl.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateAccessionCrawlRequest {
    /// The URL to crawl.
    pub url: String,
    /// Language of the metadata.
    pub metadata_language: MetadataLanguage,
    /// Title of the accession.
    pub metadata_title: String,
    /// Time period related to the accession (ISO 8601).
    pub metadata_time: String,
    /// List of subject IDs.
    pub metadata_subjects: Vec<i32>,
    /// Whether the accession is private.
    pub is_private: bool,
    /// Format of the metadata.
    pub metadata_format: DublinMetadataFormat,
    /// Optional browser profile for specific sites.
    #[serde(default)]
    pub browser_profile: Option<BrowserProfile>,
    /// Description of the accession.
    #[serde(default)]
    pub metadata_description: Option<String>,
    /// Optional S3 filename.
    #[serde(default)]
    pub s3_filename: Option<String>,
}

/// Status of a web crawl.
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub enum CrawlStatus {
    /// The crawl was unsuccessful or poor quality.
    BadCrawl,
    /// The crawl finished successfully.
    Complete,
    /// An error occurred during the crawl.
    Error,
    /// The crawl is waiting to start or in progress.
    Pending,
}

/// Detailed accession information including metadata.
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct AccessionsWithMetadataResponse {
    /// Unique identifier.
    pub id: i32,
    /// Whether the accession is private.
    pub is_private: bool,
    /// Status of the crawl.
    pub crawl_status: CrawlStatus,
    /// Timestamp when the crawl occurred.
    pub crawl_timestamp: String,
    /// The URL that was crawled.
    pub seed_url: String,
    /// Date associated with the Dublin Core metadata.
    pub dublin_metadata_date: String,
    /// Format of the metadata.
    pub dublin_metadata_format: DublinMetadataFormat,
    /// Whether English metadata exists.
    pub has_english_metadata: bool,
    /// Whether Arabic metadata exists.
    pub has_arabic_metadata: bool,
    /// Arabic description.
    pub description_ar: Option<String>,
    /// English description.
    pub description_en: Option<String>,
    /// List of subjects in Arabic.
    pub subjects_ar: Option<Vec<String>>,
    /// List of Arabic subject IDs.
    pub subjects_ar_ids: Option<Vec<i32>>,
    /// List of subjects in English.
    pub subjects_en: Option<Vec<String>>,
    /// List of English subject IDs.
    pub subjects_en_ids: Option<Vec<i32>>,
    /// Arabic title.
    pub title_ar: Option<String>,
    /// English title.
    pub title_en: Option<String>,
}

/// Response containing a list of accessions.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListAccessionsResponse {
    /// List of accessions.
    pub items: Vec<AccessionsWithMetadataResponse>,
    /// Total number of pages.
    pub num_pages: i64,
    /// Current page number.
    pub page: i64,
    /// Items per page.
    pub per_page: i64,
}

/// Response containing a single accession and its download URL.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetOneAccessionResponse {
    /// The accession details.
    pub accession: AccessionsWithMetadataResponse,
    /// URL to download the WACZ file.
    pub wacz_url: String,
}

/// Represents a single metadata subject.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DublinMetadataSubjectResponse {
    /// Unique identifier.
    pub id: i32,
    /// The subject name.
    pub subject: String,
}

/// Response containing a list of metadata subjects.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListSubjectsResponse {
    /// List of subjects.
    pub items: Vec<DublinMetadataSubjectResponse>,
    /// Total number of pages.
    pub num_pages: i64,
    /// Current page number.
    pub page: i64,
    /// Items per page.
    pub per_page: i64,
}
