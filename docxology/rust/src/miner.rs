//! Miner management for Nockchain
//!
//! Provides functionality to start, stop, and manage Nockchain miners with
//! key management and configuration.

use crate::config::MinerConfig;
use nockchain::mining::{Miner, MinerConfig as NockchainMinerConfig};
use nockchain_types::SchnorrPubkey;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};
use tracing::{info, error, warn};

/// Handle for managing a running Nockchain miner
#[derive(Debug)]
pub struct MinerHandle {
    /// Channel to send shutdown signal
    shutdown_tx: Option<mpsc::UnboundedSender<()>>,
    /// Handle to the running miner task
    miner_task: Option<tokio::task::JoinHandle<()>>,
    /// Configuration used to start the miner
    config: MinerConfig,
    /// Current mining statistics
    stats: MinerStats,
}

impl MinerHandle {
    /// Create a new miner handle
    pub fn new(
        shutdown_tx: mpsc::UnboundedSender<()>,
        miner_task: tokio::task::JoinHandle<()>,
        config: MinerConfig,
    ) -> Self {
        Self {
            shutdown_tx: Some(shutdown_tx),
            miner_task: Some(miner_task),
            config,
            stats: MinerStats::default(),
        }
    }

    /// Get the configuration used by this miner
    pub fn config(&self) -> &MinerConfig {
        &self.config
    }

    /// Get current mining statistics
    pub fn stats(&self) -> &MinerStats {
        &self.stats
    }

    /// Shutdown the miner gracefully
    pub async fn shutdown(mut self) -> Result<(), MinerError> {
        if let Some(tx) = self.shutdown_tx.take() {
            if let Err(e) = tx.send(()) {
                error!("Failed to send miner shutdown signal: {}", e);
                return Err(MinerError::ShutdownFailed);
            }
        }

        if let Some(task) = self.miner_task.take() {
            if let Err(e) = task.await {
                error!("Miner task failed during shutdown: {}", e);
                return Err(MinerError::TaskFailed(e));
            }
        }

        info!("Miner shutdown completed successfully");
        Ok(())
    }

    /// Wait for the miner to finish (without shutting it down)
    pub async fn wait(mut self) -> Result<(), MinerError> {
        if let Some(task) = self.miner_task.take() {
            if let Err(e) = task.await {
                error!("Miner task failed: {}", e);
                return Err(MinerError::TaskFailed(e));
            }
        }

        Ok(())
    }
}

/// Mining statistics
#[derive(Debug, Clone, Default)]
pub struct MinerStats {
    /// Total number of hashes computed
    pub hashes: u64,

    /// Number of blocks found
    pub blocks_found: u64,

    /// Current hashrate in hashes per second
    pub hashrate: f64,

    /// Average hashrate since start
    pub avg_hashrate: f64,

    /// Time since mining started
    pub uptime_seconds: u64,
}

/// Errors that can occur during miner operations
#[derive(Debug, thiserror::Error)]
pub enum MinerError {
    #[error("Failed to initialize miner: {0}")]
    InitFailed(String),

    #[error("Failed to start miner: {0}")]
    StartFailed(String),

    #[error("Miner task failed: {0}")]
    TaskFailed(#[from] tokio::task::JoinError),

    #[error("Failed to send shutdown signal")]
    ShutdownFailed,

    #[error("Invalid mining key: {0}")]
    InvalidKey(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Start a Nockchain miner with the given configuration
///
/// # Arguments
/// * `config` - Configuration for the miner
///
/// # Returns
/// A handle for managing the running miner
///
/// # Example
/// ```rust,no_run
/// use docxology::{MinerConfig, start_miner};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut config = MinerConfig::default();
/// config.pubkey = Some("your_mining_pubkey".to_string());
/// config.enabled = true;
/// config.threads = 4;
///
/// let miner_handle = start_miner(config).await?;
///
/// // Miner is now running...
///
/// // Shutdown gracefully
/// miner_handle.shutdown().await?;
/// # Ok(())
/// # }
/// ```
pub async fn start_miner(config: MinerConfig) -> Result<MinerHandle, MinerError> {
    info!("Starting Nockchain miner with config: {:?}", config);

    // Validate configuration
    if config.pubkey.is_none() && config.privkey.is_none() {
        return Err(MinerError::ConfigError(
            "Either pubkey or privkey must be provided for mining".to_string(),
        ));
    }

    // Create shutdown channel
    let (shutdown_tx, mut shutdown_rx) = mpsc::unbounded_channel();

    // Create a channel for the miner task to signal when it's ready
    let (ready_tx, ready_rx) = oneshot::channel();

    // Clone config for the miner task
    let miner_config = config.clone();

    // Create the miner task
    let miner_task = tokio::spawn(async move {
        if let Err(e) = run_miner(miner_config, ready_tx, &mut shutdown_rx).await {
            error!("Miner error: {}", e);
        }
    });

    // Wait for the miner to be ready
    match ready_rx.await {
        Ok(()) => {
            info!("Miner started successfully");
            Ok(MinerHandle::new(shutdown_tx, miner_task, config))
        }
        Err(e) => {
            error!("Miner failed to start: {}", e);
            // Abort the task if it failed to start
            miner_task.abort();
            Err(MinerError::StartFailed(e.to_string()))
        }
    }
}

/// Internal function to run the miner
async fn run_miner(
    config: MinerConfig,
    ready_tx: oneshot::Sender<()>,
    shutdown_rx: &mut mpsc::UnboundedReceiver<()>,
) -> Result<(), MinerError> {
    // Parse the mining key
    let pubkey = if let Some(pubkey_str) = &config.pubkey {
        SchnorrPubkey::from_base58(pubkey_str)
            .map_err(|e| MinerError::InvalidKey(format!("Invalid pubkey: {}", e)))?
    } else {
        return Err(MinerError::InvalidKey("No public key provided".to_string()));
    };

    // Create Nockchain miner configuration
    let mut miner_config = NockchainMinerConfig::default();
    miner_config.pubkey = Some(pubkey);
    miner_config.threads = config.threads;

    if let Some(target) = &config.target {
        // Parse target if provided (simplified - would need proper parsing)
        miner_config.target = Some(target.clone());
    }

    // Create the miner
    let mut miner = Miner::new(miner_config);

    // Signal that we're ready
    let _ = ready_tx.send(());

    // Mining loop
    loop {
        tokio::select! {
            _ = shutdown_rx.recv() => {
                info!("Shutdown signal received, stopping miner");
                break;
            }
            result = miner.mine_one_block() => {
                match result {
                    Ok(block) => {
                        info!("Found block! Height: {}, Hash: {}",
                              block.height, block.hash);
                        // Update stats would go here
                    }
                    Err(e) => {
                        warn!("Mining error: {}", e);
                        // Continue mining on errors
                    }
                }
            }
        }
    }

    Ok(())
}

/// Python wrapper for start_miner function
#[cfg(feature = "python")]
#[pyo3::pyfunction]
pub fn start_miner_py(config: MinerConfig) -> pyo3::PyResult<()> {
    pyo3::Python::with_gil(|py| {
        pyo3::Py::from(0) // Placeholder - would need async runtime integration
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_miner_config_validation() {
        let config = MinerConfig::default();
        assert!(!config.enabled);

        let mut config = MinerConfig::default();
        config.enabled = true;
        // This should fail validation since no keys are provided
        // In a real test, we'd test the validation logic
    }

    #[test]
    fn test_miner_stats() {
        let stats = MinerStats::default();
        assert_eq!(stats.hashes, 0);
        assert_eq!(stats.blocks_found, 0);
        assert_eq!(stats.hashrate, 0.0);
    }
}
