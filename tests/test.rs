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
    writeln!(file, "{}", outcome).expect("Unable to write to log file");
}

fn save_response_to_file(response: Value, test_name: &str) {
    let output_dir = UNIT_TEST_OUTPUT_DIR.lock().unwrap();
    let output_path = Path::new(&*output_dir).join(format!("{}.json", test_name));
    let mut file = fs::File::create(output_path).expect("Failed to create file");
    let json_pretty = to_string_pretty(&response).expect("Failed to serialize JSON");
    file.write_all(json_pretty.as_bytes()).expect("Failed to write to file");
}

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
    async fn test_bi_addresses_stats3() {
        with_context("test_bi_addresses_stats3", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;

            let address = Some("stake1u8a9qstrmj4rvc3k5z8fems7f0j2vztz8det2klgakhfc8ce79fma");
            let mut options = HashMap::new();
            // options.insert("page_no", "5");
            options.insert("page_size", "2");
            // options.insert("order", "desc");
            options.insert("query", "orderby=epoch_no asc");

            // let result = cbi.bi.addresses.stats_(Some(address), options).await.unwrap();
            let result = cbi.bi.addresses.stats_(address, options).await.unwrap();
            println!("Results: {}", result);
            save_response_to_file(result, "test_bi_addresses_stats3");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_core_blocks_latest() {
        with_context("test_core_blocks_latest", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;

            let mut options = HashMap::new();

            let result = cbi.core.blocks.latest_(options).await.unwrap();
            println!("Results: {}", result);
            save_response_to_file(result, "test_core_blocks_latest");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_core_blocks_transactions_with_specific_params_block_no_1() {
        with_context("test_core_blocks_transactions_with_specific_params_block_no_1", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let block_no: Option<i64> = Some(8931769);
            let block_hash: Option<&str> = None;
            let result = cbi.core.blocks.transactions_(block_no, block_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_core_blocks_transactions_with_specific_params_block_no_1");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_core_blocks_transactions_with_specific_params_block_hash_1() {
        with_context("test_core_blocks_transactions_with_specific_params_block_hash_1", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let block_no: Option<i64> = None;
            let block_hash: Option<&str> = Some("89ff1090614105a919c9ccc8bb3914aaef1ddd28214a4d55ff65436d2c9fc0b2");
            let result = cbi.core.blocks.transactions_(block_no, block_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_core_blocks_transactions_with_specific_params_block_hash_1");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_core_epochs_stakes_pools_with_specific_params_epoch_no_pool_hash_1() {
        with_context("test_core_epochs_stakes_pools_with_specific_params_epoch_no_pool_hash_1", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let pool_hash: Option<&str> = Some("pool1y24nj4qdkg35nvvnfawukauggsxrxuy74876cplmxsee29w5axc");
            let epoch_no: Option<i64> = Some(394);
            let result = cbi.core.epochs.stakes.pools_(pool_hash, epoch_no, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_core_epochs_stakes_pools_with_specific_params_epoch_no_pool_hash_1");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_core_epochsstakes_with_specific_params_odata_1() {
        with_context("test_core_epochsstakes_with_specific_params_odata_1", || async {
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
            save_response_to_file(result, "test_core_epochsstakes_with_specific_params_odata_1");
            Ok(())
        }).await;
    }

    #[async_test]
    async fn test_core_pools_relays_updates_with_specific_params_update_id_1() {
        with_context("test_core_pools_relays_updates_with_specific_params_update_id_1", || async {
            let (api_key, api_secret, network) = get_environment_variable();
            let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;
            let update_id: Option<i64> = Some(1);
            let vrf_key_hash: Option<&str> = None;
            let result = cbi.core.pools.relays.updates_(update_id, vrf_key_hash, HashMap::new()).await.unwrap_or_else(|e| {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            });
            println!("Results: {}", result);
            save_response_to_file(result, "test_core_pools_relays_updates_with_specific_params_update_id_1");
            Ok(())
        }).await;
    }

    // 8931769
    // 8415364
}
