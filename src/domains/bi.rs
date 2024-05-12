use std::collections::HashMap;
use crate::utils::api_client::APIClient;
use crate::utils::misc::ApiResponse;
use crate::utils::misc::ApiClientError;
use crate::utils::misc::get_query_params;
use crate::utils::misc::interpolate_str;
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
            client: client.clone(),
            addresses: BiAddresses::new(client.clone()),
            pools: BiPools::new(client.clone()),
        }
    }
}

pub struct BiAddresses {
    pub client: APIClient,
}

impl BiAddresses {
    pub fn new(client: APIClient) -> Self {
        BiAddresses {
            client: client.clone(),
        }
    }
    pub async fn stats_(&self, address: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = ["epoch_no_min", "epoch_no_max", "page_no", "page_size", "order"];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/bi/addresses/{address}/stats";
        let mut params_map = HashMap::new();
        params_map.insert("address", address.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}

pub struct BiPools {
    pub client: APIClient,
    pub stats: BiPoolsStats,
}

impl BiPools {
    pub fn new(client: APIClient) -> Self {
        BiPools {
            client: client.clone(),
            stats: BiPoolsStats::new(client.clone()),
        }
    }
    pub async fn stats_(&self, pool_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/bi/pools/{pool_hash}/stats";
        let mut params_map = HashMap::new();
        params_map.insert("pool_hash", pool_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}

pub struct BiPoolsStats {
    pub client: APIClient,
}

impl BiPoolsStats {
    pub fn new(client: APIClient) -> Self {
        BiPoolsStats {
            client: client.clone(),
        }
    }
    pub async fn epochs_(&self, epoch_no: Option<i64>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/bi/pools/stats/epochs/{epoch_no}";
        let mut params_map = HashMap::new();
        params_map.insert("epoch_no", epoch_no.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn lifetime_(&self, pool_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/bi/pools/{pool_hash}/stats/lifetime";
        let mut params_map = HashMap::new();
        params_map.insert("pool_hash", pool_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}
