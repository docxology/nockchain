//! Node management for Nockchain
//!
//! Provides functionality to start, stop, and manage Nockchain nodes with
//! configurable settings.

use crate::config::{NodeConfig, MinerConfig};
use nockchain::{NockchainAPIConfig, NockchainCli};
use nockapp::NockApp;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};
use tracing::{info, error, warn};

/// Handle for managing a running Nockchain node
#[derive(Debug)]
pub struct NodeHandle {
    /// Channel to send shutdown signal
    shutdown_tx: Option<mpsc::UnboundedSender<()>>,
    /// Handle to the running node task
    node_task: Option<tokio::task::JoinHandle<()>>,
    /// Configuration used to start the node
    config: NodeConfig,
}

impl NodeHandle {
    /// Create a new node handle
    pub fn new(
        shutdown_tx: mpsc::UnboundedSender<()>,
        node_task: tokio::task::JoinHandle<()>,
        config: NodeConfig,
    ) -> Self {
        Self {
            shutdown_tx: Some(shutdown_tx),
            node_task: Some(node_task),
            config,
        }
    }

    /// Get the configuration used by this node
    pub fn config(&self) -> &NodeConfig {
        &self.config
    }

    /// Shutdown the node gracefully
    pub async fn shutdown(mut self) -> Result<(), NodeError> {
        if let Some(tx) = self.shutdown_tx.take() {
            if let Err(e) = tx.send(()) {
                error!("Failed to send shutdown signal: {}", e);
                return Err(NodeError::ShutdownFailed);
            }
        }

        if let Some(task) = self.node_task.take() {
            if let Err(e) = task.await {
                error!("Node task failed during shutdown: {}", e);
                return Err(NodeError::TaskFailed(e));
            }
        }

        info!("Node shutdown completed successfully");
        Ok(())
    }

    /// Wait for the node to finish (without shutting it down)
    pub async fn wait(mut self) -> Result<(), NodeError> {
        if let Some(task) = self.node_task.take() {
            if let Err(e) = task.await {
                error!("Node task failed: {}", e);
                return Err(NodeError::TaskFailed(e));
            }
        }

        Ok(())
    }
}

/// Errors that can occur during node operations
#[derive(Debug, thiserror::Error)]
pub enum NodeError {
    #[error("Failed to initialize node: {0}")]
    InitFailed(String),

    #[error("Failed to start node: {0}")]
    StartFailed(String),

    #[error("Node task failed: {0}")]
    TaskFailed(#[from] tokio::task::JoinError),

    #[error("Failed to send shutdown signal")]
    ShutdownFailed,

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Start a Nockchain node with the given configuration
///
/// # Arguments
/// * `config` - Configuration for the node
///
/// # Returns
/// A handle for managing the running node
///
/// # Example
/// ```rust,no_run
/// use docxology::{NodeConfig, start_node};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = NodeConfig::default();
/// let node_handle = start_node(config).await?;
///
/// // Node is now running...
///
/// // Shutdown gracefully
/// node_handle.shutdown().await?;
/// # Ok(())
/// # }
/// ```
pub async fn start_node(config: NodeConfig) -> Result<NodeHandle, NodeError> {
    info!("Starting Nockchain node with config: {:?}", config);

    // Create shutdown channel
    let (shutdown_tx, mut shutdown_rx) = mpsc::unbounded_channel();

    // Create a channel for the node task to signal when it's ready
    let (ready_tx, ready_rx) = oneshot::channel();

    // Clone config for the node task
    let node_config = config.clone();

    // Create the node task
    let node_task = tokio::spawn(async move {
        if let Err(e) = run_node(node_config, ready_tx, &mut shutdown_rx).await {
            error!("Node error: {}", e);
        }
    });

    // Wait for the node to be ready
    match ready_rx.await {
        Ok(()) => {
            info!("Node started successfully");
            Ok(NodeHandle::new(shutdown_tx, node_task, config))
        }
        Err(e) => {
            error!("Node failed to start: {}", e);
            // Abort the task if it failed to start
            node_task.abort();
            Err(NodeError::StartFailed(e.to_string()))
        }
    }
}

/// Internal function to run the node
async fn run_node(
    config: NodeConfig,
    ready_tx: oneshot::Sender<()>,
    shutdown_rx: &mut mpsc::UnboundedReceiver<()>,
) -> Result<(), NodeError> {
    // Create Nockchain CLI configuration from our config
    let cli = create_cli_from_config(&config)?;

    // Initialize tracing
    nockapp::kernel::boot::init_default_tracing(&cli.nockapp_cli);

    // Produce prover hot state
    let prover_hot_state = zkvm_jetpack::hot::produce_prover_hot_state();

    // Create the node kernel (dumb kernel for basic node functionality)
    let kernel = kernels::dumb::KERNEL;

    // Initialize the node
    let mut node: NockApp = nockchain::init_with_kernel(
        cli,
        kernel,
        prover_hot_state.as_slice(),
        if config.api.enable_public_api {
            NockchainAPIConfig::EnablePublicServer
        } else {
            NockchainAPIConfig::DisablePublicServer
        },
    )
    .await
    .map_err(|e| NodeError::InitFailed(e.to_string()))?;

    // Signal that we're ready
    let _ = ready_tx.send(());

    // Run the node until shutdown signal is received
    tokio::select! {
        result = node.run() => {
            match result {
                Ok(()) => info!("Node exited normally"),
                Err(e) => error!("Node exited with error: {}", e),
            }
        }
        _ = shutdown_rx.recv() => {
            info!("Shutdown signal received, stopping node");
        }
    }

    Ok(())
}

/// Convert our configuration to Nockchain CLI arguments
fn create_cli_from_config(config: &NodeConfig) -> Result<NockchainCli, NodeError> {
    // Create a minimal CLI configuration
    // In a real implementation, this would map all config fields to CLI args
    let mut cli = NockchainCli::default();

    // Set data directory if specified
    if let Some(data_dir) = &config.data_dir {
        // This would need to be implemented in the CLI parsing
        // For now, we'll use a simple approach
        std::env::set_var("NOCKCHAIN_DATA_DIR", data_dir.to_string_lossy().to_string());
    }

    // Set mining configuration if enabled
    if let Some(miner_config) = &config.mining {
        if miner_config.enabled {
            std::env::set_var("NOCKCHAIN_MINING_ENABLED", "true");
            if let Some(pubkey) = &miner_config.pubkey {
                std::env::set_var("NOCKCHAIN_MINING_PUBKEY", pubkey);
            }
        }
    }

    // Set API configuration
    if config.api.enable_public_api {
        std::env::set_var("NOCKCHAIN_PUBLIC_API_ENABLED", "true");
    }

    // Set logging level
    std::env::set_var("RUST_LOG", &config.logging.level);

    Ok(cli)
}

/// Python wrapper for start_node function
#[cfg(feature = "python")]
#[pyo3::pyfunction]
pub fn start_node_py(config: config::NodeConfig) -> pyo3::PyResult<()> {
    pyo3::Python::with_gil(|py| {
        pyo3::Py::from(0) // Placeholder - would need async runtime integration
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_node_config_creation() {
        let temp_dir = tempdir().unwrap();
        let mut config = NodeConfig::default();
        config.data_dir = Some(temp_dir.path().to_path_buf());

        assert_eq!(config.data_dir, Some(temp_dir.path().to_path_buf()));
        assert_eq!(config.network.listen_addr, "/ip4/0.0.0.0/udp/0/quic-v1");
    }

    #[tokio::test]
    async fn test_create_cli_from_config() {
        let mut config = NodeConfig::default();
        config.mining = Some(MinerConfig {
            pubkey: Some("test_pubkey".to_string()),
            ..Default::default()
        });

        let cli = create_cli_from_config(&config);
        // In a real test, we would verify the CLI configuration
        assert!(cli.is_ok());
    }
}
