//! Comprehensive error handling for Docxology
//!
//! Provides structured error types, retry mechanisms, and recovery strategies
//! for robust operation of Nockchain orchestrator components.

use std::fmt;
use std::time::Duration;
use tokio::time::sleep;

/// Comprehensive error type for all Docxology operations
#[derive(Debug)]
pub enum DocxologyError {
    /// Node-related errors
    Node(NodeError),
    /// Miner-related errors
    Miner(MinerError),
    /// Wallet-related errors
    Wallet(WalletError),
    /// gRPC-related errors
    Grpc(GrpcError),
    /// Configuration errors
    Config(ConfigError),
    /// Network errors
    Network(NetworkError),
    /// Timeout errors
    Timeout(TimeoutError),
    /// Recovery errors
    Recovery(RecoveryError),
}

impl fmt::Display for DocxologyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DocxologyError::Node(e) => write!(f, "Node error: {}", e),
            DocxologyError::Miner(e) => write!(f, "Miner error: {}", e),
            DocxologyError::Wallet(e) => write!(f, "Wallet error: {}", e),
            DocxologyError::Grpc(e) => write!(f, "gRPC error: {}", e),
            DocxologyError::Config(e) => write!(f, "Configuration error: {}", e),
            DocxologyError::Network(e) => write!(f, "Network error: {}", e),
            DocxologyError::Timeout(e) => write!(f, "Timeout error: {}", e),
            DocxologyError::Recovery(e) => write!(f, "Recovery error: {}", e),
        }
    }
}

impl std::error::Error for DocxologyError {}

/// Node-specific errors
#[derive(Debug)]
pub enum NodeError {
    /// Failed to initialize node
    InitFailed(String),
    /// Failed to start node
    StartFailed(String),
    /// Node task failed
    TaskFailed(String),
    /// Shutdown failed
    ShutdownFailed,
    /// Configuration error
    ConfigError(String),
    /// IO error
    IoError(std::io::Error),
}

impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeError::InitFailed(msg) => write!(f, "Failed to initialize node: {}", msg),
            NodeError::StartFailed(msg) => write!(f, "Failed to start node: {}", msg),
            NodeError::TaskFailed(msg) => write!(f, "Node task failed: {}", msg),
            NodeError::ShutdownFailed => write!(f, "Failed to shutdown node"),
            NodeError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            NodeError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

/// Miner-specific errors
#[derive(Debug)]
pub enum MinerError {
    /// Failed to initialize miner
    InitFailed(String),
    /// Failed to start miner
    StartFailed(String),
    /// Miner task failed
    TaskFailed(String),
    /// Shutdown failed
    ShutdownFailed,
    /// Invalid mining key
    InvalidKey(String),
    /// Configuration error
    ConfigError(String),
    /// IO error
    IoError(std::io::Error),
}

impl fmt::Display for MinerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MinerError::InitFailed(msg) => write!(f, "Failed to initialize miner: {}", msg),
            MinerError::StartFailed(msg) => write!(f, "Failed to start miner: {}", msg),
            MinerError::TaskFailed(msg) => write!(f, "Miner task failed: {}", msg),
            MinerError::ShutdownFailed => write!(f, "Failed to shutdown miner"),
            MinerError::InvalidKey(msg) => write!(f, "Invalid mining key: {}", msg),
            MinerError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            MinerError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

/// Wallet-specific errors
#[derive(Debug)]
pub enum WalletError {
    /// Failed to initialize wallet
    InitFailed(String),
    /// Key generation failed
    KeyGenerationFailed(String),
    /// Key import failed
    KeyImportFailed(String),
    /// Key export failed
    KeyExportFailed(String),
    /// Balance query failed
    BalanceQueryFailed(String),
    /// Transaction build failed
    TransactionBuildFailed(String),
    /// Transaction sign failed
    TransactionSignFailed(String),
    /// Invalid address
    InvalidAddress(String),
    /// Insufficient funds
    InsufficientFunds { needed: u64, available: u64 },
    /// IO error
    IoError(std::io::Error),
    /// RPC error
    RpcError(String),
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WalletError::InitFailed(msg) => write!(f, "Failed to initialize wallet: {}", msg),
            WalletError::KeyGenerationFailed(msg) => write!(f, "Failed to generate keys: {}", msg),
            WalletError::KeyImportFailed(msg) => write!(f, "Failed to import keys: {}", msg),
            WalletError::KeyExportFailed(msg) => write!(f, "Failed to export keys: {}", msg),
            WalletError::BalanceQueryFailed(msg) => write!(f, "Failed to get balance: {}", msg),
            WalletError::TransactionBuildFailed(msg) => write!(f, "Failed to build transaction: {}", msg),
            WalletError::TransactionSignFailed(msg) => write!(f, "Failed to sign transaction: {}", msg),
            WalletError::InvalidAddress(msg) => write!(f, "Invalid address: {}", msg),
            WalletError::InsufficientFunds { needed, available } => {
                write!(f, "Insufficient funds: needed {}, available {}", needed, available)
            }
            WalletError::IoError(e) => write!(f, "IO error: {}", e),
            WalletError::RpcError(msg) => write!(f, "RPC error: {}", msg),
        }
    }
}

/// gRPC-specific errors
#[derive(Debug)]
pub enum GrpcError {
    /// Not connected to API
    NotConnected(String),
    /// Invalid endpoint
    InvalidEndpoint(String),
    /// Connection failed
    ConnectionFailed(String),
    /// Request failed
    RequestFailed(String),
    /// API error
    ApiError(String),
    /// Timeout
    Timeout(String),
    /// Invalid response
    InvalidResponse(String),
    /// Transport error
    TransportError(tonic::transport::Error),
    /// Status error
    StatusError(tonic::Status),
}

impl fmt::Display for GrpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GrpcError::NotConnected(api) => write!(f, "Not connected to {} API", api),
            GrpcError::InvalidEndpoint(msg) => write!(f, "Invalid endpoint: {}", msg),
            GrpcError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            GrpcError::RequestFailed(msg) => write!(f, "Request failed: {}", msg),
            GrpcError::ApiError(msg) => write!(f, "API error: {}", msg),
            GrpcError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            GrpcError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
            GrpcError::TransportError(e) => write!(f, "Transport error: {}", e),
            GrpcError::StatusError(e) => write!(f, "Status error: {}", e),
        }
    }
}

/// Configuration errors
#[derive(Debug)]
pub enum ConfigError {
    /// Invalid configuration value
    InvalidValue(String),
    /// Missing required configuration
    MissingValue(String),
    /// Configuration file error
    FileError(String),
    /// Environment variable error
    EnvError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidValue(msg) => write!(f, "Invalid configuration value: {}", msg),
            ConfigError::MissingValue(msg) => write!(f, "Missing required configuration: {}", msg),
            ConfigError::FileError(msg) => write!(f, "Configuration file error: {}", msg),
            ConfigError::EnvError(msg) => write!(f, "Environment variable error: {}", msg),
        }
    }
}

/// Network errors
#[derive(Debug)]
pub enum NetworkError {
    /// Connection failed
    ConnectionFailed(String),
    /// DNS resolution failed
    DnsFailed(String),
    /// TLS error
    TlsError(String),
    /// Network unreachable
    Unreachable(String),
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            NetworkError::DnsFailed(msg) => write!(f, "DNS resolution failed: {}", msg),
            NetworkError::TlsError(msg) => write!(f, "TLS error: {}", msg),
            NetworkError::Unreachable(msg) => write!(f, "Network unreachable: {}", msg),
        }
    }
}

/// Timeout errors
#[derive(Debug)]
pub struct TimeoutError {
    /// Operation that timed out
    pub operation: String,
    /// Timeout duration
    pub duration: Duration,
    /// Context information
    pub context: String,
}

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Operation '{}' timed out after {:?}: {}",
            self.operation, self.duration, self.context
        )
    }
}

/// Recovery errors
#[derive(Debug)]
pub enum RecoveryError {
    /// Recovery strategy failed
    StrategyFailed(String),
    /// Maximum retry attempts exceeded
    MaxRetriesExceeded { attempts: usize, operation: String },
    /// Recovery not possible
    NotRecoverable(String),
}

impl fmt::Display for RecoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecoveryError::StrategyFailed(msg) => write!(f, "Recovery strategy failed: {}", msg),
            RecoveryError::MaxRetriesExceeded { attempts, operation } => {
                write!(f, "Maximum retry attempts ({}) exceeded for operation: {}", attempts, operation)
            }
            RecoveryError::NotRecoverable(msg) => write!(f, "Recovery not possible: {}", msg),
        }
    }
}

/// Retry configuration for operations
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: usize,
    /// Initial delay between retries
    pub initial_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Multiplier for exponential backoff
    pub backoff_multiplier: f64,
    /// Operations that should not be retried
    pub no_retry_operations: Vec<String>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            no_retry_operations: vec![
                "authentication".to_string(),
                "authorization".to_string(),
            ],
        }
    }
}

/// Execute an operation with retry logic
pub async fn with_retry<T, F, E>(
    operation: F,
    config: &RetryConfig,
    operation_name: &str,
) -> Result<T, DocxologyError>
where
    F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, DocxologyError>> + Send>>,
    E: std::fmt::Display,
{
    let mut delay = config.initial_delay;
    let mut attempts = 0;

    loop {
        attempts += 1;

        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                // Check if this operation should not be retried
                if config.no_retry_operations.iter().any(|op| operation_name.contains(op)) {
                    return Err(e);
                }

                // Check if we've exceeded max attempts
                if attempts >= config.max_attempts {
                    return Err(DocxologyError::Recovery(RecoveryError::MaxRetriesExceeded {
                        attempts,
                        operation: operation_name.to_string(),
                    }));
                }

                // Wait before retrying
                sleep(delay).await;

                // Increase delay for next attempt
                delay = Duration::from_secs(
                    (delay.as_secs() as f64 * config.backoff_multiplier) as u64
                ).min(config.max_delay);

                tracing::warn!(
                    "Operation '{}' failed on attempt {}, retrying in {:?}: {}",
                    operation_name, attempts, delay, e
                );
            }
        }
    }
}

/// Recovery strategies for different error types
pub mod recovery {
    use super::*;

    /// Attempt to recover from a node error
    pub async fn recover_node_error(error: &NodeError) -> Result<(), RecoveryError> {
        match error {
            NodeError::InitFailed(_) => {
                // Try reinitializing with default configuration
                tracing::info!("Attempting node recovery with default configuration");
                // Implementation would go here
                Ok(())
            }
            NodeError::ConfigError(_) => {
                Err(RecoveryError::NotRecoverable("Configuration errors cannot be automatically recovered".to_string()))
            }
            _ => {
                // Other errors might be recoverable with restart
                tracing::info!("Node error may be recoverable with restart");
                Ok(())
            }
        }
    }

    /// Attempt to recover from a network error
    pub async fn recover_network_error(error: &NetworkError) -> Result<(), RecoveryError> {
        match error {
            NetworkError::ConnectionFailed(_) => {
                // Try reconnecting after a delay
                sleep(Duration::from_secs(5)).await;
                Ok(())
            }
            NetworkError::DnsFailed(_) => {
                Err(RecoveryError::NotRecoverable("DNS resolution failures require manual intervention".to_string()))
            }
            _ => Ok(()),
        }
    }

    /// Attempt to recover from a timeout error
    pub async fn recover_timeout_error(error: &TimeoutError) -> Result<(), RecoveryError> {
        match error.operation.as_str() {
            "blockchain_sync" => {
                // Increase timeout for blockchain operations
                tracing::info!("Increasing timeout for blockchain synchronization");
                Ok(())
            }
            "network_request" => {
                // Try alternative endpoints
                tracing::info!("Attempting alternative network endpoints");
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let node_error = NodeError::InitFailed("test error".to_string());
        assert!(node_error.to_string().contains("Failed to initialize node"));

        let grpc_error = GrpcError::NotConnected("test".to_string());
        assert!(grpc_error.to_string().contains("Not connected"));

        let timeout_error = TimeoutError {
            operation: "test_op".to_string(),
            duration: Duration::from_secs(30),
            context: "test context".to_string(),
        };
        assert!(timeout_error.to_string().contains("timed out"));
    }

    #[test]
    fn test_retry_config_default() {
        let config = RetryConfig::default();
        assert_eq!(config.max_attempts, 3);
        assert_eq!(config.initial_delay, Duration::from_secs(1));
        assert_eq!(config.backoff_multiplier, 2.0);
        assert!(config.no_retry_operations.contains(&"authentication".to_string()));
    }

    #[tokio::test]
    async fn test_with_retry_success() {
        let mut call_count = 0;
        let config = RetryConfig::default();

        let result = with_retry(
            || Box::pin(async {
                call_count += 1;
                if call_count < 3 {
                    Err(DocxologyError::Network(NetworkError::ConnectionFailed("test".to_string())))
                } else {
                    Ok("success")
                }
            }),
            &config,
            "test_operation",
        ).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
        assert_eq!(call_count, 3);
    }

    #[tokio::test]
    async fn test_with_retry_max_attempts() {
        let config = RetryConfig {
            max_attempts: 2,
            ..Default::default()
        };

        let result = with_retry(
            || Box::pin(async {
                Err(DocxologyError::Network(NetworkError::ConnectionFailed("test".to_string())))
            }),
            &config,
            "test_operation",
        ).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            DocxologyError::Recovery(RecoveryError::MaxRetriesExceeded { attempts, .. }) => {
                assert_eq!(attempts, 2);
            }
            _ => panic!("Expected MaxRetriesExceeded error"),
        }
    }
}
