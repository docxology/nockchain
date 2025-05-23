# Sword

## Overview

The [`sword`](../../crates/sword/) crate is the Nock interpreter and runtime environment for the [Nockchain](../../) project. It provides the core execution environment for Nock code with automatic persistence and optimization capabilities.

```mermaid
graph TD
    subgraph "Sword VM"
        NOCK_INT[Nock Interpreter]
        JETS[Jet System]
        MEMORY[Memory Management]
        PERSIST[Persistence Layer]
        EVENT[Event System]
    end
    
    NOCK_INT --> MEMORY
    JETS --> NOCK_INT
    NOCK_INT --> PERSIST
    NOCK_INT --> EVENT
    
    %% External interactions
    HOON[Hoon Code] --> NOCK_INT
    APPS[Applications] --> EVENT
    NETWORK[Network Layer] --> EVENT
    STORAGE[Storage] --> PERSIST
    
    classDef core fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef internal fill:#d3f8e2,stroke:#333,stroke-width:1px
    classDef external fill:#e3f6f5,stroke:#333,stroke-width:1px
    
    class NOCK_INT core
    class JETS,MEMORY,PERSIST,EVENT internal
    class HOON,APPS,NETWORK,STORAGE external
```

## Directory Structure

Located in [`crates/sword/`](../../crates/sword/).

### Key Subdirectories

- [`rust/sword/`](../../crates/sword/rust/sword/): Core implementation
  - [`src/jets/`](../../crates/sword/rust/sword/src/jets/): Jet (native function) implementations for optimized execution
  - [`src/substantive/`](../../crates/sword/rust/sword/src/substantive/): Core data structures and algorithms
- [`rust/sword_crypto/`](../../crates/sword/rust/sword_crypto/): Cryptographic primitives for secure operations
- [`rust/sword_macros/`](../../crates/sword/rust/sword_macros/): Procedural macros for code generation
- [`rust/ibig/`](../../crates/sword/rust/ibig/): Big integer implementation for arbitrary-precision arithmetic
- [`rust/murmur3/`](../../crates/sword/rust/murmur3/): Murmur3 hash implementation for fast hashing
- [`rust/assert_no_alloc/`](../../crates/sword/rust/assert_no_alloc/): Allocation control utilities for memory management
- [`hoon/`](../../crates/sword/hoon/): Hoon integration components including codegen
- [`docs/`](../../crates/sword/docs/): Design documents and specifications
- [`resources/pills/`](../../crates/sword/resources/pills/): Pill files for testing and development

```mermaid
classDiagram
    class Sword {
        +Interpreter
        +Jets
        +Memory management
        +Persistence
    }
    
    class SwordImpl {
        +rust/sword/
        +rust/sword_crypto/
        +rust/sword_macros/
        +rust/ibig/
        +rust/murmur3/
        +rust/assert_no_alloc/
    }
    
    class HoonIntegration {
        +hoon/
        +codegen
        +pill support
    }
    
    class Documentation {
        +docs/
        +resources/pills/
    }
    
    Sword <|-- SwordImpl
    Sword <|-- HoonIntegration
    Sword <|-- Documentation
```

## Pills

Sword uses various pill files for development and testing, located in `resources/pills/`:

- **baby.pill**: A minimal Arvo-shaped core and Hoon standard library
- **toddler.pill**: A slightly more complex Arvo and Hoon for testing jets
- **azimuth.pill**: A pill that processes an Azimuth snapshot
- **full.pill**: The complete Urbit v2.11 pill
- **slim.pill**: A slimmed down version of the Urbit v2.11 pill

```mermaid
graph LR
    subgraph "Pills"
        BABY[baby.pill]
        TODDLER[toddler.pill]
        AZIMUTH[azimuth.pill]
        FULL[full.pill]
        SLIM[slim.pill]
    end
    
    %% Size relationships
    BABY -->|smaller than| TODDLER
    TODDLER -->|smaller than| AZIMUTH
    AZIMUTH -->|smaller than| SLIM
    SLIM -->|smaller than| FULL
    
    %% Purpose
    BABY -->|"Minimal core + stdlib"| DEV[Development]
    TODDLER -->|"Test jets"| TEST[Testing]
    AZIMUTH -->|"Process Azimuth snapshot"| AZIMUTH_TEST[Azimuth Testing]
    FULL -->|"Complete Urbit v2.11"| COMPLETE[Compatibility]
    SLIM -->|"Slimmed v2.11"| PROD[Production]
    
    classDef pill fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef usage fill:#d3f8e2,stroke:#333,stroke-width:1px
    
    class BABY,TODDLER,AZIMUTH,FULL,SLIM pill
    class DEV,TEST,AZIMUTH_TEST,COMPLETE,PROD usage
```

## Components

### Nock Interpreter

```mermaid
graph LR
    subgraph "Nock Interpreter"
        PARSER[Nock Parser]
        EVAL[Evaluator]
        EXEC[Executor]
        OPT[Optimizer]
    end
    
    INPUT[Nock Code] --> PARSER
    PARSER --> OPT
    OPT --> EVAL
    EVAL --> EXEC
    EXEC --> OUTPUT[Result]
    
    %% Optimization paths
    OPT -->|jet hint| JET[Jet System]
    JET --> EXEC
    
    classDef core fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef external fill:#d3f8e2,stroke:#333,stroke-width:1px
    
    class PARSER,EVAL,EXEC,OPT core
    class INPUT,OUTPUT,JET external
```

- Core Nock interpreter with efficient evaluation
- Advanced optimizer for common patterns
- Support for all Nock operations
- Structured error handling
- Performance profiling and metrics

### Jet System

```mermaid
graph TD
    subgraph "Jet System"
        REGISTRY[Jet Registry]
        MATCHER[Pattern Matcher]
        DISPATCH[Dispatcher]
        VERIFY[Result Verifier]
    end
    
    NOCK[Nock Code] --> MATCHER
    MATCHER -->|match found| REGISTRY
    REGISTRY --> DISPATCH
    DISPATCH --> NATIVE[Native Implementation]
    NATIVE --> RESULT[Result]
    RESULT --> VERIFY
    VERIFY -->|verification| NOCK_RES[Nock Result]
    
    %% Fallback path
    MATCHER -->|no match| NOCK_INTERP[Nock Interpreter]
    NOCK_INTERP --> NOCK_RES
    
    classDef jet fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef external fill:#d3f8e2,stroke:#333,stroke-width:1px
    
    class REGISTRY,MATCHER,DISPATCH,VERIFY jet
    class NOCK,NATIVE,RESULT,NOCK_RES,NOCK_INTERP external
```

- Advanced jet (accelerated function) system for performance-critical operations
- Automatic registration and discovery of jets
- Result verification for correctness
- Hierarchical jet organization
- Fast pattern matching for jet detection

### Memory Management

```mermaid
classDiagram
    class MemoryManager {
        +allocate(size): pointer
        +deallocate(pointer)
        +gc(): stats
        +stats(): memory_stats
    }
    
    class NoAlloc {
        +assert_no_alloc()
        +with_allocations(fn)
        +track_allocations(fn)
    }
    
    class Heap {
        -blocks: vector
        -free_blocks: list
        +allocate(size): block
        +free(block)
        +compact()
    }
    
    class Cache {
        -entries: hashmap
        -policy: cache_policy
        +get(key): value
        +put(key, value)
        +evict()
    }
    
    MemoryManager --> Heap
    MemoryManager --> Cache
    MemoryManager --> NoAlloc
```

- Efficient memory management for Nock data structures
- Automatic garbage collection
- Memory pooling for common object sizes
- Cache system for frequently accessed data
- Low-overhead allocation tracking

### Persistence Layer

```mermaid
sequenceDiagram
    participant APP as Application
    participant SWORD as Sword VM
    participant CACHE as Memory Cache
    participant DB as Storage Backend
    
    APP->>SWORD: Execute code
    SWORD->>CACHE: Check for data
    alt Data in cache
        CACHE-->>SWORD: Return cached data
    else Data not in cache
        CACHE->>DB: Load data
        DB-->>CACHE: Return data
        CACHE-->>SWORD: Provide data
    end
    
    SWORD->>SWORD: Process data
    SWORD->>CACHE: Update data
    CACHE->>CACHE: Mark as dirty
    
    opt Persistence trigger
        CACHE->>DB: Write dirty data
        DB-->>CACHE: Acknowledge
    end
    
    SWORD-->>APP: Return result
```

- Automatic persistence of Nock computations
- Efficient serialization/deserialization
- Transaction support for atomic operations
- Incremental state snapshots
- Background persistence with minimal impact on performance

## Usage

The `sword` crate is used to:

```mermaid
graph LR
    subgraph "Sword Usage Scenarios"
        EXEC[Execute Nock Code]
        ACCEL[Accelerate Operations]
        MANAGE[Manage Resources]
        IO[Interface with Host]
        STORE[Persistent Storage]
    end
    
    APPS[Applications] -->|use| EXEC
    APPS -->|leverage| ACCEL
    APPS -->|rely on| MANAGE
    APPS -->|interact via| IO
    APPS -->|depend on| STORE
    
    classDef usage fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef app fill:#d3f8e2,stroke:#333,stroke-width:1px
    
    class EXEC,ACCEL,MANAGE,IO,STORE usage
    class APPS app
```

- Execute Nock code with high performance
- Provide accelerated functions via jets for common operations
- Manage memory and resources efficiently
- Interface with the host environment for I/O
- Support persistent storage of Nock computations

## Implementation Details

### Noun Structure

```mermaid
classDiagram
    class Noun {
        <<interface>>
        +is_atom(): bool
        +is_cell(): bool
        +equals(Noun): bool
        +hash(): u64
    }
    
    class Atom {
        -value: BigInt
        +get_value(): BigInt
        +to_u64(): u64
        +to_i64(): i64
    }
    
    class Cell {
        -head: Noun
        -tail: Noun
        +get_head(): Noun
        +get_tail(): Noun
    }
    
    Noun <|-- Atom
    Noun <|-- Cell
```

### Evaluation Process

```mermaid
stateDiagram-v2
    [*] --> ParseNock
    ParseNock --> CheckJets
    
    CheckJets --> JetExecution: Jet found
    CheckJets --> NockEvaluation: No jet
    
    JetExecution --> VerifyResult
    NockEvaluation --> VerifyResult
    
    VerifyResult --> CacheResult
    CacheResult --> [*]
    
    state NockEvaluation {
        [*] --> ParseFormula
        ParseFormula --> EvaluateOperator
        EvaluateOperator --> RecursiveEvaluation: Needs recursion
        EvaluateOperator --> ProduceResult: Direct result
        RecursiveEvaluation --> EvaluateOperator
        ProduceResult --> [*]
    }
```

## Related Crates

- [nockapp](./nockapp.md): Application framework
- [zkvm-jetpack](./zkvm-jetpack.md): Zero-knowledge VM jet acceleration 