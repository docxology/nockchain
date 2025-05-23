# Nockit Examples

This directory contains comprehensive examples demonstrating the full capabilities of the nockit toolkit for nockchain development, monitoring, and operations.

## Available Examples

### 1. 🚀 Comprehensive Usage (`comprehensive_usage.rs`)

**Description**: Complete demonstration of all nockit features including setup, monitoring, development tools, and analytics.

**Features Demonstrated**:
- Environment setup and configuration
- System health monitoring
- Wallet operations
- Network monitoring
- Logging and analysis
- Performance benchmarking
- Development tools
- Real-time monitoring dashboard

**Run Command**:
```bash
cargo run --package nockit --example comprehensive_usage
```

**Generated Files**: `.nockit_example/` directory with:
- Configuration files
- Health reports
- Usage summaries
- Performance metrics

---

### 2. 🔐 Wallet Setup and Confirmation (`wallet_setup_demo.rs`)

**Description**: Complete wallet setup workflow with security validation and comprehensive testing.

**Features Demonstrated**:
- Cryptographic key generation
- Wallet status confirmation
- Balance checking
- Secure backup creation
- Security validation
- Key derivation testing
- Backup/restore cycles

**Run Command**:
```bash
cargo run --package nockit --example wallet_setup_demo
```

**Generated Files**: `.wallet_demo/` directory with:
- `wallet_info.json` - Wallet metadata
- `balance_report.md` - Balance information
- `security_checklist.md` - Security recommendations
- `wallet_setup_report.md` - Comprehensive setup report
- `backups/` - Encrypted wallet backups

**Key Features**:
- **Security-First**: Comprehensive security validation and best practices
- **Testing**: Automated testing of all wallet operations
- **Documentation**: Detailed reports and security checklists
- **Backup Management**: Secure backup creation and verification

---

### 3. ⛏️ Mining with Comprehensive Analytics (`mining_with_analytics.rs`)

**Description**: Advanced mining setup with real-time log analysis, performance metrics, and visualization dashboard.

**Features Demonstrated**:
- Mining wallet setup
- Comprehensive logging configuration
- Real-time log analysis and classification
- Performance metrics collection
- Visual log parsing and reporting
- Error pattern detection
- Monitoring dashboard creation

**Run Command**:
```bash
cargo run --package nockit --example mining_with_analytics
```

**Generated Files**: `.mining_analytics_demo/` directory with:
- `logs/` - Structured mining logs
- `analytics/` - Log analysis and classification
- `reports/` - Comprehensive analytics reports
- `metrics/` - Performance time series data
- `visualizations/` - Chart and graph data
- `dashboard/` - Real-time monitoring dashboard

**Analytics Features**:
- **Log Classification**: Automatic categorization by level and component
- **Pattern Detection**: Error pattern analysis and alerting
- **Performance Tracking**: Hash rate, memory, CPU, and network metrics
- **Visual Analytics**: Charts, graphs, and trend analysis
- **Real-time Dashboard**: Web-based monitoring interface

---

### 4. 🛠️ Setup Script (`setup_nockchain.sh`)

**Description**: Automated setup script for complete nockchain development environment.

**Features**:
- OS compatibility checking
- System dependency installation
- Rust installation via rustup
- Environment file setup
- Hoon compiler installation
- Project building
- Binary installation
- Installation verification

**Run Command**:
```bash
chmod +x nockit/examples/setup_nockchain.sh
./nockit/examples/setup_nockchain.sh
```

**Options**:
- `--help` - Show usage information
- `--verify` - Only run verification checks
- `--deps-only` - Only install system dependencies

---

### 5. 📊 Mining Statistics Analyzer (`mining_stats.py`)

**Description**: Python script for analyzing nockchain mining statistics from wallet data.

**Features Demonstrated**:
- Socket-based communication with nockchain wallet
- Coinbase block detection and parsing
- Miner ranking and statistics calculation
- Configurable timeout and debug logging
- Comprehensive error handling and validation

**Run Command**:
```bash
python3 nockit/examples/mining_stats.py [--socket PATH] [--timeout SECONDS] [--debug]
```

**Options**:
- `--socket PATH` - Path to nockchain socket (default: `.socket/nockchain_npc.sock`)
- `--timeout SECONDS` - Timeout for wallet commands (default: 30)
- `--debug` - Enable debug logging output
- `--help` - Show usage information

**Requirements**:
- Python 3.6+ with standard library
- `nockchain-wallet` binary in PATH
- Active nockchain socket connection
- Optional: `pip install -r nockit/examples/requirements.txt` for enhanced features

**Example Output**:
```
⛏️  Nockchain Mining Statistics Analyzer
=============================================

Miner Rankings (out of 42 full coinbase blocks):
   #  WALLET                               BLOCKS      %
------------------------------------------------------------
  #1  a1b2c3d4e5f6...9876543210abcdef           15   35.7%
  #2  fedcba0987654321...123456789abcdef         12   28.6%
  #3  0123456789abcdef...fedcba0987654321          8   19.0%

Summary:
  Total miners: 8
  Total blocks: 42
  Average blocks per miner: 5.2
  Last updated: 2024-01-15 14:30:25
```

---

### 6. 🌐 Peer Configuration Management (`nockchain_peers.toml` & `peer_helper.py`)

**Description**: Comprehensive peer configuration system for nockchain network connectivity.

**Files**:
- `nockchain_peers.toml` - TOML configuration with known stable peers
- `peer_helper.py` - Python utility for parsing and formatting peer data

**Peer Configuration Features**:
- Regional peer grouping (EU, US, Global)
- Reliability ratings (high, medium, low)
- Provider categorization (GCP, AWS, etc.)
- Connection recommendations and timeouts
- Structured metadata for each peer

**Peer Helper Features**:
- Multiple output formats (args, list, json, table)
- Filtering by region, reliability, or provider
- Configuration validation
- Command-line argument generation

**Run Commands**:
```bash
# Show all peers in table format
python3 nockit/examples/peer_helper.py

# Generate command-line arguments for high-reliability EU peers
python3 nockit/examples/peer_helper.py --format args --region EU --reliability high

# Export JSON for programmatic use
python3 nockit/examples/peer_helper.py --format json --provider GCP

# Validate configuration
python3 nockit/examples/peer_helper.py --validate

# Show network information
python3 nockit/examples/peer_helper.py --info
```

**Example Usage in Scripts**:
```bash
# Use peer helper to generate arguments for nockchain
PEER_ARGS=$(python3 nockit/examples/peer_helper.py --format args --regional global)
nockchain-wallet $PEER_ARGS --other-options
```

---

## Usage Workflow

### Getting Started

1. **Initial Setup**:
   ```bash
   # Run the automated setup script
   ./nockit/examples/setup_nockchain.sh
   
   # Install Python dependencies (optional, for enhanced features)
   pip install -r nockit/examples/requirements.txt
   ```

2. **Comprehensive Demo**:
   ```bash
   # See all nockit features in action
   cargo run --package nockit --example comprehensive_usage
   ```

3. **Wallet Setup**:
   ```bash
   # Set up and validate a wallet
   cargo run --package nockit --example wallet_setup_demo
   ```

4. **Mining with Analytics**:
   ```bash
   # Start mining with advanced monitoring
   cargo run --package nockit --example mining_with_analytics
   ```

5. **Mining Statistics Analysis**:
   ```bash
   # Analyze current mining statistics
   python3 nockit/examples/mining_stats.py --debug
   ```

6. **Peer Configuration Management**:
   ```bash
   # View available peers
   python3 nockit/examples/peer_helper.py --format table
   
   # Generate peer arguments for mining
   PEERS=$(python3 nockit/examples/peer_helper.py --format args --reliability high)
   ```

### Production Workflow

1. **Environment Setup**: Use `setup_nockchain.sh` for initial environment setup
2. **Wallet Configuration**: Use `wallet_setup_demo.rs` as reference for secure wallet setup
3. **Mining Operations**: Use `mining_with_analytics.rs` for production mining with monitoring
4. **Ongoing Monitoring**: Use the generated dashboards and reports for operational insight

---

## Example Output Structure

### Wallet Demo Output
```
.wallet_demo/
├── wallet_info.json          # Wallet metadata
├── balance_report.md          # Balance information
├── security_checklist.md     # Security recommendations
├── wallet_setup_report.md    # Comprehensive report
└── backups/
    └── wallet_backup_demo.json # Encrypted backup
```

### Mining Analytics Output
```
.mining_analytics_demo/
├── logs/
│   └── mining_session_*.log   # Structured logs
├── analytics/
│   ├── log_analysis.json      # Analysis results
│   └── pattern_analysis.md    # Pattern detection
├── reports/
│   └── mining_analytics_report.md # Comprehensive report
├── metrics/
│   ├── performance_metrics.json   # Raw metrics
│   └── time_series.json          # Time series data
├── visualizations/
│   └── charts_data.json          # Chart data
└── dashboard/
    ├── index.html                # Dashboard interface
    ├── config.json              # Dashboard config
    └── live_data.json           # Real-time data
```

---

## Advanced Features

### Log Analysis

The mining analytics example includes sophisticated log analysis:

- **Automatic Classification**: Logs sorted by level (INFO, DEBUG, TRACE, WARN, ERROR)
- **Component Separation**: Logs grouped by component (mining, network, performance)
- **Pattern Detection**: Automatic identification of error patterns and anomalies
- **Performance Extraction**: Automatic extraction of metrics from log entries
- **Trend Analysis**: Time-series analysis of performance data

### Visualization

Generated visualization data includes:

- **Hash Rate Charts**: Real-time hash rate monitoring
- **Resource Usage**: Memory and CPU utilization graphs
- **Network Activity**: Peer connections and data transfer
- **Error Distribution**: Pie charts of log levels and error types
- **Performance Timelines**: Historical performance trends

### Security Features

Wallet setup includes comprehensive security:

- **Key Generation**: Cryptographically secure random key generation
- **Encryption Validation**: Private key and seed phrase encryption
- **File Permissions**: Proper file security setup
- **Backup Integrity**: Verification of backup completeness
- **Security Checklists**: Automated security recommendation generation

---

## Integration with Nockit CLI

All examples integrate seamlessly with the nockit CLI tools:

- **`nockit`** - Main CLI for all operations
- **`nockmon`** - System monitoring and health checks
- **`nocklog`** - Log analysis and searching
- **`nocksetup`** - Environment setup and configuration

### Example CLI Usage

```bash
# After running examples, use CLI tools:

# Monitor system health
nockmon --config-dir .mining_analytics_demo

# Search logs for patterns
nocklog search "hash rate" --config-dir .mining_analytics_demo

# Check wallet status
nockit wallet status --config-dir .wallet_demo

# Start mining with generated configuration
nockit mining start --pubkey $(cat .wallet_demo/wallet_info.json | jq -r .public_key)

# Analyze mining statistics
python3 nockit/examples/mining_stats.py --socket .mining_analytics_demo/.socket/nockchain_npc.sock

# Use peer configuration for network connectivity
PEER_ARGS=$(python3 nockit/examples/peer_helper.py --format args --regional global)
nockit mining start $PEER_ARGS --pubkey $(cat .wallet_demo/wallet_info.json | jq -r .public_key)
```

---

## Development and Customization

### Extending Examples

These examples serve as templates for:

- **Custom Analytics**: Modify log analysis patterns for specific use cases
- **Dashboard Customization**: Adapt the monitoring dashboard for your needs
- **Security Policies**: Customize security validation for your requirements
- **Integration**: Integrate with external monitoring and alerting systems

### Configuration

All examples support configuration through:

- **Environment Variables**: Override default settings
- **Configuration Files**: TOML-based configuration
- **Command Line Arguments**: Runtime parameter customization
- **JSON Metadata**: Structured configuration storage

---

## Troubleshooting

### Common Issues

1. **Rust Not Installed**: Run `setup_nockchain.sh` first
2. **Permission Errors**: Ensure proper file permissions for wallets
3. **Missing Dependencies**: Install system dependencies with `--deps-only`
4. **Network Issues**: Check firewall and network connectivity

### Support

- **CLI Help**: Use `--help` with any nockit command
- **Example Documentation**: Read the generated reports for guidance
- **Security Guidance**: Follow the generated security checklists
- **Performance Tuning**: Use the analytics reports for optimization

---

## Next Steps

After running these examples:

1. **Review Generated Reports**: Understand the comprehensive output
2. **Customize for Production**: Adapt examples for your use case
3. **Set Up Monitoring**: Use dashboards for ongoing operations
4. **Implement Security**: Follow generated security recommendations
5. **Scale Operations**: Use insights from analytics for optimization

For more information, see the main nockit documentation and the `--help` output of each tool. 