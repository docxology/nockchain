//! Configuration types for Nockchain orchestrator
//!
//! Provides typed configuration structures that can be loaded from environment variables,
//! configuration files, or created programmatically.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Configuration for a Nockchain node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Data directory for the node
    pub data_dir: Option<PathBuf>,

    /// Network configuration
    pub network: NetworkConfig,

    /// Mining configuration (if this node should mine)
    pub mining: Option<MinerConfig>,

    /// API configuration
    pub api: ApiConfig,

    /// Logging configuration
    pub logging: LoggingConfig,

    /// Custom configuration overrides
    pub custom: HashMap<String, String>,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            data_dir: None,
            network: NetworkConfig::default(),
            mining: None,
            api: ApiConfig::default(),
            logging: LoggingConfig::default(),
            custom: HashMap::new(),
        }
    }
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Listen address for P2P communication
    pub listen_addr: String,

    /// Bootstrap peers to connect to
    pub bootstrap_peers: Vec<String>,

    /// Whether to enable UPnP port mapping
    pub enable_upnp: bool,

    /// Maximum number of peers
    pub max_peers: usize,

    /// Peer port (UDP)
    pub peer_port: u16,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            listen_addr: "/ip4/0.0.0.0/udp/0/quic-v1".to_string(),
            bootstrap_peers: vec![],
            enable_upnp: true,
            max_peers: 50,
            peer_port: 0, // Use any available port
        }
    }
}

/// Mining configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerConfig {
    /// Public key for mining rewards
    pub pubkey: Option<String>,

    /// Private key for mining (if available)
    pub privkey: Option<String>,

    /// Number of mining threads
    pub threads: usize,

    /// Whether to enable mining
    pub enabled: bool,

    /// Mining difficulty target
    pub target: Option<String>,
}

impl Default for MinerConfig {
    fn default() -> Self {
        Self {
            pubkey: None,
            privkey: None,
            threads: num_cpus::get(),
            enabled: false,
            target: None,
        }
    }
}

/// API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// Whether to enable public API server
    pub enable_public_api: bool,

    /// Public API bind address
    pub public_api_addr: String,

    /// Private API bind address
    pub private_api_addr: String,

    /// CORS origins for public API
    pub cors_origins: Vec<String>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            enable_public_api: false,
            public_api_addr: "0.0.0.0:8080".to_string(),
            private_api_addr: "127.0.0.1:8081".to_string(),
            cors_origins: vec!["*".to_string()],
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,

    /// Whether to enable JSON formatting
    pub json: bool,

    /// Whether to enable file logging
    pub file_logging: bool,

    /// Log file path (if file logging enabled)
    pub log_file: Option<PathBuf>,

    /// Maximum log file size in MB
    pub max_file_size: usize,

    /// Maximum number of log files to keep
    pub max_files: usize,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            json: false,
            file_logging: false,
            log_file: None,
            max_file_size: 10,
            max_files: 5,
        }
    }
}

/// Wallet configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConfig {
    /// Wallet data directory
    pub data_dir: Option<PathBuf>,

    /// gRPC endpoint for wallet operations
    pub grpc_endpoint: String,

    /// Connection timeout in seconds
    pub timeout_secs: u64,

    /// Retry configuration
    pub retry: RetryConfig,
}

impl Default for WalletConfig {
    fn default() -> Self {
        Self {
            data_dir: None,
            grpc_endpoint: "https://nockchain-api.zorp.io".to_string(),
            timeout_secs: 30,
            retry: RetryConfig::default(),
        }
    }
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: usize,

    /// Initial delay between retries in milliseconds
    pub initial_delay_ms: u64,

    /// Maximum delay between retries in milliseconds
    pub max_delay_ms: u64,

    /// Multiplier for exponential backoff
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 30000,
            backoff_multiplier: 2.0,
        }
    }
}

impl NodeConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::Environment::with_prefix("NOCKCHAIN"))
            .build()?;

        let mut config: Self = settings.try_deserialize()?;

        // Apply defaults for missing values
        if config.data_dir.is_none() {
            config.data_dir = Some(dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("./data"))
                .join("nockchain"));
        }

        Ok(config)
    }

    /// Load configuration from file
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::from(path.as_ref()))
            .add_source(config::Environment::with_prefix("NOCKCHAIN"))
            .build()?;

        let mut config: Self = settings.try_deserialize()?;

        // Apply defaults for missing values
        if config.data_dir.is_none() {
            config.data_dir = Some(dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("./data"))
                .join("nockchain"));
        }

        Ok(config)
    }

    /// Save configuration to file
    pub fn to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
