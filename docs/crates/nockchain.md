# Nockchain

## Overview

The [`nockchain`](../../crates/nockchain/) crate is the core blockchain implementation for the [Nockchain](../../) project. It handles blockchain consensus, node networking, and Bitcoin synchronization integration.

## Directory Structure

Located in [`crates/nockchain/src/`](../../crates/nockchain/src/).

### Key Files

- [`lib.rs`](../../crates/nockchain/src/lib.rs): Core library implementation containing blockchain initialization, peer-to-peer networking, and Bitcoin integration
- [`main.rs`](../../crates/nockchain/src/main.rs): Entry point for the Nockchain binary
- [`colors.rs`](../../crates/nockchain/src/colors.rs): Terminal color output utilities

## Components

- Consensus mechanism with proof-of-work validation
- Block processing and chain management 
- State management with snapshot capabilities
- Peer-to-peer networking layer based on libp2p
- Genesis block generation and synchronization
- Bitcoin blockchain integration

## Configuration Options

The Nockchain node supports numerous configuration options:

- Genesis block settings (leader/watcher/fakenet modes)
- Mining configuration with multiple key options
- Bitcoin Core RPC connection parameters 
- Peer discovery and connection settings
- Network limits and performance tuning

## Usage

The `nockchain` crate provides the primary blockchain infrastructure, including:

- Block validation and chain management
- Transaction processing and verification
- P2P network communication via libp2p
- Consensus algorithm implementation based on proof of work
- Bitcoin synchronization for cross-chain interoperability

## Related Crates

- [nockchain-bitcoin-sync](./nockchain-bitcoin-sync.md): Bitcoin synchronization
- [nockchain-libp2p-io](./nockchain-libp2p-io.md): Network communication 