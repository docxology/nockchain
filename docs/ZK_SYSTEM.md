# Nockchain Zero-Knowledge System

This document provides a detailed overview of the Zero-Knowledge (ZK) proof system implemented in Nockchain, explaining its architecture, components, and how it enables private verifiable computation.

## Overview

The Nockchain ZK system allows computation to be performed privately off-chain while providing cryptographic proof of correctness that can be efficiently verified on-chain. This approach enables:

- **Privacy**: The details of computation remain confidential
- **Scalability**: Complex computation happens off-chain
- **Verifiability**: Results can be cryptographically verified
- **Efficiency**: Verification is lightweight compared to re-execution

```mermaid
graph TD
    subgraph "Off-Chain Prover"
        COMP[Computation]
        EXEC[Execution Trace]
        PROVE[Proof Generation]
    end
    
    subgraph "On-Chain Verifier"
        VERIFY[Proof Verification]
        CONSENSUS[Consensus Agreement]
        STATE[State Update]
    end
    
    USER[User] -->|Submits task| COMP
    COMP -->|Executes| EXEC
    EXEC -->|Generates| PROVE
    PROVE -->|Proof| VERIFY
    VERIFY -->|Validated| CONSENSUS
    CONSENSUS -->|Updates| STATE
    STATE -->|Result available| USER
    
    classDef offchain fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef onchain fill:#d3f8e2,stroke:#333,stroke-width:1px
    classDef external fill:#e3f6f5,stroke:#333,stroke-width:1px
    
    class COMP,EXEC,PROVE offchain
    class VERIFY,CONSENSUS,STATE onchain
    class USER external
```

## STARK Implementation

Nockchain uses STARKs (Scalable Transparent ARguments of Knowledge) as its primary ZK proof system. STARKs offer several advantages:

- No trusted setup required
- Post-quantum security
- Scalable proof generation
- Relatively simple cryptographic assumptions

### STARK Architecture

```mermaid
flowchart TB
    subgraph "STARK System"
        subgraph "Algebraic Intermediate Representation (AIR)"
            CONSTRAINTS[Polynomial Constraints]
            BOUNDARY[Boundary Constraints]
            TRANSITION[Transition Constraints]
        end
        
        subgraph "FRI Protocol"
            COMMIT[Merkle Commitments]
            QUERY[Random Queries]
            FRI_PROOF[FRI Proof]
        end
        
        subgraph "Proof Generation"
            TRACE[Execution Trace]
            EXTENSIONS[Trace Extensions]
            COMMIT_PHASE[Commitment Phase]
            QUERY_PHASE[Query Phase]
        end
        
        subgraph "Verification"
            VERIFY_COMMIT[Verify Commitments]
            VERIFY_CONSTRAINTS[Verify Constraints]
            VERIFY_FRI[Verify FRI]
        end
    end
    
    %% Connections
    TRACE --> CONSTRAINTS
    TRACE --> EXTENSIONS
    EXTENSIONS --> COMMIT_PHASE
    COMMIT_PHASE --> COMMIT
    COMMIT --> QUERY_PHASE
    QUERY_PHASE --> QUERY
    QUERY --> FRI_PROOF
    
    FRI_PROOF --> VERIFY_FRI
    CONSTRAINTS --> VERIFY_CONSTRAINTS
    COMMIT --> VERIFY_COMMIT
    
    VERIFY_FRI --> RESULT[Verification Result]
    VERIFY_CONSTRAINTS --> RESULT
    VERIFY_COMMIT --> RESULT
    
    classDef air fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef fri fill:#d3f8e2,stroke:#333,stroke-width:1px
    classDef prove fill:#e3f6f5,stroke:#333,stroke-width:1px
    classDef verify fill:#f9ebe0,stroke:#333,stroke-width:1px
    
    class CONSTRAINTS,BOUNDARY,TRANSITION air
    class COMMIT,QUERY,FRI_PROOF fri
    class TRACE,EXTENSIONS,COMMIT_PHASE,QUERY_PHASE prove
    class VERIFY_COMMIT,VERIFY_CONSTRAINTS,VERIFY_FRI,RESULT verify
```

## Components

### Hoon STARK Implementation

Located in [`hoon/common/stark/`](../hoon/common/stark/), the Hoon STARK implementation provides:

- Proof generation for Nock computation
- Field arithmetic operations
- Polynomial commitment schemes
- FRI protocol implementation
- Constraint system construction

```mermaid
classDiagram
    class StarkProver {
        +generateProof(trace): proof
        +createConstraints(program): constraints
        +buildTrace(execution): trace
        +extend(trace): extended_trace
        +commit(polynomials): commitments
    }
    
    class StarkVerifier {
        +verifyProof(proof, public_input): bool
        +checkConstraints(point, constraints): bool
        +verifyCommitment(commitment, point, value): bool
        +verifyFRI(proof): bool
    }
    
    class FiniteField {
        +add(a, b): element
        +multiply(a, b): element
        +inverse(a): element
        +pow(a, n): element
        +random(): element
    }
    
    class Polynomial {
        +evaluate(x): y
        +interpolate(points): polynomial
        +extend(domain): extended_poly
        +commit(): commitment
    }
    
    class MerkleTree {
        +build(leaves): root
        +createProof(index): proof
        +verify(proof, root, leaf): bool
    }
    
    StarkProver --> FiniteField
    StarkProver --> Polynomial
    StarkProver --> MerkleTree
    
    StarkVerifier --> FiniteField
    StarkVerifier --> Polynomial
    StarkVerifier --> MerkleTree
```

### ZKVM Jetpack

Located in [`crates/zkvm-jetpack/`](../crates/zkvm-jetpack/), the ZKVM Jetpack provides:

- Accelerated VM operations for zero-knowledge proofs
- Specialized circuits for common operations
- Integration with the Sword runtime
- Optimized proof generation

```mermaid
graph TD
    subgraph "ZKVM Jetpack"
        JETS[Accelerated Operations]
        CIRCUITS[ZK Circuits]
        SWORD_INT[Sword Integration]
        OPT[Optimization Layer]
    end
    
    JETS --> CIRCUITS
    CIRCUITS --> SWORD_INT
    SWORD_INT --> OPT
    
    %% External connections
    VM[Sword VM] --> JETS
    PROVER[STARK Prover] --> CIRCUITS
    
    classDef jetpack fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef external fill:#d3f8e2,stroke:#333,stroke-width:1px
    
    class JETS,CIRCUITS,SWORD_INT,OPT jetpack
    class VM,PROVER external
```

## Integration with Nockchain

### Transaction Lifecycle with ZK Proofs

```mermaid
sequenceDiagram
    participant User
    participant Prover
    participant Network
    participant Verifier
    participant Blockchain
    
    User->>Prover: Submit Transaction
    Prover->>Prover: Execute Computation
    Prover->>Prover: Generate Execution Trace
    Prover->>Prover: Create STARK Proof
    
    Prover->>Network: Submit Transaction + Proof
    Network->>Verifier: Distribute to Validators
    
    Verifier->>Verifier: Verify STARK Proof
    Verifier->>Blockchain: Confirm Valid Transaction
    
    Blockchain->>Blockchain: Update State
    Blockchain->>User: Confirmation
    
    Note over Prover: Private, Off-Chain
    Note over Verifier,Blockchain: Public, On-Chain
```

### Nock Verification

Nockchain provides special components for proving and verifying Nock computation:

- [`nock-prover.hoon`](../hoon/common/nock-prover.hoon): Generates proofs for Nock execution
- [`nock-verifier.hoon`](../hoon/common/nock-verifier.hoon): Verifies proofs of Nock execution

```mermaid
graph TD
    subgraph "Nock ZK System"
        NOCK_CODE[Nock Code]
        SWORD[Sword VM]
        TRACE[Execution Trace]
        CONSTRAINTS[Constraint System]
        PROVER[Nock Prover]
        PROOF[ZK Proof]
        VERIFIER[Nock Verifier]
        RESULT[Verification Result]
    end
    
    NOCK_CODE -->|executed by| SWORD
    SWORD -->|produces| TRACE
    TRACE -->|converted to| CONSTRAINTS
    CONSTRAINTS -->|used by| PROVER
    PROVER -->|generates| PROOF
    PROOF -->|verified by| VERIFIER
    VERIFIER -->|produces| RESULT
    
    classDef input fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef process fill:#d3f8e2,stroke:#333,stroke-width:1px
    classDef output fill:#e3f6f5,stroke:#333,stroke-width:1px
    
    class NOCK_CODE,PROOF input
    class SWORD,TRACE,CONSTRAINTS,PROVER,VERIFIER process
    class RESULT output
```

## Nock to STARK Conversion

The process of converting Nock computation to STARK-provable form involves:

1. **Trace Generation**: Recording all steps of Nock execution
2. **Constraint Formulation**: Expressing the computation as polynomial constraints
3. **Proof Construction**: Generating a STARK proof of correct execution
4. **Verification**: Validating the proof against public inputs and outputs

```mermaid
flowchart LR
    subgraph "Nock to STARK Pipeline"
        NOCK[Nock Formula]
        EXEC[Execution]
        TRACE[Execution Trace]
        AIR[Algebraic Constraints]
        STARK[STARK Proof]
    end
    
    NOCK -->|execute| EXEC
    EXEC -->|record| TRACE
    TRACE -->|constrain| AIR
    AIR -->|prove| STARK
    
    classDef stage fill:#f9d5e5,stroke:#333,stroke-width:1px
    
    class NOCK,EXEC,TRACE,AIR,STARK stage
```

## Performance Considerations

The ZK system in Nockchain balances several performance factors:

```mermaid
graph TD
    subgraph "Performance Tradeoffs"
        PROOF_SIZE[Proof Size]
        PROVE_TIME[Proving Time]
        VERIFY_TIME[Verification Time]
        SECURITY[Security Level]
    end
    
    PROOF_SIZE -->|impacts| NETWORK[Network Overhead]
    PROVE_TIME -->|affects| USER_EXP[User Experience]
    VERIFY_TIME -->|influences| THROUGHPUT[System Throughput]
    SECURITY -->|determines| TRUST[Trust Level]
    
    classDef factor fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef impact fill:#d3f8e2,stroke:#333,stroke-width:1px
    
    class PROOF_SIZE,PROVE_TIME,VERIFY_TIME,SECURITY factor
    class NETWORK,USER_EXP,THROUGHPUT,TRUST impact
```

### Optimizations

Nockchain implements several optimizations for its ZK system:

1. **Specialized Circuits**: Custom-designed circuits for common operations
2. **Batched Proofs**: Combining multiple proofs to amortize verification costs
3. **Recursive Proving**: Using proof composition for incremental verification
4. **Hardware Acceleration**: Support for GPU acceleration of proof generation
5. **Caching**: Reusing intermediate computations when applicable

## Example: ZK Transaction Flow

```mermaid
sequenceDiagram
    participant User as User/App
    participant Wallet as Wallet
    participant Prover as ZK Prover
    participant Network as Network
    participant Validators as Validators
    participant Chain as Blockchain
    
    User->>Wallet: Create Transaction
    Wallet->>Prover: Request Proof
    
    Prover->>Prover: Generate Trace
    Prover->>Prover: Create Constraints
    Prover->>Prover: Compute Commitment
    Prover->>Prover: Generate Proof
    
    Prover->>Wallet: Return Proof
    Wallet->>Network: Submit Transaction + Proof
    Network->>Validators: Broadcast
    
    par Validation
        Validators->>Validators: Verify Proof
        Validators->>Validators: Check Transaction Validity
    end
    
    Validators->>Chain: Add to Block
    Chain->>Network: Broadcast Confirmation
    Network->>Wallet: Confirmation
    Wallet->>User: Success Notification
```

## Security Considerations

The security of the ZK system depends on:

1. **Soundness**: Impossible to generate valid proofs for invalid computations
2. **Completeness**: Valid computations always have valid proofs
3. **Zero-Knowledge**: Proofs reveal nothing about the private inputs
4. **Collisions**: Cryptographic hash function security

### Threat Model

```mermaid
graph TD
    subgraph "Threat Model"
        INVALID_PROOF[Invalid Proof Generation]
        INFO_LEAK[Information Leakage]
        COLLUSION[Validator Collusion]
        REPLAY[Replay Attacks]
    end
    
    INVALID_PROOF -->|mitigated by| SOUNDNESS[Mathematical Soundness]
    INFO_LEAK -->|addressed by| ZK_PROPERTY[Zero-Knowledge Property]
    COLLUSION -->|prevented by| CONSENSUS[Consensus Protocol]
    REPLAY -->|defended by| NONCE[Unique Identifiers]
    
    classDef threat fill:#f9d5e5,stroke:#333,stroke-width:1px
    classDef defense fill:#d3f8e2,stroke:#333,stroke-width:1px
    
    class INVALID_PROOF,INFO_LEAK,COLLUSION,REPLAY threat
    class SOUNDNESS,ZK_PROPERTY,CONSENSUS,NONCE defense
```

## Development Guide

### Creating ZK-Verifiable Nock Computations

To create ZK-verifiable Nock computations:

1. Define your computation in Hoon
2. Compile to optimized Nock code
3. Execute with the prover to generate a trace
4. Create STARK proof
5. Submit for verification

```hoon
|=  [public-input=* private-input=*]
=/  computation=_|=(* *)  
  |=(* (add private-input public-input))
=/  result=*  (computation private-input)
=/  proof=*  (generate-proof computation private-input)
[result proof]
```

### Verifying ZK Proofs

To verify ZK proofs:

```hoon
|=  [public-input=* result=* proof=*]
?.  (verify-proof proof public-input result)
  %.n
%.y
```

## Future Developments

Planned enhancements to the ZK system include:

1. **Recursive STARKs**: For more efficient composable proofs
2. **Specialized Circuits**: For common blockchain operations
3. **Multi-Prover Protocol**: Distributed proof generation
4. **Privacy Enhancements**: Additional protections for metadata
5. **Hardware Acceleration**: Dedicated ASIC/FPGA support

## Related Documentation

- [Sword VM](./crates/sword.md): Runtime environment for Nock code
- [ZKVM Jetpack](./crates/zkvm-jetpack.md): Accelerated VM operations
- [Hoon Common](./hoon/common.md): Common Hoon libraries including STARK
- [Technical Roadmap](./ROADMAP.md): Future development plans 