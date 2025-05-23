# Nockit - Nockchain Development Toolkit

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.5.0-green.svg)](Cargo.toml)

A comprehensive toolkit for nockchain development, monitoring, and operations. Nockit provides a unified interface for managing nockchain nodes, mining operations, wallet functionality, network monitoring, and development workflows.

## 🏗️ Architecture Overview

```mermaid
graph TB
    subgraph "Nockit Toolkit"
        CLI[nockit CLI]
        LOG[nocklog]
        MON[nockmon]
        SETUP[nocksetup]
    end
    
    subgraph "Core Modules"
        CONFIG[config.rs]
        CRYPTO[crypto.rs]
        WALLET[wallet.rs]
        MINING[mining.rs]
        NETWORK[network.rs]
        MONITOR[monitoring.rs]
        LOGGING[logging.rs]
        UTILS[utils.rs]
        BENCH[bench.rs]
    end
    
    subgraph "External Dependencies"
        NOCKCHAIN[nockchain binary]
        NOCKWALLET[nockchain-wallet]
        SYSTEM[System Resources]
        NETWORK_EXT[Network Peers]
    end
    
    CLI --> CONFIG
    CLI --> WALLET
    CLI --> MINING
    CLI --> NETWORK
    CLI --> MONITOR
    CLI --> LOGGING
    CLI --> UTILS
    CLI --> BENCH
    
    LOG --> LOGGING
    MON --> MONITOR
    SETUP --> CONFIG
    SETUP --> CRYPTO
    
    WALLET --> CRYPTO
    MINING --> NOCKCHAIN
    WALLET --> NOCKWALLET
    NETWORK --> NETWORK_EXT
    MONITOR --> SYSTEM
    
    CONFIG --> CRYPTO
    LOGGING --> SYSTEM
    
    style CLI fill:#e1f5fe
    style CONFIG fill:#f3e5f5
    style CRYPTO fill:#fff3e0
    style NOCKCHAIN fill:#e8f5e8
```

## 🚀 Features

### Core Functionality
- **Configuration Management**: Centralized TOML-based configuration with environment variable integration
- **Wallet Operations**: Key generation, backup, restore, and status checking
- **Mining Management**: Process control, statistics tracking, and performance analysis
- **Network Monitoring**: Connectivity diagnostics, peer management, and traffic analysis
- **System Health**: Real-time monitoring with multiple output formats
- **Development Tools**: Project scaffolding, testing, and build automation
- **Benchmarking**: Performance testing and optimization tools

### Standalone Tools
- **`nockit`**: Main CLI with comprehensive subcommands
- **`nocklog`**: Focused log management and analysis
- **`nockmon`**: Real-time system monitoring
- **`nocksetup`**: Environment initialization and setup

## 🔄 Data Flow Architecture

```mermaid
flowchart TD
    subgraph "User Interface Layer"
        USER[User Commands]
        CLI[CLI Interface]
        TUI[Terminal UI]
    end
    
    subgraph "Application Layer"
        PARSER[Command Parser]
        VALIDATOR[Input Validator]
        EXECUTOR[Command Executor]
    end
    
    subgraph "Business Logic Layer"
        WALLET_LOGIC[Wallet Logic]
        MINING_LOGIC[Mining Logic]
        NETWORK_LOGIC[Network Logic]
        MONITOR_LOGIC[Monitor Logic]
        CONFIG_LOGIC[Config Logic]
    end
    
    subgraph "Data Access Layer"
        CONFIG_STORE[Configuration Store]
        KEY_STORE[Key Store]
        LOG_STORE[Log Store]
        STATS_STORE[Statistics Store]
        CACHE[Memory Cache]
    end
    
    subgraph "External Systems"
        NOCKCHAIN_NODE[Nockchain Node]
        FILESYSTEM[File System]
        NETWORK_PEERS[Network Peers]
        SYSTEM_METRICS[System Metrics]
    end
    
    USER --> CLI
    CLI --> PARSER
    PARSER --> VALIDATOR
    VALIDATOR --> EXECUTOR
    
    EXECUTOR --> WALLET_LOGIC
    EXECUTOR --> MINING_LOGIC
    EXECUTOR --> NETWORK_LOGIC
    EXECUTOR --> MONITOR_LOGIC
    EXECUTOR --> CONFIG_LOGIC
    
    WALLET_LOGIC --> KEY_STORE
    MINING_LOGIC --> STATS_STORE
    NETWORK_LOGIC --> CACHE
    MONITOR_LOGIC --> LOG_STORE
    CONFIG_LOGIC --> CONFIG_STORE
    
    KEY_STORE --> FILESYSTEM
    CONFIG_STORE --> FILESYSTEM
    LOG_STORE --> FILESYSTEM
    STATS_STORE --> FILESYSTEM
    
    MINING_LOGIC --> NOCKCHAIN_NODE
    NETWORK_LOGIC --> NETWORK_PEERS
    MONITOR_LOGIC --> SYSTEM_METRICS
    
    TUI --> MONITOR_LOGIC
    TUI --> LOG_STORE
    
    style USER fill:#e3f2fd
    style CLI fill:#f1f8e9
    style EXECUTOR fill:#fff3e0
    style NOCKCHAIN_NODE fill:#e8f5e8
```

## 📦 Installation

### Prerequisites
- Internet connection for downloading dependencies
- Administrative privileges for system package installation
- Minimum 2GB free disk space

### Installation Flow

```mermaid
flowchart TD
    START([Start Installation])
    CHECK_PREREQ{Check Prerequisites}
    INSTALL_DEPS[Install System Dependencies]
    INSTALL_RUST[Install Rust Toolchain]
    INSTALL_TOOLS[Install Development Tools]
    BUILD_NOCKIT[Build Nockit]
    VERIFY[Verify Installation]
    COMPLETE([Installation Complete])
    
    ERROR_DEPS[Error: Missing Dependencies]
    ERROR_RUST[Error: Rust Installation Failed]
    ERROR_BUILD[Error: Build Failed]
    
    START --> CHECK_PREREQ
    CHECK_PREREQ -->|Pass| INSTALL_DEPS
    CHECK_PREREQ -->|Fail| ERROR_DEPS
    
    INSTALL_DEPS --> INSTALL_RUST
    INSTALL_RUST -->|Success| INSTALL_TOOLS
    INSTALL_RUST -->|Fail| ERROR_RUST
    
    INSTALL_TOOLS --> BUILD_NOCKIT
    BUILD_NOCKIT -->|Success| VERIFY
    BUILD_NOCKIT -->|Fail| ERROR_BUILD
    
    VERIFY --> COMPLETE
    
    ERROR_DEPS --> START
    ERROR_RUST --> INSTALL_DEPS
    ERROR_BUILD --> INSTALL_TOOLS
    
    style START fill:#e8f5e8
    style COMPLETE fill:#e8f5e8
    style ERROR_DEPS fill:#ffebee
    style ERROR_RUST fill:#ffebee
    style ERROR_BUILD fill:#ffebee
```

### Automated Setup (Recommended)

The easiest way to get started is with the automated setup tool:

```bash
# Download and run the setup script (if available)
curl -sSf https://setup.nockchain.com | sh

# Or clone the repository and run setup
git clone https://github.com/nockchain/nockchain.git
cd nockchain/nockit
./setup.sh
```

### Manual Installation

#### Step 1: Install System Dependencies

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev libclang-dev cmake curl wget git
```

**Fedora/RHEL/CentOS:**
```bash
sudo dnf install -y gcc gcc-c++ make cmake pkg-config openssl-devel clang-devel curl wget git
```

**Arch Linux:**
```bash
sudo pacman -Sy base-devel pkg-config openssl clang cmake curl wget git
```

**macOS:**
```bash
# Install Homebrew if not already installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install pkg-config openssl cmake git
xcode-select --install
```

#### Step 2: Install Rust

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install additional components
rustup component add clippy rustfmt rust-src rust-analyzer
```

#### Step 3: Install Development Tools

```bash
# Install useful cargo tools
cargo install cargo-audit cargo-outdated cargo-tree cargo-watch
cargo install cargo-edit cargo-expand cargo-udeps cargo-deny
```

#### Step 4: Build and Install Nockit

```bash
# Clone the repository
git clone https://github.com/nockchain/nockchain.git
cd nockchain/nockit

# Build the toolkit
cargo build --release

# Install binaries
cargo install --path .
```

### Using Nocksetup Tool

The `nocksetup` tool provides comprehensive installation management:

```mermaid
graph TD
    subgraph "Nocksetup Operations"
        SETUP_MAIN[nocksetup]
        INSTALL[install]
        UPDATE[update]
        CHECK[check]
        VERIFY[verify]
        CLEAN[clean]
        CONFIG[config]
    end
    
    subgraph "Installation Targets"
        RUST_INSTALL[Rust Toolchain]
        DEPS_INSTALL[System Dependencies]
        NOCKCHAIN_INSTALL[Nockchain Binaries]
        NOCKIT_INSTALL[Nockit Tools]
    end
    
    subgraph "Configuration"
        ENV_CONFIG[Environment Setup]
        KEY_CONFIG[Key Generation]
        NETWORK_CONFIG[Network Configuration]
        LOG_CONFIG[Logging Configuration]
    end
    
    SETUP_MAIN --> INSTALL
    SETUP_MAIN --> UPDATE
    SETUP_MAIN --> CHECK
    SETUP_MAIN --> VERIFY
    SETUP_MAIN --> CLEAN
    SETUP_MAIN --> CONFIG
    
    INSTALL --> RUST_INSTALL
    INSTALL --> DEPS_INSTALL
    INSTALL --> NOCKCHAIN_INSTALL
    INSTALL --> NOCKIT_INSTALL
    
    CONFIG --> ENV_CONFIG
    CONFIG --> KEY_CONFIG
    CONFIG --> NETWORK_CONFIG
    CONFIG --> LOG_CONFIG
    
    UPDATE --> RUST_INSTALL
    UPDATE --> NOCKCHAIN_INSTALL
    UPDATE --> NOCKIT_INSTALL
    
    style SETUP_MAIN fill:#e1f5fe
    style INSTALL fill:#f3e5f5
    style CONFIG fill:#fff3e0
```

#### Complete Setup
```bash
# Run complete automated setup
nocksetup

# Or with specific options
nocksetup install --generate-keys --non-interactive
```

#### Selective Installation
```bash
# Install only Rust and development tools
nocksetup rust --with-components --with-dev-tools

# Install only system dependencies
nocksetup deps

# Install only nockchain binaries
nocksetup nockchain --from-source

# Configure environment
nocksetup config --mining-pubkey <PUBKEY> --network testnet --log-level debug
```

#### System Checks
```bash
# Check system requirements and current status
nocksetup check

# Verify complete installation
nocksetup verify
```

#### Maintenance
```bash
# Update Rust toolchain
nocksetup update --rust

# Update nockchain binaries
nocksetup update --nockchain

# Update nockit tools
nocksetup update --nockit

# Clean temporary files
nocksetup clean --temp-only

# Remove all configuration (careful!)
nocksetup clean --all
```

### Docker Installation (Alternative)

For containerized environments:

```bash
# Build Docker image
docker build -t nockit .

# Run setup in container
docker run -it --rm -v $(pwd):/workspace nockit nocksetup
```

### Verification

After installation, verify everything is working:

```bash
# Check installed tools
rustc --version
cargo --version
git --version
nockit --version

# Run system check
nocksetup check

# Test functionality
nockit --help
nocklog --help
nockmon --help
```

## 🛠️ Usage

### Command Structure

```mermaid
graph TD
    subgraph "Main CLI Commands"
        NOCKIT[nockit]
        SETUP_CMD[setup]
        WALLET_CMD[wallet]
        MINING_CMD[mining]
        NETWORK_CMD[network]
        LOGS_CMD[logs]
        MONITOR_CMD[monitor]
        DEV_CMD[dev]
        BENCH_CMD[bench]
    end
    
    subgraph "Wallet Operations"
        KEYGEN[keygen]
        STATUS[status]
        BACKUP[backup]
        RESTORE[restore]
        IMPORT[import]
        EXPORT[export]
    end
    
    subgraph "Mining Operations"
        START_MINING[start]
        STOP_MINING[stop]
        MINING_STATUS[status]
        MINING_STATS[stats]
        MINING_CONFIG[config]
    end
    
    subgraph "Network Operations"
        NET_STATUS[status]
        PEERS[peers]
        PING[ping]
        TRAFFIC[traffic]
        TOPOLOGY[topology]
    end
    
    subgraph "Log Operations"
        TAIL[tail]
        SEARCH[search]
        ANALYZE[analyze]
        EXPORT_LOGS[export]
        CLEAN_LOGS[clean]
    end
    
    subgraph "Development Operations"
        INIT[init]
        TEST[test]
        BUILD[build]
        CLEAN_DEV[clean]
        SCAFFOLD[scaffold]
    end
    
    NOCKIT --> SETUP_CMD
    NOCKIT --> WALLET_CMD
    NOCKIT --> MINING_CMD
    NOCKIT --> NETWORK_CMD
    NOCKIT --> LOGS_CMD
    NOCKIT --> MONITOR_CMD
    NOCKIT --> DEV_CMD
    NOCKIT --> BENCH_CMD
    
    WALLET_CMD --> KEYGEN
    WALLET_CMD --> STATUS
    WALLET_CMD --> BACKUP
    WALLET_CMD --> RESTORE
    WALLET_CMD --> IMPORT
    WALLET_CMD --> EXPORT
    
    MINING_CMD --> START_MINING
    MINING_CMD --> STOP_MINING
    MINING_CMD --> MINING_STATUS
    MINING_CMD --> MINING_STATS
    MINING_CMD --> MINING_CONFIG
    
    NETWORK_CMD --> NET_STATUS
    NETWORK_CMD --> PEERS
    NETWORK_CMD --> PING
    NETWORK_CMD --> TRAFFIC
    NETWORK_CMD --> TOPOLOGY
    
    LOGS_CMD --> TAIL
    LOGS_CMD --> SEARCH
    LOGS_CMD --> ANALYZE
    LOGS_CMD --> EXPORT_LOGS
    LOGS_CMD --> CLEAN_LOGS
    
    DEV_CMD --> INIT
    DEV_CMD --> TEST
    DEV_CMD --> BUILD
    DEV_CMD --> CLEAN_DEV
    DEV_CMD --> SCAFFOLD
    
    style NOCKIT fill:#e1f5fe
    style WALLET_CMD fill:#f3e5f5
    style MINING_CMD fill:#fff3e0
    style NETWORK_CMD fill:#e8f5e8
    style LOGS_CMD fill:#fce4ec
    style DEV_CMD fill:#f1f8e9
```

### Configuration Management

```bash
# Initialize configuration
nockit setup

# View current configuration
cat .nockit/config.toml

# Set mining public key
nockit wallet keygen
```

### Wallet Operations

```mermaid
sequenceDiagram
    participant User
    participant Nockit
    participant Crypto
    participant FileSystem
    participant NockchainWallet
    
    User->>Nockit: nockit wallet keygen
    Nockit->>Crypto: generate_keypair()
    Crypto-->>Nockit: keypair
    Nockit->>FileSystem: save_keys()
    Nockit-->>User: keys generated
    
    User->>Nockit: nockit wallet backup
    Nockit->>FileSystem: read_wallet_data()
    Nockit->>Crypto: encrypt_backup()
    Nockit->>FileSystem: save_backup()
    Nockit-->>User: backup created
    
    User->>Nockit: nockit wallet status
    Nockit->>NockchainWallet: check_wallet_status()
    NockchainWallet-->>Nockit: status_info
    Nockit-->>User: wallet status
```

```bash
# Generate new wallet keys
nockit wallet keygen --output my_keys.json

# Check wallet status
nockit wallet status

# Backup wallet keys
nockit wallet backup --output ./backups/

# Restore from backup
nockit wallet restore --input ./backups/wallet_backup_20240101_120000.export
```

### Mining Management

```mermaid
stateDiagram-v2
    [*] --> Stopped
    Stopped --> Starting : start command
    Starting --> Running : process started
    Running --> Stopping : stop command
    Running --> Error : process failed
    Stopping --> Stopped : process stopped
    Error --> Stopped : restart/reset
    
    Running --> Running : monitoring
    Running --> Running : stats collection
    
    note right of Running
        - Hash rate monitoring
        - Block discovery tracking
        - Performance metrics
        - Error logging
    end note
    
    note right of Stopped
        - Process not running
        - No resource usage
        - Statistics preserved
    end note
```

```bash
# Start mining
nockit mining start --pubkey <YOUR_PUBKEY>

# Check mining status
nockit mining status

# Analyze mining performance
nockit mining stats --period 1d

# Stop mining
nockit mining stop
```

### Network Monitoring

```mermaid
graph TD
    subgraph "Network Monitoring Components"
        CONNECTIVITY[Connectivity Check]
        PEER_DISCOVERY[Peer Discovery]
        TRAFFIC_ANALYSIS[Traffic Analysis]
        LATENCY_TEST[Latency Testing]
        TOPOLOGY_MAP[Network Topology]
    end
    
    subgraph "Data Sources"
        NOCKCHAIN_NODE[Nockchain Node]
        SYSTEM_NET[System Network]
        PEER_NODES[Peer Nodes]
        DNS_RESOLVER[DNS Resolver]
    end
    
    subgraph "Outputs"
        STATUS_REPORT[Status Report]
        PEER_LIST[Peer List]
        TRAFFIC_STATS[Traffic Statistics]
        TOPOLOGY_GRAPH[Topology Graph]
        ALERTS[Network Alerts]
    end
    
    CONNECTIVITY --> NOCKCHAIN_NODE
    CONNECTIVITY --> SYSTEM_NET
    CONNECTIVITY --> STATUS_REPORT
    
    PEER_DISCOVERY --> NOCKCHAIN_NODE
    PEER_DISCOVERY --> DNS_RESOLVER
    PEER_DISCOVERY --> PEER_LIST
    
    TRAFFIC_ANALYSIS --> SYSTEM_NET
    TRAFFIC_ANALYSIS --> TRAFFIC_STATS
    
    LATENCY_TEST --> PEER_NODES
    LATENCY_TEST --> STATUS_REPORT
    
    TOPOLOGY_MAP --> PEER_NODES
    TOPOLOGY_MAP --> TOPOLOGY_GRAPH
    
    STATUS_REPORT --> ALERTS
    TRAFFIC_STATS --> ALERTS
    
    style CONNECTIVITY fill:#e3f2fd
    style PEER_DISCOVERY fill:#f1f8e9
    style TRAFFIC_ANALYSIS fill:#fff3e0
    style TOPOLOGY_MAP fill:#fce4ec
```

```bash
# Check network status
nockit network status

# List connected peers
nockit network peers

# Test connectivity
nockit network ping

# Monitor traffic
nockit network traffic --duration 300
```

### Log Management

```bash
# Tail live logs
nockit logs tail --follow

# Search logs
nockit logs search "error|warning" --file nockchain.log

# Analyze log patterns
nockit logs analyze --period 6h

# Export logs
nockit logs export --format json --output logs_export.json

# Clean old logs (standalone tool)
nocklog clean --days 7
```

### System Monitoring

```mermaid
graph TD
    subgraph "System Monitoring Dashboard"
        CPU[CPU Usage]
        MEMORY[Memory Usage]
        DISK[Disk Usage]
        NETWORK_IO[Network I/O]
        PROCESSES[Process Count]
        NOCKCHAIN_HEALTH[Nockchain Health]
    end
    
    subgraph "Data Collection"
        SYSINFO[System Info API]
        PROC_FS[/proc filesystem]
        NOCKCHAIN_API[Nockchain API]
        LOG_ANALYSIS[Log Analysis]
    end
    
    subgraph "Output Formats"
        TABLE[Table Format]
        JSON[JSON Format]
        COMPACT[Compact Format]
        TUI[Terminal UI]
    end
    
    subgraph "Alerting"
        THRESHOLDS[Threshold Monitoring]
        NOTIFICATIONS[Notifications]
        HEALTH_STATUS[Health Status]
    end
    
    CPU --> SYSINFO
    MEMORY --> SYSINFO
    DISK --> SYSINFO
    NETWORK_IO --> PROC_FS
    PROCESSES --> SYSINFO
    NOCKCHAIN_HEALTH --> NOCKCHAIN_API
    NOCKCHAIN_HEALTH --> LOG_ANALYSIS
    
    SYSINFO --> TABLE
    SYSINFO --> JSON
    SYSINFO --> COMPACT
    SYSINFO --> TUI
    
    CPU --> THRESHOLDS
    MEMORY --> THRESHOLDS
    DISK --> THRESHOLDS
    NOCKCHAIN_HEALTH --> THRESHOLDS
    
    THRESHOLDS --> NOTIFICATIONS
    THRESHOLDS --> HEALTH_STATUS
    
    style CPU fill:#ffcdd2
    style MEMORY fill:#c8e6c9
    style DISK fill:#bbdefb
    style NOCKCHAIN_HEALTH fill:#fff9c4
```

```bash
# Real-time monitoring
nockit monitor --interval 5 --format table

# Single health check
nockmon --once --format json

# Compact monitoring
nockmon --format compact --interval 2
```

### Development Utilities

```bash
# Initialize new project
nockit dev init my_nockchain_project

# Run tests
nockit dev test

# Build project
nockit dev build --target release

# Clean artifacts
nockit dev clean
```

### Benchmarking

```mermaid
graph TD
    subgraph "Benchmark Categories"
        CRYPTO_BENCH[Cryptographic Operations]
        NETWORK_BENCH[Network Performance]
        STORAGE_BENCH[Storage I/O]
        MINING_BENCH[Mining Performance]
        SYSTEM_BENCH[System Resources]
    end
    
    subgraph "Benchmark Metrics"
        THROUGHPUT[Throughput]
        LATENCY[Latency]
        CPU_USAGE[CPU Usage]
        MEMORY_USAGE[Memory Usage]
        ERROR_RATE[Error Rate]
    end
    
    subgraph "Output Formats"
        BENCH_TABLE[Table Report]
        BENCH_JSON[JSON Report]
        BENCH_GRAPH[Performance Graphs]
        BENCH_COMPARISON[Historical Comparison]
    end
    
    CRYPTO_BENCH --> THROUGHPUT
    CRYPTO_BENCH --> LATENCY
    NETWORK_BENCH --> THROUGHPUT
    NETWORK_BENCH --> LATENCY
    STORAGE_BENCH --> THROUGHPUT
    MINING_BENCH --> CPU_USAGE
    SYSTEM_BENCH --> MEMORY_USAGE
    
    THROUGHPUT --> BENCH_TABLE
    LATENCY --> BENCH_TABLE
    CPU_USAGE --> BENCH_JSON
    MEMORY_USAGE --> BENCH_JSON
    ERROR_RATE --> BENCH_GRAPH
    
    BENCH_TABLE --> BENCH_COMPARISON
    BENCH_JSON --> BENCH_COMPARISON
    
    style CRYPTO_BENCH fill:#fff3e0
    style NETWORK_BENCH fill:#e8f5e8
    style STORAGE_BENCH fill:#e3f2fd
    style MINING_BENCH fill:#fce4ec
```

```bash
# Run comprehensive benchmarks
nockit bench --all

# Run specific benchmark category
nockit bench crypto --iterations 1000

# Compare with baseline
nockit bench --compare baseline.json

# Export benchmark results
nockit bench --output benchmark_results.json
```

## 📚 API Documentation

### Module Relationships

```mermaid
classDiagram
    class NockitConfig {
        +load_or_create(path: &str) NockitConfig
        +save(path: &str) Result
        +set_mining_pubkey(pubkey: String)
        +get_nockchain_config() NockchainConfig
        +get_wallet_config() WalletConfig
        +get_mining_config() MiningConfig
    }
    
    class KeyPair {
        +generate() Result~KeyPair~
        +from_bytes(bytes: &[u8]) Result~KeyPair~
        +public_key() PublicKey
        +private_key() PrivateKey
        +sign(message: &[u8]) Signature
    }
    
    class WalletManager {
        +generate_keys(output: Option~&Path~) Result
        +check_status(pubkey: Option~&str~) Result~WalletStatus~
        +backup_wallet(output: &Path) Result
        +restore_wallet(input: &Path) Result
    }
    
    class MiningManager {
        +start_mining(pubkey: &str, difficulty: Option~u64~) Result
        +stop_mining() Result
        +get_status() Result~MiningStatus~
        +get_statistics(period: &str) Result~MiningStats~
    }
    
    class NetworkMonitor {
        +check_status() Result~NetworkStatus~
        +list_peers() Result~Vec~PeerInfo~~
        +ping_peers() Result~Vec~PingResult~~
        +monitor_traffic(duration: u64) Result~TrafficStats~
    }
    
    class SystemMonitor {
        +collect_health() Result~SystemHealth~
        +run_monitor(interval: u64, format: &str) Result
        +get_process_info() Result~ProcessInfo~
    }
    
    class LogManager {
        +tail_logs(lines: usize, follow: bool) Result
        +search_logs(pattern: &str, file: Option~&str~) Result
        +analyze_logs(period: &str) Result~LogAnalysis~
        +export_logs(format: &str, output: &Path) Result
    }
    
    NockitConfig --> WalletManager
    NockitConfig --> MiningManager
    NockitConfig --> NetworkMonitor
    
    WalletManager --> KeyPair
    MiningManager --> SystemMonitor
    NetworkMonitor --> LogManager
    
    KeyPair --> NockitConfig
    SystemMonitor --> LogManager
```

### Configuration Module (`config.rs`)

The configuration module provides centralized management of nockit settings:

```rust
use nockit::config::NockitConfig;

// Load or create default configuration
let config = NockitConfig::load_or_create(".nockit")?;

// Set mining public key
config.set_mining_pubkey("your_pubkey_here".to_string());

// Save configuration
config.save(".nockit/config.toml")?;
```

#### Configuration Structure
```toml
[nockchain]
binary_path = "nockchain"
data_dir = ".data.nockchain"
bind_address = "/ip4/0.0.0.0/udp/0/quic-v1"
peer_port = 0

[wallet]
binary_path = "nockchain-wallet"
wallet_dir = ".nockchain-wallet"
backup_dir = "wallet-backups"

[mining]
default_pubkey = "your_pubkey_here"
difficulty_target = 12345
stats_retention_days = 30

[network]
bootstrap_peers = []
connection_timeout = 30
max_peers = 50

[logging]
level = "info"
format = "pretty"
rotation_size_mb = 100
retention_days = 7

[benchmarking]
iterations = 1000
warmup_iterations = 100
output_format = "table"
save_results = true
```

### Cryptographic Module (`crypto.rs`)

Provides cryptographic utilities for key management and security:

```rust
use nockit::crypto::{KeyPair, hash_data, PublicKey};

// Generate new key pair
let keypair = KeyPair::generate()?;

// Sign message
let message = b"Hello, Nockchain!";
let signature = keypair.private_key.sign(message)?;

// Verify signature
let is_valid = keypair.public_key.verify(message, &signature)?;

// Hash data
let data = b"data to hash";
let hash = hash_data(data);
```

### Wallet Module (`wallet.rs`)

Manages wallet operations and key management:

```rust
use nockit::wallet::{generate_keys, check_status};

// Generate new wallet keys
generate_keys(Some(&output_path), &config_dir).await?;

// Check wallet status
check_status(Some("pubkey"), &config_dir).await?;
```

### Mining Module (`mining.rs`)

Controls mining operations and statistics:

```rust
use nockit::mining::{start_mining, MiningStats};

// Start mining process
start_mining("pubkey", Some(12345), &config_dir).await?;

// Create mining statistics
let stats = MiningStats {
    start_time: chrono::Utc::now(),
    blocks_mined: 10,
    hash_rate: 1000.0,
    // ... other fields
};
```

### Network Module (`network.rs`)

Provides network monitoring and diagnostics:

```rust
use nockit::network::{check_status, NetworkStatus};

// Check network connectivity
check_status(&config_dir).await?;

// Create network status
let status = NetworkStatus {
    connected_peers: 5,
    connectivity: ConnectivityStatus::Connected,
    // ... other fields
};
```

### Monitoring Module (`monitoring.rs`)

Real-time system health monitoring:

```rust
use nockit::monitoring::{run_monitor, SystemHealth};

// Run continuous monitoring
run_monitor(5, "table", &config_dir).await?;

// Collect system health
let health = collect_system_health(&config_dir).await?;
```

### Logging Module (`logging.rs`)

Advanced log management and analysis:

```rust
use nockit::logging::{init_logging, tail_logs};

// Initialize logging system
init_logging("info", &log_dir)?;

// Tail logs with following
tail_logs(100, true, &config_dir).await?;
```

### Benchmarking Module (`bench.rs`)

Performance testing and optimization:

```rust
use nockit::bench::{run_benchmarks, BenchmarkConfig};

// Run comprehensive benchmarks
let config = BenchmarkConfig::default();
let results = run_benchmarks(&config).await?;

// Run specific benchmark
let crypto_results = run_crypto_benchmarks(1000).await?;
```

## 🔧 Configuration

### Environment Variables

Nockit supports configuration through environment variables:

```bash
# Logging configuration
export RUST_LOG=info,nockchain=debug
export MINIMAL_LOG_FORMAT=true

# Mining configuration
export MINING_PUBKEY=your_public_key_here

# Network configuration
export PEER_PORT=8080
export BIND_ADDRESS=/ip4/0.0.0.0/udp/8080/quic-v1

# Benchmarking configuration
export BENCH_ITERATIONS=1000
export BENCH_OUTPUT_FORMAT=json
```

### Configuration Files

```mermaid
graph TD
    subgraph "Configuration Hierarchy"
        ENV_VARS[Environment Variables]
        CLI_ARGS[CLI Arguments]
        CONFIG_FILE[config.toml]
        DEFAULTS[Default Values]
    end
    
    subgraph "Configuration Files"
        MAIN_CONFIG[.nockit/config.toml]
        ENV_FILE[.nockit/.env]
        SCRIPTS_DIR[.nockit/scripts/]
        LOGS_DIR[.nockit/logs/]
        BACKUPS_DIR[.nockit/backups/]
    end
    
    subgraph "Priority Order"
        P1[1. CLI Arguments]
        P2[2. Environment Variables]
        P3[3. Configuration File]
        P4[4. Default Values]
    end
    
    CLI_ARGS --> P1
    ENV_VARS --> P2
    CONFIG_FILE --> P3
    DEFAULTS --> P4
    
    P1 --> MAIN_CONFIG
    P2 --> ENV_FILE
    P3 --> SCRIPTS_DIR
    P4 --> LOGS_DIR
    
    style P1 fill:#e8f5e8
    style P2 fill:#fff3e0
    style P3 fill:#e3f2fd
    style P4 fill:#fce4ec
```

#### Main Configuration (`.nockit/config.toml`)
Primary configuration file with all nockit settings.

#### Environment File (`.nockit/.env`)
Environment variables for nockchain operations.

#### Helper Scripts (`.nockit/scripts/`)
- `start_mining.sh`: Start mining operations
- `stop_mining.sh`: Stop mining operations  
- `check_status.sh`: Check system status

## 🧪 Testing

### Test Architecture

```mermaid
graph TD
    subgraph "Test Categories"
        UNIT[Unit Tests]
        INTEGRATION[Integration Tests]
        PERFORMANCE[Performance Tests]
        E2E[End-to-End Tests]
        SECURITY[Security Tests]
    end
    
    subgraph "Test Modules"
        CONFIG_TEST[Config Tests]
        CRYPTO_TEST[Crypto Tests]
        WALLET_TEST[Wallet Tests]
        MINING_TEST[Mining Tests]
        NETWORK_TEST[Network Tests]
        MONITOR_TEST[Monitor Tests]
        BENCH_TEST[Benchmark Tests]
    end
    
    subgraph "Test Infrastructure"
        MOCK_SERVICES[Mock Services]
        TEST_DATA[Test Data]
        FIXTURES[Test Fixtures]
        HELPERS[Test Helpers]
    end
    
    UNIT --> CONFIG_TEST
    UNIT --> CRYPTO_TEST
    UNIT --> WALLET_TEST
    
    INTEGRATION --> MINING_TEST
    INTEGRATION --> NETWORK_TEST
    INTEGRATION --> MONITOR_TEST
    
    PERFORMANCE --> BENCH_TEST
    
    E2E --> MOCK_SERVICES
    E2E --> TEST_DATA
    
    SECURITY --> CRYPTO_TEST
    SECURITY --> WALLET_TEST
    
    CONFIG_TEST --> FIXTURES
    CRYPTO_TEST --> HELPERS
    WALLET_TEST --> TEST_DATA
    
    style UNIT fill:#e8f5e8
    style INTEGRATION fill:#fff3e0
    style PERFORMANCE fill:#e3f2fd
    style E2E fill:#fce4ec
    style SECURITY fill:#ffebee
```

### Running Tests

```bash
# Run all tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_config_management

# Run performance tests
cargo test --release test_performance_benchmarks
```

### Test Coverage

The test suite covers:
- Configuration management and serialization
- Cryptographic operations and key management
- Wallet operations and backup/restore
- Mining statistics and process management
- Network monitoring and connectivity
- System health monitoring
- Error handling and edge cases
- Concurrent operations
- Performance benchmarks
- Security vulnerabilities

### Benchmarking

```bash
# Run performance benchmarks
cargo test test_performance_benchmarks -- --nocapture

# Run specific benchmark suite
nockit bench crypto --iterations 10000

# Compare performance over time
nockit bench --compare previous_results.json
```

## 📊 Monitoring and Metrics

### Health Status Levels

```mermaid
graph TD
    subgraph "Health Status Hierarchy"
        HEALTHY[Healthy]
        WARNING[Warning]
        CRITICAL[Critical]
        UNKNOWN[Unknown]
    end
    
    subgraph "Health Indicators"
        CPU_OK[CPU < 80%]
        MEM_OK[Memory < 85%]
        DISK_OK[Disk < 90%]
        NOCK_OK[Nockchain Running]
        NET_OK[Network Connected]
    end
    
    subgraph "Warning Conditions"
        CPU_WARN[CPU 80-90%]
        MEM_WARN[Memory 85-95%]
        DISK_WARN[Disk 90-95%]
        NOCK_WARN[Nockchain Slow]
        NET_WARN[Limited Connectivity]
    end
    
    subgraph "Critical Conditions"
        CPU_CRIT[CPU > 90%]
        MEM_CRIT[Memory > 95%]
        DISK_CRIT[Disk > 95%]
        NOCK_CRIT[Nockchain Down]
        NET_CRIT[No Connectivity]
    end
    
    CPU_OK --> HEALTHY
    MEM_OK --> HEALTHY
    DISK_OK --> HEALTHY
    NOCK_OK --> HEALTHY
    NET_OK --> HEALTHY
    
    CPU_WARN --> WARNING
    MEM_WARN --> WARNING
    DISK_WARN --> WARNING
    NOCK_WARN --> WARNING
    NET_WARN --> WARNING
    
    CPU_CRIT --> CRITICAL
    MEM_CRIT --> CRITICAL
    DISK_CRIT --> CRITICAL
    NOCK_CRIT --> CRITICAL
    NET_CRIT --> CRITICAL
    
    style HEALTHY fill:#c8e6c9
    style WARNING fill:#fff9c4
    style CRITICAL fill:#ffcdd2
    style UNKNOWN fill:#f5f5f5
```

- **Healthy**: All systems operational
- **Warning**: Minor issues or missing configuration
- **Critical**: Major failures requiring attention
- **Unknown**: Unable to determine status

### System Metrics
- CPU usage percentage
- Memory usage percentage
- Disk usage percentage
- Network traffic (RX/TX bytes)
- Process count

### Mining Statistics
- Blocks mined
- Hash rate (H/s)
- Difficulty target
- Rewards earned
- Uptime tracking
- Error logging

### Network Metrics
- Connected peer count
- Connection status
- Traffic statistics
- Latency measurements

### Performance Metrics
- Operation throughput
- Response latency
- Resource utilization
- Error rates

## 🔒 Security

### Security Architecture

```mermaid
graph TD
    subgraph "Security Layers"
        CRYPTO_LAYER[Cryptographic Layer]
        ACCESS_LAYER[Access Control Layer]
        AUDIT_LAYER[Audit Layer]
        NETWORK_LAYER[Network Security Layer]
    end
    
    subgraph "Key Management"
        KEY_GEN[Key Generation]
        KEY_STORAGE[Secure Storage]
        KEY_BACKUP[Encrypted Backup]
        KEY_ROTATION[Key Rotation]
    end
    
    subgraph "Data Protection"
        ENCRYPTION[Data Encryption]
        SIGNING[Digital Signing]
        VERIFICATION[Signature Verification]
        HASHING[Secure Hashing]
    end
    
    subgraph "Security Practices"
        SECURE_DEFAULTS[Secure Defaults]
        INPUT_VALIDATION[Input Validation]
        LOG_SANITIZATION[Log Sanitization]
        ERROR_HANDLING[Secure Error Handling]
    end
    
    CRYPTO_LAYER --> KEY_GEN
    CRYPTO_LAYER --> ENCRYPTION
    CRYPTO_LAYER --> SIGNING
    CRYPTO_LAYER --> HASHING
    
    ACCESS_LAYER --> KEY_STORAGE
    ACCESS_LAYER --> INPUT_VALIDATION
    
    AUDIT_LAYER --> LOG_SANITIZATION
    AUDIT_LAYER --> ERROR_HANDLING
    
    NETWORK_LAYER --> VERIFICATION
    
    KEY_GEN --> KEY_BACKUP
    KEY_STORAGE --> KEY_ROTATION
    
    style CRYPTO_LAYER fill:#fff3e0
    style ACCESS_LAYER fill:#e8f5e8
    style AUDIT_LAYER fill:#e3f2fd
    style NETWORK_LAYER fill:#fce4ec
```

### Key Management
- Private keys are handled securely with redacted debug output
- Backup encryption support
- Secure key derivation from passwords
- Base58 encoding for public keys and signatures

### Best Practices
- Regular key backups
- Secure storage of private keys
- Environment variable protection
- Log sanitization
- Input validation
- Secure defaults

## 🚀 Development

### Project Structure
```
nockit/
├── src/
│   ├── bin/           # Standalone binaries
│   │   ├── nocklog.rs    # Log management tool
│   │   ├── nockmon.rs    # System monitoring tool
│   │   └── nocksetup.rs  # Setup and installation tool
│   ├── config.rs      # Configuration management
│   ├── crypto.rs      # Cryptographic utilities
│   ├── logging.rs     # Log management
│   ├── mining.rs      # Mining operations
│   ├── monitoring.rs  # System monitoring
│   ├── network.rs     # Network diagnostics
│   ├── setup.rs       # Environment setup
│   ├── utils.rs       # Development utilities
│   ├── wallet.rs      # Wallet operations
│   ├── bench.rs       # Benchmarking tools
│   ├── lib.rs         # Library exports
│   └── main.rs        # Main CLI
├── tests/
│   ├── integration_tests.rs
│   ├── performance_tests.rs
│   └── security_tests.rs
├── examples/
│   ├── basic_usage.rs
│   ├── advanced_config.rs
│   └── custom_benchmarks.rs
├── Cargo.toml
├── setup.sh
└── README.md
```

### Development Workflow

```mermaid
gitGraph
    commit id: "Initial"
    branch feature
    checkout feature
    commit id: "Feature Dev"
    commit id: "Add Tests"
    commit id: "Documentation"
    checkout main
    merge feature
    commit id: "Release"
    branch hotfix
    checkout hotfix
    commit id: "Bug Fix"
    checkout main
    merge hotfix
    commit id: "Patch Release"
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

### Code Style
- Follow Rust standard formatting (`cargo fmt`)
- Use Clippy for linting (`cargo clippy`)
- Add comprehensive documentation
- Include unit and integration tests
- Follow security best practices

## 📋 Troubleshooting

### Diagnostic Flow

```mermaid
flowchart TD
    ISSUE[Issue Reported]
    IDENTIFY{Identify Category}
    
    INSTALL_ISSUE[Installation Issue]
    CONFIG_ISSUE[Configuration Issue]
    RUNTIME_ISSUE[Runtime Issue]
    NETWORK_ISSUE[Network Issue]
    PERFORMANCE_ISSUE[Performance Issue]
    
    CHECK_DEPS[Check Dependencies]
    CHECK_CONFIG[Check Configuration]
    CHECK_LOGS[Check Logs]
    CHECK_NETWORK[Check Network]
    RUN_BENCH[Run Benchmarks]
    
    FIX_DEPS[Fix Dependencies]
    FIX_CONFIG[Fix Configuration]
    RESTART_SERVICE[Restart Service]
    FIX_NETWORK[Fix Network]
    OPTIMIZE[Optimize Performance]
    
    RESOLVED[Issue Resolved]
    ESCALATE[Escalate to Support]
    
    ISSUE --> IDENTIFY
    
    IDENTIFY --> INSTALL_ISSUE
    IDENTIFY --> CONFIG_ISSUE
    IDENTIFY --> RUNTIME_ISSUE
    IDENTIFY --> NETWORK_ISSUE
    IDENTIFY --> PERFORMANCE_ISSUE
    
    INSTALL_ISSUE --> CHECK_DEPS
    CONFIG_ISSUE --> CHECK_CONFIG
    RUNTIME_ISSUE --> CHECK_LOGS
    NETWORK_ISSUE --> CHECK_NETWORK
    PERFORMANCE_ISSUE --> RUN_BENCH
    
    CHECK_DEPS --> FIX_DEPS
    CHECK_CONFIG --> FIX_CONFIG
    CHECK_LOGS --> RESTART_SERVICE
    CHECK_NETWORK --> FIX_NETWORK
    RUN_BENCH --> OPTIMIZE
    
    FIX_DEPS --> RESOLVED
    FIX_CONFIG --> RESOLVED
    RESTART_SERVICE --> RESOLVED
    FIX_NETWORK --> RESOLVED
    OPTIMIZE --> RESOLVED
    
    CHECK_DEPS --> ESCALATE
    CHECK_CONFIG --> ESCALATE
    CHECK_LOGS --> ESCALATE
    CHECK_NETWORK --> ESCALATE
    RUN_BENCH --> ESCALATE
    
    style ISSUE fill:#ffebee
    style RESOLVED fill:#e8f5e8
    style ESCALATE fill:#fff3e0
```

### Common Issues

#### "nockchain binary not found"
```bash
# Install nockchain
make install-nockchain

# Or specify path in config
nockit setup --force
```

#### "Mining process not starting"
```bash
# Check configuration
nockit wallet status

# Generate keys if needed
nockit wallet keygen

# Check logs
nocklog tail --follow
```

#### "Network connectivity issues"
```bash
# Test basic connectivity
nockit network status

# Check firewall settings
# Ensure ports are open for P2P communication
```

#### "Performance issues"
```bash
# Run performance benchmarks
nockit bench --all

# Check system resources
nockmon --once

# Analyze logs for bottlenecks
nocklog analyze --period 1h
```

### Debug Mode
```bash
# Enable verbose logging
nockit --verbose <command>

# Or set environment variable
export RUST_LOG=debug
nockit <command>
```

### Log Analysis
```bash
# Search for errors
nocklog search "error|failed|panic"

# Analyze recent activity
nocklog analyze --period 1h

# Export logs for analysis
nocklog export --format json --output debug_logs.json
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Support

- **Documentation**: This README and inline code documentation
- **Issues**: GitHub Issues for bug reports and feature requests
- **Community**: Join the nockchain community discussions

## 🗺️ Roadmap

### Development Timeline

```mermaid
timeline
    title Nockit Development Roadmap
    
    section Version 0.6.0
        Enhanced Mining Pool Support    : Mining pool integration
                                       : Pool statistics tracking
                                       : Automated pool switching
        
        Advanced Network Visualization : Network topology mapping
                                      : Real-time peer visualization
                                      : Connection quality metrics
        
        Automated Backup Scheduling   : Scheduled wallet backups
                                     : Incremental backup support
                                     : Cloud storage integration
        
        Performance Optimization      : Memory usage optimization
                                     : CPU efficiency improvements
                                     : I/O performance tuning
    
    section Version 0.7.0
        Web Dashboard                 : Browser-based monitoring
                                    : Real-time metrics display
                                    : Remote management interface
        
        Plugin System                : Extension framework
                                   : Custom module support
                                   : Third-party integrations
        
        Advanced Analytics           : Predictive analytics
                                   : Trend analysis
                                   : Performance forecasting
        
        Multi-node Management        : Cluster coordination
                                   : Distributed monitoring
                                   : Load balancing
    
    section Version 1.0.0
        Production Stability         : Enterprise-grade reliability
                                   : High availability features
                                   : Disaster recovery
        
        Comprehensive Documentation  : Complete API documentation
                                   : Tutorial series
                                   : Best practices guide
        
        Enterprise Features          : Role-based access control
                                   : Audit logging
                                   : Compliance reporting
        
        Long-term Support           : LTS release
                                  : Extended maintenance
                                  : Professional support
```

### Version 0.6.0
- [ ] Enhanced mining pool support
- [ ] Advanced network topology visualization
- [ ] Automated backup scheduling
- [ ] Performance optimization

### Version 0.7.0
- [ ] Web-based monitoring dashboard
- [ ] Plugin system for extensions
- [ ] Advanced analytics and reporting
- [ ] Multi-node cluster management

### Version 1.0.0
- [ ] Production-ready stability
- [ ] Comprehensive documentation
- [ ] Enterprise features
- [ ] Long-term support

---

## 🧠 Intelligent Design Rationale

The nockit toolkit represents a comprehensive approach to nockchain development and operations, designed with the following principles:

### Modular Architecture
The toolkit employs a modular design where each component serves a specific purpose while maintaining clear interfaces for interaction. This approach ensures maintainability, testability, and extensibility.

### Security-First Design
Security considerations are embedded throughout the architecture, from cryptographic key management to secure logging practices. The toolkit implements defense-in-depth strategies to protect sensitive operations.

### Performance Optimization
The inclusion of comprehensive benchmarking tools and performance monitoring ensures that the toolkit not only provides functionality but also maintains optimal performance characteristics under various operational conditions.

### Developer Experience
The toolkit prioritizes developer experience through intuitive CLI interfaces, comprehensive documentation, and clear error messages. The inclusion of development utilities and scaffolding tools accelerates the development workflow.

### Operational Excellence
Real-time monitoring, logging, and diagnostic capabilities ensure that operators have the visibility and tools necessary to maintain healthy nockchain deployments in production environments.

This comprehensive design ensures that nockit serves as both a development toolkit and an operational platform, supporting the full lifecycle of nockchain applications from development through production deployment and maintenance. 