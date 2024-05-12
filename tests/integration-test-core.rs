use dotenv::dotenv;
use std::env;
use cardanobi_rust::{APIClient, CardanoBI}; 
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_json::{Value, to_string_pretty};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::Write;  // Include the Write trait
use colored::*;
use regex::Regex;

fn strip_ansi_codes(s: &str) -> String {
    let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(s, "").to_string()
}

lazy_static! {
    static ref UNIT_TEST_OUTPUT_DIR: Mutex<String> = {
        dotenv::from_filename("tests/.env").ok(); // Specify the path to your .env file
        let dir = env::var("UNIT_TEST_OUTPUT_DIRECTORY").unwrap_or_else(|_| "test_output".to_string());
        // Create the output directory if it doesn't exist
        fs::create_dir_all(&dir).expect("Failed to create output directory");
        Mutex::new(dir)
    };

    static ref LOG_FILE_PATH: Mutex<String> = {
        dotenv::from_filename("tests/.env").ok(); // Specify the path to your .env file
        let log_file = env::var("LOG_FILE").unwrap_or_else(|_| "test_log.txt".to_string());
        Mutex::new(log_file)
    };
}

fn get_environment_variable() -> (String, String, String) {
    dotenv::from_filename("tests/.env").ok(); // Specify the path to your .env file
    let api_key = env::var("CBI_API_KEY").expect("CBI_API_KEY must be set");
    let api_secret = env::var("CBI_API_SECRET").expect("CBI_API_SECRET must be set");
    let network = env::var("CBI_ENV").unwrap_or_else(|_| "mainnet".to_string());
    (api_key, api_secret, network)
}

async fn with_context<F, Fut>(test_name: &str, test_fn: F)
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error>>>,
{
    println!("\nStarting test: {}", test_name); // Log the test name at the beginning
    let start = Instant::now();
    let log_file = LOG_FILE_PATH.lock().unwrap();
    let result = test_fn().await;
    let duration = start.elapsed().as_millis();
    let outcome = match result {
        Ok(_) => format!("{} PASSED in {}ms", test_name, duration).green().to_string(),
        Err(e) => format!("{} FAILED in {}ms. Error: {}", test_name, duration, e).red().to_string(),
    };

    println!("{}", outcome);
    let mut file = OpenOptions::new().append(true).create(true).open(&*log_file).expect("Unable to open log file");
    writeln!(file, "{}", strip_ansi_codes(&outcome)).expect("Unable to write to log file");
}


fn save_response_to_file(response: Value, test_name: &str) {
    let output_dir = UNIT_TEST_OUTPUT_DIR.lock().unwrap();
    let output_path = Path::new(&*output_dir).join(format!("{}.json", test_name));
    let mut file = fs::File::create(output_path).expect("Failed to create file");
    let json_pretty = to_string_pretty(&response).expect("Failed to serialize JSON");
    file.write_all(json_pretty.as_bytes()).expect("Failed to write to file");
}

// Test functions using the above context manager and fixture

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test as async_test;

    // Helper function that returns CardanoBI or exits/logs error
    async fn initialize_cardanobi(api_key: &str, api_secret: &str, network: &str) -> CardanoBI {
        CardanoBI::new(Some(api_key), Some(api_secret), Some(network))
            .await
            .unwrap_or_else(|err| {
                eprintln!("Failed to initialize CardanoBI: {:?}", err);
                std::process::exit(1); // Exit or handle as appropriate for your application
            })
    }

    #[async_test]
    async fn test_api_core_accounts_with_specific_params_stake_address() {
        with_context("test_api_core_accounts_with_specific_params_stake_address", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let stake_address: Option<&str> = Some("stake1u8a9qstrmj4rvc3k5z8fems7f0j2vztz8det2klgakhfc8ce79fma");
            let result = cbi.core.accounts_(stake_address, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_accounts_with_specific_params_stake_address");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_addressesinfo_with_specific_params_address_odata() {
        with_context("test_api_core_addressesinfo_with_specific_params_address_odata", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let address: Option<&str> = Some("stake1u8a9qstrmj4rvc3k5z8fems7f0j2vztz8det2klgakhfc8ce79fma");
            let result = cbi.core.addressesinfo_(address, { let mut opts = HashMap::new(); opts.insert("odata", "true"); opts }).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_addressesinfo_with_specific_params_address_odata");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_assets_with_specific_params_fingerprint() {
        with_context("test_api_core_assets_with_specific_params_fingerprint", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let fingerprint: Option<&str> = Some("asset1w8wujx5xpxk88u94t0c60lsjlgwpd635a3c3lc");
            let result = cbi.core.assets_(fingerprint, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_assets_with_specific_params_fingerprint");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_blocks_with_specific_params_block_no() {
        with_context("test_api_core_blocks_with_specific_params_block_no", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let block_no: Option<i64> = Some(10306321);
            let block_hash: Option<&str> = None;
            let result = cbi.core.blocks_(block_no, block_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_blocks_with_specific_params_block_no");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_blocks_with_specific_params_block_hash() {
        with_context("test_api_core_blocks_with_specific_params_block_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let block_no: Option<i64> = None;
            let block_hash: Option<&str> = Some("89ff1090614105a919c9ccc8bb3914aaef1ddd28214a4d55ff65436d2c9fc0b2");
            let result = cbi.core.blocks_(block_no, block_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_blocks_with_specific_params_block_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_blocks_with_specific_params_odata() {
        with_context("test_api_core_blocks_with_specific_params_odata", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let block_no: Option<i64> = None;
            let block_hash: Option<&str> = None;
            let result = cbi.core.blocks_(block_no, block_hash, { let mut opts = HashMap::new(); opts.insert("odata", "true"); opts }).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_blocks_with_specific_params_odata");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_epochs_without_parameters() {
        with_context("test_api_core_epochs_without_parameters", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let epoch_no: Option<i64> = None;
            let result = cbi.core.epochs_(epoch_no, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_epochs_without_parameters");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_epochs_with_specific_params_epoch_no() {
        with_context("test_api_core_epochs_with_specific_params_epoch_no", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let epoch_no: Option<i64> = Some(394);
            let result = cbi.core.epochs_(epoch_no, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_epochs_with_specific_params_epoch_no");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_epochs_with_specific_params_odata() {
        with_context("test_api_core_epochs_with_specific_params_odata", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let epoch_no: Option<i64> = None;
            let result = cbi.core.epochs_(epoch_no, { let mut opts = HashMap::new(); opts.insert("odata", "true"); opts }).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_epochs_with_specific_params_odata");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_epochs_with_specific_params_epoch_no_odata() {
        with_context("test_api_core_epochs_with_specific_params_epoch_no_odata", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let epoch_no: Option<i64> = Some(394);
            let result = cbi.core.epochs_(epoch_no, { let mut opts = HashMap::new(); opts.insert("odata", "true"); opts }).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_epochs_with_specific_params_epoch_no_odata");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_epochsparams_with_specific_params_odata() {
        with_context("test_api_core_epochsparams_with_specific_params_odata", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let epoch_no: Option<i64> = None;
            let result = cbi.core.epochsparams_(epoch_no, { let mut opts = HashMap::new(); opts.insert("odata", "true"); opts }).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_epochsparams_with_specific_params_odata");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_epochsparams_with_specific_params_epoch_no_odata() {
        with_context("test_api_core_epochsparams_with_specific_params_epoch_no_odata", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let epoch_no: Option<i64> = Some(394);
            let result = cbi.core.epochsparams_(epoch_no, { let mut opts = HashMap::new(); opts.insert("odata", "true"); opts }).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_epochsparams_with_specific_params_epoch_no_odata");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_epochsstakes_with_specific_params_odata() {
        with_context("test_api_core_epochsstakes_with_specific_params_odata", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let mut options = HashMap::new();
            options.insert("odata", "true");
            options.insert("epoch_no", "394");
            options.insert("pool_hash", "pool1y24nj4qdkg35nvvnfawukauggsxrxuy74876cplmxsee29w5axc");
            
            // let result = cbi.core.epochsstakes_({ let mut opts = HashMap::new(); opts.insert("odata", "true"); opts }).await.unwrap_or_else(|e| {
            let result = cbi.core.epochsstakes_(options).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_epochsstakes_with_specific_params_odata");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_polls_with_specific_params_poll_hash() {
        with_context("test_api_core_polls_with_specific_params_poll_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let poll_hash: Option<&str> = Some("96861fe7da8d45ba5db95071ed3889ed1412929f33610636c072a4b5ab550211");
            let result = cbi.core.polls_(poll_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_polls_with_specific_params_poll_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_poolshashes_with_specific_params_odata() {
        with_context("test_api_core_poolshashes_with_specific_params_odata", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            
            let result = cbi.core.poolshashes_({ let mut opts = HashMap::new(); opts.insert("odata", "true"); opts }).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_poolshashes_with_specific_params_odata");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_poolsmetadata_with_specific_params_pool_id_odata() {
        with_context("test_api_core_poolsmetadata_with_specific_params_pool_id_odata", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let pool_id: Option<i64> = Some(4268);
            let result = cbi.core.poolsmetadata_(pool_id, { let mut opts = HashMap::new(); opts.insert("odata", "true"); opts }).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_poolsmetadata_with_specific_params_pool_id_odata");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_poolsofflinedata_with_specific_params_pool_id_odata() {
        with_context("test_api_core_poolsofflinedata_with_specific_params_pool_id_odata", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let pool_id: Option<i64> = Some(4268);
            let result = cbi.core.poolsofflinedata_(pool_id, { let mut opts = HashMap::new(); opts.insert("odata", "true"); opts }).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_poolsofflinedata_with_specific_params_pool_id_odata");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_poolsofflinefetcherrors_with_specific_params_pool_id_odata() {
        with_context("test_api_core_poolsofflinefetcherrors_with_specific_params_pool_id_odata", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let pool_id: Option<i64> = Some(4268);
            let result = cbi.core.poolsofflinefetcherrors_(pool_id, { let mut opts = HashMap::new(); opts.insert("odata", "true"); opts }).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_poolsofflinefetcherrors_with_specific_params_pool_id_odata");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_poolsrelays_with_specific_params_update_id_odata() {
        with_context("test_api_core_poolsrelays_with_specific_params_update_id_odata", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let update_id: Option<i64> = Some(1);
            let result = cbi.core.poolsrelays_(update_id, { let mut opts = HashMap::new(); opts.insert("odata", "true"); opts }).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_poolsrelays_with_specific_params_update_id_odata");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_poolsupdates_with_specific_params_pool_id_odata() {
        with_context("test_api_core_poolsupdates_with_specific_params_pool_id_odata", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let pool_id: Option<i64> = Some(4268);
            let result = cbi.core.poolsupdates_(pool_id, { let mut opts = HashMap::new(); opts.insert("odata", "true"); opts }).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_poolsupdates_with_specific_params_pool_id_odata");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_transactions_with_specific_params_transaction_hash() {
        with_context("test_api_core_transactions_with_specific_params_transaction_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let transaction_hash: Option<&str> = Some("5f6f72b00ae982492823fb541153e6c2afc9cb7231687f2a5d82a994f61764a0");
            let result = cbi.core.transactions_(transaction_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_transactions_with_specific_params_transaction_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_accounts_rewards_with_specific_params_stake_address() {
        with_context("test_api_core_accounts_rewards_with_specific_params_stake_address", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let stake_address: Option<&str> = Some("stake1u8a9qstrmj4rvc3k5z8fems7f0j2vztz8det2klgakhfc8ce79fma");
            let result = cbi.core.accounts.rewards_(stake_address, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_accounts_rewards_with_specific_params_stake_address");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_accounts_staking_with_specific_params_stake_address() {
        with_context("test_api_core_accounts_staking_with_specific_params_stake_address", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let stake_address: Option<&str> = Some("stake1u8a9qstrmj4rvc3k5z8fems7f0j2vztz8det2klgakhfc8ce79fma");
            let result = cbi.core.accounts.staking_(stake_address, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_accounts_staking_with_specific_params_stake_address");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_accounts_delegations_with_specific_params_stake_address() {
        with_context("test_api_core_accounts_delegations_with_specific_params_stake_address", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let stake_address: Option<&str> = Some("stake1u8a9qstrmj4rvc3k5z8fems7f0j2vztz8det2klgakhfc8ce79fma");
            let result = cbi.core.accounts.delegations_(stake_address, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_accounts_delegations_with_specific_params_stake_address");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_accounts_registrations_with_specific_params_stake_address() {
        with_context("test_api_core_accounts_registrations_with_specific_params_stake_address", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let stake_address: Option<&str> = Some("stake1u8a9qstrmj4rvc3k5z8fems7f0j2vztz8det2klgakhfc8ce79fma");
            let result = cbi.core.accounts.registrations_(stake_address, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_accounts_registrations_with_specific_params_stake_address");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_accounts_withdrawals_with_specific_params_stake_address() {
        with_context("test_api_core_accounts_withdrawals_with_specific_params_stake_address", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let stake_address: Option<&str> = Some("stake1u9frlh9lvpdjva46ge0yc4c8zg5e0d37ch42zyyrzmu2hygnmy4xc");
            let result = cbi.core.accounts.withdrawals_(stake_address, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_accounts_withdrawals_with_specific_params_stake_address");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_accounts_mirs_with_specific_params_stake_address() {
        with_context("test_api_core_accounts_mirs_with_specific_params_stake_address", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let stake_address: Option<&str> = Some("stake1uypy44wqjznc5w9ns9gsguz4ta83jekrg9d0wupa7j3zsacwvq5ex");
            let result = cbi.core.accounts.mirs_(stake_address, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_accounts_mirs_with_specific_params_stake_address");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_accounts_addresses_with_specific_params_stake_address() {
        with_context("test_api_core_accounts_addresses_with_specific_params_stake_address", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let stake_address: Option<&str> = Some("stake1u8a9qstrmj4rvc3k5z8fems7f0j2vztz8det2klgakhfc8ce79fma");
            let result = cbi.core.accounts.addresses_(stake_address, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_accounts_addresses_with_specific_params_stake_address");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_accounts_assets_with_specific_params_stake_address() {
        with_context("test_api_core_accounts_assets_with_specific_params_stake_address", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let stake_address: Option<&str> = Some("stake1uyq4f9rye96ywptukdypkdu69gc4sd34hwzd940pxslczhc7n5vyt");
            let result = cbi.core.accounts.assets_(stake_address, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_accounts_assets_with_specific_params_stake_address");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_addresses_info_with_specific_params_address() {
        with_context("test_api_core_addresses_info_with_specific_params_address", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let address: Option<&str> = Some("stake1u8a9qstrmj4rvc3k5z8fems7f0j2vztz8det2klgakhfc8ce79fma");
            let result = cbi.core.addresses.info_(address, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_addresses_info_with_specific_params_address");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_assets_history_with_specific_params_fingerprint() {
        with_context("test_api_core_assets_history_with_specific_params_fingerprint", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let fingerprint: Option<&str> = Some("asset1gqp4wdmclgw2tqmkm3nq7jdstvqpesdj3agnel");
            let result = cbi.core.assets.history_(fingerprint, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_assets_history_with_specific_params_fingerprint");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_assets_transactions_with_specific_params_fingerprint() {
        with_context("test_api_core_assets_transactions_with_specific_params_fingerprint", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let fingerprint: Option<&str> = Some("asset1gqp4wdmclgw2tqmkm3nq7jdstvqpesdj3agnel");
            let result = cbi.core.assets.transactions_(fingerprint, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_assets_transactions_with_specific_params_fingerprint");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_assets_addresses_with_specific_params_fingerprint() {
        with_context("test_api_core_assets_addresses_with_specific_params_fingerprint", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let fingerprint: Option<&str> = Some("asset1gqp4wdmclgw2tqmkm3nq7jdstvqpesdj3agnel");
            let result = cbi.core.assets.addresses_(fingerprint, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_assets_addresses_with_specific_params_fingerprint");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_assets_policies_with_specific_params_policy_hash() {
        with_context("test_api_core_assets_policies_with_specific_params_policy_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let policy_hash: Option<&str> = Some("706e1c53ed984b016f2c0fc79a450fdb572aa21e4e87d6f74d0b6e8a");
            let result = cbi.core.assets.policies_(policy_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_assets_policies_with_specific_params_policy_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_blocks_latest_without_parameters() {
        with_context("test_api_core_blocks_latest_without_parameters", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            
            let result = cbi.core.blocks.latest_(HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_blocks_latest_without_parameters");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_blocks_transactions_with_specific_params_block_hash() {
        with_context("test_api_core_blocks_transactions_with_specific_params_block_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let block_no: Option<i64> = None;
            let block_hash: Option<&str> = Some("89ff1090614105a919c9ccc8bb3914aaef1ddd28214a4d55ff65436d2c9fc0b2");
            let result = cbi.core.blocks.transactions_(block_no, block_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_blocks_transactions_with_specific_params_block_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_blocks_epochs_slots_with_specific_params_epoch_no_slot_no() {
        with_context("test_api_core_blocks_epochs_slots_with_specific_params_epoch_no_slot_no", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let epoch_no: Option<i64> = Some(394);
            let slot_no: Option<i64> = Some(85165743);
            let result = cbi.core.blocks.epochs.slots_(epoch_no, slot_no, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_blocks_epochs_slots_with_specific_params_epoch_no_slot_no");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_blocks_history_prev_with_specific_params_block_no() {
        with_context("test_api_core_blocks_history_prev_with_specific_params_block_no", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let block_no: Option<i64> = Some(10306321);
            let result = cbi.core.blocks.history.prev_(block_no, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_blocks_history_prev_with_specific_params_block_no");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_blocks_history_next_with_specific_params_block_no() {
        with_context("test_api_core_blocks_history_next_with_specific_params_block_no", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let block_no: Option<i64> = Some(10306321);
            let result = cbi.core.blocks.history.next_(block_no, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_blocks_history_next_with_specific_params_block_no");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_blocks_latest_pools_with_specific_params_pool_hash() {
        with_context("test_api_core_blocks_latest_pools_with_specific_params_pool_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let pool_hash: Option<&str> = Some("pool1y24nj4qdkg35nvvnfawukauggsxrxuy74876cplmxsee29w5axc");
            let result = cbi.core.blocks.latest.pools_(pool_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_blocks_latest_pools_with_specific_params_pool_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_blocks_latest_transactions_without_parameters() {
        with_context("test_api_core_blocks_latest_transactions_without_parameters", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            
            let result = cbi.core.blocks.latest.transactions_(HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_blocks_latest_transactions_without_parameters");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_blocks_pools_history_with_specific_params_pool_hash() {
        with_context("test_api_core_blocks_pools_history_with_specific_params_pool_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let pool_hash: Option<&str> = Some("pool1y24nj4qdkg35nvvnfawukauggsxrxuy74876cplmxsee29w5axc");
            let result = cbi.core.blocks.pools.history_(pool_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_blocks_pools_history_with_specific_params_pool_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_epochs_latest_without_parameters() {
        with_context("test_api_core_epochs_latest_without_parameters", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            
            let result = cbi.core.epochs.latest_(HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_epochs_latest_without_parameters");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_epochs_params_with_specific_params_epoch_no() {
        with_context("test_api_core_epochs_params_with_specific_params_epoch_no", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let epoch_no: Option<i64> = Some(394);
            let result = cbi.core.epochs.params_(epoch_no, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_epochs_params_with_specific_params_epoch_no");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_epochs_params_latest_without_parameters() {
        with_context("test_api_core_epochs_params_latest_without_parameters", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            
            let result = cbi.core.epochs.params.latest_(HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_epochs_params_latest_without_parameters");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_epochs_stakes_pools_with_specific_params_pool_hash() {
        with_context("test_api_core_epochs_stakes_pools_with_specific_params_pool_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let pool_hash: Option<&str> = Some("pool1y24nj4qdkg35nvvnfawukauggsxrxuy74876cplmxsee29w5axc");
            let epoch_no: Option<i64> = None;
            let result = cbi.core.epochs.stakes.pools_(pool_hash, epoch_no, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_epochs_stakes_pools_with_specific_params_pool_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_epochs_stakes_pools_with_specific_params_epoch_no_pool_hash() {
        with_context("test_api_core_epochs_stakes_pools_with_specific_params_epoch_no_pool_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let pool_hash: Option<&str> = Some("pool1y24nj4qdkg35nvvnfawukauggsxrxuy74876cplmxsee29w5axc");
            let epoch_no: Option<i64> = Some(394);
            let result = cbi.core.epochs.stakes.pools_(pool_hash, epoch_no, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_epochs_stakes_pools_with_specific_params_epoch_no_pool_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_epochs_latest_stakes_pools_with_specific_params_pool_hash() {
        with_context("test_api_core_epochs_latest_stakes_pools_with_specific_params_pool_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let pool_hash: Option<&str> = Some("pool1y24nj4qdkg35nvvnfawukauggsxrxuy74876cplmxsee29w5axc");
            let result = cbi.core.epochs.latest.stakes.pools_(pool_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_epochs_latest_stakes_pools_with_specific_params_pool_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_pools_hashes_without_parameters() {
        with_context("test_api_core_pools_hashes_without_parameters", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            
            let result = cbi.core.pools.hashes_(HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_pools_hashes_without_parameters");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_pools_metadata_with_specific_params_pool_id() {
        with_context("test_api_core_pools_metadata_with_specific_params_pool_id", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let pool_id: Option<i64> = Some(4268);
            let result = cbi.core.pools.metadata_(pool_id, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_pools_metadata_with_specific_params_pool_id");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_pools_offlinedata_with_specific_params_pool_id() {
        with_context("test_api_core_pools_offlinedata_with_specific_params_pool_id", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let pool_id: Option<i64> = Some(4268);
            let ticker: Option<&str> = None;
            let result = cbi.core.pools.offlinedata_(pool_id, ticker, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_pools_offlinedata_with_specific_params_pool_id");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_pools_offlinedata_with_specific_params_ticker() {
        with_context("test_api_core_pools_offlinedata_with_specific_params_ticker", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let pool_id: Option<i64> = None;
            let ticker: Option<&str> = Some("ADACT");
            let result = cbi.core.pools.offlinedata_(pool_id, ticker, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_pools_offlinedata_with_specific_params_ticker");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_pools_offlinefetcherrors_with_specific_params_pool_id() {
        with_context("test_api_core_pools_offlinefetcherrors_with_specific_params_pool_id", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let pool_id: Option<i64> = Some(4268);
            let result = cbi.core.pools.offlinefetcherrors_(pool_id, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_pools_offlinefetcherrors_with_specific_params_pool_id");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_pools_updates_with_specific_params_pool_id() {
        with_context("test_api_core_pools_updates_with_specific_params_pool_id", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let pool_id: Option<i64> = Some(4268);
            let vrf_key_hash: Option<&str> = None;
            let result = cbi.core.pools.updates_(pool_id, vrf_key_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_pools_updates_with_specific_params_pool_id");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_pools_updates_with_specific_params_vrf_key_hash() {
        with_context("test_api_core_pools_updates_with_specific_params_vrf_key_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let pool_id: Option<i64> = None;
            let vrf_key_hash: Option<&str> = Some("9be345bcbcb0cf0559b1135467fd2e4c78c741898cdf8bcb737b2dc5122632df");
            let result = cbi.core.pools.updates_(pool_id, vrf_key_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_pools_updates_with_specific_params_vrf_key_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_pools_metadata_hashes_with_specific_params_meta_hash() {
        with_context("test_api_core_pools_metadata_hashes_with_specific_params_meta_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let meta_hash: Option<&str> = Some("42771b05b30f180890980613b3147f6bb797fe1f8a83e92d39a3135ec9559ea8");
            let result = cbi.core.pools.metadata.hashes_(meta_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_pools_metadata_hashes_with_specific_params_meta_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_pools_metadata_offlinedata_with_specific_params_meta_hash() {
        with_context("test_api_core_pools_metadata_offlinedata_with_specific_params_meta_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let meta_hash: Option<&str> = Some("42771b05b30f180890980613b3147f6bb797fe1f8a83e92d39a3135ec9559ea8");
            let result = cbi.core.pools.metadata.offlinedata_(meta_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_pools_metadata_offlinedata_with_specific_params_meta_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_pools_relays_updates_with_specific_params_update_id() {
        with_context("test_api_core_pools_relays_updates_with_specific_params_update_id", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let update_id: Option<i64> = Some(1);
            let vrf_key_hash: Option<&str> = None;
            let result = cbi.core.pools.relays.updates_(update_id, vrf_key_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_pools_relays_updates_with_specific_params_update_id");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_pools_relays_updates_with_specific_params_vrf_key_hash() {
        with_context("test_api_core_pools_relays_updates_with_specific_params_vrf_key_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let update_id: Option<i64> = None;
            let vrf_key_hash: Option<&str> = Some("9be345bcbcb0cf0559b1135467fd2e4c78c741898cdf8bcb737b2dc5122632df");
            let result = cbi.core.pools.relays.updates_(update_id, vrf_key_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_pools_relays_updates_with_specific_params_vrf_key_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_transactions_utxos_with_specific_params_transaction_hash() {
        with_context("test_api_core_transactions_utxos_with_specific_params_transaction_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let transaction_hash: Option<&str> = Some("5f6f72b00ae982492823fb541153e6c2afc9cb7231687f2a5d82a994f61764a0");
            let result = cbi.core.transactions.utxos_(transaction_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_transactions_utxos_with_specific_params_transaction_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_transactions_stake_address_registrations_with_specific_params_transaction_hash() {
        with_context("test_api_core_transactions_stake_address_registrations_with_specific_params_transaction_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let transaction_hash: Option<&str> = Some("13919fc14338f13fa10497293f709f9c12c6275c5b38baa0c60786ffdd51bebb");
            let result = cbi.core.transactions.stake_address_registrations_(transaction_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_transactions_stake_address_registrations_with_specific_params_transaction_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_transactions_stake_address_delegations_with_specific_params_transaction_hash() {
        with_context("test_api_core_transactions_stake_address_delegations_with_specific_params_transaction_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let transaction_hash: Option<&str> = Some("e963b50c5a1078f0fbe11c375d047af3a1b2112538ed6cf852809ebbf4dd8440");
            let result = cbi.core.transactions.stake_address_delegations_(transaction_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_transactions_stake_address_delegations_with_specific_params_transaction_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_transactions_withdrawals_with_specific_params_transaction_hash() {
        with_context("test_api_core_transactions_withdrawals_with_specific_params_transaction_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let transaction_hash: Option<&str> = Some("cb44c5dd07ab3fee81f05ddd3e4596d2664e6c0ae77bccf99d1c9605dd01808d");
            let result = cbi.core.transactions.withdrawals_(transaction_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_transactions_withdrawals_with_specific_params_transaction_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_transactions_treasury_with_specific_params_transaction_hash() {
        with_context("test_api_core_transactions_treasury_with_specific_params_transaction_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let transaction_hash: Option<&str> = Some("0bc50b20e16268419048790f6ae3667a1480418dd9faed543bc0e8ca32ea7a08");
            let result = cbi.core.transactions.treasury_(transaction_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_transactions_treasury_with_specific_params_transaction_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_transactions_reserves_with_specific_params_transaction_hash() {
        with_context("test_api_core_transactions_reserves_with_specific_params_transaction_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let transaction_hash: Option<&str> = Some("27dff3f43c460e779e35eff505f5f159c4283a8221b31ee17cdcd5b31ad221ba");
            let result = cbi.core.transactions.reserves_(transaction_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_transactions_reserves_with_specific_params_transaction_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_transactions_param_proposals_with_specific_params_transaction_hash() {
        with_context("test_api_core_transactions_param_proposals_with_specific_params_transaction_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let transaction_hash: Option<&str> = Some("62c3c13187423c47f629e6187f36fbd61a9ba1d05d101588340cfbfdf47b22d2");
            let result = cbi.core.transactions.param_proposals_(transaction_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_transactions_param_proposals_with_specific_params_transaction_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_transactions_retiring_pools_with_specific_params_transaction_hash() {
        with_context("test_api_core_transactions_retiring_pools_with_specific_params_transaction_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let transaction_hash: Option<&str> = Some("0d8eadd3bd58bd1a34641ea4100de509f081fe5dd7ecd33d7da52cbeb8e93494");
            let result = cbi.core.transactions.retiring_pools_(transaction_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_transactions_retiring_pools_with_specific_params_transaction_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_transactions_updating_pools_with_specific_params_transaction_hash() {
        with_context("test_api_core_transactions_updating_pools_with_specific_params_transaction_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let transaction_hash: Option<&str> = Some("37b67370c0e71b6e15d6d5f564a5069461e472a26e6f46a813743458285aef8d");
            let result = cbi.core.transactions.updating_pools_(transaction_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_transactions_updating_pools_with_specific_params_transaction_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_transactions_metadata_with_specific_params_transaction_hash() {
        with_context("test_api_core_transactions_metadata_with_specific_params_transaction_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let transaction_hash: Option<&str> = Some("6b85afe3fc01c6d3503a5dac8343b56b67f504bb2399deba8b09f8024790b9c4");
            let result = cbi.core.transactions.metadata_(transaction_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_transactions_metadata_with_specific_params_transaction_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_transactions_assetmints_with_specific_params_transaction_hash() {
        with_context("test_api_core_transactions_assetmints_with_specific_params_transaction_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let transaction_hash: Option<&str> = Some("5f6f72b00ae982492823fb541153e6c2afc9cb7231687f2a5d82a994f61764a0");
            let result = cbi.core.transactions.assetmints_(transaction_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_transactions_assetmints_with_specific_params_transaction_hash");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_api_core_transactions_redeemers_with_specific_params_transaction_hash() {
        with_context("test_api_core_transactions_redeemers_with_specific_params_transaction_hash", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let transaction_hash: Option<&str> = Some("e584995ed133ae25e5c918d794efa415e10352b0d0e08aa02a196bbd605b9e69");
            let result = cbi.core.transactions.redeemers_(transaction_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_api_core_transactions_redeemers_with_specific_params_transaction_hash");
            Ok(())
        }).await;
    }

}
