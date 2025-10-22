//! # Docxology: Thin Orchestrator for Nockchain
//!
//! Docxology provides both thin wrappers and high-level convenience flows for interacting
//! with Nockchain components including nodes, miners, wallets, and gRPC APIs.
//!
//! ## Architecture
//!
//! Docxology exposes a stable API for:
//! - **Node Management**: Start/stop Nockchain nodes with configurable settings
//! - **Mining Operations**: Start/stop miners with key management
//! - **Wallet Operations**: Key generation, balance queries, transaction construction
//! - **gRPC Clients**: Public and private API access
//!
//! ## Error Handling
//!
//! Docxology provides comprehensive error handling with:
//! - **Structured Error Types**: Specific error variants for different failure modes
//! - **Retry Mechanisms**: Automatic retry with exponential backoff
//! - **Recovery Strategies**: Graceful degradation and fallback options
//! - **Context Preservation**: Detailed error context for debugging
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use docxology::{NodeConfig, WalletConfig, start_node, get_balance};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Configure and start a node
//! let config = NodeConfig::default();
//! let node_handle = start_node(config).await?;
//!
//! // Create wallet and check balance
//! let wallet_config = WalletConfig::default();
//! let balance = get_balance(wallet_config, "your_address").await?;
//! println!("Balance: {}", balance);
//!
//! // Stop the node
//! node_handle.shutdown().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Features
//!
//! - **Thin Wrappers**: Direct access to Nockchain internals with minimal abstraction
//! - **High-level Flows**: Convenient methods that handle common workflows
//! - **Robust Error Handling**: Comprehensive error types and recovery mechanisms
//! - **Python Bindings**: Optional PyO3 integration for Python usage
//! - **Comprehensive Testing**: Unit, integration, and end-to-end test coverage

#![warn(missing_docs, clippy::missing_docs_in_private_items)]
#![cfg_attr(docsrs, feature(doc_cfg))]

// Re-export commonly used types
pub use crate::{
    config::{NodeConfig, WalletConfig, MinerConfig},
    node::{start_node, NodeHandle, NodeError},
    miner::{start_miner, MinerHandle, MinerError},
    wallet::{WalletManager, WalletError, create_wallet, import_keys, export_keys},
    grpc::{GrpcClient, GrpcError, public_client, private_client},
};

// Module declarations
pub mod config;
pub mod error;
pub mod node;
pub mod miner;
pub mod wallet;
pub mod grpc;

// High-level convenience functions
pub mod flows {
    //! High-level convenience flows that combine multiple operations

    use super::*;
    use anyhow::Result;

    /// Set up and start a miner with a new wallet
    ///
    /// This is a convenience function that handles the complete flow of:
    /// 1. Creating a new wallet
    /// 2. Setting up mining configuration
    /// 3. Starting the miner
    ///
    /// # Arguments
    /// * `node_config` - Configuration for the Nockchain node
    /// * `miner_config` - Configuration for the miner
    ///
    /// # Returns
    /// A tuple of (NodeHandle, MinerHandle, WalletManager) for managing the services
    pub async fn setup_and_start_miner(
        node_config: config::NodeConfig,
        miner_config: config::MinerConfig,
    ) -> Result<(node::NodeHandle, miner::MinerHandle, wallet::WalletManager)> {
        // Start the node first
        let node_handle = node::start_node(node_config).await?;

        // Create a new wallet
        let wallet = wallet::create_wallet(wallet::WalletConfig::default()).await?;

        // Configure miner with wallet keys
        let mut miner_config = miner_config;
        miner_config.pubkey = Some(wallet.get_public_key());

        // Start the miner
        let miner_handle = miner::start_miner(miner_config).await?;

        Ok((node_handle, miner_handle, wallet))
    }

    /// Create and send a transaction in one flow
    ///
    /// This convenience function handles:
    /// 1. Building a transaction
    /// 2. Signing it with wallet keys
    /// 3. Sending it via gRPC
    /// 4. Waiting for acceptance
    ///
    /// # Arguments
    /// * `wallet` - The wallet to use for signing
    /// * `recipient` - The recipient address
    /// * `amount` - The amount to send
    /// * `grpc_client` - gRPC client for sending
    ///
    /// # Returns
    /// The transaction ID if successful
    pub async fn create_and_send_transaction(
        wallet: &wallet::WalletManager,
        recipient: &str,
        amount: u64,
        grpc_client: &grpc::GrpcClient,
    ) -> Result<String> {
        // Build transaction
        let tx = wallet.build_transaction(recipient, amount).await?;

        // Sign transaction
        let signed_tx = wallet.sign_transaction(&tx).await?;

        // Send via gRPC
        let tx_id = grpc_client.send_transaction(&signed_tx).await?;

        Ok(tx_id)
    }

    /// Get wallet balance using public API
    ///
    /// Convenience function that creates a gRPC client and queries balance
    ///
    /// # Arguments
    /// * `address` - The address to check balance for
    /// * `grpc_endpoint` - Optional gRPC endpoint (defaults to public)
    ///
    /// # Returns
    /// The balance amount
    pub async fn get_balance(address: &str, grpc_endpoint: Option<&str>) -> Result<u64> {
        let client = grpc::public_client(grpc_endpoint.unwrap_or("https://nockchain-api.zorp.io"));
        let balance = client.get_balance(address).await?;
        Ok(balance)
    }
}

// Python bindings (when the "python" feature is enabled)
#[cfg(feature = "python")]
mod python_bindings {
    use super::*;
    use pyo3::prelude::*;

    /// Python bindings for the docxology crate
    #[pymodule]
    fn _docxology(m: &Bound<'_, PyModule>) -> PyResult<()> {
        // Register classes
        m.add_class::<config::NodeConfig>()?;
        m.add_class::<config::WalletConfig>()?;
        m.add_class::<config::MinerConfig>()?;
        m.add_class::<node::NodeHandle>()?;
        m.add_class::<miner::MinerHandle>()?;
        m.add_class::<wallet::WalletManager>()?;
        m.add_class::<grpc::GrpcClient>()?;

        // Register functions
        m.add_function(wrap_pyfunction!(node::start_node_py, m)?)?;
        m.add_function(wrap_pyfunction!(miner::start_miner_py, m)?)?;
        m.add_function(wrap_pyfunction!(wallet::create_wallet_py, m)?)?;
        m.add_function(wrap_pyfunction!(grpc::public_client_py, m)?)?;

        Ok(())
    }
}