//! Configuration management for nockit
//! 
//! Handles loading, saving, and managing configuration files for nockchain operations.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

/// Main configuration structure for nockit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NockitConfig {
    /// Nockchain configuration
    pub nockchain: NockchainConfig,
    /// Wallet configuration
    pub wallet: WalletConfig,
    /// Mining configuration
    pub mining: MiningConfig,
    /// Network configuration
    pub network: NetworkConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NockchainConfig {
    /// Path to nockchain binary
    pub binary_path: Option<PathBuf>,
    /// Default data directory
    pub data_dir: PathBuf,
    /// Default bind address
    pub bind_address: String,
    /// Default peer port
    pub peer_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConfig {
    /// Path to wallet binary
    pub binary_path: Option<PathBuf>,
    /// Default wallet directory
    pub wallet_dir: PathBuf,
    /// Default key backup directory
    pub backup_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningConfig {
    /// Default mining public key
    pub default_pubkey: Option<String>,
    /// Mining difficulty target
    pub difficulty_target: Option<u64>,
    /// Mining statistics retention period (days)
    pub stats_retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Bootstrap peers
    pub bootstrap_peers: Vec<String>,
    /// Connection timeout (seconds)
    pub connection_timeout: u64,
    /// Maximum number of peers
    pub max_peers: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,
    /// Log format (json, pretty, compact)
    pub format: String,
    /// Log rotation size (MB)
    pub rotation_size_mb: u64,
    /// Log retention period (days)
    pub retention_days: u32,
}

impl Default for NockitConfig {
    fn default() -> Self {
        Self {
            nockchain: NockchainConfig {
                binary_path: None,
                data_dir: PathBuf::from(".data.nockchain"),
                bind_address: "/ip4/0.0.0.0/udp/0/quic-v1".to_string(),
                peer_port: 0,
            },
            wallet: WalletConfig {
                binary_path: None,
                wallet_dir: PathBuf::from(".nockchain-wallet"),
                backup_dir: PathBuf::from("wallet-backups"),
            },
            mining: MiningConfig {
                default_pubkey: None,
                difficulty_target: None,
                stats_retention_days: 30,
            },
            network: NetworkConfig {
                bootstrap_peers: vec![],
                connection_timeout: 30,
                max_peers: 50,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "pretty".to_string(),
                rotation_size_mb: 100,
                retention_days: 7,
            },
        }
    }
}

impl NockitConfig {
    /// Load configuration from file, creating default if it doesn't exist
    pub fn load_or_create<P: AsRef<Path>>(config_dir: P) -> Result<Self> {
        let config_path = config_dir.as_ref().join("config.toml");
        
        if config_path.exists() {
            Self::load(&config_path)
        } else {
            let config = Self::default();
            config.save(&config_path)?;
            Ok(config)
        }
    }
    
    /// Load configuration from file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;
        
        toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.as_ref().display()))
    }
    
    /// Save configuration to file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize configuration")?;
        
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
        }
        
        fs::write(&path, content)
            .with_context(|| format!("Failed to write config file: {}", path.as_ref().display()))?;
        
        Ok(())
    }
    
    /// Update mining public key
    pub fn set_mining_pubkey(&mut self, pubkey: String) {
        self.mining.default_pubkey = Some(pubkey);
    }
    
    /// Get mining public key from config or environment
    pub fn get_mining_pubkey(&self) -> Option<String> {
        self.mining.default_pubkey.clone()
            .or_else(|| std::env::var("MINING_PUBKEY").ok())
    }
    
    /// Get log level from config or environment
    pub fn get_log_level(&self) -> String {
        std::env::var("RUST_LOG")
            .unwrap_or_else(|_| self.logging.level.clone())
    }
}

/// Environment variable management
pub struct EnvManager;

impl EnvManager {
    /// Load environment variables from .env file
    pub fn load_env_file<P: AsRef<Path>>(path: P) -> Result<()> {
        if path.as_ref().exists() {
            dotenv::from_path(path)
                .context("Failed to load .env file")?;
        }
        Ok(())
    }
    
    /// Set environment variable for nockchain operations
    pub fn set_nockchain_env(config: &NockitConfig) {
        if let Some(pubkey) = &config.mining.default_pubkey {
            std::env::set_var("MINING_PUBKEY", pubkey);
        }
        
        std::env::set_var("RUST_LOG", config.get_log_level());
        std::env::set_var("MINIMAL_LOG_FORMAT", "true");
    }
    
    /// Get all nockchain-related environment variables
    pub fn get_nockchain_env() -> Vec<(String, String)> {
        let env_vars = [
            "RUST_LOG",
            "MINIMAL_LOG_FORMAT", 
            "MINING_PUBKEY",
            "RUST_BACKTRACE",
        ];
        
        env_vars.iter()
            .filter_map(|&var| {
                std::env::var(var).ok().map(|val| (var.to_string(), val))
            })
            .collect()
    }
} 