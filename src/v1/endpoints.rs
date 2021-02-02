pub use reqwest::{ Url };
use super::models::{ TranscriptVersion };

pub static JOBS: &'static str = "https://api.temi.com/v1/jobs";
pub static ACCOUNT: &'static str = "https://api.temi.com/v1/account";

pub fn job_url(job_id: &str) -> Result<Url, url::ParseError> {
    Url::parse(&format!("{}/{}", JOBS, job_id))
}

pub fn list_jobs_url(limit: Option<u32>, starting_after: Option<&str>) -> Result<Url, url::ParseError> {
    if let (None, None) = (limit, starting_after) {
        return Url::parse(JOBS);
    }

    let mut params = Vec::new();

    if let Some(limit_value) = limit {
        params.push(("limit", limit_value.to_string()));
    }

    if let Some(starting_after_value) = starting_after {
        params.push(("starting_after", starting_after_value.to_owned()));
    }

    Url::parse_with_params(JOBS, params)
}

pub fn get_transcript_url(job_id: &str, version: Option<TranscriptVersion>) -> Result<Url, url::ParseError> {
    let base_url = &format!("{}/{}/transcript", JOBS, job_id);

    if let None = version {
        return Url::parse(base_url);
    }

    let params = vec![("version", version.unwrap().to_query_str().to_owned())];

    Url::parse_with_params(base_url, params)
}