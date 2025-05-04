# Common Hoon Libraries

## Overview

This document describes the [common Hoon libraries and utilities](../../hoon/common/) in the [Nockchain](../../) project. These libraries provide essential functionality used across the Nockchain platform and applications.

```mermaid
graph TD
    subgraph "Common Hoon Libraries"
        CORE["Core System Components"]
        CRYPTO["Cryptographic Components"]
        UTIL["Utilities"]
        
        subgraph "Domain Libraries"
            MARKDOWN["Markdown"]
            STARK["STARK"]
            TABLE["Table"]
            ZTD["ZTD Library"]
        end
    end
    
    %% Connections
    CORE --> UTIL
    CRYPTO --> UTIL
    DOMAIN_LIBS --> UTIL
    
    MARKDOWN --> DOMAIN_LIBS
    STARK --> DOMAIN_LIBS
    TABLE --> DOMAIN_LIBS
    ZTD --> DOMAIN_LIBS
    
    %% External connections
    APPS["Hoon Applications"] --> CORE
    APPS --> CRYPTO
    APPS --> DOMAIN_LIBS
    
    classDef core fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef crypto fill:#d3f8e2,stroke:#333,stroke-width:1px
    classDef util fill:#e3f6f5,stroke:#333,stroke-width:1px
    classDef domainlib fill:#f9ebe0,stroke:#333,stroke-width:1px
    classDef apps fill:#eeeeee,stroke:#333,stroke-width:1px
    
    class CORE core
    class CRYPTO crypto
    class UTIL util
    class MARKDOWN,STARK,TABLE,ZTD,DOMAIN_LIBS domainlib
    class APPS apps
```

## Libraries

### Markdown

Located in [`hoon/common/markdown/`](../../hoon/common/markdown/).

A library for Markdown processing that provides:
- Parsing of Markdown syntax
- Conversion to HTML and other formats
- Document structure manipulation
- Text formatting utilities

```mermaid
classDiagram
    class MarkdownLibrary {
        +Parser
        +Renderer
        +Transformer
        +Utils
    }
    
    class Parser {
        +parseMd(text): AST
        +tokenize(text): tokens
        +parseInline(text): inlineAST
    }
    
    class Renderer {
        +toHTML(ast): html
        +toText(ast): text
        +toJSON(ast): json
    }
    
    class Transformer {
        +transform(ast, rules): ast
        +addAttributes(ast, attrs): ast
        +filter(ast, predicate): ast
    }
    
    class Utils {
        +escape(text): text
        +unescape(text): text
        +normalize(text): text
    }
    
    MarkdownLibrary --> Parser
    MarkdownLibrary --> Renderer
    MarkdownLibrary --> Transformer
    MarkdownLibrary --> Utils
```

### STARK

Located in [`hoon/common/stark/`](../../hoon/common/stark/).

Implementation of STARK (Scalable Transparent ARguments of Knowledge) for zero-knowledge proofs:
- Proof generation algorithms
- Verification procedures
- Field arithmetic operations
- Polynomial commitment schemes

```mermaid
graph TD
    subgraph "STARK Implementation"
        PROVER["Prover"]
        VERIFIER["Verifier"]
        FIELD["Field Arithmetic"]
        POLY["Polynomial Operations"]
        TRACE["Execution Trace"]
        COMMIT["Commitments"]
        FRI["FRI Protocol"]
    end
    
    %% Connections
    PROVER --> FIELD
    PROVER --> POLY
    PROVER --> TRACE
    PROVER --> COMMIT
    PROVER --> FRI
    
    VERIFIER --> FIELD
    VERIFIER --> POLY
    VERIFIER --> COMMIT
    VERIFIER --> FRI
    
    classDef prover fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef verifier fill:#d3f8e2,stroke:#333,stroke-width:1px
    classDef math fill:#e3f6f5,stroke:#333,stroke-width:1px
    
    class PROVER prover
    class VERIFIER verifier
    class FIELD,POLY,TRACE,COMMIT,FRI math
```

### Table

Located in [`hoon/common/table/`](../../hoon/common/table/).

Table data structure implementation with prover and verifier components:
- Efficient key-value storage
- Range-query operations
- Merkle tree integration
- Proof generation for table operations

```mermaid
classDiagram
    class Table {
        +put(key, value): table
        +get(key): value
        +del(key): table
        +has(key): bool
        +keys(): list
        +values(): list
        +size(): atom
        +prove(key): proof
        +verifyProof(key, value, proof): bool
    }
    
    class TableImpl {
        -tree: binary tree
        -merkleRoot: hash
        +put(key, value): table
        +get(key): value
        +merkleProof(key): proof
    }
    
    class MerkleTree {
        -root: node
        -depth: atom
        +getRoot(): hash
        +getProof(key): proof
        +verify(key, value, proof): bool
    }
    
    Table <|-- TableImpl
    TableImpl --> MerkleTree
```

### ZTD

Located in [`hoon/common/ztd/`](../../hoon/common/ztd/).

Standard library extensions and utilities (Zero-knowledge Toolkit and Dependencies):
- Common data structures
- String manipulation functions
- Mathematical utilities
- Type conversion helpers

```mermaid
graph TD
    subgraph "ZTD Library"
        DATA["Data Structures"]
        STRING["String Utils"]
        MATH["Math Utils"]
        CONVERT["Type Conversion"]
        ITER["Iterators"]
        TIME["Time Utils"]
    end
    
    %% Usage examples
    DATA --> APPS["Applications"]
    STRING --> APPS
    MATH --> APPS
    CONVERT --> APPS
    ITER --> APPS
    TIME --> APPS
    
    classDef ztd fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef app fill:#d3f8e2,stroke:#333,stroke-width:1px
    
    class DATA,STRING,MATH,CONVERT,ITER,TIME ztd
    class APPS app
```

## Core System Components

```mermaid
graph TD
    subgraph "Core System Components"
        NOCK_COMMON["nock-common.hoon"]
        NOCK_PROVER["nock-prover.hoon"]
        NOCK_VERIFIER["nock-verifier.hoon"]
    end
    
    NOCK_COMMON -->|used by| NOCK_PROVER
    NOCK_COMMON -->|used by| NOCK_VERIFIER
    NOCK_PROVER -->|generates proofs for| NOCK_VERIFIER
    
    %% External components
    VM["Sword VM"] -->|executes| NOCK_COMMON
    ZK["ZK System"] -->|implements| NOCK_PROVER
    ZK -->|implements| NOCK_VERIFIER
    
    classDef core fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef external fill:#d3f8e2,stroke:#333,stroke-width:1px
    
    class NOCK_COMMON,NOCK_PROVER,NOCK_VERIFIER core
    class VM,ZK external
```

- [`nock-common.hoon`](../../hoon/common/nock-common.hoon): Common Nock functions for VM interaction
  - Core Nock interpreter functions
  - Memory management helpers
  - Operation optimizations
  - Debugging utilities

- [`nock-prover.hoon`](../../hoon/common/nock-prover.hoon): Nock proof generation for verifiable computation
  - Execution trace generation
  - Constraint system construction
  - Proof serialization
  - Integration with STARK library

- [`nock-verifier.hoon`](../../hoon/common/nock-verifier.hoon): Nock proof verification system
  - Proof parsing and validation
  - Constraint checking
  - Verification algorithm implementation
  - Integration with consensus system

## Cryptographic Components

```mermaid
graph TD
    subgraph "Cryptographic Components"
        POW["pow.hoon"]
        SLIP10["slip10.hoon"]
        BIP39["bip39.hoon"]
        BIP39_ENG["bip39-english.hoon"]
    end
    
    BIP39 --> BIP39_ENG
    BIP39 --> SLIP10
    
    %% External components
    WALLET["Wallet App"] --> BIP39
    WALLET --> SLIP10
    BLOCKCHAIN["Blockchain"] --> POW
    
    classDef crypto fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef external fill:#d3f8e2,stroke:#333,stroke-width:1px
    
    class POW,SLIP10,BIP39,BIP39_ENG crypto
    class WALLET,BLOCKCHAIN external
```

- [`pow.hoon`](../../hoon/common/pow.hoon): Proof-of-work implementation for blockchain consensus
  - Hash function implementation
  - Difficulty adjustment
  - Mining algorithm
  - Block validation

- [`slip10.hoon`](../../hoon/common/slip10.hoon): SLIP-10 hierarchical deterministic key derivation
  - Master key generation
  - Child key derivation
  - Path navigation
  - Key serialization

- [`bip39.hoon`](../../hoon/common/bip39.hoon): BIP-39 mnemonic code implementation
  - Entropy to mnemonic conversion
  - Mnemonic validation
  - Mnemonic to seed conversion
  - Checksum validation

- [`bip39-english.hoon`](../../hoon/common/bip39-english.hoon): English wordlist for BIP-39
  - 2048 word dictionary
  - Word lookup functions
  - Index-to-word mapping
  - Word-to-index mapping

## Other Utilities

```mermaid
graph TD
    subgraph "Utility Components"
        WRAPPER["wrapper.hoon"]
        ZEKE["zeke.hoon"]
        ZOON["zoon.hoon"]
        ZOSE["zose.hoon"]
        TEST["test.hoon"]
        TX_ENGINE["tx-engine.hoon"]
        SCHEDULE["schedule.hoon"]
    end
    
    %% Relations
    ZOSE --> ZOON
    ZOON --> ZEKE
    TX_ENGINE --> ZOON
    TEST --> ZOON
    SCHEDULE --> ZOON
    
    %% Usage examples
    APPS["Applications"] --> WRAPPER
    APPS --> ZOON
    APPS --> ZOSE
    APPS --> TX_ENGINE
    APPS --> SCHEDULE
    TEST_SUITE["Test Suite"] --> TEST
    
    classDef util fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef usage fill:#d3f8e2,stroke:#333,stroke-width:1px
    
    class WRAPPER,ZEKE,ZOON,ZOSE,TEST,TX_ENGINE,SCHEDULE util
    class APPS,TEST_SUITE usage
```

- [`wrapper.hoon`](../../hoon/common/wrapper.hoon): Function wrappers and higher-order functions
  - Error handling wrappers
  - Function composition helpers
  - Currying and partial application
  - Tracing and debugging wrappers

- [`zeke.hoon`](../../hoon/common/zeke.hoon): Runtime utilities for efficient execution
  - Memory optimization helpers
  - Fast path operations
  - Runtime configuration
  - Performance monitoring

- [`zoon.hoon`](../../hoon/common/zoon.hoon): Comprehensive standard library extensions
  - Enhanced list operations
  - Advanced text processing
  - Extended math functions
  - Improved data structure operations

- [`zose.hoon`](../../hoon/common/zose.hoon): Zero-knowledge system environment
  - ZK runtime configuration
  - Proof system integration
  - Constraint system helpers
  - Verification utilities

- [`test.hoon`](../../hoon/common/test.hoon): Test framework and utilities
  - Unit testing framework
  - Assertion helpers
  - Test reporting
  - Mocking utilities

- [`tx-engine.hoon`](../../hoon/common/tx-engine.hoon): Transaction processing engine
  - Transaction validation
  - State transition logic
  - Fee calculation
  - Transaction batching

- [`schedule.hoon`](../../hoon/common/schedule.hoon): Job scheduling and event handling
  - Time-based scheduling
  - Event queue management
  - Recurring job handling
  - Priority queue implementation

## Library Dependencies

```mermaid
flowchart TD
    subgraph "Applications"
        WALLET["Wallet App"]
        DUMBNET["DumbNet App"]
    end
    
    subgraph "Core Components"
        TX_ENGINE["Transaction Engine"]
        NOCK_COMMON["Nock Common"]
        NOCK_PROVER["Nock Prover"]
        NOCK_VERIFIER["Nock Verifier"]
    end
    
    subgraph "Domain Libraries"
        TABLE["Table"]
        STARK["STARK"]
        MARKDOWN["Markdown"]
        ZTD["ZTD"]
    end
    
    subgraph "Cryptographic Components"
        POW["Proof of Work"]
        SLIP10["SLIP-10"]
        BIP39["BIP-39"]
    end
    
    subgraph "Utilities"
        ZOON["Zoon"]
        ZOSE["Zose"]
        WRAPPER["Wrapper"]
        TEST["Test"]
        SCHEDULE["Schedule"]
    end
    
    %% Application dependencies
    WALLET --> TX_ENGINE
    WALLET --> TABLE
    WALLET --> BIP39
    WALLET --> SLIP10
    WALLET --> ZTD
    
    DUMBNET --> TX_ENGINE
    DUMBNET --> ZTD
    DUMBNET --> SCHEDULE
    
    %% Core component dependencies
    TX_ENGINE --> NOCK_COMMON
    TX_ENGINE --> ZOON
    
    NOCK_PROVER --> NOCK_COMMON
    NOCK_PROVER --> STARK
    
    NOCK_VERIFIER --> NOCK_COMMON
    NOCK_VERIFIER --> STARK
    
    %% Library interdependencies
    STARK --> ZTD
    TABLE --> ZTD
    MARKDOWN --> ZTD
    
    %% Utility dependencies
    ZOSE --> ZOON
    TEST --> ZOON
    SCHEDULE --> ZOON
    
    classDef app fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef core fill:#d3f8e2,stroke:#333,stroke-width:1px
    classDef domain fill:#e3f6f5,stroke:#333,stroke-width:1px
    classDef crypto fill:#f9ebe0,stroke:#333,stroke-width:1px
    classDef util fill:#eeeeee,stroke:#333,stroke-width:1px
    
    class WALLET,DUMBNET app
    class TX_ENGINE,NOCK_COMMON,NOCK_PROVER,NOCK_VERIFIER core
    class TABLE,STARK,MARKDOWN,ZTD domain
    class POW,SLIP10,BIP39 crypto
    class ZOON,ZOSE,WRAPPER,TEST,SCHEDULE util
``` 