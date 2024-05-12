use std::collections::HashMap;
use crate::utils::api_client::APIClient;
use crate::utils::misc::ApiResponse;
use crate::utils::misc::ApiClientError;
use crate::utils::misc::get_query_params;
use crate::utils::misc::interpolate_str;
use serde_json::Value;
use reqwest::Error as ReqwestError;

pub struct Core {
    pub client: APIClient,
    pub accounts: CoreAccounts,
    pub addresses: CoreAddresses,
    pub assets: CoreAssets,
    pub blocks: CoreBlocks,
    pub epochs: CoreEpochs,
    pub pools: CorePools,
    pub transactions: CoreTransactions,
}

impl Core {
    pub fn new(client: APIClient) -> Self {
        Core {
            client: client.clone(),
            accounts: CoreAccounts::new(client.clone()),
            addresses: CoreAddresses::new(client.clone()),
            assets: CoreAssets::new(client.clone()),
            blocks: CoreBlocks::new(client.clone()),
            epochs: CoreEpochs::new(client.clone()),
            pools: CorePools::new(client.clone()),
            transactions: CoreTransactions::new(client.clone()),
        }
    }
    pub async fn accounts_(&self, stake_address: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/accounts/{stake_address}";
        let mut params_map = HashMap::new();
        params_map.insert("stake_address", stake_address.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn addressesinfo_(&self, address: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/odata/addressesinfo/{address}";
        let mut params_map = HashMap::new();
        params_map.insert("address", address.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn assets_(&self, fingerprint: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = ["page_no", "page_size", "order"];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/assets/{fingerprint}";
        let mut params_map = HashMap::new();
        params_map.insert("fingerprint", fingerprint.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn blocks_(&self, block_no: Option<i64>, block_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = ["block_no", "depth"];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = match (block_no.is_some(), block_hash.is_some()) {
            (true, false) => "/api/core/blocks/{block_no}",
            (false, true) => "/api/core/blocks/{block_hash}",
            (false, false) => "/api/core/odata/blocks",
            _ => "/api/core/blocks/{block_no}"
        };
        let mut params_map = HashMap::new();
        params_map.insert("block_no", block_no.map(|v| v.to_string()));
        params_map.insert("block_hash", block_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn epochs_(&self, epoch_no: Option<i64>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/epochs/{epoch_no}";
        let mut params_map = HashMap::new();
        params_map.insert("epoch_no", epoch_no.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn epochsparams_(&self, epoch_no: Option<i64>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/odata/epochsparams/{epoch_no}";
        let mut params_map = HashMap::new();
        params_map.insert("epoch_no", epoch_no.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn epochsstakes_(&self, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = ["epoch_no", "pool_hash", "page_no", "page_size", "order"];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/odata/epochsstakes";
        let mut params_map = HashMap::new();
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn polls_(&self, poll_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = ["page_no", "page_size", "order"];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/polls/{poll_hash}";
        let mut params_map = HashMap::new();
        params_map.insert("poll_hash", poll_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn poolshashes_(&self, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/odata/poolshashes";
        let mut params_map = HashMap::new();
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn poolsmetadata_(&self, pool_id: Option<i64>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/odata/poolsmetadata/{pool_id}";
        let mut params_map = HashMap::new();
        params_map.insert("pool_id", pool_id.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn poolsofflinedata_(&self, pool_id: Option<i64>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/odata/poolsofflinedata/{pool_id}";
        let mut params_map = HashMap::new();
        params_map.insert("pool_id", pool_id.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn poolsofflinefetcherrors_(&self, pool_id: Option<i64>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/odata/poolsofflinefetcherrors/{pool_id}";
        let mut params_map = HashMap::new();
        params_map.insert("pool_id", pool_id.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn poolsrelays_(&self, update_id: Option<i64>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/odata/poolsrelays/{update_id}";
        let mut params_map = HashMap::new();
        params_map.insert("update_id", update_id.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn poolsupdates_(&self, pool_id: Option<i64>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/odata/poolsupdates/{pool_id}";
        let mut params_map = HashMap::new();
        params_map.insert("pool_id", pool_id.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn transactions_(&self, transaction_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/transactions/{transaction_hash}";
        let mut params_map = HashMap::new();
        params_map.insert("transaction_hash", transaction_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}

pub struct CoreAccounts {
    pub client: APIClient,
}

impl CoreAccounts {
    pub fn new(client: APIClient) -> Self {
        CoreAccounts {
            client: client.clone(),
        }
    }
    pub async fn rewards_(&self, stake_address: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/accounts/{stake_address}/rewards";
        let mut params_map = HashMap::new();
        params_map.insert("stake_address", stake_address.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn staking_(&self, stake_address: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/accounts/{stake_address}/staking";
        let mut params_map = HashMap::new();
        params_map.insert("stake_address", stake_address.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn delegations_(&self, stake_address: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/accounts/{stake_address}/delegations";
        let mut params_map = HashMap::new();
        params_map.insert("stake_address", stake_address.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn registrations_(&self, stake_address: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/accounts/{stake_address}/registrations";
        let mut params_map = HashMap::new();
        params_map.insert("stake_address", stake_address.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn withdrawals_(&self, stake_address: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/accounts/{stake_address}/withdrawals";
        let mut params_map = HashMap::new();
        params_map.insert("stake_address", stake_address.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn mirs_(&self, stake_address: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/accounts/{stake_address}/mirs";
        let mut params_map = HashMap::new();
        params_map.insert("stake_address", stake_address.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn addresses_(&self, stake_address: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/accounts/{stake_address}/addresses";
        let mut params_map = HashMap::new();
        params_map.insert("stake_address", stake_address.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn assets_(&self, stake_address: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/accounts/{stake_address}/assets";
        let mut params_map = HashMap::new();
        params_map.insert("stake_address", stake_address.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}

pub struct CoreAddresses {
    pub client: APIClient,
}

impl CoreAddresses {
    pub fn new(client: APIClient) -> Self {
        CoreAddresses {
            client: client.clone(),
        }
    }
    pub async fn info_(&self, address: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/addresses/{address}/info";
        let mut params_map = HashMap::new();
        params_map.insert("address", address.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}

pub struct CoreAssets {
    pub client: APIClient,
}

impl CoreAssets {
    pub fn new(client: APIClient) -> Self {
        CoreAssets {
            client: client.clone(),
        }
    }
    pub async fn history_(&self, fingerprint: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = ["page_no", "page_size", "order"];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/assets/{fingerprint}/history";
        let mut params_map = HashMap::new();
        params_map.insert("fingerprint", fingerprint.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn transactions_(&self, fingerprint: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = ["page_no", "page_size", "order"];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/assets/{fingerprint}/transactions";
        let mut params_map = HashMap::new();
        params_map.insert("fingerprint", fingerprint.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn addresses_(&self, fingerprint: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = ["page_no", "page_size", "order"];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/assets/{fingerprint}/addresses";
        let mut params_map = HashMap::new();
        params_map.insert("fingerprint", fingerprint.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn policies_(&self, policy_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/assets/policies/{policy_hash}";
        let mut params_map = HashMap::new();
        params_map.insert("policy_hash", policy_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}

pub struct CoreBlocks {
    pub client: APIClient,
    pub epochs: CoreBlocksEpochs,
    pub history: CoreBlocksHistory,
    pub latest: CoreBlocksLatest,
    pub pools: CoreBlocksPools,
}

impl CoreBlocks {
    pub fn new(client: APIClient) -> Self {
        CoreBlocks {
            client: client.clone(),
            epochs: CoreBlocksEpochs::new(client.clone()),
            history: CoreBlocksHistory::new(client.clone()),
            latest: CoreBlocksLatest::new(client.clone()),
            pools: CoreBlocksPools::new(client.clone()),
        }
    }
    pub async fn latest_(&self, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/blocks/latest";
        let mut params_map = HashMap::new();
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn history_(&self, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = ["block_no", "depth"];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/blocks/history";
        let mut params_map = HashMap::new();
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn transactions_(&self, block_no: Option<i64>, block_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = match (block_no.is_some(), block_hash.is_some()) {
            (true, false) => "/api/core/blocks/{block_no}/transactions",
            (false, true) => "/api/core/blocks/{block_hash}/transactions",
            _ => "/api/core/blocks/{block_no}/transactions"
        };
        let mut params_map = HashMap::new();
        params_map.insert("block_no", block_no.map(|v| v.to_string()));
        params_map.insert("block_hash", block_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}

pub struct CoreBlocksEpochs {
    pub client: APIClient,
}

impl CoreBlocksEpochs {
    pub fn new(client: APIClient) -> Self {
        CoreBlocksEpochs {
            client: client.clone(),
        }
    }
    pub async fn slots_(&self, epoch_no: Option<i64>, slot_no: Option<i64>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = match (epoch_no.is_some(), slot_no.is_some()) {
            (true, true) => "/api/core/blocks/epochs/{epoch_no}/slots/{slot_no}",
            _ => "/api/core/blocks/epochs/{epoch_no}/slots/{slot_no}"
        };
        let mut params_map = HashMap::new();
        params_map.insert("epoch_no", epoch_no.map(|v| v.to_string()));
        params_map.insert("slot_no", slot_no.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}

pub struct CoreBlocksHistory {
    pub client: APIClient,
}

impl CoreBlocksHistory {
    pub fn new(client: APIClient) -> Self {
        CoreBlocksHistory {
            client: client.clone(),
        }
    }
    pub async fn prev_(&self, block_no: Option<i64>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = ["depth"];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/blocks/history/prev/{block_no}";
        let mut params_map = HashMap::new();
        params_map.insert("block_no", block_no.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn next_(&self, block_no: Option<i64>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = ["depth"];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/blocks/history/next/{block_no}";
        let mut params_map = HashMap::new();
        params_map.insert("block_no", block_no.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}

pub struct CoreBlocksLatest {
    pub client: APIClient,
}

impl CoreBlocksLatest {
    pub fn new(client: APIClient) -> Self {
        CoreBlocksLatest {
            client: client.clone(),
        }
    }
    pub async fn pools_(&self, pool_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/blocks/latest/pools/{pool_hash}";
        let mut params_map = HashMap::new();
        params_map.insert("pool_hash", pool_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn transactions_(&self, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/blocks/latest/transactions";
        let mut params_map = HashMap::new();
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}

pub struct CoreBlocksPools {
    pub client: APIClient,
}

impl CoreBlocksPools {
    pub fn new(client: APIClient) -> Self {
        CoreBlocksPools {
            client: client.clone(),
        }
    }
    pub async fn history_(&self, pool_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/blocks/pools/{pool_hash}/history";
        let mut params_map = HashMap::new();
        params_map.insert("pool_hash", pool_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}

pub struct CoreEpochs {
    pub client: APIClient,
    pub params: CoreEpochsParams,
    pub stakes: CoreEpochsStakes,
    pub latest: CoreEpochsLatest,
}

impl CoreEpochs {
    pub fn new(client: APIClient) -> Self {
        CoreEpochs {
            client: client.clone(),
            params: CoreEpochsParams::new(client.clone()),
            stakes: CoreEpochsStakes::new(client.clone()),
            latest: CoreEpochsLatest::new(client.clone()),
        }
    }
    pub async fn latest_(&self, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/epochs/latest";
        let mut params_map = HashMap::new();
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn params_(&self, epoch_no: Option<i64>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/epochs/{epoch_no}/params";
        let mut params_map = HashMap::new();
        params_map.insert("epoch_no", epoch_no.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}

pub struct CoreEpochsParams {
    pub client: APIClient,
}

impl CoreEpochsParams {
    pub fn new(client: APIClient) -> Self {
        CoreEpochsParams {
            client: client.clone(),
        }
    }
    pub async fn latest_(&self, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/epochs/params/latest";
        let mut params_map = HashMap::new();
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}

pub struct CoreEpochsStakes {
    pub client: APIClient,
}

impl CoreEpochsStakes {
    pub fn new(client: APIClient) -> Self {
        CoreEpochsStakes {
            client: client.clone(),
        }
    }
    pub async fn pools_(&self, pool_hash: Option<&str>, epoch_no: Option<i64>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = ["page_no", "page_size", "order"];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = match (pool_hash.is_some(), epoch_no.is_some()) {
            (true, false) => "/api/core/epochs/stakes/pools/{pool_hash}",
            (true, true) => "/api/core/epochs/{epoch_no}/stakes/pools/{pool_hash}",
            _ => "/api/core/epochs/stakes/pools/{pool_hash}"
        };
        println!("path_template: {}", path_template);
        let mut params_map = HashMap::new();
        params_map.insert("pool_hash", pool_hash.map(|v| v.to_string()));
        params_map.insert("epoch_no", epoch_no.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        println!("path: {}", path);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}

pub struct CoreEpochsLatest {
    pub client: APIClient,
    pub stakes: CoreEpochsLatestStakes,
}

impl CoreEpochsLatest {
    pub fn new(client: APIClient) -> Self {
        CoreEpochsLatest {
            client: client.clone(),
            stakes: CoreEpochsLatestStakes::new(client.clone()),
        }
    }
}

pub struct CoreEpochsLatestStakes {
    pub client: APIClient,
}

impl CoreEpochsLatestStakes {
    pub fn new(client: APIClient) -> Self {
        CoreEpochsLatestStakes {
            client: client.clone(),
        }
    }
    pub async fn pools_(&self, pool_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = ["page_no", "page_size", "order"];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/epochs/latest/stakes/pools/{pool_hash}";
        let mut params_map = HashMap::new();
        params_map.insert("pool_hash", pool_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}

pub struct CorePools {
    pub client: APIClient,
    pub metadata: CorePoolsMetadata,
    pub relays: CorePoolsRelays,
}

impl CorePools {
    pub fn new(client: APIClient) -> Self {
        CorePools {
            client: client.clone(),
            metadata: CorePoolsMetadata::new(client.clone()),
            relays: CorePoolsRelays::new(client.clone()),
        }
    }
    pub async fn hashes_(&self, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/pools/hashes";
        let mut params_map = HashMap::new();
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn metadata_(&self, pool_id: Option<i64>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/pools/{pool_id}/metadata";
        let mut params_map = HashMap::new();
        params_map.insert("pool_id", pool_id.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn offlinedata_(&self, pool_id: Option<i64>, ticker: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = match (pool_id.is_some(), ticker.is_some()) {
            (false, false) => "/api/core/pools/offlinedata",
            (true, false) => "/api/core/pools/{pool_id}/offlinedata",
            (false, true) => "/api/core/pools/{ticker}/offlinedata",
            _ => "/api/core/pools/offlinedata"
        };
        let mut params_map = HashMap::new();
        params_map.insert("pool_id", pool_id.map(|v| v.to_string()));
        params_map.insert("ticker", ticker.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn offlinefetcherrors_(&self, pool_id: Option<i64>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/pools/{pool_id}/offlinefetcherrors";
        let mut params_map = HashMap::new();
        params_map.insert("pool_id", pool_id.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn updates_(&self, pool_id: Option<i64>, vrf_key_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = match (pool_id.is_some(), vrf_key_hash.is_some()) {
            (false, false) => "/api/core/pools/updates",
            (true, false) => "/api/core/pools/{pool_id}/updates",
            (false, true) => "/api/core/pools/{vrf_key_hash}/updates",
            _ => "/api/core/pools/updates"
        };
        let mut params_map = HashMap::new();
        params_map.insert("pool_id", pool_id.map(|v| v.to_string()));
        params_map.insert("vrf_key_hash", vrf_key_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}

pub struct CorePoolsMetadata {
    pub client: APIClient,
}

impl CorePoolsMetadata {
    pub fn new(client: APIClient) -> Self {
        CorePoolsMetadata {
            client: client.clone(),
        }
    }
    pub async fn hashes_(&self, meta_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/pools/metadata/hashes/{meta_hash}";
        let mut params_map = HashMap::new();
        params_map.insert("meta_hash", meta_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn offlinedata_(&self, meta_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/pools/metadata/{meta_hash}/offlinedata";
        let mut params_map = HashMap::new();
        params_map.insert("meta_hash", meta_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}

pub struct CorePoolsRelays {
    pub client: APIClient,
}

impl CorePoolsRelays {
    pub fn new(client: APIClient) -> Self {
        CorePoolsRelays {
            client: client.clone(),
        }
    }
    pub async fn updates_(&self, update_id: Option<i64>, vrf_key_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = match (update_id.is_some(), vrf_key_hash.is_some()) {
            (false, false) => "/api/core/pools/relays/updates",
            (true, false) => "/api/core/pools/relays/updates/{update_id}",
            (false, true) => "/api/core/pools/{vrf_key_hash}/relays/updates",
            _ => "/api/core/pools/relays/updates"
        };
        let mut params_map = HashMap::new();
        params_map.insert("update_id", update_id.map(|v| v.to_string()));
        params_map.insert("vrf_key_hash", vrf_key_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}

pub struct CoreTransactions {
    pub client: APIClient,
}

impl CoreTransactions {
    pub fn new(client: APIClient) -> Self {
        CoreTransactions {
            client: client.clone(),
        }
    }
    pub async fn utxos_(&self, transaction_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/transactions/{transaction_hash}/utxos";
        let mut params_map = HashMap::new();
        params_map.insert("transaction_hash", transaction_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn stake_address_registrations_(&self, transaction_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/transactions/{transaction_hash}/stake_address_registrations";
        let mut params_map = HashMap::new();
        params_map.insert("transaction_hash", transaction_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn stake_address_delegations_(&self, transaction_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/transactions/{transaction_hash}/stake_address_delegations";
        let mut params_map = HashMap::new();
        params_map.insert("transaction_hash", transaction_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn withdrawals_(&self, transaction_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/transactions/{transaction_hash}/withdrawals";
        let mut params_map = HashMap::new();
        params_map.insert("transaction_hash", transaction_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn treasury_(&self, transaction_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/transactions/{transaction_hash}/treasury";
        let mut params_map = HashMap::new();
        params_map.insert("transaction_hash", transaction_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn reserves_(&self, transaction_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/transactions/{transaction_hash}/reserves";
        let mut params_map = HashMap::new();
        params_map.insert("transaction_hash", transaction_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn param_proposals_(&self, transaction_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/transactions/{transaction_hash}/param_proposals";
        let mut params_map = HashMap::new();
        params_map.insert("transaction_hash", transaction_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn retiring_pools_(&self, transaction_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/transactions/{transaction_hash}/retiring_pools";
        let mut params_map = HashMap::new();
        params_map.insert("transaction_hash", transaction_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn updating_pools_(&self, transaction_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/transactions/{transaction_hash}/updating_pools";
        let mut params_map = HashMap::new();
        params_map.insert("transaction_hash", transaction_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn metadata_(&self, transaction_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/transactions/{transaction_hash}/metadata";
        let mut params_map = HashMap::new();
        params_map.insert("transaction_hash", transaction_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn assetmints_(&self, transaction_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/transactions/{transaction_hash}/assetmints";
        let mut params_map = HashMap::new();
        params_map.insert("transaction_hash", transaction_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
    pub async fn redeemers_(&self, transaction_hash: Option<&str>, options: HashMap<&str, &str>) -> Result<Value, ApiClientError> {
        let allowed_params = [];
        let query_string = get_query_params(&options, &allowed_params);
        let path_template = "/api/core/transactions/{transaction_hash}/redeemers";
        let mut params_map = HashMap::new();
        params_map.insert("transaction_hash", transaction_hash.map(|v| v.to_string()));
        let mut path = interpolate_str(&path_template, &params_map);
        if !query_string.is_empty() {
            path = format!("{}?{}", path, query_string);
        }
        self.client.get_private(&path).await
    }
}
