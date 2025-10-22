# Docxology: Nockchain Orchestrator

[![Crates.io](https://img.shields.io/crates/v/docxology)](https://crates.io/crates/docxology)
[![PyPI version](https://badge.fury.io/py/docxology.svg)](https://badge.fury.io/py/docxology)
[![Documentation](https://docs.rs/docxology/badge.svg)](https://docs.rs/docxology)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

Docxology provides both thin wrappers and high-level convenience flows for interacting with Nockchain components including nodes, miners, wallets, and gRPC APIs.

## Features

- **Thin Wrappers**: Direct access to Nockchain internals with minimal abstraction
- **High-Level Flows**: Convenient methods that handle common workflows
- **Dual Language Support**: Available as both Rust crate and Python package
- **Comprehensive Testing**: Unit, integration, and end-to-end test coverage
- **Rich Documentation**: Auto-generated docs with semantic search capabilities

## Quick Start

### Rust

```rust
use docxology::{NodeConfig, start_node};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = NodeConfig::default();
    let node_handle = start_node(config).await?;

    // Node is now running...
    node_handle.shutdown().await?;
    Ok(())
}
```

### Python

```python
import asyncio
from docxology import NodeConfig, start_node

async def main():
    config = NodeConfig()
    node_handle = await start_node(config)

    # Node is now running...
    await node_handle.shutdown()

asyncio.run(main())
```

## Installation

### Rust Crate

Add to your `Cargo.toml`:

```toml
[dependencies]
docxology = "0.1.0"
```

### Python Package

```bash
pip install docxology
```

## Documentation

- **[User Guide](docs/book/)**: Comprehensive guides and tutorials
- **[API Reference](https://docs.rs/docxology)**: Complete API documentation
- **[Examples](./rust/examples/)**: Working code samples

## Architecture

Docxology provides a layered architecture:

```
Applications
    ↓
Docxology API (Node, Miner, Wallet, gRPC)
    ↓
Nockchain Components (NockApp, Kernels, APIs)
```

### Core Modules

- **Node Management**: Start/stop/configure Nockchain nodes
- **Miner Operations**: Control mining with key management
- **Wallet Operations**: Key generation, balances, transactions
- **gRPC Clients**: Public and private API access

## Testing

```bash
# Rust tests
cargo test

# Python tests (when available)
python -m pytest
```

## Examples

See the [`examples/`](rust/examples/) directory for complete working examples:

- [`start_node.rs`](rust/examples/start_node.rs) - Starting a Nockchain node
- [`start_miner.rs`](rust/examples/start_miner.rs) - Starting a miner
- [`wallet_operations.rs`](rust/examples/wallet_operations.rs) - Wallet management
- [`high_level_flows.rs`](rust/examples/high_level_flows.rs) - Convenience workflows

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please see the main Nockchain repository for contribution guidelines.

## Support

- **Issues**: Report bugs and feature requests on GitHub
- **Discussions**: Join the community discussions
- **Documentation**: Help improve our guides and examples

---

*Docxology is experimental software built on Nockchain. Use at your own risk.*
