# Introduction

Welcome to Docxology, the comprehensive orchestrator for Nockchain operations.

## What is Docxology?

Docxology is a sidecar package that provides both thin wrappers and high-level convenience flows for interacting with Nockchain components. It serves as an intelligent interface between applications and the complex Nockchain ecosystem.

### Key Features

- **Thin Wrappers**: Direct access to Nockchain internals with minimal abstraction
- **High-Level Flows**: Convenient methods that handle common workflows end-to-end
- **Dual Language Support**: Available as both Rust crate and Python package
- **Comprehensive Testing**: Unit, integration, and end-to-end test coverage
- **Rich Documentation**: Auto-generated docs with semantic search capabilities

### Use Cases

Docxology is designed for:

- **Application Developers**: Building applications that need to interact with Nockchain
- **Node Operators**: Managing Nockchain infrastructure with programmatic interfaces
- **DevOps Teams**: Automating Nockchain deployment and operations
- **Protocol Researchers**: Testing and experimenting with Nockchain functionality

## Architecture Overview

Docxology provides a layered architecture:

```
┌─────────────────────────────────────────────────────────┐
│                    Applications                          │
├─────────────────────────────────────────────────────────┤
│                 Docxology API                           │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐      │
│  │   Node      │ │   Miner     │ │   Wallet    │      │
│  │ Management  │ │ Operations  │ │ Operations  │      │
│  └─────────────┘ └─────────────┘ └─────────────┘      │
├─────────────────────────────────────────────────────────┤
│              Nockchain Components                       │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐      │
│  │   NockApp   │ │   Kernels   │ │  gRPC APIs  │      │
│  │             │ │             │ │             │      │
│  │  • Node     │ │  • Dumb     │ │  • Public   │      │
│  │  • Miner    │ │  • Wallet   │ │  • Private  │      │
│  └─────────────┘ └─────────────┘ └─────────────┘      │
└─────────────────────────────────────────────────────────┘
```

### Components

1. **Node Management**: Start, stop, and configure Nockchain nodes
2. **Miner Operations**: Control mining activities with key management
3. **Wallet Operations**: Handle key generation, balance queries, and transactions
4. **gRPC Clients**: Interact with both public and private Nockchain APIs

## Getting Started

### Prerequisites

- Rust 1.70+ (for Rust usage)
- Python 3.8+ (for Python usage)
- Nockchain dependencies (see main README)

### Quick Start

#### Using Rust

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

#### Using Python

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

### Next Steps

- Explore the [User Guides](./guides/) for detailed usage instructions
- Check out the [API Reference](./api/) for complete function documentation
- Review the [Examples](./guides/rust-examples.md) for practical implementations
- See [Operations](./ops/) for deployment and management guidance

## Support

- **Documentation**: Comprehensive guides and API reference
- **Examples**: Working code samples in both Rust and Python
- **Testing**: Extensive test coverage to ensure reliability
- **Community**: Integration with the broader Nockchain ecosystem

---

*Docxology is experimental software. Use at your own risk and ensure you understand the implications of running Nockchain nodes and miners.*
