// Use the reqwest library for HTTP requests, urlencoding for query params, and base64 for encoding
use reqwest::{Client, Error, header};
use serde::{Deserialize};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde_json::Value;
// use reqwest::Error as ReqwestError;

use crate::utils::constants::{API_BASE_URLS, IDS_BASE_URLS};
// use crate::utils::misc::ApiResponse;
use crate::utils::misc::ApiClientError;


#[derive(Deserialize)]
struct AccessTokenResponse {
    access_token: String,
}

// #[derive(Clone)]
pub struct APIClient {
    pub api_key: String, // consider which other fields might also need to be public
    pub api_secret: Option<String>,
    pub network: String,
    pub base_url: String,
    pub ids_base_url: String,
    pub auth_token: String,
    pub access_token: Option<String>,  // Made public
    client: Client,
}

impl Clone for APIClient {
    fn clone(&self) -> Self {
        Self {
            api_key: self.api_key.clone(),
            api_secret: self.api_secret.clone(),
            network: self.network.clone(),
            base_url: self.base_url.clone(),
            ids_base_url: self.ids_base_url.clone(),
            auth_token: self.auth_token.clone(),
            access_token: self.access_token.clone(),
            client: self.client.clone(), // Make sure Client supports cloning or has a similar mechanism
        }
    }
}

impl APIClient {
    pub fn new(api_key: Option<&str>, api_secret: Option<&str>, network: Option<&str>) ->  Result<APIClient, ApiClientError> {
        // Check and handle potential errors
        if api_key.is_none() {
            return Err(ApiClientError::Other("API key is required".to_string()));
        }

        // Default values are provided if `None` is passed
        let network = network.unwrap_or("mainnet");
        let api_key = api_key.unwrap_or("onlypublic");
        let api_secret = api_secret.unwrap_or_default();

        // Derive URLs based on the network
        let base_url = API_BASE_URLS.get(network).unwrap_or(&"").to_string();
        let ids_base_url = IDS_BASE_URLS.get(network).unwrap_or(&"").to_string();

        // Create the auth_token by base64 encoding the api_key and api_secret
        let auth_token = if !api_secret.is_empty() {
            let encode_input = format!("{}:{}", api_key, api_secret);
            STANDARD.encode(encode_input.as_bytes())
        } else {
            String::new()
        };

        Ok(APIClient {
            api_key: api_key.to_string(),
            api_secret: Some(api_secret.to_string()),
            network: network.to_string(),
            base_url,
            ids_base_url,
            auth_token,
            access_token: None,
            client: Client::new(),
        })
    }

    // Asynchronous function to get an access token
    pub async fn get_access_token(&mut self) -> Result<(), Error> {
        let client = Client::new();
        let data = [("grant_type", "client_credentials")];
        let url = format!("{}/connect/token", self.ids_base_url);

        // println!("get_access_token, url:{}",url);

        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, format!("Basic {}", self.auth_token).parse().unwrap());
        headers.insert(header::CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());

        // println!("get_access_token, auth_token:{}",self.auth_token);

        let response = client.post(&url)
            .headers(headers)
            .form(&data)
            .send()
            .await?;

        // println!("get_access_token, response:{}",response);

        if response.status().is_success() {
            let resp_json: AccessTokenResponse = response.json().await?;
            self.access_token = Some(resp_json.access_token);
        } else {
            // You might want to handle different status codes and errors differently
            eprintln!("Failed to fetch access token: Status code {}", response.status());
        }

        Ok(())
    }

    // async fn get_request(&self, url: &str, private: bool) -> Result<ApiResponse, ApiClientError> {
    //     let headers = self.prepare_headers(private);

    //     println!("get_request, url:{}",url);

    //     let response = self.client.get(url)
    //         .headers(headers)
    //         .send()
    //         .await.map_err(ApiClientError::from)?;
    
    //     // Check and handle the HTTP status
    //     // let response = response.error_for_status()?; // Propagate error if non-2xx status
    //     // response.json::<ApiResponse>().await  // Deserialize and return ApiResponse
    //     // response.json::<ApiResponse>().await.map_err(ApiClientError::from)

    //     // Deserialize the JSON into the ApiResponse struct
    //     let api_response = response.json::<ApiResponse>().await.map_err(ApiClientError::from)?;
    //     Ok(api_response)
    // }

    // async fn get_request(&self, url: &str, private: bool) -> Result<ApiResponse, ApiClientError> {
    //     let headers = self.prepare_headers(private);

    //     println!("get_request, url:{}", url);
    
    //     let response = self.client.get(url)
    //         .headers(headers)
    //         .send()
    //         .await.map_err(ApiClientError::from)?;
    
    //     // Fetch and log the raw response text
    //     let text = response.text().await.map_err(ApiClientError::from)?;
    //     println!("Raw JSON response: {}", text);  // Log raw response text
    
    //     // Attempt to parse the text as JSON
    //     let api_response = serde_json::from_str::<ApiResponse>(&text).map_err(ApiClientError::from)?;
    //     Ok(api_response)
    // }
    
    // Update the return type to use serde_json::Value
    async fn get_request(&self, url: &str, private: bool) -> Result<Value, ApiClientError> {
        let headers = self.prepare_headers(private);

        println!("get_request, url:{}", url);

        let response = self.client.get(url)
            .headers(headers)
            .send()
            .await.map_err(ApiClientError::RequestError)?;

        // Fetch and log the raw response text
        let text = response.text().await.map_err(ApiClientError::RequestError)?;
        // println!("Raw JSON response: {}", text);  // Log raw response text

        // Attempt to parse the text as JSON
        let json_value = serde_json::from_str::<Value>(&text).map_err(ApiClientError::JsonError)?;
        Ok(json_value)
    }

    fn prepare_headers(&self, private: bool) -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::ACCEPT, "application/json".parse().unwrap());
        headers.insert(header::CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
        headers.insert("Client-Api-Key", self.api_key.parse().unwrap());

        // let authorization_value = if private && self.access_token.is_some() {
        //     format!("Bearer {}", self.access_token.as_ref().unwrap())
        // } else {
        //     format!("Basic {}", self.auth_token)
        // };

        let authorization_value = if private {
            self.access_token.as_ref()
                .map(|token| format!("Bearer {}", token))
                .unwrap_or_else(|| format!("Basic {}", self.auth_token))  // Fallback to Basic if None unexpectedly
        } else {
            format!("Basic {}", self.auth_token)
        };
        

        // println!("prepare_headers, authorization_value:{}",authorization_value);

        headers.insert(header::AUTHORIZATION, authorization_value.parse().unwrap());
        headers
    }

    pub async fn get_private(&self, path: &str) -> Result<Value, ApiClientError> {
        if self.access_token.is_none() {
            return Err(ApiClientError::Unauthorized);
        }

        let url = format!("{}{}", self.base_url, path);

        self.get_request(&url, true).await
    }

    pub async fn get_public(&self, path: &str) -> Result<Value, ApiClientError> {
        let url = format!("{}{}", self.base_url, path);
        self.get_request(&url, false).await
    }
}
