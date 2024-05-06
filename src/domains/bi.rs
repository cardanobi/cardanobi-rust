use std::collections::HashMap;
use crate::utils::api_client::APIClient;
use crate::utils::misc::ApiResponse;
use crate::utils::misc::ApiClientError;
use serde_json::Value;
use reqwest::Error as ReqwestError;


pub struct Bi {
    pub client: APIClient,
    pub addresses: BiAddresses,
    pub pools: BiPools,
}

impl Bi {
    pub fn new(client: APIClient) -> Self {
        Bi {
            client: client.clone(), // Assuming APIClient implements Clone
            addresses: BiAddresses::new(client.clone()), // Provide a separate instance or reference as needed
            pools: BiPools::new(client.clone()),
        }
    }
}

pub struct BiAddresses {
    client: APIClient,
}

impl BiAddresses {
    pub fn new(client: APIClient) -> Self {
        BiAddresses { 
            client: client.clone(),
        }
    }

    pub async fn stats_(&self, address: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let mut path = format!("/api/bi/addresses/{}/stats", address.unwrap_or_default());
        // let mut path = format!("/api/core/blocks/latest");
        let query_string = get_query_params(&options);
        if !query_string.is_empty() {
            path.push_str(&format!("?{}", query_string));
        }
        self.client.get_private(&path).await
    }
}

pub struct BiPools {
    client: APIClient,
    pub stats: BiPoolsStats,
}

impl BiPools {
    pub fn new(client: APIClient) -> Self {
        BiPools {
            client: client.clone(),
            stats: BiPoolsStats::new(client.clone()), // Directly instantiate stats here
        }
    }
}

pub struct BiPoolsStats {
    client: APIClient,
}

impl BiPoolsStats {
    pub fn new(client: APIClient) -> Self {
        BiPoolsStats { 
            client: client.clone(),
        }
    }

    pub async fn epochs_(&self, epoch_no: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let mut path = format!("/api/bi/pools/stats/epochs/{}", epoch_no.unwrap_or_default());
        let query_string = get_query_params(&options);
        if !query_string.is_empty() {
            path.push_str(&format!("?{}", query_string));
        }
        self.client.get_private(&path).await
    }
}

// Helper function to construct query parameters from HashMap
fn get_query_params(params: &HashMap<&str, &str>) -> String {
    params.iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join("&")
}
