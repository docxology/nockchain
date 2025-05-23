# Nockchain API Reference

## Overview

This document provides comprehensive API reference for Nockchain, covering command-line interfaces, RPC protocols, and internal APIs.

## Command Line Interface

### Nockchain Node CLI

#### Basic Usage
```bash
nockchain [OPTIONS]
```

#### Global Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--help` | flag | - | Show help message |
| `--version` | flag | - | Show version information |

#### Core Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--npc-socket` | string | `.socket/nockchain_npc.sock` | NPC socket path |
| `--mine` | flag | false | Enable in-kernel mining |
| `--mining-pubkey` | string | - | Public key to mine to |
| `--mining-key-adv` | string[] | - | Advanced mining key configuration |

#### Genesis Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--genesis-watcher` | flag | false | Watch for genesis block |
| `--genesis-leader` | flag | false | Mine genesis block |
| `--fakenet` | flag | false | Use fake genesis block |
| `--genesis-message` | string | "Hail Zorp" | Genesis block message |

#### Bitcoin Integration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--btc-node-url` | string | `http://100.98.183.39:8332` | Bitcoin Core RPC URL |
| `--btc-username` | string | - | Bitcoin Core RPC username |
| `--btc-password` | string | - | Bitcoin Core RPC password |
| `--btc-auth-cookie` | string | - | Bitcoin Core auth cookie path |

#### Network Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--peer` | string[] | - | Initial peer addresses |
| `--allowed-peers-path` | string | - | Allowed peer IDs file |
| `--no-default-peers` | flag | false | Don't dial default peers |
| `--bind` | string[] | `/ip4/0.0.0.0/udp/0/quic-v1` | Bind addresses |
| `--new-peer-id` | flag | false | Generate new peer ID |

#### Connection Limits

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--max-established-incoming` | u32 | 100 | Max incoming connections |
| `--max-established-outgoing` | u32 | 100 | Max outgoing connections |
| `--max-pending-incoming` | u32 | 100 | Max pending incoming |
| `--max-pending-outgoing` | u32 | 100 | Max pending outgoing |
| `--max-established` | u32 | 200 | Max total connections |
| `--max-established-per-peer` | u32 | 5 | Max connections per peer |
| `--max-system-memory-fraction` | f64 | 0.8 | Max memory fraction |
| `--max-system-memory-bytes` | usize | - | Max memory bytes |

#### Boot Options (inherited from NockApp)

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--new` | flag | false | Start with new data directory |
| `--trace` | flag | false | Make Sword trace |
| `--save-interval` | u64 | 1000 | Checkpoint save interval (ms) |
| `--color` | enum | auto | Control colored output |
| `--state-jam` | string | - | Path to existing kernel state |
| `--export-state-jam` | string | - | Export kernel state path |

#### Examples

```bash
# Basic node
nockchain

# Mining node
nockchain --mining-pubkey <pubkey> --mine

# Genesis leader
nockchain --genesis-leader --btc-node-url <url> --btc-username <user>

# Custom network configuration
nockchain --bind /ip4/0.0.0.0/udp/9000/quic-v1 --peer <peer-addr>

# Test network
nockchain --fakenet --genesis-leader
```

### Nockchain Wallet CLI

#### Basic Usage
```bash
nockchain-wallet [GLOBAL_OPTIONS] <COMMAND> [COMMAND_OPTIONS]
```

#### Global Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--nockchain-socket` | path | - | Path to nockchain socket |
| `--help` | flag | - | Show help message |
| `--version` | flag | - | Show version information |

#### Boot Options (inherited)

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--new` | flag | false | Start with new data directory |
| `--trace` | flag | false | Make Sword trace |
| `--save-interval` | u64 | 1000 | Checkpoint save interval (ms) |
| `--color` | enum | auto | Control colored output |

#### Commands

##### Key Management Commands

###### `keygen`
Generate a new key pair.

```bash
nockchain-wallet keygen
```

**Output**: Displays new public/private key pair and seed phrase.

###### `derive-child`
Derive a child key from the current master key.

```bash
nockchain-wallet derive-child --key-type <TYPE> --index <INDEX>
```

**Options**:
- `--key-type, -k`: Key type (`pub` or `priv`)
- `--index, -i`: Child key index (0-255)

###### `import-keys`
Import keys from a file.

```bash
nockchain-wallet import-keys --input <FILE>
```

**Options**:
- `--input, -i`: Path to jammed keys file

###### `export-keys`
Export keys to a file.

```bash
nockchain-wallet export-keys
```

**Output**: Creates `keys.export` file in current directory.

###### `gen-master-privkey`
Generate master private key from seed phrase.

```bash
nockchain-wallet gen-master-privkey --seedphrase <PHRASE>
```

**Options**:
- `--seedphrase, -s`: Seed phrase to generate master private key

###### `gen-master-pubkey`
Generate master public key from master private key.

```bash
nockchain-wallet gen-master-pubkey --master-privkey <KEY>
```

**Options**:
- `--master-privkey, -m`: Master private key

###### `export-master-pubkey`
Export the master public key.

```bash
nockchain-wallet export-master-pubkey
```

###### `import-master-pubkey`
Import a master public key.

```bash
nockchain-wallet import-master-pubkey --key-path <PATH>
```

**Options**:
- `--key-path`: Path to keys file

###### `list-pubkeys`
List all public keys in the wallet.

```bash
nockchain-wallet list-pubkeys
```

###### `show-seedphrase`
Show the seed phrase for the current master key.

```bash
nockchain-wallet show-seedphrase
```

###### `show-master-pubkey`
Show the master public key.

```bash
nockchain-wallet show-master-pubkey
```

###### `show-master-privkey`
Show the master private key.

```bash
nockchain-wallet show-master-privkey
```

##### Transaction Commands

###### `sign-tx`
Sign a transaction.

```bash
nockchain-wallet sign-tx --draft <FILE> [--index <INDEX>]
```

**Options**:
- `--draft, -d`: Path to input bundle file
- `--index, -i`: Optional key index (0-255)

###### `make-tx`
Create a transaction from a draft file.

```bash
nockchain-wallet make-tx --draft <FILE>
```

**Options**:
- `--draft, -d`: Draft file to create transaction from

###### `simple-spend`
Perform a simple spend operation.

```bash
nockchain-wallet simple-spend --names <NAMES> --recipients <ADDRS> --gifts <AMOUNTS> --fee <FEE>
```

**Options**:
- `--names`: Names of notes to spend (comma-separated)
- `--recipients`: Recipient addresses (comma-separated)
- `--gifts`: Amounts to send (comma-separated)
- `--fee`: Transaction fee

##### Blockchain Commands

###### `scan`
Perform a simple scan of the blockchain.

```bash
nockchain-wallet scan --master-pubkey <PUBKEY> [OPTIONS]
```

**Options**:
- `--master-pubkey, -m`: Master public key to scan for
- `--search-depth, -s`: Search depth (default: 100)
- `--include-timelocks`: Include timelocks in scan
- `--include-multisig`: Include multisig in scan

###### `list-notes`
List all notes in the wallet.

```bash
nockchain-wallet list-notes
```

###### `list-notes-by-pubkey`
List notes by public key.

```bash
nockchain-wallet list-notes-by-pubkey --pubkey <PUBKEY>
```

**Options**:
- `--pubkey, -p`: Public key to filter notes

###### `update-balance`
Update the wallet balance.

```bash
nockchain-wallet update-balance
```

### Hoon Compiler CLI

#### Basic Usage
```bash
hoonc [OPTIONS] <ENTRY> [DIRECTORY]
```

#### Arguments

| Argument | Type | Description |
|----------|------|-------------|
| `<ENTRY>` | path | Path to file to compile |
| `[DIRECTORY]` | path | Path to root of dependency directory (default: "hoon") |

#### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--arbitrary` | flag | false | Build raw, without file hash injection |
| `--output` | path | - | Output file path |

#### Boot Options (inherited)

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `--new` | flag | false | Start with new data directory |
| `--trace` | flag | false | Make Sword trace |
| `--save-interval` | u64 | 1000 | Checkpoint save interval (ms) |
| `--color` | enum | auto | Control colored output |

#### Examples

```bash
# Compile Hoon file
hoonc app.hoon

# Compile with custom dependency directory
hoonc app.hoon ./my-hoon-libs

# Compile with custom output
hoonc app.hoon --output app.jam

# Raw compilation
hoonc app.hoon --arbitrary
```

## RPC Protocol

### Unix Domain Socket Protocol

Nockchain uses Unix domain sockets for local RPC communication between the node and wallet.

#### Socket Location
- Default: `.socket/nockchain_npc.sock`
- Configurable via `--npc-socket` option

#### Message Format

Messages use the NockApp wire protocol with the following structure:

```rust
pub struct WireRepr {
    pub source: String,
    pub version: u64,
    pub tags: Vec<String>,
}
```

#### Wire Types

##### System Wire
Used for core blockchain operations.

```rust
pub struct SystemWire;

impl Wire for SystemWire {
    const VERSION: u64 = 1;
    const SOURCE: &str = "system";
}
```

##### Wallet Wire
Used for wallet operations.

```rust
pub enum WalletWire {
    ListNotes,
    UpdateBalance,
    UpdateBlock,
    Exit,
    Command(Commands),
}
```

### P2P Protocol

#### Transport Layer
- **Protocol**: QUIC over UDP
- **Encryption**: TLS 1.3
- **Multiplexing**: Stream-based

#### Peer Discovery
- **Bootstrap**: Hardcoded bootstrap peers
- **DHT**: Kademlia-based distributed hash table
- **mDNS**: Local network discovery

#### Message Types

##### Block Messages
```rust
pub struct BlockMessage {
    pub block: Block,
    pub proof: Option<Proof>,
}
```

##### Transaction Messages
```rust
pub struct TransactionMessage {
    pub transaction: Transaction,
    pub signature: Signature,
}
```

##### Peer Messages
```rust
pub enum PeerMessage {
    Ping,
    Pong,
    GetPeers,
    Peers(Vec<PeerInfo>),
}
```

#### Anti-Spam Protection

##### Proof of Work Requirements
- **Algorithm**: Equix
- **Difficulty**: Dynamic based on network load
- **Verification**: Required for all requests

```rust
pub struct ProofOfWork {
    pub nonce: u64,
    pub solution: [u8; 16],
    pub difficulty: u64,
}
```

## Internal APIs

### NockApp Framework

#### Core Types

```rust
pub struct NockApp {
    pub kernel: Kernel,
    pub drivers: Vec<IODriver>,
    pub state: AppState,
}
```

#### Driver Interface

```rust
pub trait IODriver {
    async fn run(&mut self, handle: DriverHandle) -> Result<(), DriverError>;
}

pub struct DriverHandle {
    pub poke_tx: Sender<(WireRepr, NounSlab)>,
    pub peek_tx: Sender<PeekRequest>,
}
```

#### Operations

```rust
pub enum Operation {
    Poke,
    Peek,
    Exit,
}
```

### Nock Virtual Machine

#### Noun Types

```rust
pub enum Noun {
    Atom(Atom),
    Cell(Cell),
}

pub struct Atom {
    pub data: Vec<u8>,
}

pub struct Cell {
    pub head: Noun,
    pub tail: Noun,
}
```

#### Instruction Set

```rust
pub enum NockOp {
    Slot = 0,      // /
    Same = 1,      // =
    Slot0 = 2,     // /[0 b]
    Slot1 = 3,     // /[1 b]
    Inc = 4,       // +
    Eq = 5,        // =
    Cell = 6,      // ?
    Compose = 7,   // *
    Push = 8,      // +
    Call = 9,      // *
    Hint = 10,     // #
    Const = 11,    // =
}
```

#### Interpreter Interface

```rust
pub struct Interpreter {
    pub stack: Vec<Noun>,
    pub memory: NounSlab,
}

impl Interpreter {
    pub fn eval(&mut self, subject: Noun, formula: Noun) -> Result<Noun, NockError>;
    pub fn step(&mut self) -> Result<StepResult, NockError>;
}
```

### Blockchain Core

#### Block Structure

```rust
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub proof: ProofOfWork,
}

pub struct BlockHeader {
    pub parent_hash: Hash,
    pub merkle_root: Hash,
    pub timestamp: u64,
    pub height: u64,
    pub target: u64,
}
```

#### Transaction Structure

```rust
pub struct Transaction {
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub fee: u64,
    pub signature: Signature,
}

pub struct TxInput {
    pub note_name: NoteName,
    pub unlock_script: Script,
}

pub struct TxOutput {
    pub amount: u64,
    pub lock_script: Script,
}
```

#### Consensus Interface

```rust
pub trait Consensus {
    fn validate_block(&self, block: &Block) -> Result<(), ConsensusError>;
    fn validate_transaction(&self, tx: &Transaction) -> Result<(), ConsensusError>;
    fn calculate_difficulty(&self, blocks: &[Block]) -> u64;
}
```

### Mining Interface

#### Mining Configuration

```rust
pub struct MiningConfig {
    pub pubkey: PublicKey,
    pub threads: usize,
    pub target: u64,
}

pub enum MiningKeyConfig {
    Simple(PublicKey),
    MultiSig {
        share: u8,
        m: u8,
        keys: Vec<PublicKey>,
    },
}
```

#### Mining Operations

```rust
pub trait Miner {
    async fn start_mining(&mut self, template: BlockTemplate) -> Result<(), MiningError>;
    async fn stop_mining(&mut self) -> Result<(), MiningError>;
    fn get_status(&self) -> MiningStatus;
}

pub struct BlockTemplate {
    pub parent_hash: Hash,
    pub transactions: Vec<Transaction>,
    pub coinbase: CoinbaseTransaction,
    pub target: u64,
}
```

## Error Handling

### Error Types

#### NockApp Errors
```rust
pub enum NockAppError {
    KernelError(KernelError),
    DriverError(DriverError),
    IOError(std::io::Error),
    SerializationError(SerializationError),
}
```

#### Nock VM Errors
```rust
pub enum NockError {
    InvalidInstruction(u8),
    StackUnderflow,
    MemoryError,
    ComputeError(String),
}
```

#### Blockchain Errors
```rust
pub enum BlockchainError {
    InvalidBlock(String),
    InvalidTransaction(String),
    ConsensusError(String),
    NetworkError(String),
}
```

#### Wallet Errors
```rust
pub enum WalletError {
    KeyError(String),
    TransactionError(String),
    NetworkError(String),
    SerializationError(String),
}
```

### Error Codes

| Code | Category | Description |
|------|----------|-------------|
| 1000-1999 | NockApp | Application framework errors |
| 2000-2999 | NockVM | Virtual machine errors |
| 3000-3999 | Blockchain | Consensus and validation errors |
| 4000-4999 | Network | P2P networking errors |
| 5000-5999 | Wallet | Wallet operation errors |
| 6000-6999 | Mining | Mining operation errors |

## Configuration Reference

### Environment Variables

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `RUST_LOG` | string | `info` | Logging level configuration |
| `MINIMAL_LOG_FORMAT` | bool | `true` | Use minimal log format |
| `MINING_PUBKEY` | string | - | Default mining public key |
| `PEER_PORT` | u16 | random | P2P networking port |

### Configuration Files

#### Allowed Peers File
```
# peers.txt - Base58 encoded peer IDs
12D3KooWExample1PeerID...
12D3KooWExample2PeerID...
12D3KooWExample3PeerID...
```

#### Genesis Configuration
```rust
pub struct GenesisConfig {
    pub bitcoin_height: u64,
    pub message: String,
    pub target: u64,
    pub timestamp: u64,
}
```

## Performance Metrics

### Monitoring Endpoints

#### Node Metrics
- Block height
- Peer count
- Memory usage
- CPU usage
- Network I/O

#### Mining Metrics
- Hash rate
- Blocks mined
- Mining efficiency
- Power consumption

#### Wallet Metrics
- Transaction count
- Balance updates
- Key operations
- Sync status

## Security Considerations

### Cryptographic Primitives
- **Hashing**: Blake3
- **Signatures**: Ed25519
- **Key Derivation**: BIP32-style
- **Proof of Work**: Equix

### Security Best Practices
- Regular key rotation
- Secure key storage
- Network isolation
- Access control
- Audit logging

## Versioning

### API Versioning
- **Wire Protocol**: Version field in all messages
- **RPC API**: Semantic versioning
- **CLI Interface**: Backward compatibility maintained

### Compatibility Matrix

| Nockchain Version | Wire Protocol | RPC API | CLI Interface |
|-------------------|---------------|---------|---------------|
| 0.1.x | 1 | 1.0 | 1.0 |
| 0.2.x | 1 | 1.1 | 1.0 |
| 1.0.x | 2 | 2.0 | 2.0 |

## Examples and Tutorials

### Basic Node Setup
```bash
# 1. Install dependencies
make install-hoonc

# 2. Build project
make build

# 3. Generate keys
nockchain-wallet keygen

# 4. Start node
nockchain --mining-pubkey <pubkey> --mine
```

### Wallet Integration
```rust
use nockchain_wallet::Wallet;

async fn example_wallet_usage() -> Result<(), WalletError> {
    let mut wallet = Wallet::new().await?;
    
    // Generate new key
    let keypair = wallet.keygen().await?;
    
    // Check balance
    let balance = wallet.get_balance().await?;
    
    // Create transaction
    let tx = wallet.create_transaction(
        &inputs,
        &outputs,
        fee,
    ).await?;
    
    Ok(())
}
```

### Custom Driver Development
```rust
use nockapp::driver::{IODriver, DriverHandle};

struct CustomDriver {
    config: CustomConfig,
}

impl IODriver for CustomDriver {
    async fn run(&mut self, handle: DriverHandle) -> Result<(), DriverError> {
        // Driver implementation
        loop {
            // Handle incoming messages
            // Send outgoing messages
            // Perform driver-specific operations
        }
    }
}
```

## Conclusion

This API reference provides comprehensive coverage of Nockchain's interfaces and protocols. For implementation details and examples, refer to the source code and additional documentation.

Regular updates to this reference will be provided as the API evolves. For the latest information, consult the project repository and community resources. 