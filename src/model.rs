use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MetadataLanguage {
    #[default]
    None,
    English,
    Arabic,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub enum DublinMetadataFormat {
    #[serde(rename = "wacz")]
    Wacz,
}

fn default_pagination() -> i64 {
    -1
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListAccessionsArgs {
    #[serde(default = "default_pagination")]
    pub page: i64,
    #[serde(default = "default_pagination", alias = "per_page")]
    pub per_page: i64,
    #[serde(default)]
    pub lang: MetadataLanguage,
    #[serde(default)]
    pub metadata_subjects: Vec<i32>,
    #[serde(default)]
    pub metadata_subjects_inclusive_filter: bool,
    #[serde(default)]
    pub query_term: String,
    #[serde(default)]
    pub url_filter: String,
    #[serde(default)]
    pub date_from: String,
    #[serde(default)]
    pub date_to: String,
    #[serde(default)]
    pub is_private: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListSubjectsArgs {
    #[serde(default = "default_pagination")]
    pub page: i64,
    #[serde(default = "default_pagination", alias = "per_page")]
    pub per_page: i64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IdArgs {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateAccessionArgs {
    pub id: i32,
    pub request: UpdateAccessionRequest,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateSubjectArgs {
    pub request: CreateSubjectRequest,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteSubjectArgs {
    pub id: i32,
    pub lang: MetadataLanguage,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateAccessionRequest {
    pub is_private: bool,
    #[serde(default)]
    pub metadata_description: String,
    pub metadata_language: MetadataLanguage,
    pub metadata_subjects: Vec<i32>,
    pub metadata_time: String,
    pub metadata_title: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateSubjectRequest {
    pub lang: MetadataLanguage,
    pub metadata_subject: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteSubjectRequest {
    pub lang: MetadataLanguage,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub enum CrawlStatus {
    BadCrawl,
    Complete,
    Error,
    Pending,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct AccessionsWithMetadataResponse {
    pub id: i32,
    pub is_private: bool,
    pub crawl_status: CrawlStatus,
    pub crawl_timestamp: String,
    pub seed_url: String,
    pub dublin_metadata_date: String,
    pub dublin_metadata_format: DublinMetadataFormat,
    pub has_english_metadata: bool,
    pub has_arabic_metadata: bool,
    pub description_ar: Option<String>,
    pub description_en: Option<String>,
    pub subjects_ar: Option<Vec<String>>,
    pub subjects_ar_ids: Option<Vec<i32>>,
    pub subjects_en: Option<Vec<String>>,
    pub subjects_en_ids: Option<Vec<i32>>,
    pub title_ar: Option<String>,
    pub title_en: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListAccessionsResponse {
    pub items: Vec<AccessionsWithMetadataResponse>,
    pub num_pages: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetOneAccessionResponse {
    pub accession: AccessionsWithMetadataResponse,
    pub wacz_url: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DublinMetadataSubjectResponse {
    pub id: i32,
    pub subject: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListSubjectsResponse {
    pub items: Vec<DublinMetadataSubjectResponse>,
    pub num_pages: i64,
    pub page: i64,
    pub per_page: i64,
}
