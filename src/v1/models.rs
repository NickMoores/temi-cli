use serde::{ Deserialize, Serialize };
use rust_decimal::{ Decimal };

#[derive(Deserialize, Debug)]
pub struct Problem {
    pub r#type: Option<String>,
    pub title: String,
    pub detail: Option<String>
}

#[derive(Serialize, Debug)]
pub struct SubmitJobOptions {
    pub media_url: String,
    pub callback_url: Option<String>,
    pub metadata: Option<String>
}

pub struct SubmitJobResponse {
    id: String,
    status: String,
    created_on: String
}

#[derive(Deserialize, Debug)]
pub struct Account {
    pub balance: Decimal,
    pub email: String
}

#[derive(Deserialize, Debug)]
pub struct Job {
    pub id: String,
    pub status: String,
    pub callback_url: Option<String>,
    //created_on
    pub web_url: Option<String>,
    pub duration_seconds: Option<u64>,
    pub name: String,
    pub metadata: Option<String>,
    pub failure: Option<String>,
    pub failure_detail: Option<String>,
    //last_modified_on
}
#[derive(Deserialize, Debug)]
pub enum TranscriptVersion {
    Latest,
    Machine
}

impl TranscriptVersion {
    pub fn to_query_str(&self) -> &str {
        match &self {
            TranscriptVersion::Latest => "latest",
            TranscriptVersion::Machine => "machine"
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum TranscriptFormat {
    PlainText,
    Json,
    MsWord,
    Pdf
}

impl TranscriptFormat {
    pub fn mime_type(&self) -> &str {
        match &self {
            TranscriptFormat::PlainText => "text/plain",
            TranscriptFormat::Json => "application/json",
            TranscriptFormat::MsWord => "application/msword",
            TranscriptFormat::Pdf => "application/pdf"
        }
    }
}
