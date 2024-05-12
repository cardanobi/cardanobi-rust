use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use serde_urlencoded;
use regex::Regex;

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

// pub fn get_query_params(options: &mut HashMap<&str, &str>, allowed_params: &[&str]) -> String {
//     // Extract and remove 'query' from options if it exists
//     let query_value = options.remove("query");

//     let filtered_options: HashMap<&str, &str> = options.iter()
//         .filter(|(key, _)| allowed_params.contains(key))
//         .map(|(&key, &value)| (key, value))
//         .collect();

//     // Use serde_urlencoded to construct the query string with proper encoding for the allowed_params
//     let mut query_string = serde_urlencoded::to_string(&filtered_options).unwrap_or_default();

//     // If 'query' was present, append its value directly to the query string without its name
//     if let Some(query) = query_value {
//         // Check if there are already other parameters in the query string to append correctly
//         if !query_string.is_empty() {
//             query_string += "&";
//         }
//         query_string += &query;
//     }

//     query_string
// }

pub fn get_query_params(options: &HashMap<&str, &str>, allowed_params: &[&str]) -> String {
    // Clone the original options and then modify the clone
    let mut temp_options = options.clone();
    let query_value = temp_options.remove("query");

    let filtered_options: HashMap<&str, &str> = temp_options.iter()
        .filter(|(key, _)| allowed_params.contains(key))
        .map(|(&key, &value)| (key, value))
        .collect();

    // Use serde_urlencoded to construct the query string with proper encoding for the allowed_params
    let mut query_string = serde_urlencoded::to_string(&filtered_options).unwrap_or_default();

    // If 'query' was present, append its value directly to the query string without its name
    if let Some(query) = query_value {
        // Check if there are already other parameters in the query string to append correctly
        if !query_string.is_empty() {
            query_string += "&";
        }
        query_string += &query;
    }

    query_string
}



// pub fn interpolate_str(template: &str, params: &HashMap<&str, &str>) -> String {
//     let mut result = template.to_string();
//     for (key, value) in params {
//         result = result.replace(&format!("{{{}}}", key), value);
//     }
//     result
// }

// pub fn interpolate_str(template: &str, params: &HashMap<&str, String>) -> String {
//     let mut result = template.to_string();
//     for (key, value) in params {
//         result = result.replace(&format!("{{{}}}", key), value);
//     }
//     result
// }

pub fn interpolate_str(template: &str, params: &HashMap<&str, Option<String>>) -> String {
    let mut result = template.to_string();
    for (key, value) in params {
        if let Some(ref v) = value {
            result = result.replace(&format!("{{{}}}", key), v);
        }
    }

    // Clean up any unresolved placeholders
    let re = Regex::new(r"\{[^}]+\}").unwrap();
    result = re.replace_all(&result, "").to_string();

    result
}

