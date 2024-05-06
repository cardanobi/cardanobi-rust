use dotenv::dotenv;
use std::env;
use cardanobi_rust::{APIClient, CardanoBI}; 
use std::collections::HashMap;

fn get_environment_variable() -> (String, String, String) {
    // dotenv().ok();  // Load environment variables from .env file
    dotenv::from_filename("tests/.env").ok(); // Specify the path to your .env file

    println!("START - get_environment_variable");

    let api_key = env::var("CBI_API_KEY").expect("CBI_API_KEY must be set");
    let api_secret = env::var("CBI_API_SECRET").expect("CBI_API_SECRET must be set");
    let network = env::var("CBI_ENV").unwrap_or_else(|_| "mainnet".to_string());

    println!("get_environment_variable, api_key:{}, api_secret:{}, network:{}",api_key,api_secret,network);

    (api_key, api_secret, network)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test as async_test;

    #[async_test]
    async fn test_get_access_token() {
        println!("START - test_get_access_token");

        // Call get_environment_variable() to load configurations
        let (api_key, api_secret, network) = get_environment_variable();

        let mut client = APIClient::new(Some(&api_key), Some(&api_secret), Some(&network)).expect("Failed to create APIClient");

        match client.get_access_token().await {
            Ok(_) => {
                assert!(client.access_token.is_some(), "Access token should be set");
                println!("Access token obtained: {}", client.access_token.unwrap());
            },
            Err(e) => panic!("Failed to get access token: {:?}", e),
        }
    }

    #[async_test]
    async fn test_bi_addresses_stats() {
        println!("START - test_bi_addresses_stats");
        let (api_key, api_secret, network) = get_environment_variable();
        // let CBI = CardanoBI::new(Some(&api_key), Some(&api_secret), Some(&network)).await;

        // Await here to get the CardanoBI instance from the Future
        let cbi_result = CardanoBI::new(Some(&api_key), Some(&api_secret), Some(&network)).await;

        // Check if we got an Ok result and extract CardanoBI or handle error
        let CBI = match cbi_result {
            Ok(cbi) => cbi,
            Err(e) => {
                println!("Failed to initialize CardanoBI: {:?}", e);
                return;
            }
        };

        let address = "stake1u8a9qstrmj4rvc3k5z8fems7f0j2vztz8det2klgakhfc8ce79fma";
        let options = HashMap::new();  // Populate this as needed based on `BiAddresses::stats_` requirements
        match CBI.bi.addresses.stats_(Some(address), options).await {
            Ok(result) => {
                println!("Results: {}", result);
                // println!("Stats result!");

            },
            Err(e) => {
                println!("Error fetching address stats: {:?}", e);
                assert!(false, "Failed to fetch address stats");
            }
        }
    }

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
    async fn test_bi_addresses_stats2() {
        println!("START - test_bi_addresses_stats");
        let (api_key, api_secret, network) = get_environment_variable();
        let cbi = initialize_cardanobi(&api_key, &api_secret, &network).await;

        let address = "stake1u8a9qstrmj4rvc3k5z8fems7f0j2vztz8det2klgakhfc8ce79fma";
        let options = HashMap::new(); // Populate based on requirements
        let result = cbi.bi.addresses.stats_(Some(address), options).await.unwrap();
        println!("Results: {}", result);
    }
}

