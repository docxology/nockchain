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

### Component Interaction Model

```mermaid
sequenceDiagram
    participant User
    participant App as Hoon Application
    participant Runtime as Nock Runtime (Sword)
    participant ZK as ZK System
    participant Chain as Blockchain
    participant Net as P2P Network

    User->>App: Submit Transaction
    App->>Runtime: Process Transaction
    Runtime->>ZK: Generate Proof
    ZK-->>Runtime: Return Proof
    Runtime->>Chain: Submit Transaction + Proof
    
    Chain->>Chain: Verify Proof
    Chain->>Chain: Update State
    Chain->>Net: Broadcast Transaction
    Net-->>Chain: Consensus Achieved
    
    Chain-->>Runtime: Confirmation
    Runtime-->>App: Update UI
    App-->>User: Display Result
    
    Note over ZK: Private ZK proving<br>happens off-chain
    Note over Chain: Lightweight verification<br>happens on-chain
    Note over App,Runtime: Hoon applications run<br>on the Sword VM
```

## Zero-Knowledge System Architecture

```mermaid
flowchart TB
    subgraph "Zero-Knowledge System"
        subgraph "Proving Layer"
            STARK_PROVER[STARK Prover]
            JETPACK[ZK VM Jetpack]
            ARITHMETIZATION[Arithmetization]
            POLY_COMMIT[Polynomial Commitments]
        end
        
        subgraph "Verification Layer"
            STARK_VERIFY[STARK Verifier]
            CONSTRAINT_VERIFY[Constraint Verification]
            FRI_VERIFY[FRI Protocol]
        end
        
        subgraph "Cryptographic Primitives"
            HASH[Hash Functions]
            MERKLE[Merkle Trees]
            FEC[Forward Error Correction]
            CRYPTO_FIELD[Finite Field Arithmetic]
        end
        
        subgraph "Integration Layer"
            SWORD_INTEGRATION[Sword VM Integration]
            CONSENSUS_INTEGRATION[Consensus Integration]
            TX_INTEGRATION[Transaction Integration]
        end
        
        %% Internal connections
        STARK_PROVER --> ARITHMETIZATION
        STARK_PROVER --> POLY_COMMIT
        JETPACK --> STARK_PROVER
        
        STARK_VERIFY --> CONSTRAINT_VERIFY
        STARK_VERIFY --> FRI_VERIFY
        
        HASH <--> MERKLE
        HASH <--> FEC
        CRYPTO_FIELD <--> HASH
        CRYPTO_FIELD <--> POLY_COMMIT
        
        SWORD_INTEGRATION <--> STARK_PROVER
        SWORD_INTEGRATION <--> STARK_VERIFY
        CONSENSUS_INTEGRATION <--> STARK_VERIFY
        TX_INTEGRATION <--> STARK_PROVER
        TX_INTEGRATION <--> STARK_VERIFY
    end
    
    classDef proving fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef verification fill:#d3f8e2,stroke:#333,stroke-width:1px
    classDef crypto fill:#e3f6f5,stroke:#333,stroke-width:1px
    classDef integration fill:#eeeeee,stroke:#333,stroke-width:1px
    
    class STARK_PROVER,JETPACK,ARITHMETIZATION,POLY_COMMIT proving
    class STARK_VERIFY,CONSTRAINT_VERIFY,FRI_VERIFY verification
    class HASH,MERKLE,FEC,CRYPTO_FIELD crypto
    class SWORD_INTEGRATION,CONSENSUS_INTEGRATION,TX_INTEGRATION integration
```

## Technology Stack

```mermaid
graph BT
    subgraph "Application Layer"
        HOON_APPS[Hoon Applications]
        RUST_APPS[Rust Applications]
        CLI[Command Line Tools]
        API[API Services]
    end
    
    subgraph "Runtime Layer"
        SWORD[Sword VM]
        NOCK[Nock Runtime]
        JETS[Jet Acceleration]
        COMP_MODEL[Computation Model]
    end
    
    subgraph "Blockchain Layer"
        CONSENSUS[Consensus]
        LIBP2P[LibP2P Network]
        STORAGE[Storage Engine]
        TX_POOL[Transaction Pool]
        SYNC[Block Synchronization]
    end
    
    subgraph "Cryptographic Layer"
        ZK_PROOFS[ZK Proofs]
        CRYPTO[Cryptographic Primitives]
        SIG[Signature Schemes]
        HASHING[Hash Functions]
        COMMITMENT[Commitment Schemes]
    end
    
    %% Layer connections
    HOON_APPS --> SWORD
    RUST_APPS --> NOCK
    CLI --> RUST_APPS
    API --> HOON_APPS
    API --> RUST_APPS
    
    SWORD --> NOCK
    NOCK --> JETS
    NOCK --> COMP_MODEL
    JETS --> CONSENSUS
    JETS --> ZK_PROOFS
    
    CONSENSUS --> LIBP2P
    CONSENSUS --> STORAGE
    CONSENSUS --> TX_POOL
    CONSENSUS --> SYNC
    CONSENSUS --> CRYPTO
    
    ZK_PROOFS --> CRYPTO
    ZK_PROOFS --> SIG
    ZK_PROOFS --> HASHING
    ZK_PROOFS --> COMMITMENT
    
    style HOON_APPS fill:#f9d5e5,stroke:#333,stroke-width:1px
    style RUST_APPS fill:#d3f8e2,stroke:#333,stroke-width:1px
    style CLI fill:#f9d5e5,stroke:#333,stroke-width:1px
    style API fill:#d3f8e2,stroke:#333,stroke-width:1px
    
    style SWORD fill:#eeeeee,stroke:#333,stroke-width:1px
    style NOCK fill:#eeeeee,stroke:#333,stroke-width:1px
    style JETS fill:#eeeeee,stroke:#333,stroke-width:1px
    style COMP_MODEL fill:#eeeeee,stroke:#333,stroke-width:1px
    
    style CONSENSUS fill:#e3f6f5,stroke:#333,stroke-width:1px
    style LIBP2P fill:#e3f6f5,stroke:#333,stroke-width:1px
    style STORAGE fill:#e3f6f5,stroke:#333,stroke-width:1px
    style TX_POOL fill:#e3f6f5,stroke:#333,stroke-width:1px
    style SYNC fill:#e3f6f5,stroke:#333,stroke-width:1px
    
    style ZK_PROOFS fill:#f9ebe0,stroke:#333,stroke-width:1px
    style CRYPTO fill:#f9ebe0,stroke:#333,stroke-width:1px
    style SIG fill:#f9ebe0,stroke:#333,stroke-width:1px
    style HASHING fill:#f9ebe0,stroke:#333,stroke-width:1px
    style COMMITMENT fill:#f9ebe0,stroke:#333,stroke-width:1px
```

## Project Structure

```mermaid
classDiagram
    class Nockchain {
        +hoon
        +crates
        +docs
        +scripts
        +resources
    }
    
    class Hoon {
        +apps
        +common
        +test-jams
        +trivial.hoon
    }
    
    class Crates {
        +nockchain
        +sword
        +nockapp
        +auxiliary crates
    }
    
    class HoonApps {
        +dumbnet
        +wallet
    }
    
    class HoonCommon {
        +table
        +markdown
        +stark
        +ztd
        +wrapper.hoon
        +zeke.hoon
        +zoon.hoon
        +zose.hoon
        +test.hoon
        +tx-engine.hoon
        +nock-common.hoon
        +nock-prover.hoon
        +nock-verifier.hoon
        +pow.hoon
        +schedule.hoon
        +slip10.hoon
        +bip39.hoon
    }
    
    class CoreCrates {
        +nockchain: blockchain implementation
        +nockapp: application framework
        +sword: Nock interpreter
    }
    
    class AuxiliaryCrates {
        +equix-latency
        +nockchain-bitcoin-sync
        +nockchain-libp2p-io
        +kernels
        +zkvm-jetpack
    }
    
    class Documentation {
        +README.md
        +hoon/
        +crates/
    }
    
    Nockchain --> Hoon
    Nockchain --> Crates
    Nockchain --> Documentation
    
    Hoon --> HoonApps
    Hoon --> HoonCommon
    
    Crates --> CoreCrates
    Crates --> AuxiliaryCrates
    
    Documentation ..> Hoon: documents
    Documentation ..> Crates: documents
```

## Data Flow Architecture

```mermaid
graph LR
    subgraph "External"
        USER[User]
        EXT_CHAIN[External Blockchain]
    end
    
    subgraph "Client Layer"
        WALLET[Wallet App]
        UI[User Interface]
        CLI[Command Line Interface]
    end
    
    subgraph "Network Layer"
        P2P[P2P Network]
        SYNC[Sync Service]
        DISCOVERY[Peer Discovery]
        MSG_ROUTER[Message Router]
    end
    
    subgraph "Execution Layer"
        TX_ENGINE[Transaction Engine]
        SWORD[Sword VM]
        ZK[ZK Proving System]
        JETS[Optimized Jets]
    end
    
    subgraph "Storage Layer"
        CHAIN_DB[Chain Database]
        STATE_DB[State Database]
        MERKLE_DB[Merkle Tree Store]
        TX_POOL[Transaction Pool]
    end
    
    %% External connections
    USER <--> WALLET
    USER <--> UI
    USER <--> CLI
    EXT_CHAIN <--> SYNC
    
    %% Client to Network
    WALLET --> P2P
    UI --> P2P
    CLI --> P2P
    
    %% Network interconnections
    P2P <--> DISCOVERY
    P2P <--> MSG_ROUTER
    SYNC <--> MSG_ROUTER
    
    %% Network to Execution
    P2P --> TX_ENGINE
    SYNC --> TX_ENGINE
    
    %% Execution interconnections
    TX_ENGINE <--> SWORD
    SWORD <--> ZK
    SWORD <--> JETS
    
    %% Execution to Storage
    TX_ENGINE --> CHAIN_DB
    TX_ENGINE --> TX_POOL
    SWORD --> STATE_DB
    ZK --> MERKLE_DB
    
    %% Feedback loops
    CHAIN_DB --> SYNC
    STATE_DB --> UI
    TX_POOL --> TX_ENGINE
    
    style USER fill:#f9d5e5,stroke:#333,stroke-width:1px
    style EXT_CHAIN fill:#f9d5e5,stroke:#333,stroke-width:1px
    style WALLET fill:#d3f8e2,stroke:#333,stroke-width:1px
    style UI fill:#d3f8e2,stroke:#333,stroke-width:1px
    style CLI fill:#d3f8e2,stroke:#333,stroke-width:1px
    style P2P fill:#e3f6f5,stroke:#333,stroke-width:1px
    style SYNC fill:#e3f6f5,stroke:#333,stroke-width:1px
    style DISCOVERY fill:#e3f6f5,stroke:#333,stroke-width:1px
    style MSG_ROUTER fill:#e3f6f5,stroke:#333,stroke-width:1px
    style TX_ENGINE fill:#f9ebe0,stroke:#333,stroke-width:1px
    style SWORD fill:#f9ebe0,stroke:#333,stroke-width:1px
    style ZK fill:#f9ebe0,stroke:#333,stroke-width:1px
    style JETS fill:#f9ebe0,stroke:#333,stroke-width:1px
    style CHAIN_DB fill:#eeeeee,stroke:#333,stroke-width:1px
    style STATE_DB fill:#eeeeee,stroke:#333,stroke-width:1px
    style MERKLE_DB fill:#eeeeee,stroke:#333,stroke-width:1px
    style TX_POOL fill:#eeeeee,stroke:#333,stroke-width:1px
```

## Transaction Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Created: User creates transaction
    Created --> Submitted: Transaction submitted to network
    Submitted --> Pending: Transaction enters mempool
    Pending --> Executing: Transaction selected for block
    Executing --> Proving: Transaction execution complete
    Proving --> Verified: Proof generated and verified
    Verified --> Confirmed: Transaction included in block
    Confirmed --> Finalized: Block finalized
    Finalized --> [*]
    
    Submitted --> Rejected: Invalid transaction
    Pending --> Expired: Transaction timeout
    Executing --> Failed: Execution error
    Proving --> Invalid: Proof verification fails
    
    Rejected --> [*]
    Expired --> [*]
    Failed --> [*]
    Invalid --> [*]
    
    note right of Created: Transaction created by user or application
    note right of Submitted: Transaction sent to the network
    note right of Pending: Waiting in the mempool for processing
    note right of Executing: Running on the Sword VM
    note right of Proving: Generating ZK proof
    note right of Verified: Proof validated by verifier
    note right of Confirmed: Included in a valid block
    note right of Finalized: Block is confirmed and finalized
```

## Consensus Algorithm

```mermaid
flowchart TD
    A[New Transaction] --> B{In Mempool?}
    B -->|Yes| C[Already Processing]
    B -->|No| D[Add to Mempool]
    D --> E[Block Proposal]
    
    E --> F{Valid Transactions?}
    F -->|No| G[Reject Invalid Tx]
    F -->|Yes| H[Create Block Proposal]
    
    H --> I[Broadcast Block]
    I --> J{Consensus Round}
    J -->|Insufficient Votes| K[Wait for Votes]
    K --> J
    J -->|Sufficient Votes| L[Block Confirmed]
    
    L --> M[Execute Transactions]
    M --> N[Verify ZK Proofs]
    N --> O[Update State]
    O --> P[Finalize Block]
    
    style A fill:#f9d5e5,stroke:#333,stroke-width:1px
    style B fill:#d3f8e2,stroke:#333,stroke-width:1px
    style D fill:#e3f6f5,stroke:#333,stroke-width:1px
    style E fill:#f9ebe0,stroke:#333,stroke-width:1px
    style H fill:#eeeeee,stroke:#333,stroke-width:1px
    style J fill:#bbf,stroke:#333,stroke-width:2px
    style L fill:#bfb,stroke:#333,stroke-width:2px
    style N fill:#fbb,stroke:#333,stroke-width:2px
    style P fill:#fbf,stroke:#333,stroke-width:2px
```

## Contents

- [Hoon](./hoon/): Documentation for [Hoon](../hoon/) components
  - [Apps](./hoon/apps.md): Documentation for Hoon applications
  - [Common](./hoon/common.md): Documentation for common Hoon libraries
  - [Test Jams](./hoon/test-jams.md): Documentation for test jam files
- [Crates](./crates/): Documentation for [Rust crates](../crates/)

## Component Relationships

```mermaid
graph TD
    subgraph "Documentation Structure"
        DOCS[Nockchain Documentation]
        
        HOON_DOCS[Hoon Documentation]
        APPS_DOCS[Apps Documentation]
        COMMON_DOCS[Common Documentation]
        TEST_JAMS_DOCS[Test Jams Documentation]
        
        CRATES_DOCS[Crates Documentation]
        CORE_CRATES_DOCS[Core Crates Documentation]
        AUX_CRATES_DOCS[Auxiliary Crates Documentation]
        
        DOCS --> HOON_DOCS
        DOCS --> CRATES_DOCS
        
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
        
        class DOCS,HOON_DOCS,CRATES_DOCS docsNode
        class APPS_DOCS,WALLET_DOCS,DUMBNET_DOCS appsNode
        class COMMON_DOCS,TABLE_DOCS,STARK_DOCS,MARKDOWN_DOCS,ZTD_DOCS,CORE_COMPONENTS_DOCS libNode
        class CORE_CRATES_DOCS,AUX_CRATES_DOCS,NOCKCHAIN_DOCS,NOCKAPP_DOCS,SWORD_DOCS,EQUIX_DOCS,BTC_SYNC_DOCS,LIBP2P_DOCS,KERNELS_DOCS,ZKVM_DOCS crateNode
        class TEST_JAMS_DOCS libNode
    end
```

## Getting Started

To get started with Nockchain, refer to the main [README](../README.md) for setup instructions, including:

- Installing dependencies
- Building the codebase
- Running a node
- Testing the system

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

## Development Workflow

```mermaid
gitGraph
    commit id: "Initial Setup"
    branch develop
    checkout develop
    commit id: "Core Components"
    branch feature/sword-vm
    checkout feature/sword-vm
    commit id: "Basic VM"
    commit id: "Add Jets"
    checkout develop
    merge feature/sword-vm
    branch feature/hoon-apps
    checkout feature/hoon-apps
    commit id: "Wallet App"
    commit id: "DumbNet App"
    checkout develop
    merge feature/hoon-apps
    branch feature/zk-system
    checkout feature/zk-system
    commit id: "Prover"
    commit id: "Verifier"
    checkout develop
    merge feature/zk-system
    checkout main
    merge develop tag: "v0.1.0"
    branch hotfix/security
    checkout hotfix/security
    commit id: "Security Fix"
    checkout main
    merge hotfix/security tag: "v0.1.1"
    checkout develop
    merge hotfix/security
    commit id: "New Features"
    checkout main
    merge develop tag: "v0.2.0"
```

## Performance Metrics

```mermaid
xychart-beta
    title "Nockchain Performance Metrics"
    x-axis [10, 100, 1000, 10000]
    y-axis "Transactions/sec" 0 --> 1000
    bar [50, 450, 900, 850]
    line [30, 300, 750, 700]
    
```

## Further Reading

- [DEVELOPERS.md](../crates/sword/DEVELOPERS.md): Development guidelines for Sword
- [Makefile](../Makefile): Build commands and project structure 