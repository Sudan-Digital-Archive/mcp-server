use crate::model::*;
use anyhow::Result;
use reqwest::Client;

#[derive(Clone)]
pub struct SdaClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl SdaClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            api_key,
        }
    }

    fn auth_header(&self) -> (&str, &str) {
        ("x-api-key", &self.api_key)
    }

    fn build_accession_query(
        &self,
        args: ListAccessionsArgs,
    ) -> Result<Vec<(&'static str, String)>> {
        let mut query = vec![];
        if let Some(p) = args.page {
            query.push(("page", p.to_string()));
        }
        if let Some(pp) = args.per_page {
            query.push(("per_page", pp.to_string()));
        }
        if let Some(l) = args.lang {
            let lang_str = match l {
                MetadataLanguage::English => "english",
                MetadataLanguage::Arabic => "arabic",
            };
            query.push(("lang", lang_str.to_string()));
        }
        if let Some(ms) = args.metadata_subjects {
            for s in ms {
                query.push(("metadata_subjects", s.to_string()));
            }
        }
        if let Some(msif) = args.metadata_subjects_inclusive_filter {
            query.push(("metadata_subjects_inclusive_filter", msif.to_string()));
        }
        if let Some(qt) = args.query_term {
            query.push(("query_term", qt));
        }
        if let Some(uf) = args.url_filter {
            query.push(("url_filter", uf));
        }
        if let Some(df) = args.date_from {
            query.push(("date_from", df));
        }
        if let Some(dt) = args.date_to {
            query.push(("date_to", dt));
        }
        Ok(query)
    }

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
