# Nockchain Technical Documentation

Welcome to the comprehensive technical documentation for Nockchain, a lightweight blockchain for heavyweight verifiable applications.

## Documentation Structure

This documentation is organized into several key areas:

### Core System Documentation
- **[Architecture Overview](architecture.md)** - High-level system design and component relationships
- **[Nock Virtual Machine](nockvm.md)** - Detailed documentation of the Nock VM implementation
- **[Blockchain Core](blockchain.md)** - Consensus, transactions, and chain management
- **[Networking](networking.md)** - P2P networking layer with libp2p and anti-spam mechanisms
- **[Wallet System](wallet.md)** - Hierarchical deterministic wallet implementation
- **[Build System](build-system.md)** - Cargo workspace and Makefile orchestration

### Development Documentation
- **[Hoon Programming Language](hoon-system.md)** - Complete guide to Hoon language and compilation
- **[API Reference](api-reference.md)** - Comprehensive CLI, RPC, and internal API documentation

### Operations Documentation
- **[Operations Guide](operations.md)** - Deployment, monitoring, troubleshooting, and maintenance procedures

## Quick Start Guide

### Prerequisites
- Rust toolchain (nightly-2025-02-14)
- System dependencies: `clang`, `llvm-dev`, `libclang-dev`

### Installation
```bash
# Clone the repository
git clone https://github.com/zorp-corp/nockchain.git
cd nockchain

# Copy environment configuration
cp .env_example .env

# Install Hoon compiler
make install-hoonc
export PATH="$HOME/.cargo/bin:$PATH"

# Build the system
make build

# Install Nockchain and wallet
make install-nockchain
make install-nockchain-wallet
```

### Basic Usage
```bash
# Generate keys
nockchain-wallet keygen

# Run a node (non-mining)
sh ./scripts/run_nockchain_node.sh

# Run a mining node
sh ./scripts/run_nockchain_miner.sh
```

## Project Structure

```
nockchain/
├── crates/                    # Rust workspace crates
│   ├── nockchain/            # Main blockchain node
│   ├── nockchain-wallet/     # Wallet implementation
│   ├── nockvm/               # Nock virtual machine
│   ├── nockapp/              # Application framework
│   ├── hoonc/                # Hoon compiler
│   └── ...                   # Additional crates
├── hoon/                     # Hoon source code
│   ├── apps/                 # Applications (wallet, dumbnet)
│   ├── common/               # Shared libraries
│   └── ...                   # Additional Hoon code
├── docs/                     # Technical documentation
├── scripts/                  # Utility scripts
└── assets/                   # Compiled JAM files
```

## Key Concepts

### Nockchain Philosophy
Nockchain represents a paradigm shift from traditional blockchains:
- **Lightweight Settlement**: Minimal on-chain computation
- **Heavyweight Verification**: Complex off-chain proving
- **Verifiability via Proving**: Replace public replication with private proving
- **Deterministic Execution**: Functional programming guarantees

### Core Technologies
- **Nock VM**: Minimal virtual machine with 12 opcodes
- **Hoon Language**: Functional language compiling to Nock
- **EquiX Mining**: Memory-hard proof-of-work algorithm
- **libp2p Networking**: Modern P2P networking with QUIC transport
- **JAM Serialization**: Efficient noun serialization format

## Development Workflow

### Building Applications
1. Write Hoon code in `hoon/apps/`
2. Compile with `hoonc` to generate JAM files
3. Load JAM files into NockApp framework
4. Deploy to Nockchain network

### Testing
```bash
# Run all tests
make test

# Test specific components
cargo test --package nockvm
cargo test --package nockchain
```

### Contributing
1. Follow the coding standards in each language
2. Add comprehensive tests for new features
3. Update documentation for API changes
4. Ensure all builds pass before submitting PRs

## Architecture Highlights

### Layered Design
```
┌─────────────────────────────────────────┐
│           Applications                  │
├─────────────────────────────────────────┤
│            Kernels                      │
├─────────────────────────────────────────┤
│         NockApp Framework               │
├─────────────────────────────────────────┤
│          Nock VM                        │
├─────────────────────────────────────────┤
│        Infrastructure                   │
│  (Networking, Storage, Crypto)          │
└─────────────────────────────────────────┘
```

### Key Components
- **Applications**: User-facing functionality (wallet, mining)
- **Kernels**: Core blockchain logic (consensus, transactions)
- **NockApp Framework**: Application runtime and lifecycle management
- **Nock VM**: Deterministic computation engine
- **Infrastructure**: Networking, storage, and cryptographic primitives

## Security Model

### Cryptographic Foundations
- **Ed25519**: Digital signatures for transactions and identity
- **SHA-256**: Primary hashing algorithm
- **TIP5**: Custom hash function for Nock computations
- **EquiX**: Memory-hard proof-of-work for mining

### Network Security
- **Proof-of-Work Anti-Spam**: EquiX PoW prevents network flooding
- **Peer Validation**: All messages cryptographically verified
- **Connection Limits**: Resource exhaustion protection
- **Automatic Blocking**: Malicious peers automatically banned

## Performance Characteristics

### Nock VM Optimizations
- **Jets**: Native implementations of common operations
- **Tail Call Optimization**: Efficient recursive computation
- **Memoization**: Caching of expensive computations
- **SIMD Instructions**: Vectorized operations where applicable

### Network Performance
- **QUIC Transport**: Low-latency, multiplexed connections
- **Message Batching**: Efficient bulk data transfer
- **DHT Routing**: Scalable peer discovery
- **Connection Pooling**: Optimized resource usage

## Troubleshooting

### Common Issues
- **Build Failures**: Check Rust toolchain version and system dependencies
- **Network Connectivity**: Verify firewall and NAT configuration
- **Mining Problems**: Ensure valid mining keys and peer connections
- **Wallet Issues**: Check key import/export and socket connections

### Mining Memory Issues
If you encounter "serf - panicked" errors during mining on memory-constrained systems:
```bash
# Fix for memory-poor miners
sudo sysctl -w vm.overcommit_memory=1
```
See [Operations Guide](operations.md#memory-related-mining-issues) for detailed troubleshooting.

### Debug Tools
```bash
# Enable detailed logging
RUST_LOG=debug nockchain

# Monitor specific components
RUST_LOG=nockvm=trace,nockchain=debug nockchain

# Network debugging
RUST_LOG=libp2p=debug,nockchain_libp2p_io=trace nockchain

# Monitor mining activity
nockchain --mining-pubkey <pubkey> --mine | grep -aE "serf|panic|mining|validated|candidate"
```

## License and Contributing

Nockchain is licensed under the MIT License. See [LICENSE](../LICENSE) for details.

### Contributing Guidelines
1. **Code Quality**: Follow Rust and Hoon best practices
2. **Testing**: Comprehensive test coverage required
3. **Documentation**: Update docs for all changes
4. **Performance**: Consider performance implications
5. **Security**: Security-first development approach

### Community
- **GitHub**: [https://github.com/zorp-corp/nockchain](https://github.com/zorp-corp/nockchain)
- **Issues**: Report bugs and feature requests on GitHub
- **Discussions**: Technical discussions in GitHub Discussions

## Future Roadmap

### Short Term
- Enhanced wallet functionality
- Improved mining efficiency
- Better developer tooling
- Comprehensive test coverage

### Medium Term
- Zero-knowledge proof integration
- Advanced smart contract capabilities
- Cross-chain interoperability
- Scalability improvements

### Long Term
- Formal verification tools
- Advanced privacy features
- Enterprise integration
- Ecosystem expansion

This documentation provides a comprehensive guide to understanding, building, and contributing to the Nockchain ecosystem. Each section contains detailed technical information suitable for developers, researchers, and system administrators. 