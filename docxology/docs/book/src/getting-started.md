# Getting Started

This guide will help you get up and running with Docxology quickly.

## Prerequisites

### System Requirements

- **Operating System**: Linux, macOS, or Windows
- **Rust**: 1.70+ (for Rust development)
- **Python**: 3.8+ (for Python usage)
- **Memory**: At least 4GB RAM recommended
- **Storage**: At least 10GB free space for blockchain data
- **Network**: Internet connection for peer discovery

### Dependencies

Docxology depends on the Nockchain ecosystem:

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Nockchain dependencies (see main README)
# This includes hoonc, protoc, and other build dependencies
```

## Installation

### Rust Crate

Add Docxology to your `Cargo.toml`:

```toml
[dependencies]
docxology = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Python Package

Install via pip (once published):

```bash
pip install docxology
```

Or install from source:

```bash
# Clone the repository
git clone https://github.com/zorp-corp/nockchain.git
cd nockchain/docxology/python

# Install maturin if not already installed
pip install maturin

# Build and install the Python package
maturin develop
```

## Quick Examples

### Starting a Node (Rust)

```rust
use docxology::{NodeConfig, start_node};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the node
    let mut config = NodeConfig::default();
    config.data_dir = Some(PathBuf::from("./my_node_data"));
    config.api.enable_public_api = true;

    // Start the node
    let node_handle = start_node(config).await?;

    println!("Node started! API available at http://127.0.0.1:8080");

    // Keep running until interrupted
    tokio::signal::ctrl_c().await?;

    // Clean shutdown
    node_handle.shutdown().await?;

    Ok(())
}
```

### Starting a Node (Python)

```python
import asyncio
from docxology import NodeConfig, start_node

async def main():
    # Configure the node
    config = NodeConfig()
    config.data_dir = "./my_node_data"
    config.api.enable_public_api = True

    # Start the node
    node_handle = await start_node(config)

    print("Node started! API available at http://127.0.0.1:8080")

    try:
        # Keep running
        await asyncio.Future()
    except KeyboardInterrupt:
        print("Shutting down...")
        await node_handle.shutdown()

asyncio.run(main())
```

### Wallet Operations (Rust)

```rust
use docxology::{WalletConfig, create_wallet};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a wallet
    let config = WalletConfig::default();
    let wallet = create_wallet(config).await?;

    // Generate new keys
    let keypair = wallet.keygen().await?;
    println!("Public key: {}", keypair.public_key);

    // Check balance
    let balance = wallet.get_balance(&keypair.public_key).await?;
    println!("Balance: {} units", balance.balance);

    Ok(())
}
```

### Wallet Operations (Python)

```python
import asyncio
from docxology import WalletConfig, create_wallet

async def main():
    # Create a wallet
    config = WalletConfig()
    wallet = await create_wallet(config)

    # Generate new keys
    keypair = await wallet.keygen()
    print(f"Public key: {keypair.public_key}")

    # Check balance
    balance = await wallet.get_balance(keypair.public_key)
    print(f"Balance: {balance.balance} units")

asyncio.run(main())
```

## Configuration

### Node Configuration

```rust
use docxology::{NodeConfig, MinerConfig};

let mut config = NodeConfig::default();

// Basic settings
config.data_dir = Some(PathBuf::from("./node_data"));
config.network.listen_addr = "/ip4/0.0.0.0/udp/0/quic-v1".to_string();

// Enable mining
config.mining = Some(MinerConfig {
    enabled: true,
    threads: 4,
    pubkey: Some("your_mining_key".to_string()),
    ..Default::default()
});

// Enable public API
config.api.enable_public_api = true;
config.api.public_api_addr = "0.0.0.0:8080".to_string();
```

### Environment Variables

You can also configure Docxology using environment variables:

```bash
# Node configuration
export NOCKCHAIN_DATA_DIR="./node_data"
export NOCKCHAIN_NETWORK__LISTEN_ADDR="/ip4/0.0.0.0/udp/0/quic-v1"
export NOCKCHAIN_LOGGING__LEVEL="info"

# Mining configuration
export NOCKCHAIN_MINING__ENABLED="true"
export NOCKCHAIN_MINING__THREADS="4"
export NOCKCHAIN_MINING__PUBKEY="your_mining_key"

# API configuration
export NOCKCHAIN_API__ENABLE_PUBLIC="true"
export NOCKCHAIN_API__PUBLIC_ADDR="0.0.0.0:8080"
```

## Running Tests

### Rust Tests

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test unit_        # Unit tests
cargo test it_         # Integration tests
cargo test e2e_        # End-to-end tests

# Run with output
cargo test -- --nocapture
```

### Python Tests

```bash
# Run Python tests (once implemented)
python -m pytest tests/

# Run specific test file
python -m pytest tests/test_wallet.py -v
```

## Next Steps

1. **Explore Examples**: Check out the [examples](./guides/rust-examples.md) for more detailed usage patterns
2. **API Reference**: Dive into the [API documentation](./api/) for comprehensive function reference
3. **Advanced Usage**: Learn about [high-level flows](./guides/high-level-flows.md) and advanced configuration
4. **Production**: See [deployment guides](./ops/) for production setup

## Troubleshooting

If you encounter issues:

1. **Check Logs**: Enable debug logging with `RUST_LOG=debug`
2. **Verify Dependencies**: Ensure all Nockchain dependencies are properly installed
3. **Network Issues**: Check firewall settings and port availability
4. **Resource Limits**: Ensure sufficient memory and CPU resources

For more detailed troubleshooting, see the [Troubleshooting Guide](./ops/troubleshooting.md).
