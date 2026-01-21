use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MetadataLanguage {
    English,
    Arabic,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub enum DublinMetadataFormat {
    #[serde(rename = "wacz")]
    Wacz,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ListAccessionsArgs {
    pub page: Option<i64>,
    #[serde(alias = "per_page")]
    pub per_page: Option<i64>,
    pub lang: Option<MetadataLanguage>,
    pub metadata_subjects: Option<Vec<i32>>,
    pub metadata_subjects_inclusive_filter: Option<bool>,
    pub query_term: Option<String>,
    pub url_filter: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateAccessionRequest {
    pub is_private: bool,
    pub metadata_description: Option<String>,
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
