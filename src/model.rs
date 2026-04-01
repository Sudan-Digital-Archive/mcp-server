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

/// Default value for collection ID filter.
fn default_collection_id() -> i32 {
    -1
}

/// Default value for ID fields (unset).
fn default_id() -> i64 {
    -1
}

/// Arguments for creating a new accession (crawl).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateAccessionCrawlArgs {
    /// The URL to crawl.
    pub url: String,
    /// Language of the metadata.
    pub metadata_language: MetadataLanguage,
    /// Title of the accession.
    pub metadata_title: String,
    /// Time period related to the accession (ISO 8601, e.g. "2026-02-01T00:00:00" - do NOT include the "Z" suffix).
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
    /// List of contributor IDs (Arabic).
    #[serde(default)]
    pub metadata_contributor_ar_ids: Vec<i32>,
    /// List of contributor IDs (English).
    #[serde(default)]
    pub metadata_contributor_en_ids: Vec<i32>,
    /// List of contributor role IDs (Arabic) - must be 1:1 with contributors.
    #[serde(default)]
    pub metadata_contributor_role_ar_ids: Vec<Option<i32>>,
    /// List of contributor role IDs (English) - must be 1:1 with contributors.
    #[serde(default)]
    pub metadata_contributor_role_en_ids: Vec<Option<i32>>,
    /// Creator ID (Arabic).
    #[serde(default = "default_id")]
    pub metadata_creator_ar_id: i64,
    /// Creator ID (English).
    #[serde(default = "default_id")]
    pub metadata_creator_en_id: i64,
    /// Location ID (Arabic).
    #[serde(default = "default_id")]
    pub metadata_location_ar_id: i64,
    /// Location ID (English).
    #[serde(default = "default_id")]
    pub metadata_location_en_id: i64,
    /// Whether to send email notification after crawl completes.
    #[serde(default)]
    pub send_email_notification: bool,
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

impl Default for ListAccessionsArgs {
    fn default() -> Self {
        Self {
            page: -1,
            per_page: -1,
            lang: MetadataLanguage::default(),
            metadata_subjects: Vec::new(),
            metadata_subjects_inclusive_filter: false,
            query_term: String::new(),
            url_filter: String::new(),
            date_from: String::new(),
            date_to: String::new(),
            is_private: false,
        }
    }
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
    /// Language for subjects - REQUIRED field.
    pub lang: MetadataLanguage,
    /// Filter subjects by collection ID.
    #[serde(default = "default_collection_id")]
    pub in_collection_id: i32,
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
    /// List of contributor IDs (Arabic).
    #[serde(default)]
    pub metadata_contributor_ar_ids: Vec<i32>,
    /// List of contributor IDs (English).
    #[serde(default)]
    pub metadata_contributor_en_ids: Vec<i32>,
    /// List of contributor role IDs (Arabic) - must be 1:1 with contributors.
    #[serde(default)]
    pub metadata_contributor_role_ar_ids: Vec<Option<i32>>,
    /// List of contributor role IDs (English) - must be 1:1 with contributors.
    #[serde(default)]
    pub metadata_contributor_role_en_ids: Vec<Option<i32>>,
    /// Creator ID (Arabic).
    #[serde(default = "default_id")]
    pub metadata_creator_ar_id: i64,
    /// Creator ID (English).
    #[serde(default = "default_id")]
    pub metadata_creator_en_id: i64,
    /// Location ID (Arabic).
    #[serde(default = "default_id")]
    pub metadata_location_ar_id: i64,
    /// Location ID (English).
    #[serde(default = "default_id")]
    pub metadata_location_en_id: i64,
}

/// Arguments for creating a metadata subject.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateSubjectArgs {
    /// Language of the subject.
    pub lang: MetadataLanguage,
    /// The subject name/term.
    pub metadata_subject: String,
}

/// Arguments for deleting a metadata subject.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteSubjectArgs {
    /// The ID of the subject to delete.
    pub id: i32,
    /// The language of the subject.
    pub lang: MetadataLanguage,
}

/// Arguments for updating a metadata subject.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateSubjectArgs {
    /// The ID of the subject to update.
    pub id: i32,
    /// Language of the subject.
    pub lang: MetadataLanguage,
    /// The subject name/term.
    pub metadata_subject: String,
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
    /// List of contributor IDs (Arabic).
    #[serde(default)]
    pub metadata_contributor_ar_ids: Vec<i32>,
    /// List of contributor IDs (English).
    #[serde(default)]
    pub metadata_contributor_en_ids: Vec<i32>,
    /// List of contributor role IDs (Arabic) - must be 1:1 with contributors.
    #[serde(default)]
    pub metadata_contributor_role_ar_ids: Vec<Option<i32>>,
    /// List of contributor role IDs (English) - must be 1:1 with contributors.
    #[serde(default)]
    pub metadata_contributor_role_en_ids: Vec<Option<i32>>,
    /// Creator ID (Arabic).
    pub metadata_creator_ar_id: Option<i64>,
    /// Creator ID (English).
    pub metadata_creator_en_id: Option<i64>,
    /// Location ID (Arabic).
    pub metadata_location_ar_id: Option<i64>,
    /// Location ID (English).
    pub metadata_location_en_id: Option<i64>,
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

/// Request body for updating a metadata subject.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateSubjectRequest {
    /// Language of the subject.
    pub lang: MetadataLanguage,
    /// The subject name/term.
    pub metadata_subject: String,
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
    /// List of contributor IDs (Arabic).
    #[serde(default)]
    pub metadata_contributor_ar_ids: Vec<i32>,
    /// List of contributor IDs (English).
    #[serde(default)]
    pub metadata_contributor_en_ids: Vec<i32>,
    /// List of contributor role IDs (Arabic) - must be 1:1 with contributors.
    #[serde(default)]
    pub metadata_contributor_role_ar_ids: Vec<Option<i32>>,
    /// List of contributor role IDs (English) - must be 1:1 with contributors.
    #[serde(default)]
    pub metadata_contributor_role_en_ids: Vec<Option<i32>>,
    /// Creator ID (Arabic).
    pub metadata_creator_ar_id: Option<i64>,
    /// Creator ID (English).
    pub metadata_creator_en_id: Option<i64>,
    /// Location ID (Arabic).
    pub metadata_location_ar_id: Option<i64>,
    /// Location ID (English).
    pub metadata_location_en_id: Option<i64>,
    /// Whether to send email notification after crawl completes.
    #[serde(default)]
    pub send_email_notification: bool,
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
    /// Unique crawl identifier (UUID).
    pub crawl_id: Option<String>,
    /// Organization identifier (UUID).
    pub org_id: Option<String>,
    /// Job run identifier.
    pub job_run_id: Option<String>,
    /// The URL that was crawled.
    pub seed_url: String,
    /// Date associated with the Dublin Core metadata.
    pub dublin_metadata_date: String,
    /// Format of the metadata.
    pub dublin_metadata_format: DublinMetadataFormat,
    /// English title.
    pub title_en: Option<String>,
    /// English description.
    pub description_en: Option<String>,
    /// English location.
    pub location_en: Option<String>,
    /// Creator ID (English).
    pub creator_en_id: Option<i32>,
    /// Creator name (English).
    pub creator_en: Option<String>,
    /// List of subjects in English.
    pub subjects_en: Option<Vec<String>>,
    /// List of English subject IDs.
    pub subjects_en_ids: Option<Vec<i32>>,
    /// List of contributors (English).
    pub contributors_en: Option<Vec<String>>,
    /// List of contributor roles (English).
    pub contributor_roles_en: Option<Vec<String>>,
    /// Relations (English).
    pub relations_en: Option<serde_json::Value>,
    /// Arabic title.
    pub title_ar: Option<String>,
    /// Arabic description.
    pub description_ar: Option<String>,
    /// Arabic location.
    pub location_ar: Option<String>,
    /// Creator ID (Arabic).
    pub creator_ar_id: Option<i32>,
    /// Creator name (Arabic).
    pub creator_ar: Option<String>,
    /// List of subjects in Arabic.
    pub subjects_ar: Option<Vec<String>>,
    /// List of Arabic subject IDs.
    pub subjects_ar_ids: Option<Vec<i32>>,
    /// List of contributors (Arabic).
    pub contributors_ar: Option<Vec<String>>,
    /// List of contributor roles (Arabic).
    pub contributor_roles_ar: Option<Vec<String>>,
    /// Relations (Arabic).
    pub relations_ar: Option<serde_json::Value>,
    /// Whether English metadata exists.
    pub has_english_metadata: bool,
    /// Whether Arabic metadata exists.
    pub has_arabic_metadata: bool,
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

/// Arguments for listing collections.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListCollectionsArgs {
    /// Page number for pagination.
    #[serde(default = "default_pagination")]
    pub page: i64,
    /// Number of items per page.
    #[serde(default = "default_pagination", alias = "per_page")]
    pub per_page: i64,
    /// Language filter.
    #[serde(default)]
    pub lang: MetadataLanguage,
}

impl Default for ListCollectionsArgs {
    fn default() -> Self {
        Self {
            page: -1,
            per_page: -1,
            lang: MetadataLanguage::default(),
        }
    }
}

/// Arguments for listing private collections.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListPrivateCollectionsArgs {
    /// Page number for pagination.
    #[serde(default = "default_pagination")]
    pub page: i64,
    /// Number of items per page.
    #[serde(default = "default_pagination", alias = "per_page")]
    pub per_page: i64,
    /// Language filter.
    #[serde(default)]
    pub lang: MetadataLanguage,
    /// Filter by private status.
    #[serde(default)]
    pub is_private: bool,
}

impl Default for ListPrivateCollectionsArgs {
    fn default() -> Self {
        Self {
            page: -1,
            per_page: -1,
            lang: MetadataLanguage::default(),
            is_private: true,
        }
    }
}

/// Arguments for getting a single collection.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetCollectionArgs {
    /// The collection ID.
    pub id: i32,
    /// Language for the collection.
    #[serde(default)]
    pub lang: MetadataLanguage,
}

/// Arguments for getting a single subject.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetSubjectArgs {
    /// The subject ID.
    pub id: i32,
    /// Language for the subject.
    #[serde(default)]
    pub lang: MetadataLanguage,
}

/// Arguments for creating a collection.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateCollectionArgs {
    /// Language of the collection.
    pub lang: MetadataLanguage,
    /// Title of the collection.
    pub title: String,
    /// Whether the collection is private.
    pub is_private: bool,
    /// List of subject IDs.
    pub subject_ids: Vec<i32>,
    /// Description of the collection.
    #[serde(default)]
    pub description: String,
}

/// Arguments for updating a collection.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateCollectionArgs {
    /// The ID of the collection to update.
    pub id: i32,
    /// Language of the collection.
    pub lang: MetadataLanguage,
    /// Title of the collection.
    pub title: String,
    /// Whether the collection is private.
    pub is_private: bool,
    /// List of subject IDs.
    pub subject_ids: Vec<i32>,
    /// Description of the collection.
    #[serde(default)]
    pub description: String,
}

/// Request body for creating a collection.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateCollectionRequest {
    /// Language of the collection.
    pub lang: MetadataLanguage,
    /// Title of the collection.
    pub title: String,
    /// Whether the collection is private.
    pub is_private: bool,
    /// List of subject IDs.
    pub subject_ids: Vec<i32>,
    /// Description of the collection.
    #[serde(default)]
    pub description: String,
}

/// Request body for updating a collection.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateCollectionRequest {
    /// Language of the collection.
    pub lang: MetadataLanguage,
    /// Title of the collection.
    pub title: String,
    /// Whether the collection is private.
    pub is_private: bool,
    /// List of subject IDs.
    pub subject_ids: Vec<i32>,
    /// Description of the collection.
    #[serde(default)]
    pub description: String,
}

/// Response containing a single collection.
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct CollectionResponse {
    /// Unique identifier.
    pub id: i32,
    /// Title of the collection.
    pub title: String,
    /// Whether the collection is private.
    pub is_private: bool,
    /// Description of the collection.
    pub description: Option<String>,
}

/// Response containing a list of collections.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListCollectionsResponse {
    /// List of collections.
    pub items: Vec<CollectionResponse>,
    /// Total number of pages.
    pub num_pages: i64,
    /// Current page number.
    pub page: i64,
    /// Items per page.
    pub per_page: i64,
}

/// Relation types for accession relations.
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DublinMetadataRelationType {
    HasPart,
    IsPartOf,
    HasVersion,
    IsVersionOf,
    References,
    IsReferencedBy,
    ConformsTo,
    Requires,
}

/// Arguments for listing contributors.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListContributorsArgs {
    #[serde(default = "default_pagination")]
    pub page: i64,
    #[serde(default = "default_pagination", alias = "per_page")]
    pub per_page: i64,
    #[serde(default)]
    pub lang: MetadataLanguage,
    #[serde(default)]
    pub query_term: String,
}

/// Arguments for getting a single contributor.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetContributorArgs {
    pub id: i32,
    #[serde(default)]
    pub lang: MetadataLanguage,
}

/// Arguments for creating a contributor.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateContributorArgs {
    pub lang: MetadataLanguage,
    pub contributor: String,
}

/// Arguments for updating a contributor.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateContributorArgs {
    pub id: i32,
    pub lang: MetadataLanguage,
    pub contributor: String,
}

/// Arguments for deleting a contributor.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteContributorArgs {
    pub id: i32,
    pub lang: MetadataLanguage,
}

/// Request body for creating a contributor.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateContributorRequest {
    pub lang: MetadataLanguage,
    pub contributor: String,
}

/// Request body for updating a contributor.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateContributorRequest {
    pub lang: MetadataLanguage,
    pub contributor: String,
}

/// Request body for deleting a contributor.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteContributorRequest {
    pub lang: MetadataLanguage,
}

/// Response containing a single contributor.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ContributorResponse {
    pub id: i32,
    pub contributor: String,
}

/// Response containing a list of contributors.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListContributorsResponse {
    pub items: Vec<ContributorResponse>,
    pub num_pages: i64,
    pub page: i64,
    pub per_page: i64,
}

/// Arguments for listing contributor roles.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListContributorRolesArgs {
    #[serde(default = "default_pagination")]
    pub page: i64,
    #[serde(default = "default_pagination", alias = "per_page")]
    pub per_page: i64,
    #[serde(default)]
    pub lang: MetadataLanguage,
    #[serde(default)]
    pub query_term: String,
}

/// Arguments for getting a single contributor role.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetContributorRoleArgs {
    pub id: i32,
    #[serde(default)]
    pub lang: MetadataLanguage,
}

/// Arguments for creating a contributor role.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateContributorRoleArgs {
    pub lang: MetadataLanguage,
    pub role: String,
}

/// Arguments for updating a contributor role.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateContributorRoleArgs {
    pub id: i32,
    pub lang: MetadataLanguage,
    pub role: String,
}

/// Arguments for deleting a contributor role.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteContributorRoleArgs {
    pub id: i32,
    pub lang: MetadataLanguage,
}

/// Request body for creating a contributor role.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateContributorRoleRequest {
    pub lang: MetadataLanguage,
    pub role: String,
}

/// Request body for updating a contributor role.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateContributorRoleRequest {
    pub lang: MetadataLanguage,
    pub role: String,
}

/// Request body for deleting a contributor role.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteContributorRoleRequest {
    pub lang: MetadataLanguage,
}

/// Response containing a single contributor role.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ContributorRoleResponse {
    pub id: i32,
    pub role: String,
}

/// Response containing a list of contributor roles.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListContributorRolesResponse {
    pub items: Vec<ContributorRoleResponse>,
    pub num_pages: i64,
    pub page: i64,
    pub per_page: i64,
}

/// Arguments for listing creators.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListCreatorsArgs {
    #[serde(default = "default_pagination")]
    pub page: i64,
    #[serde(default = "default_pagination", alias = "per_page")]
    pub per_page: i64,
    #[serde(default)]
    pub lang: MetadataLanguage,
    #[serde(default)]
    pub query_term: String,
}

/// Arguments for getting a single creator.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetCreatorArgs {
    pub id: i32,
    #[serde(default)]
    pub lang: MetadataLanguage,
}

/// Arguments for creating a creator.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateCreatorArgs {
    pub lang: MetadataLanguage,
    pub creator: String,
}

/// Arguments for updating a creator.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateCreatorArgs {
    pub id: i32,
    pub lang: MetadataLanguage,
    pub creator: String,
}

/// Arguments for deleting a creator.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteCreatorArgs {
    pub id: i32,
    pub lang: MetadataLanguage,
}

/// Request body for creating a creator.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateCreatorRequest {
    pub lang: MetadataLanguage,
    pub creator: String,
}

/// Request body for updating a creator.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateCreatorRequest {
    pub lang: MetadataLanguage,
    pub creator: String,
}

/// Request body for deleting a creator.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteCreatorRequest {
    pub lang: MetadataLanguage,
}

/// Response containing a single creator.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreatorResponse {
    pub id: i32,
    pub creator: String,
}

/// Response containing a list of creators.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListCreatorsResponse {
    pub items: Vec<CreatorResponse>,
    pub num_pages: i64,
    pub page: i64,
    pub per_page: i64,
}

/// Arguments for listing locations.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListLocationsArgs {
    #[serde(default = "default_pagination")]
    pub page: i64,
    #[serde(default = "default_pagination", alias = "per_page")]
    pub per_page: i64,
    #[serde(default)]
    pub lang: MetadataLanguage,
    #[serde(default)]
    pub query_term: String,
}

/// Arguments for getting a single location.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetLocationArgs {
    pub id: i32,
    #[serde(default)]
    pub lang: MetadataLanguage,
}

/// Arguments for creating a location.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateLocationArgs {
    pub lang: MetadataLanguage,
    pub location: String,
}

/// Arguments for updating a location.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateLocationArgs {
    pub id: i32,
    pub lang: MetadataLanguage,
    pub location: String,
}

/// Arguments for deleting a location.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteLocationArgs {
    pub id: i32,
    pub lang: MetadataLanguage,
}

/// Request body for creating a location.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateLocationRequest {
    pub lang: MetadataLanguage,
    pub location: String,
}

/// Request body for updating a location.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateLocationRequest {
    pub lang: MetadataLanguage,
    pub location: String,
}

/// Request body for deleting a location.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteLocationRequest {
    pub lang: MetadataLanguage,
}

/// Response containing a single location.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LocationResponse {
    pub id: i32,
    pub location: String,
}

/// Response containing a list of locations.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListLocationsResponse {
    pub items: Vec<LocationResponse>,
    pub num_pages: i64,
    pub page: i64,
    pub per_page: i64,
}

/// Arguments for listing relations.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListRelationsArgs {
    pub accession_id: i32,
    #[serde(default)]
    pub lang: MetadataLanguage,
}

/// Arguments for getting a single relation.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetRelationArgs {
    pub accession_id: i32,
    pub relation_id: i32,
    #[serde(default)]
    pub lang: MetadataLanguage,
}

/// Arguments for creating a relation.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateRelationArgs {
    pub accession_id: i32,
    pub related_accession_id: i32,
    pub relation_type: DublinMetadataRelationType,
    #[serde(default)]
    pub lang: MetadataLanguage,
}

/// Arguments for deleting a relation.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteRelationArgs {
    pub accession_id: i32,
    pub relation_id: i32,
    #[serde(default)]
    pub lang: MetadataLanguage,
}

/// Request body for creating a relation.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateRelationRequest {
    pub related_accession_id: i32,
    pub relation_type: DublinMetadataRelationType,
}

/// Response containing a single relation.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RelationResponse {
    pub id: i32,
    pub related_accession_id: i32,
    pub relation_type: String,
}

/// Response containing a list of relations.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListRelationsResponse {
    pub items: Vec<RelationResponse>,
}
