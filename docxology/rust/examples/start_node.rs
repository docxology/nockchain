//! Example: Starting a Nockchain node
//!
//! This example demonstrates how to start a Nockchain node with custom configuration.

use docxology::{config::{NodeConfig, LoggingConfig}, node::start_node};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("Starting Nockchain node example...");

    // Create a custom configuration
    let mut config = NodeConfig::default();

    // Set up data directory
    config.data_dir = Some(PathBuf::from("./node_data"));

    // Configure network settings
    config.network.listen_addr = "/ip4/0.0.0.0/udp/0/quic-v1".to_string();
    config.network.bootstrap_peers = vec![
        // Add bootstrap peers here if needed
    ];

    // Configure logging
    config.logging.level = "info".to_string();
    config.logging.file_logging = false;

    // Enable public API
    config.api.enable_public_api = true;
    config.api.public_api_addr = "127.0.0.1:8080".to_string();

    println!("Configuration: {:?}", config);

    // Start the node
    let node_handle = start_node(config).await?;

    println!("Node started successfully!");
    println!("Public API available at: http://127.0.0.1:8080");
    println!("Press Ctrl+C to stop the node...");

    // Wait for shutdown signal
    tokio::signal::ctrl_c().await?;

    println!("Shutting down node...");
    node_handle.shutdown().await?;

    println!("Node stopped successfully!");
    Ok(())
}
