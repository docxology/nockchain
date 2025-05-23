# Hoon Programming Language System

## Overview

Hoon is a functional programming language that compiles to Nock, designed for deterministic computation and formal verification. In Nockchain, Hoon serves as the primary language for writing smart contracts, kernels, and applications that run on the Nock virtual machine.

## Language Fundamentals

### Philosophy and Design

Hoon embodies several key principles:

- **Functional Purity**: All computation is referentially transparent
- **Deterministic Execution**: Same inputs always produce same outputs
- **Formal Verification**: Code can be mathematically proven correct
- **Composability**: Programs built from small, reusable components
- **Type Safety**: Strong static typing prevents runtime errors

### Syntax Overview

Hoon uses a distinctive syntax with runes (two-character operators) that define program structure:

```hoon
::  Comments start with ::
|=  a=@                    ::  |= creates a gate (function)
^-  @                      ::  ^- casts to type
(add a 1)                  ::  Function call with parentheses
```

## Core Language Features

### 1. Types and Data Structures

#### Atoms and Cells
```hoon
::  Atoms (unsigned integers)
42                         ::  Decimal atom
0x2a                       ::  Hexadecimal atom
'hello'                    ::  Text atom

::  Cells (pairs)
[1 2]                      ::  Cell of atoms
[a=1 b=2]                  ::  Named cell elements
[[1 2] [3 4]]             ::  Nested cells
```

#### Type Definitions
```hoon
::  Basic types
+$  flag  ?                ::  Boolean
+$  cord  @t               ::  Text string
+$  list  (list @)         ::  List of atoms

::  Custom types
+$  person
  $:  name=@t
      age=@ud
      active=?
  ==
```

### 2. Functions and Gates

#### Gate Creation
```hoon
::  Simple gate
|=  a=@
^-  @
(add a 1)

::  Named gate
++  increment
  |=  a=@
  ^-  @
  (add a 1)

::  Multi-argument gate
|=  [a=@ b=@]
^-  @
(add a b)
```

#### Higher-Order Functions
```hoon
::  Map function over list
(turn (limo ~[1 2 3]) |=(a=@ (mul a 2)))

::  Filter function
(skim (limo ~[1 2 3 4]) |=(a=@ =(0 (mod a 2))))

::  Fold/reduce function
(roll (limo ~[1 2 3 4]) add)
```

### 3. Control Flow

#### Conditionals
```hoon
::  If-then-else
?:  =(a 0)
  'zero'
'not zero'

::  Switch statement
?-  color
  %red    'stop'
  %yellow 'caution'
  %green  'go'
==
```

#### Pattern Matching
```hoon
::  Match on cell structure
?~  list
  'empty'
'has items'

::  Match on union types
?-  -.message
  %hello  'greeting'
  %goodbye 'farewell'
==
```

### 4. Cores and Doors

#### Core Structure
```hoon
|%
++  add-one
  |=  a=@
  (add a 1)

++  multiply-two
  |=  a=@
  (mul a 2)
--
```

#### Door (Stateful Core)
```hoon
|_  state=@
++  increment
  ^-  @
  +(state)

++  decrement
  ^-  @
  (dec state)
--
```

## Hoon Compilation System (`hoonc`)

### Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    Hoon Compilation Pipeline                    │
├─────────────────────────────────────────────────────────────────┤
│  Source Processing                                              │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │   File Parser   │  │  Dependency     │  │   Build Runes   │ │
│  │                 │  │   Resolution    │  │                 │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│  Compilation Stages                                             │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │   AST Builder   │  │  Type Checker   │  │  Nock Compiler  │ │
│  │                 │  │                 │  │                 │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│  Output Generation                                              │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │   JAM Encoder   │  │  Cache Manager  │  │  Asset Builder  │ │
│  │                 │  │                 │  │                 │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### Build System Integration

#### Makefile Targets
```makefile
# Build all Hoon assets
build-hoon-all: nuke-assets update-hoonc ensure-dirs build-trivial $(HOON_TARGETS)

# Individual asset targets
assets/dumb.jam: hoon/apps/dumbnet/outer.hoon
    hoonc hoon/apps/dumbnet/outer.hoon hoon
    mv out.jam assets/dumb.jam

assets/wal.jam: hoon/apps/wallet/wallet.hoon
    hoonc hoon/apps/wallet/wallet.hoon hoon
    mv out.jam assets/wal.jam

assets/miner.jam: hoon/apps/dumbnet/miner.hoon
    hoonc hoon/apps/dumbnet/miner.hoon hoon
    mv out.jam assets/miner.jam
```

#### Dependency Management
The build system supports several dependency runes:

- `/+` - Load from `/lib` directory
- `/-` - Load from `/sur` directory (structures)
- `/=` - Load from specified path with `%hoon` mark
- `/*` - Load from specified path via specified mark
- `/#` - Load and execute from `/dat` directory
- `/?` - Version pinning (currently ignored)

### Compilation Process

#### 1. Source Parsing
```rust
// Parse Hoon source file
let pile = parse_pile(path, source_text);

// Resolve dependencies
let deps = resolve_pile(pile, directory);

// Build dependency graph
let graph = build_graph_view(nodes);
```

#### 2. Dependency Resolution
```hoon
++  resolve-pile
  |=  [=pile dir=(map path cord)]
  ^-  (list raut)
  ;:  weld
    (turn sur.pile |=(taut ^-(raut [face (need (get-fit %sur pax dir))])))
    (turn lib.pile |=(taut ^-(raut [face (need (get-fit %lib pax dir))])))
    (turn raw.pile |=([face=(unit term) pax=path] [face pax]))
    (turn bar.pile |=([face=term mark=@tas pax=path] [`face pax]))
    (turn hax.pile |=(taut ^-(raut [face (need (get-fit %dat pax dir))])))
  ==
```

#### 3. Compilation Stages
```hoon
++  compile-target
  |=  [pat=path =path-dag nodes=(map path node) bc=build-cache]
  ^-  [(trap vase) build-cache]
  
  ::  Build dependency graph
  =/  graph  (build-graph-view nodes)
  =/  next=(map path node)  (update-next nodes graph)
  
  ::  Compile in topological order
  |-  ^-  [(trap vase) build-cache]
  ?:  .=(~ next)
    ::  Compile target node
    (compile-node target-node path-dag bc)
  ::  Compile dependencies first
  (compile-dependencies next path-dag bc)
```

### Caching System

#### Build Cache
```hoon
+$  build-cache  (map @ (trap vase))
+$  parse-cache  (map @ pile)

::  Cache hit/miss logic
?:  (~(has by bc) dep-hash)
  ~&  >  build-cache-hit+path.n
  :_  bc
  [%.y (~(got by bc) dep-hash)]
~&  >  build-cache-miss+path.n
::  Compile and cache result
```

#### Content-Addressed Storage
The system uses content-addressed caching based on file hashes:

```hoon
++  calculate-hash
  |=  [n=node dep-dag=merk-dag =path-dag]
  ^-  @
  %+  roll  deps.n
  |=  [raut hash=_hash.n]
  =/  [dep-hash=@ *]  (~(got by path-dag) pax)
  (shax (rep 8 ~[hash dep-hash]))
```

## Standard Library

### Core Functions

#### Arithmetic
```hoon
++  add  |=([a=@ b=@] ^-(@) (sum a b))    ::  Addition
++  sub  |=([a=@ b=@] ^-(@) (dif a b))    ::  Subtraction  
++  mul  |=([a=@ b=@] ^-(@) (pro a b))    ::  Multiplication
++  div  |=([a=@ b=@] ^-(@) (fra a b))    ::  Division
++  mod  |=([a=@ b=@] ^-(@) (rem a b))    ::  Modulo
```

#### List Operations
```hoon
++  limo  |*(a=(list) a)                 ::  List constructor
++  turn  |*([a=(list) b=gate] (map b a)) ::  Map function
++  skim  |*([a=(list) b=gate] (filter b a)) ::  Filter
++  roll  |*([a=(list) b=gate] (fold b a))   ::  Fold left
```

#### Text Processing
```hoon
++  trip  |=(a=@ (rip 3 a))              ::  Atom to tape
++  crip  |=(a=tape (rap 3 a))           ::  Tape to atom
++  weld  |*([a=(list) b=(list)] (cat a b)) ::  Concatenate
```

### Cryptographic Functions

#### Hashing
```hoon
++  shax  |=(a=@ (sha-256 a))            ::  SHA-256 hash
++  shay  |=(a=@ (sha-1 a))              ::  SHA-1 hash
++  sham  |=(a=@ (murmur3 a))            ::  Murmur3 hash
```

#### Digital Signatures
```hoon
++  sign  |=([key=@ msg=@] (ed25519-sign key msg))
++  veri  |=([pub=@ sig=@ msg=@] (ed25519-verify pub sig msg))
```

## Application Development

### NockApp Structure

#### Basic Application Template
```hoon
/=  types  /apps/myapp/lib/types
/=  utils  /apps/myapp/lib/utils
/=  *      /common/zoon

|_  [state=app-state:types =blockchain-constants]
+*  t  ~(. utils blockchain-constants)

++  load
  |=  =app-state:types
  app-state

++  peek
  |=  arg=*
  =/  pax  ((soft path) arg)
  ?~  pax  ~|(not-a-path+arg !!)
  ~|(invalid-peek+pax !!)

++  poke
  |=  [wir=wire eny=@ our=@ux now=@da dat=*]
  ^-  [(list effect:types) app-state:types]
  ::  Handle poke logic here
  `state
--
```

#### Effect System
```hoon
+$  effect
  $%  [%command command]
      [%request request-type]
      [%response response-data]
      [%log message=@t]
  ==

++  handle-command
  |=  [cmd=command state=app-state]
  ^-  [(list effect) app-state]
  ?-  -.cmd
    %start   [[%log 'Application started']~ state]
    %stop    [[%log 'Application stopped']~ state]
    %update  [[%log 'State updated']~ (update-state +.cmd state)]
  ==
```

### Testing Framework

#### Unit Tests
```hoon
++  test-addition
  ^-  tang
  =/  result  (add 2 3)
  =/  expected  5
  ?:  =(result expected)
    ~
  ~[leaf+"Addition test failed: expected {<expected>}, got {<result>}"]

++  test-list-operations
  ^-  tang
  =/  input  (limo ~[1 2 3])
  =/  doubled  (turn input |=(a=@ (mul a 2)))
  =/  expected  (limo ~[2 4 6])
  ?:  =(doubled expected)
    ~
  ~[leaf+"List operation test failed"]
```

#### Integration Tests
```hoon
++  test-full-workflow
  ^-  tang
  =/  initial-state  *app-state
  =/  [effects new-state]  (poke /test/wire 0 0x0 now %start)
  =/  expected-effects  [%log 'Application started']~
  ?:  =(effects expected-effects)
    ~
  ~[leaf+"Workflow test failed"]
```

## Performance Optimization

### Compilation Optimizations

#### Tail Call Optimization
```hoon
::  Recursive function with tail call optimization
++  factorial-helper
  |=  [n=@ acc=@]
  ^-  @
  ?:  =(n 0)
    acc
  $(n (dec n), acc (mul n acc))
```

#### Memoization
```hoon
::  Memoized fibonacci
=/  fib-cache  *(map @ @)
++  fibonacci
  |=  n=@
  ^-  @
  ?:  (~(has by fib-cache) n)
    (~(got by fib-cache) n)
  =/  result
    ?:  (lte n 1)
      n
    (add (fibonacci (dec n)) (fibonacci (sub n 2)))
  =.  fib-cache  (~(put by fib-cache) n result)
  result
```

### Memory Management

#### Efficient Data Structures
```hoon
::  Use sets for membership testing
=/  valid-ids  (~(gas in *(set @)) ~[1 2 3 4 5])
++  is-valid
  |=  id=@
  (~(has in valid-ids) id)

::  Use maps for key-value lookups
=/  user-data  (~(gas by *(map @t @)) ~[['alice' 25] ['bob' 30]])
++  get-age
  |=  name=@t
  (~(get by user-data) name)
```

## Debugging and Development Tools

### Debugging Techniques

#### Debug Prints
```hoon
::  Simple debug output
~&  debug-value+some-variable
computation-continues

::  Conditional debug output
~&  ?:(debug-mode debug-info+data ~)
computation-continues
```

#### Error Handling
```hoon
::  Assertion with custom message
?>  (gth input 0)
(process-positive-input input)

::  Error with context
~|  "Invalid input: {<input>}"
?>  (valid-input input)
(process-input input)
```

### Development Workflow

#### Live Reloading
```bash
# Watch for changes and rebuild
make build-hoon-all

# Test specific application
hoonc --arbitrary hoon/apps/myapp/main.hoon
```

#### Profiling
```hoon
::  Timing measurements
=/  start-time  now
=/  result  (expensive-computation input)
=/  end-time  now
~&  computation-time+(sub end-time start-time)
result
```

## Integration with Nockchain

### Blockchain Applications

#### Transaction Processing
```hoon
++  process-transaction
  |=  [tx=transaction state=chain-state]
  ^-  [(list effect) chain-state]
  ?:  (valid-transaction tx state)
    =/  new-state  (apply-transaction tx state)
    [[%broadcast-tx tx]~ new-state]
  [[%reject-tx tx 'Invalid transaction']~ state]
```

#### Smart Contracts
```hoon
++  contract-execution
  |=  [contract=@ input=@ state=@]
  ^-  [@  @]  ::  [new-state output]
  =/  result  (nock [state input] contract)
  [-.result +.result]
```

### Mining Integration

#### Proof of Work
```hoon
++  mine-block
  |=  [candidate=block target=@]
  ^-  (unit block)
  =/  nonce  0
  |-
  =/  hash  (block-hash candidate(nonce nonce))
  ?:  (lte hash target)
    `candidate(nonce nonce)
  ?:  (gth nonce max-nonce)
    ~
  $(nonce +(nonce))
```

## Future Developments

### Language Enhancements
1. **Pattern Matching**: Enhanced pattern matching capabilities
2. **Type Inference**: Improved type inference for cleaner code
3. **Macro System**: Compile-time code generation
4. **Module System**: Better namespace management

### Tooling Improvements
1. **IDE Support**: Language server protocol implementation
2. **Debugger**: Interactive debugging capabilities
3. **Profiler**: Performance analysis tools
4. **Documentation**: Automatic documentation generation

### Optimization Research
1. **Parallel Compilation**: Multi-threaded compilation
2. **Incremental Compilation**: Faster rebuild times
3. **Dead Code Elimination**: Smaller output binaries
4. **Loop Optimization**: Better performance for iterative code

This Hoon system provides a robust foundation for developing deterministic, verifiable applications on the Nockchain platform, with comprehensive tooling for development, testing, and deployment. 