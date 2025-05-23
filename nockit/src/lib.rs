//! # Nockit - Nockchain Development Toolkit
//!
//! A comprehensive toolkit for nockchain development, monitoring, and operations.
//! Provides modular functionality for logging, wallet management, mining operations,
//! network monitoring, and development utilities.

pub mod config;
pub mod crypto;
pub mod logging;
pub mod mining;
pub mod monitoring;
pub mod network;
pub mod setup;
pub mod utils;
pub mod wallet;
pub mod bench;

pub use config::*;
pub use crypto::*;
pub use logging::*;
pub use mining::*;
pub use monitoring::*;
pub use network::*;
pub use setup::*;
pub use utils::*;
pub use wallet::*;
pub use bench::*;

/// Common result type used throughout nockit
pub type Result<T> = anyhow::Result<T>;

/// Version information for nockit
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default configuration directory for nockit
pub const NOCKIT_CONFIG_DIR: &str = ".nockit";

/// Default log directory for nockit
pub const NOCKIT_LOG_DIR: &str = "logs"; 