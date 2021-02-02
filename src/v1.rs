mod endpoints;
mod models;
mod results;

use core::panic;

use models::{SubmitJobOptions };
use reqwest::{ StatusCode };
use reqwest::header::{ self, HeaderValue, HeaderMap };
pub use models::{ Problem, TranscriptFormat, TranscriptVersion };
pub use results::{GetAccountDetailsResult, GetAccountDetailsErrorKind, GetJobErrorKind, GetJobResult, ListJobsResult, DeleteJobResult, DeleteJobErrorKind };

pub struct Client {
    http: reqwest::blocking::Client
}

impl Client {
    pub fn new(api_key: &str) -> Client {
        let auth_header = HeaderValue::from_str(&(String::from("Bearer ") + api_key))
            .expect("Unable to create header value from API key.");

        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, auth_header);

        Client {
            http: reqwest::blocking::Client::builder()
                .default_headers(headers)
                .build()
                .expect("Unable to create HTTP client.")
        }
    }

    pub fn submit_job(&self, media_url: &str, callback_url: &str, metadata: &str) {
        let options = SubmitJobOptions {
            media_url: media_url.to_owned(),
            callback_url: Some(callback_url.to_owned()),
            metadata: Some(metadata.to_owned())
        };
        let response = self.http
            .post(endpoints::JOBS)
            .json(&options)
            .send()
            .expect("Failed to submit Temi job.");
    }

    pub fn delete_job(&self, job_id: &str) -> DeleteJobResult {
        let endpoint = endpoints::job_url(job_id)
            .expect("Failed to create a job URL");
        let response = self.http
            .delete(endpoint)
            .send()
            .expect("Failed to get Temi job.");

        match response.status() {
            StatusCode::NO_CONTENT => Ok(()),
            StatusCode::BAD_REQUEST => Err(DeleteJobErrorKind::InvalidParameters(response.json().expect("Unable to deserialize JSON."))),
            StatusCode::NOT_FOUND => Err(DeleteJobErrorKind::JobNotFound(response.json().expect("Unable to deserialize JSON."))),
            StatusCode::CONFLICT => Err(DeleteJobErrorKind::JobNotTranscribed(response.json().expect("Unable to deserialize JSON."))),
            other => Err(DeleteJobErrorKind::Other(other, response.json().expect("Unable to deserialize JSON.")))
        }
    }

    pub fn get_account_details(&self) -> GetAccountDetailsResult {
        let response = self.http
            .get(endpoints::ACCOUNT)
            .send()
            .expect("Failed to get Temi account");
            
            match response.status() {
                StatusCode::OK => Ok(response.json().expect("Unable to deserialize response.")),
                StatusCode::UNAUTHORIZED => Err(GetAccountDetailsErrorKind::Unauthorized(response.json().expect("Unable to deserialize JSON."))),
                status => Err(GetAccountDetailsErrorKind::Other(status, response.json().expect("Unable to deserialize JSON.")))
            }
    }

    pub fn get_job_status(&self, job_id: &str) -> GetJobResult {
        let endpoint = endpoints::job_url(job_id).expect("Failed to get a job URL.");
        let response = self.http
            .get(endpoint)
            .send()
            .expect("Failed to get Temi job");

        match response.status() {
            StatusCode::BAD_REQUEST => Err(GetJobErrorKind::InvalidParameters(response.json().expect("Unable to deserialize JSON."))),
            StatusCode::NOT_FOUND => Err(GetJobErrorKind::JobNotFound(response.json().expect("Unable to deserialize JSON."))),
            StatusCode::OK => Ok(response.json().expect("Unable to deserialize response.")),
            s => Err(GetJobErrorKind::Other(s, response.json().expect("Unable to deserialize JSON.")))
        }
    }

    pub fn list_jobs(&self, limit: Option<u32>, starting_after: Option<&str>) -> ListJobsResult  {
        let endpoint = endpoints::list_jobs_url(limit, starting_after)
            .expect("Failed to create a list jobs URL.");

        let response = self.http
            .get(endpoint)
            .send()
            .expect("Failed to list Temi jobs.");

        match response.status() {
            StatusCode::OK => Ok(response.json().expect("Unable to deserialize response.")),
            s => panic!("uh oh. Got a status code of {:?}.", s)
        }
    }

    pub fn get_transcript(&self, job_id: &str, format: TranscriptFormat, version: Option<TranscriptVersion>) -> () {
        let endpoint = endpoints::get_transcript_url(job_id, version)
            .expect("Failed to create a transcript URL.");

        let mut response = self.http
            .get(endpoint)
            .header(header::ACCEPT, format.mime_type())
            .send()
            .expect("Failed to get transcript.");

        match response.status() {
            StatusCode::OK => response.copy_to(&mut std::io::stdout()).expect("Failed to write to stdout."),
            _ => panic!("Uh oh...sphagettios")
        };

        ()
    }
}