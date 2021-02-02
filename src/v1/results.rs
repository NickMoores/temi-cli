use super::models::{ Job, Problem, Account };
pub use reqwest::StatusCode;

#[derive(Debug)]
pub enum GetJobErrorKind {
    InvalidParameters(Problem),
    JobNotFound(Problem),
    Other(StatusCode, Problem)
}

#[derive(Debug)]
pub enum DeleteJobErrorKind {
    InvalidParameters(Problem),
    JobNotFound(Problem),
    JobNotTranscribed(Problem),
    Other(StatusCode, Problem)
}

#[derive(Debug)]
pub enum GetAccountDetailsErrorKind {
    Unauthorized(Problem),
    Other(StatusCode, Problem)
}

pub type GetJobResult = Result<Job, GetJobErrorKind>;

pub type ListJobsResult = Result<Vec<Job>, GetJobErrorKind>;

pub type DeleteJobResult = Result<(), DeleteJobErrorKind>;

pub type GetAccountDetailsResult = Result<Account, GetAccountDetailsErrorKind>;