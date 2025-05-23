# Nock Virtual Machine

## Overview

The Nock Virtual Machine (NockVM) is the computational foundation of Nockchain. It implements the Nock specification - a minimal, deterministic virtual machine designed for functional programming with cryptographic guarantees.

## Core Concepts

### Nouns
Everything in Nock is a "noun" - either an atom (unsigned integer) or a cell (ordered pair of nouns):

```
noun = atom | cell
atom = unsigned integer
cell = [noun noun]
```

### Deterministic Computation
- All operations are pure functions
- No side effects or mutable state
- Reproducible execution across all platforms
- Cryptographic hashing ensures integrity

## Instruction Set

Nock has exactly 12 opcodes, making it one of the most minimal virtual machines:

| Opcode | Name | Description |
|--------|------|-------------|
| 0 | `/` | Tree addressing |
| 1 | `=` | Constant |
| 2 | `?` | Cell test |
| 3 | `+` | Increment |
| 4 | `=` | Equality test |
| 5 | `/` | Tree addressing |
| 6 | `?:` | Conditional |
| 7 | `=<` | Composition |
| 8 | `=>` | Push |
| 9 | `=|` | Call |
| 10 | `=.` | Edit |
| 11 | `=^` | Hint |

### Instruction Details

#### Tree Addressing (`/`)
```
/[1 a]           = a
/[2 a b]         = a
/[3 a b]         = b
/[(a + a) b]     = /[2 /[a b]]
/[(a + a + 1) b] = /[3 /[a b]]
```

#### Equality Test (`=`)
```
=[a a]           = 0 (true)
=[a b]           = 1 (false, if a != b)
```

#### Conditional (`?:`)
```
?:[0 b c]        = b
?:[1 b c]        = c
?:[a b c]        = ?:[=(0 a) b c]
```

## Memory Management

### Stack Architecture
The NockVM uses a stack-based memory model with multiple regions:

```rust
pub struct NockStack {
    start: *mut u8,
    size: usize,
    frame_pointer: *mut u64,
    stack_pointer: *mut u64,
    alloc_pointer: *mut u64,
}
```

### Memory Regions

#### Frame Stack
- Stores call frames and local variables
- Grows upward from low addresses
- Automatic cleanup on function return

#### Allocation Stack (West)
- Stores noun allocations
- Grows upward from frame stack
- Garbage collected when frames are popped

#### Temporary Stack (East)
- Temporary allocations during computation
- Grows downward from high addresses
- Automatically reclaimed

### Garbage Collection
- **Stack-based GC**: Automatic cleanup when stack frames are popped
- **Generational**: Young objects in temp stack, old objects in west stack
- **Conservative**: Scans for potential pointers during collection
- **Incremental**: Can be interrupted and resumed

## Jets System

Jets are optimized native implementations of common Nock operations:

### Jet Architecture
```rust
pub struct Jet {
    pub name: &'static str,
    pub code: fn(&mut NockStack, Noun) -> Result<Noun, JetErr>,
    pub path: &'static [u64],
}
```

### Core Jets

#### Cryptographic Jets
- **SHA-256/512**: Cryptographic hashing
- **Blake3**: High-performance hashing
- **Ed25519**: Digital signatures
- **AES-SIV**: Authenticated encryption

#### Mathematical Jets
- **Add/Sub/Mul/Div**: Arithmetic operations
- **Modular arithmetic**: Finite field operations
- **Bit operations**: AND, OR, XOR, shifts

#### Data Structure Jets
- **Tree operations**: Efficient tree manipulation
- **List operations**: Functional list processing
- **Map operations**: Associative array operations

### Jet Matching
Jets are matched by their Nock formula path:
```rust
fn find_jet(path: &[u64]) -> Option<Jet> {
    JETS.iter().find(|jet| jet.path == path)
}
```

## Serialization

### Jam/Cue Protocol
Efficient binary serialization of nouns:

#### Jam (Serialize)
```
jam(atom)     = atom with length prefix
jam([a b])    = 01 + jam(a) + jam(b)
```

#### Cue (Deserialize)
```rust
pub fn cue(stack: &mut NockStack, buffer: &[u8]) -> Result<Noun, CueError> {
    let mut cursor = 0;
    cue_noun(stack, buffer, &mut cursor)
}
```

### Mug (Hash)
32-bit hash function for nouns:
```rust
pub fn mug(noun: Noun) -> u32 {
    match noun.as_either() {
        Either::Left(atom) => mug_atom(atom),
        Either::Right(cell) => mug_cell(cell),
    }
}
```

## Implementation Details

### Noun Representation
```rust
#[repr(C)]
pub struct Noun(u64);

impl Noun {
    pub fn is_atom(&self) -> bool { self.0 & 1 == 0 }
    pub fn is_cell(&self) -> bool { self.0 & 1 == 1 }
    
    pub fn as_atom(&self) -> Result<Atom, ()> { ... }
    pub fn as_cell(&self) -> Result<Cell, ()> { ... }
}
```

### Atom Types
```rust
pub enum Atom {
    Direct(DirectAtom),    // Small atoms (< 64 bits)
    Indirect(IndirectAtom), // Large atoms (>= 64 bits)
}

#[repr(C)]
pub struct DirectAtom(u64);

#[repr(C)]
pub struct IndirectAtom {
    size: usize,
    data: *mut u8,
}
```

### Cell Structure
```rust
#[repr(C)]
pub struct Cell {
    head: Noun,
    tail: Noun,
}
```

## Interpreter

### Main Evaluation Loop
```rust
pub fn interpret(
    stack: &mut NockStack,
    subject: Noun,
    formula: Noun,
) -> Result<Noun, NockError> {
    loop {
        match formula.as_cell()?.head().as_direct_atom()?.data() {
            0 => { /* Tree addressing */ }
            1 => { /* Constant */ }
            2 => { /* Cell test */ }
            // ... other opcodes
        }
    }
}
```

### Error Handling
```rust
#[derive(Debug)]
pub enum NockError {
    Deterministic(Deterministic),
    NonDeterministic(NonDeterministic),
}

#[derive(Debug)]
pub enum Deterministic {
    Exit(Noun),
    Crash(Noun),
}
```

## Performance Optimizations

### Tail Call Optimization
- Recognizes tail recursive patterns
- Reuses stack frames to prevent overflow
- Maintains constant stack space for loops

### Memoization
- Caches results of expensive computations
- Uses content-addressed storage
- Automatic cache invalidation

### SIMD Instructions
- Vectorized operations for large atoms
- Platform-specific optimizations
- Fallback to scalar operations

### Memory Layout Optimization
```rust
// Optimized for cache locality
#[repr(C, align(8))]
pub struct OptimizedCell {
    head: Noun,
    tail: Noun,
    mug: u32,      // Cached hash
    metadata: u32, // Type information
}
```

## Debugging and Tracing

### Trace System
```rust
pub struct Trace {
    pub depth: usize,
    pub subject: Noun,
    pub formula: Noun,
    pub result: Option<Noun>,
}

pub fn trace_interpret(
    stack: &mut NockStack,
    subject: Noun,
    formula: Noun,
) -> (Result<Noun, NockError>, Vec<Trace>) {
    // Implementation with tracing
}
```

### Debug Utilities
```rust
// Pretty printing for debugging
pub fn pretty_noun(noun: Noun) -> String {
    match noun.as_either() {
        Either::Left(atom) => format!("{}", atom.as_u64()),
        Either::Right(cell) => format!("[{} {}]", 
            pretty_noun(cell.head()), 
            pretty_noun(cell.tail())
        ),
    }
}
```

## Safety and Security

### Memory Safety
- All pointer operations are bounds-checked
- Stack overflow protection
- Use-after-free prevention
- Double-free protection

### Determinism Guarantees
- No undefined behavior
- Platform-independent results
- Reproducible across architectures
- Cryptographic verification of execution

### Resource Limits
```rust
pub struct ResourceLimits {
    pub max_memory: usize,
    pub max_computation: u64,
    pub max_stack_depth: usize,
}
```

## Testing and Verification

### Property Testing
```rust
#[quickcheck]
fn jam_cue_roundtrip(noun: ArbitraryNoun) -> bool {
    let mut stack = NockStack::new(1 << 20, 0);
    let jammed = jam(&mut stack, noun.0);
    let cued = cue(&mut stack, &jammed).unwrap();
    noun.0.raw_equals(&cued)
}
```

### Formal Verification
- Mathematical specification of all operations
- Proof of termination for all programs
- Verification of cryptographic properties
- Model checking for critical paths

## Integration Points

### Hot State
Pre-computed cryptographic state for performance:
```rust
pub struct HotEntry {
    pub path: &'static [u64],
    pub hash: [u8; 32],
    pub code: &'static [u8],
}
```

### External Interfaces
- **FFI**: Safe foreign function interface
- **Syscalls**: Controlled system interactions
- **I/O**: Event-driven input/output
- **Networking**: Protocol-aware communication

## Configuration

### Build Features
```toml
[features]
default = ["mmap"]
malloc = []           # Use malloc instead of mmap
mmap = []            # Use memory mapping
check_all = []       # Enable all runtime checks
sham_hints = []      # Disable hint processing
```

### Runtime Configuration
```rust
pub struct NockConfig {
    pub stack_size: usize,
    pub enable_jets: bool,
    pub enable_tracing: bool,
    pub memory_limit: Option<usize>,
}
``` 