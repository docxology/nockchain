# Wallet System

## Overview

The Nockchain wallet is a comprehensive key management and transaction system built on hierarchical deterministic (HD) key derivation. It provides secure storage, transaction creation, and blockchain interaction capabilities.

## Architecture

### Core Components

```
┌─────────────────────────────────────────────────────────────┐
│                    Wallet System                           │
├─────────────────────────────────────────────────────────────┤
│  CLI Interface                                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   Keygen    │  │ Transaction │  │   Balance   │        │
│  │  Commands   │  │  Commands   │  │  Commands   │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
├─────────────────────────────────────────────────────────────┤
│  Wallet Core                                                │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  Key Management | Transaction Logic | State Sync       ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│  Wallet Kernel                                              │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  Hoon Implementation | Cryptographic Operations        ││
│  └─────────────────────────────────────────────────────────┘│
├─────────────────────────────────────────────────────────────┤
│  NockApp Framework                                          │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  I/O Drivers | Wire Protocol | Event System            ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

### Key Management

#### Hierarchical Deterministic Keys
The wallet uses BIP32-style HD key derivation:

```
Master Seed (256 bits)
    ↓
Master Private Key + Chain Code
    ↓
Child Keys (m/0, m/1, m/2, ...)
    ↓
Public Keys (derived from private keys)
```

#### Key Types
```rust
pub enum KeyType {
    Pub,  // Public key derivation
    Prv,  // Private key derivation
}
```

#### Key Storage
- **Master Keys**: Encrypted and stored securely
- **Derived Keys**: Generated on-demand from master key
- **Chain Codes**: Used for key derivation
- **Seed Phrases**: BIP39-compatible mnemonic phrases

## Command Line Interface

### Key Management Commands

#### Generate New Key Pair
```bash
nockchain-wallet keygen
```
- Generates new master private/public key pair
- Creates BIP39 seed phrase
- Outputs public key for mining configuration

#### Derive Child Keys
```bash
nockchain-wallet derive-child --key-type pub --index 0
nockchain-wallet derive-child --key-type priv --index 1
```
- Derives child keys from master key
- Supports up to 256 child keys (index 0-255)
- Can derive both public and private child keys

#### Show Keys
```bash
nockchain-wallet show-master-pubkey
nockchain-wallet show-master-privkey
nockchain-wallet show-seedphrase
```

### Key Import/Export

#### Export Keys
```bash
nockchain-wallet export-keys
```
- Exports keys to `keys.export` file
- JAM-encoded binary format
- Includes master keys and metadata

#### Import Keys
```bash
nockchain-wallet import-keys --input keys.export
```
- Imports keys from JAM file
- Validates key integrity
- Merges with existing wallet state

#### Master Public Key Operations
```bash
nockchain-wallet export-master-pubkey
nockchain-wallet import-master-pubkey --key-path master.keys
```

### Transaction Operations

#### Simple Spend
```bash
nockchain-wallet simple-spend \
    --names "note1,note2" \
    --recipients "addr1,addr2" \
    --gifts "100,200" \
    --fee 10
```
- Spends specified notes (UTXOs)
- Sends to multiple recipients
- Includes transaction fee

#### Transaction Creation
```bash
nockchain-wallet make-tx --draft transaction.draft
```
- Creates transaction from draft file
- Validates inputs and outputs
- Generates unsigned transaction

#### Transaction Signing
```bash
nockchain-wallet sign-tx --draft transaction.draft --index 0
```
- Signs transaction with specified key index
- Uses entropy for signature randomness
- Outputs signed transaction

### Balance and Notes

#### List Notes
```bash
nockchain-wallet list-notes
nockchain-wallet list-notes-by-pubkey <pubkey>
```
- Lists all available notes (UTXOs)
- Filters by public key
- Shows note values and metadata

#### Update Balance
```bash
nockchain-wallet update-balance
```
- Synchronizes with blockchain
- Updates note availability
- Refreshes wallet state

#### Blockchain Scanning
```bash
nockchain-wallet scan \
    --master-pubkey <pubkey> \
    --search-depth 100 \
    --include-timelocks \
    --include-multisig
```
- Scans blockchain for wallet transactions
- Configurable search depth
- Optional timelock and multisig support

## Implementation Details

### Wallet Structure
```rust
pub struct Wallet {
    app: NockApp,
}

impl Wallet {
    fn new(nockapp: NockApp) -> Self {
        Wallet { app: nockapp }
    }
}
```

### Command Processing
```rust
type CommandNoun<T> = Result<(T, Operation), NockAppError>;

fn wallet(
    command: &str,
    args: &[Noun],
    operation: Operation,
    slab: &mut NounSlab,
) -> CommandNoun<NounSlab> {
    let head = make_tas(slab, command).as_noun();
    let tail = match args.len() {
        0 => D(0),
        1 => args[0],
        _ => T(slab, args),
    };
    let full = T(slab, &[head, tail]);
    slab.set_root(full);
    Ok((slab.clone(), operation))
}
```

### Wire Protocol
```rust
#[derive(Debug)]
pub enum WalletWire {
    ListNotes,
    UpdateBalance,
    UpdateBlock,
    Exit,
    Command(Commands),
}

impl Wire for WalletWire {
    const VERSION: u64 = 1;
    const SOURCE: &str = "wallet";
    
    fn to_wire(&self) -> WireRepr {
        let tags = match self {
            WalletWire::ListNotes => vec!["list-notes".into()],
            WalletWire::UpdateBalance => vec!["update-balance".into()],
            WalletWire::Command(command) => {
                vec!["command".into(), command.as_wire_tag().into()]
            }
            // ...
        };
        WireRepr::new(WalletWire::SOURCE, WalletWire::VERSION, tags)
    }
}
```

## Cryptographic Operations

### Key Generation
```rust
fn keygen(entropy: &[u8; 32], sal: &[u8; 16]) -> CommandNoun<NounSlab> {
    let mut slab = NounSlab::new();
    let ent: Byts = Byts::new(entropy.to_vec());
    let ent_noun = ent.into_noun(&mut slab);
    let sal: Byts = Byts::new(sal.to_vec());
    let sal_noun = sal.into_noun(&mut slab);
    Self::wallet("keygen", &[ent_noun, sal_noun], Operation::Poke, &mut slab)
}
```

### Transaction Signing
```rust
fn sign_tx(draft_path: &str, index: Option<u64>) -> CommandNoun<NounSlab> {
    let mut slab = NounSlab::new();
    
    // Read and decode the input bundle
    let draft_data = fs::read(draft_path)?;
    let draft_noun = slab.cue_into(draft_data.as_bytes()?)?;
    
    let index_noun = match index {
        Some(i) => D(i),
        None => D(0),
    };
    
    // Generate random entropy
    let mut entropy_bytes = [0u8; 32];
    getrandom(&mut entropy_bytes)?;
    let entropy = from_bytes(&mut slab, &entropy_bytes).as_noun();
    
    Self::wallet(
        "sign-tx",
        &[draft_noun, index_noun, entropy],
        Operation::Poke,
        &mut slab,
    )
}
```

## Blockchain Integration

### Node Communication
The wallet communicates with Nockchain nodes via Unix domain sockets:

```rust
if let Some(socket_path) = cli.nockchain_socket {
    match UnixStream::connect(&socket_path).await {
        Ok(stream) => {
            info!("Connected to nockchain NPC socket at {:?}", socket_path);
            wallet
                .app
                .add_io_driver(nockapp::npc_client_driver(stream))
                .await;
        }
        Err(e) => {
            error!("Failed to connect to nockchain NPC socket: {}", e);
        }
    }
}
```

### Synchronization
Commands requiring blockchain state are wrapped with sync operations:

```rust
fn wrap_with_sync_run(
    command_noun_slab: NounSlab,
    operation: Operation,
) -> Result<(NounSlab, Operation), NockAppError> {
    let original_root_noun_clone = unsafe { command_noun_slab.root() };
    let mut sync_slab = command_noun_slab.clone();
    let sync_tag = make_tas(&mut sync_slab, "sync-run");
    let tag_noun = sync_tag.as_noun();
    let sync_run_cell = Cell::new(&mut sync_slab, tag_noun, *original_root_noun_clone);
    let sync_run_noun = sync_run_cell.as_noun();
    sync_slab.set_root(sync_run_noun);
    
    Ok((sync_slab, operation))
}
```

## Security Model

### Key Security
- **Entropy Sources**: Cryptographically secure random number generation
- **Key Derivation**: BIP32-compatible hierarchical deterministic keys
- **Storage Encryption**: Keys encrypted at rest
- **Memory Protection**: Secure memory handling for sensitive data

### Transaction Security
- **Digital Signatures**: Ed25519 signatures for all transactions
- **Replay Protection**: Unique transaction identifiers
- **Input Validation**: Comprehensive validation of all inputs
- **Error Handling**: Secure error handling without information leakage

### Network Security
- **Encrypted Communication**: All network communication encrypted
- **Authentication**: Mutual authentication with blockchain nodes
- **Integrity Checking**: Cryptographic verification of all data

## Data Structures

### Notes (UTXOs)
```rust
pub struct Note {
    pub value: u64,
    pub script: Script,
    pub outpoint: OutPoint,
}

pub struct OutPoint {
    pub txid: [u8; 32],
    pub index: u32,
}
```

### Transactions
```rust
pub struct Transaction {
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub fee: u64,
    pub signatures: Vec<Signature>,
}

pub struct TxInput {
    pub outpoint: OutPoint,
    pub script_sig: Script,
}

pub struct TxOutput {
    pub value: u64,
    pub script_pubkey: Script,
}
```

## Error Handling

### Error Types
```rust
#[derive(Debug)]
pub enum WalletError {
    KeyError(KeyError),
    TransactionError(TransactionError),
    NetworkError(NetworkError),
    CryptoError(CryptoError),
}

#[derive(Debug)]
pub enum KeyError {
    InvalidIndex,
    KeyNotFound,
    DerivationFailed,
}
```

### Error Recovery
- **Graceful Degradation**: Continues operation when possible
- **State Recovery**: Automatic recovery from corrupted state
- **Backup Mechanisms**: Multiple backup and recovery options
- **User Feedback**: Clear error messages and recovery instructions

## Configuration

### Data Directory
```rust
pub async fn wallet_data_dir() -> Result<PathBuf, NockAppError> {
    let wallet_data_dir = system_data_dir().join("wallet");
    if !wallet_data_dir.exists() {
        tokio_fs::create_dir_all(&wallet_data_dir).await?;
    }
    Ok(wallet_data_dir)
}
```

### Environment Variables
```bash
# Wallet configuration
WALLET_DATA_DIR=/path/to/wallet/data
NOCKCHAIN_SOCKET=/path/to/nockchain.sock

# Security settings
WALLET_ENCRYPTION_KEY=<encryption-key>
WALLET_BACKUP_INTERVAL=3600
```

## Testing

### Unit Tests
```rust
#[tokio::test]
async fn test_keygen() -> Result<(), NockAppError> {
    let cli = BootCli::parse_from(&["--new"]);
    let nockapp = boot::setup(KERNEL, Some(cli), &[], "wallet", None).await?;
    let mut wallet = Wallet::new(nockapp);
    
    let mut entropy = [0u8; 32];
    let mut salt = [0u8; 16];
    let (noun, op) = Wallet::keygen(&entropy, &salt)?;
    
    let wire = WalletWire::Command(Commands::Keygen).to_wire();
    let result = wallet.app.poke(wire, noun).await?;
    
    assert!(result.len() == 1);
    Ok(())
}
```

### Integration Tests
- **End-to-end transaction flow**
- **Key derivation and recovery**
- **Blockchain synchronization**
- **Multi-signature operations**

## Performance Considerations

### Optimization Strategies
- **Lazy Loading**: Load keys and data on demand
- **Caching**: Cache frequently accessed data
- **Batch Operations**: Batch multiple operations
- **Parallel Processing**: Parallelize independent operations

### Memory Management
- **Secure Allocation**: Use secure memory for sensitive data
- **Cleanup**: Automatic cleanup of sensitive data
- **Limits**: Configurable memory limits
- **Monitoring**: Memory usage monitoring and alerts

## Future Enhancements

### Planned Features
- **Hardware Wallet Support**: Integration with hardware security modules
- **Multi-signature Wallets**: Advanced multi-signature capabilities
- **Time-locked Transactions**: Support for time-locked outputs
- **Atomic Swaps**: Cross-chain atomic swap functionality

### Research Areas
- **Post-quantum Cryptography**: Quantum-resistant key algorithms
- **Zero-knowledge Proofs**: Privacy-preserving transactions
- **Threshold Signatures**: Distributed key management
- **Formal Verification**: Mathematical proof of correctness 