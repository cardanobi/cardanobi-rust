use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
pub struct ApiResponse {
    pub data: Value,
}

#[derive(Debug)]
pub enum ApiClientError {
    Unauthorized,
    RequestError(reqwest::Error),  // Wrap the reqwest::Error
    JsonError(serde_json::Error),  // Wrap JSON parsing errors
    Other(String),  // Other errors, possibly with descriptions
}

impl From<reqwest::Error> for ApiClientError {
    fn from(err: reqwest::Error) -> Self {
        ApiClientError::RequestError(err)
    }
}

impl From<serde_json::Error> for ApiClientError {
    fn from(err: serde_json::Error) -> Self {
        ApiClientError::JsonError(err)  // Convert the error to a string and store it
    }
}