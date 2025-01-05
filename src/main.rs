mod meteora_pools_models;

use dotenvy::dotenv;
use solana_account_decoder::UiDataSliceConfig;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::env;
use std::error::Error;
use std::str::FromStr;
use tracing::{error, info, instrument};

const METEORA_PROGRAM_ID: &str = "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB";

#[instrument(skip(client))]
async fn get_pool_from_meteora(
    client: &RpcClient,
    token_mint_str: &str,
) -> Result<Vec<Pubkey>, Box<dyn Error + Send + Sync>> {
    info!("Starting pool search for token mint: {}", token_mint_str);

    let token_pubkey = Pubkey::from_str(token_mint_str).expect("Invalid pubkey");
    let data_size = std::mem::size_of::<meteora_pools_models::Pool>() as u64;

    info!("Searching with data size: {} bytes", data_size);

    // Create filters for both token_a_mint and token_b_mint
    let filters_a = vec![
        RpcFilterType::DataSize(data_size),
        RpcFilterType::Memcmp(Memcmp::new(
            40, // offset for token_a_mint
            MemcmpEncodedBytes::Base58(token_pubkey.to_string()),
        )),
    ];

    let filters_b = vec![
        RpcFilterType::DataSize(data_size),
        RpcFilterType::Memcmp(Memcmp::new(
            72, // offset for token_b_mint
            MemcmpEncodedBytes::Base58(token_pubkey.to_string()),
        )),
    ];

    info!("Configured filters for token A and B positions");

    let config_a = RpcProgramAccountsConfig {
        filters: Some(filters_a),
        account_config: RpcAccountInfoConfig {
            commitment: Some(CommitmentConfig::confirmed()),
            encoding: Some(solana_account_decoder::UiAccountEncoding::Base64),
            data_slice: Some(UiDataSliceConfig {
                offset: 0,
                length: 0,
            }),
            min_context_slot: None,
        },
        with_context: None,
        sort_results: Some(true),
    };

    let config_b = RpcProgramAccountsConfig {
        filters: Some(filters_b),
        ..config_a.clone()
    };

    let program_id = Pubkey::from_str(METEORA_PROGRAM_ID).expect("Invalid pubkey");

    // Make both requests
    info!("Fetching pools where token is in position A...");
    let accounts_a = client.get_program_accounts_with_config(&program_id, config_a)?;
    info!("Found {} pools with token in position A", accounts_a.len());

    info!("Fetching pools where token is in position B...");
    let accounts_b = client.get_program_accounts_with_config(&program_id, config_b)?;
    info!("Found {} pools with token in position B", accounts_b.len());

    // Combine and deduplicate results
    let mut pools = vec![];
    let mut seen_pubkeys = std::collections::HashSet::new();

    info!("Processing and deduplicating results...");
    for (pubkey, _) in accounts_a.into_iter().chain(accounts_b) {
        if !seen_pubkeys.contains(&pubkey) {
            seen_pubkeys.insert(pubkey);
            pools.push(pubkey);
        }
    }

    info!("Completed search. Found {} unique pools", pools.len());
    Ok(pools)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load .env file
    dotenv().ok();

    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("Starting Meteora pool scanner");

    // Get RPC URL from environment
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set in environment");

    info!("Connecting to RPC endpoint: {}", rpc_url);
    let client = RpcClient::new(rpc_url);

    let token = "7vsKatZ8BAKXXb16ZZMJyg9X3iLn8Zpq4yBPg8mWBLMd";
    info!("Searching for token (mint: {})", token);

    match get_pool_from_meteora(&client, token).await {
        Ok(pools) => {
            info!("----------------------------------------");
            info!("Search completed successfully");
            info!("Found {} pools", pools.len());
            for pubkey in pools {
                info!("Pool address: {}", pubkey);
            }
            info!("----------------------------------------");
        }
        Err(e) => {
            error!("Failed to fetch pools: {}", e);
            println!("Error: {}", e);
        }
    }

    Ok(())
}
