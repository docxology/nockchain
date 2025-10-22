//! Example: Starting a Nockchain miner
//!
//! This example demonstrates how to start a Nockchain miner with key management.

use docxology::{config::MinerConfig, miner::start_miner};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("Starting Nockchain miner example...");

    // Create miner configuration
    let mut config = MinerConfig::default();

    // Enable mining
    config.enabled = true;

    // Set mining key (in practice, this would come from your wallet)
    config.pubkey = Some("your_mining_public_key_here".to_string());

    // Configure mining threads (adjust based on your CPU)
    config.threads = 4;

    // Optionally set a custom target (advanced users)
    // config.target = Some("custom_target_value".to_string());

    println!("Miner configuration: {:?}", config);

    // Start the miner
    let miner_handle = start_miner(config).await?;

    println!("Miner started successfully!");
    println!("Mining with {} threads", miner_handle.config().threads);
    println!("Press Ctrl+C to stop mining...");

    // Wait for shutdown signal
    tokio::signal::ctrl_c().await?;

    println!("Shutting down miner...");
    miner_handle.shutdown().await?;

    println!("Miner stopped successfully!");
    Ok(())
}
