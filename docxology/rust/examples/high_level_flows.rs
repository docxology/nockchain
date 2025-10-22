//! Example: High-level convenience flows
//!
//! This example demonstrates the high-level convenience functions that combine
//! multiple operations into simple workflows.

use docxology::{
    config::{NodeConfig, MinerConfig},
    flows::{setup_and_start_miner, get_balance},
    grpc::GrpcClient,
};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("Starting high-level flows example...");

    // Example 1: Set up and start a miner (complete workflow)
    println!("\n=== Example 1: Setup and Start Miner ===");

    let node_config = NodeConfig {
        data_dir: Some(PathBuf::from("./miner_node_data")),
        ..Default::default()
    };

    let miner_config = MinerConfig {
        enabled: true,
        threads: 2,
        ..Default::default()
    };

    match setup_and_start_miner(node_config, miner_config).await {
        Ok((node_handle, miner_handle, wallet)) => {
            println!("Miner setup completed successfully!");
            println!("Node is running at: {:?}", node_handle.config().data_dir);
            println!("Miner is using {} threads", miner_handle.config().threads);
            println!("Wallet public key: {}", wallet.get_public_key().await?);

            // Let it run for a bit
            println!("Mining for 10 seconds...");
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

            // Clean shutdown
            miner_handle.shutdown().await?;
            node_handle.shutdown().await?;

            println!("Miner stopped successfully!");
        }
        Err(e) => {
            println!("Miner setup failed (expected if dependencies not fully implemented): {}", e);
            println!("This is expected in the current development state.");
        }
    }

    // Example 2: Get balance using public API
    println!("\n=== Example 2: Get Balance ===");

    let test_address = "test_address_placeholder";

    match get_balance(test_address, None).await {
        Ok(balance) => {
            println!("Balance for {}: {} units", test_address, balance);
        }
        Err(e) => {
            println!("Could not get balance (expected if no network connection): {}", e);
            println!("This would work with a live Nockchain network.");
        }
    }

    // Example 3: Create and send transaction (would need wallet and node)
    println!("\n=== Example 3: Create and Send Transaction ===");
    println!("This example would demonstrate:");
    println!("1. Creating a wallet");
    println!("2. Building a transaction");
    println!("3. Signing the transaction");
    println!("4. Sending it via gRPC");
    println!("5. Waiting for acceptance");
    println!();
    println!("The flows module provides convenient functions for these operations.");
    println!("See the flows module documentation for more details.");

    println!("\nHigh-level flows example completed!");

    Ok(())
}
