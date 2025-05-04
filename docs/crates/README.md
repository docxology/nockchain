# Rust Crates

This directory contains technical documentation for the [Rust crates](../../crates/) in the [Nockchain](../../) project.

## Core Crates

- [nockchain](./nockchain.md): Core [blockchain implementation](../../crates/nockchain/)
- [nockapp](./nockapp.md): [Application framework](../../crates/nockapp/)
- [sword](./sword.md): [Nock interpreter and runtime](../../crates/sword/)
  - Core execution environment for Nock code
  - Automatic persistence and optimization capabilities
  - Jets for performance-critical operations

## Auxiliary Crates

- [equix-latency](./equix-latency.md): [Latency measurement utilities](../../crates/equix-latency/)
- [nockchain-bitcoin-sync](./nockchain-bitcoin-sync.md): [Bitcoin synchronization](../../crates/nockchain-bitcoin-sync/)
- [nockchain-libp2p-io](./nockchain-libp2p-io.md): [libp2p networking](../../crates/nockchain-libp2p-io/)
- [kernels](./kernels.md): [Kernel implementations](../../crates/kernels/)
- [zkvm-jetpack](./zkvm-jetpack.md): [Zero-knowledge VM jet acceleration](../../crates/zkvm-jetpack/)
  - Accelerated VM operations using zero-knowledge proofs
  - Integration with the Sword runtime 