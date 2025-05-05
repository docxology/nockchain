# Nockchain Documentation

This directory contains technical documentation for the [Nockchain](https://github.com/tetra/nockchain) project, a lightweight blockchain for heavyweight verifiable applications.

## Project Overview

Nockchain is designed to enable trustless settlement of heavyweight verifiable computation by replacing verifiability-via-public-replication with verifiability-via-private-proving. This approach means:

- Proving happens off-chain
- Verification occurs on-chain
- Computation is scalable and private

```mermaid
graph TD
    A[User] -->|Submits Computation| B[Prover]
    B -->|Executes Computation| C[Results]
    B -->|Generates Proof| D[Proof]
    C -->|Results + Proof| E[Nockchain Network]
    D -->|Results + Proof| E
    E -->|Verification| F[Consensus]
    F -->|Verified Results| G[Blockchain]
    G -->|Confirmed Results| A
    B -->|Private Computation| H[Zero-Knowledge System]
    H -->|Verification Circuit| D

    style A fill:#f9f,stroke:#333,stroke-width:2px
    style B fill:#bbf,stroke:#333,stroke-width:2px
    style C fill:#bfb,stroke:#333,stroke-width:2px
    style D fill:#fbb,stroke:#333,stroke-width:2px
    style E fill:#fbf,stroke:#333,stroke-width:2px
    style F fill:#bff,stroke:#333,stroke-width:2px
    style G fill:#ffb,stroke:#333,stroke-width:2px
    style H fill:#ddf,stroke:#333,stroke-width:2px
```

## Architecture

The Nockchain platform consists of several key components:

1. **Blockchain Layer**: A lightweight blockchain for consensus and verification
2. **Nock Runtime**: The Sword VM providing the execution environment
3. **Hoon Applications**: User-facing applications and services
4. **Zero-Knowledge System**: Enabling private computation with public verification

```mermaid
flowchart TB
    subgraph "Nockchain System Architecture"
        subgraph "Blockchain Layer"
            BC[Blockchain Core]
            CONS[Consensus]
            STATE[State Management]
            P2P[P2P Network]
            MEM[Mempool]
        end

        subgraph "Nock Runtime"
            SWORD[Sword VM]
            JETS[Jets/Optimizations]
            MEMORY[Memory Management]
            PERSIST[Persistence]
            IBIG[Big Integer Library]
        end

        subgraph "Hoon Applications"
            WALLET[Wallet]
            DUMBNET[DumbNet]
            APPS[User Applications]
            COMMON[Common Libraries]
        end

        subgraph "Zero-Knowledge System"
            ZKVM[ZK VM]
            PROVER[Prover]
            VERIFIER[Verifier]
            JETPACK[Jetpack]
            STARK[STARK Implementation]
        end

        %% Connections between components
        BC <--> CONS
        BC <--> STATE
        BC <--> P2P
        BC <--> MEM
        
        SWORD <--> JETS
        SWORD <--> MEMORY
        SWORD <--> PERSIST
        SWORD <--> IBIG
        
        WALLET --> APPS
        DUMBNET --> APPS
        COMMON --> WALLET
        COMMON --> DUMBNET
        COMMON --> APPS
        
        ZKVM <--> PROVER
        ZKVM <--> VERIFIER
        ZKVM <--> JETPACK
        ZKVM <--> STARK
        
        %% Inter-system connections
        BC <--> SWORD
        SWORD <--> APPS
        ZKVM <--> SWORD
        P2P <--> DUMBNET
    end

    classDef blockchain fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef runtime fill:#eeeeee,stroke:#333,stroke-width:1px
    classDef apps fill:#d3f8e2,stroke:#333,stroke-width:1px
    classDef zk fill:#e3f6f5,stroke:#333,stroke-width:1px
    
    class BC,CONS,STATE,P2P,MEM blockchain
    class SWORD,JETS,MEMORY,PERSIST,IBIG runtime
    class WALLET,DUMBNET,APPS,COMMON apps
    class ZKVM,PROVER,VERIFIER,JETPACK,STARK zk
```

## Contents

### Primary Documentation

- [**Installation Guide**](./INSTALLATION.md): Detailed setup instructions for all platforms
- [**Technical Roadmap**](./ROADMAP.md): Current status and future development plans
- [**Contributing Guide**](./CONTRIBUTING.md): How to contribute to the Nockchain project
- [**Zero-Knowledge System**](./ZK_SYSTEM.md): Comprehensive overview of the ZK proof system

### Component Documentation

- [**Hoon**](./hoon/): Documentation for [Hoon](../hoon/) components
  - [Apps](./hoon/apps.md): Documentation for Hoon applications
  - [Common](./hoon/common.md): Documentation for common Hoon libraries
  - [Test Jams](./hoon/test-jams.md): Documentation for test jam files

- [**Crates**](./crates/): Documentation for [Rust crates](../crates/)
  - [Nockchain](./crates/nockchain.md): Core blockchain implementation
  - [Sword](./crates/sword.md): Nock interpreter and runtime
  - [NockApp](./crates/nockapp.md): Application framework
  - [ZKVM Jetpack](./crates/zkvm-jetpack.md): ZK VM acceleration
  - [Network Crates](./crates/nockchain-libp2p-io.md): P2P networking layer
  - [Auxiliary Crates](./crates/README.md#auxiliary-crates): Supporting components

## Documentation Map

```mermaid
graph TD
    subgraph "Documentation Structure"
        DOCS[Nockchain Documentation]
        
        GUIDES[Guides]
        INSTALL[Installation Guide]
        CONTRIB[Contributing Guide]
        ROADMAP[Technical Roadmap]
        ZK_DOCS[ZK System]
        
        COMP_DOCS[Component Documentation]
        HOON_DOCS[Hoon Documentation]
        CRATES_DOCS[Crates Documentation]
        
        APPS_DOCS[Apps Documentation]
        COMMON_DOCS[Common Documentation]
        TEST_JAMS_DOCS[Test Jams Documentation]
        
        CORE_CRATES_DOCS[Core Crates Documentation]
        AUX_CRATES_DOCS[Auxiliary Crates Documentation]
        
        DOCS --> GUIDES
        DOCS --> COMP_DOCS
        
        GUIDES --> INSTALL
        GUIDES --> CONTRIB
        GUIDES --> ROADMAP
        GUIDES --> ZK_DOCS
        
        COMP_DOCS --> HOON_DOCS
        COMP_DOCS --> CRATES_DOCS
        
        HOON_DOCS --> APPS_DOCS
        HOON_DOCS --> COMMON_DOCS
        HOON_DOCS --> TEST_JAMS_DOCS
        
        CRATES_DOCS --> CORE_CRATES_DOCS
        CRATES_DOCS --> AUX_CRATES_DOCS
        
        %% Specific components
        APPS_DOCS --> WALLET_DOCS[Wallet Documentation]
        APPS_DOCS --> DUMBNET_DOCS[DumbNet Documentation]
        
        COMMON_DOCS --> TABLE_DOCS[Table Documentation]
        COMMON_DOCS --> STARK_DOCS[STARK Documentation]
        COMMON_DOCS --> MARKDOWN_DOCS[Markdown Documentation]
        COMMON_DOCS --> ZTD_DOCS[ZTD Documentation]
        COMMON_DOCS --> CORE_COMPONENTS_DOCS[Core Components Documentation]
        
        CORE_CRATES_DOCS --> NOCKCHAIN_DOCS[Nockchain Documentation]
        CORE_CRATES_DOCS --> NOCKAPP_DOCS[NockApp Documentation]
        CORE_CRATES_DOCS --> SWORD_DOCS[Sword Documentation]
        
        AUX_CRATES_DOCS --> EQUIX_DOCS[Equix Latency Documentation]
        AUX_CRATES_DOCS --> BTC_SYNC_DOCS[Bitcoin Sync Documentation]
        AUX_CRATES_DOCS --> LIBP2P_DOCS[LibP2P Documentation]
        AUX_CRATES_DOCS --> KERNELS_DOCS[Kernels Documentation]
        AUX_CRATES_DOCS --> ZKVM_DOCS[ZKVM Jetpack Documentation]
        
        classDef docsNode fill:#f9d5e5,stroke:#333,stroke-width:1px
        classDef appsNode fill:#d3f8e2,stroke:#333,stroke-width:1px
        classDef libNode fill:#e3f6f5,stroke:#333,stroke-width:1px
        classDef crateNode fill:#f9ebe0,stroke:#333,stroke-width:1px
        classDef guideNode fill:#bbf,stroke:#333,stroke-width:2px
        
        class DOCS,HOON_DOCS,CRATES_DOCS,COMP_DOCS docsNode
        class APPS_DOCS,WALLET_DOCS,DUMBNET_DOCS appsNode
        class COMMON_DOCS,TABLE_DOCS,STARK_DOCS,MARKDOWN_DOCS,ZTD_DOCS,CORE_COMPONENTS_DOCS libNode
        class CORE_CRATES_DOCS,AUX_CRATES_DOCS,NOCKCHAIN_DOCS,NOCKAPP_DOCS,SWORD_DOCS,EQUIX_DOCS,BTC_SYNC_DOCS,LIBP2P_DOCS,KERNELS_DOCS,ZKVM_DOCS crateNode
        class TEST_JAMS_DOCS libNode
        class GUIDES,INSTALL,CONTRIB,ROADMAP,ZK_DOCS guideNode
    end
```

## Getting Started

To get started with Nockchain, refer to these documents:

1. [Installation Guide](./INSTALLATION.md) for setting up your environment
2. [Main README](../README.md) for a project overview
3. [Technical Roadmap](./ROADMAP.md) for understanding the project's direction

For developers interested in contributing:

1. [Contributing Guide](./CONTRIBUTING.md) for development workflow
2. [Component documentation](#component-documentation) for technical details
3. [Zero-Knowledge System](./ZK_SYSTEM.md) for ZK proof system details

```mermaid
graph LR
    A[Install Dependencies] --> B[Build Codebase]
    B --> C[Run Node]
    C --> D[Test System]
    
    subgraph "Development Environment"
        A
        B
    end
    
    subgraph "Runtime Environment"
        C
        D
    end
    
    style A fill:#f9f,stroke:#333,stroke-width:2px
    style B fill:#bbf,stroke:#333,stroke-width:2px
    style C fill:#bfb,stroke:#333,stroke-width:2px
    style D fill:#fbb,stroke:#333,stroke-width:2px
```

## Further Reading

- [DEVELOPERS.md](../crates/sword/DEVELOPERS.md): Development guidelines for Sword
- [Makefile](../Makefile): Build commands and project structure 