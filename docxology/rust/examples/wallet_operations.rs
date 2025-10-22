//! Example: Wallet operations with Nockchain
//!
//! This example demonstrates key generation, balance checking, and transaction building.

use docxology::{
    wallet::{create_wallet, KeySource, WalletConfig},
    grpc::public_client,
};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("Starting wallet operations example...");

    // Create a wallet
    let mut config = WalletConfig::default();
    config.data_dir = Some(PathBuf::from("./wallet_data"));

    let wallet = create_wallet(config).await?;

    println!("Wallet created successfully!");

    // Generate a new key pair
    let keypair = wallet.keygen().await?;

    println!("Generated new key pair:");
    println!("  Public key: {}", keypair.public_key);
    println!("  Private key: {}", keypair.private_key);
    if let Some(seed_phrase) = keypair.seed_phrase {
        println!("  Seed phrase: {}", seed_phrase);
    }

    // Check balance (will likely be 0 for new addresses)
    println!("\nChecking balance...");
    match wallet.get_balance(&keypair.public_key).await {
        Ok(balance) => {
            println!("Balance for {}: {} units", keypair.public_key, balance.balance);
        }
        Err(e) => {
            println!("Could not check balance (expected if no node running): {}", e);
        }
    }

    // Build a sample transaction
    println!("\nBuilding sample transaction...");
    match wallet.build_transaction("recipient_address_here", 1000).await {
        Ok(tx) => {
            println!("Transaction built:");
            println!("  ID: {}", tx.id);
            println!("  Outputs: {} units to {}", tx.outputs[0].amount, tx.outputs[0].recipient);
            println!("  Fee: {} units", tx.fee);

            // Sign the transaction
            match wallet.sign_transaction(&tx).await {
                Ok(signed_tx) => {
                    println!("Transaction signed successfully!");
                    println!("  Signature: {}", signed_tx.signature);

                    // In a real scenario, you would send this to the network
                    println!("\nTo send this transaction, you would:");
                    println!("  1. Connect to a Nockchain node");
                    println!("  2. Call grpc_client.send_transaction(&signed_tx)");
                    println!("  3. Wait for confirmation");
                }
                Err(e) => {
                    println!("Could not sign transaction (expected if wallet not fully configured): {}", e);
                }
            }
        }
        Err(e) => {
            println!("Could not build transaction (expected if wallet not connected): {}", e);
        }
    }

    // Demonstrate importing keys
    println!("\nImporting watch-only key...");
    let watch_key = "imported_watch_only_public_key";
    let import_result = wallet.import_keys(KeySource::WatchOnly(watch_key.to_string())).await;

    match import_result {
        Ok(()) => {
            println!("Successfully imported watch-only key: {}", watch_key);
        }
        Err(e) => {
            println!("Could not import key (expected in current implementation): {}", e);
        }
    }

    println!("\nWallet operations example completed!");
    println!("Note: Many operations require a running Nockchain node to function fully.");

    Ok(())
}
