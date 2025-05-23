# Blockchain Core

## Overview

Nockchain implements a lightweight blockchain designed for verifiable computation. The blockchain uses a UTXO model with proof-of-work consensus and is synchronized with the Bitcoin blockchain for genesis block coordination.

## Architecture

### Core Components

```
┌─────────────────────────────────────────────────────────────┐
│                  Blockchain Core                           │
├─────────────────────────────────────────────────────────────┤
│  Consensus Layer                                            │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │ Proof of    │  │   Mining    │  │   Block     │        │
│  │    Work     │  │  Algorithm  │  │ Validation  │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
├─────────────────────────────────────────────────────────────┤
│  Transaction Layer                                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │    UTXO     │  │ Transaction │  │   Script    │        │
│  │    Model    │  │ Validation  │  │  Execution  │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
├─────────────────────────────────────────────────────────────┤
│  State Management                                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   Chain     │  │   Merkle    │  │    State    │        │
│  │   State     │  │    Trees    │  │   Storage   │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
├─────────────────────────────────────────────────────────────┤
│  Storage Layer                                              │
│  ┌─────────────────────────────────────────────────────────┐│
│  │  Block Storage | Transaction Pool | State Database     ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

## Consensus Mechanism

### Proof of Work

#### Equix Algorithm
Nockchain uses the Equix proof-of-work algorithm:

```rust
use equix::EquiX;

pub fn mine_block(
    block_header: &BlockHeader,
    difficulty: u64,
) -> Option<(u64, [u8; 16])> {
    let equix = EquiX::new();
    let mut nonce = 0u64;
    
    loop {
        let mut hasher = Blake3::new();
        hasher.update(&block_header.serialize());
        hasher.update(&nonce.to_le_bytes());
        let hash = hasher.finalize();
        
        if let Ok(solution) = equix.solve(&hash.as_bytes()) {
            if meets_difficulty(&solution, difficulty) {
                return Some((nonce, solution));
            }
        }
        
        nonce += 1;
        if nonce % 1000000 == 0 {
            // Check for stop signal
            if should_stop_mining() {
                return None;
            }
        }
    }
}
```

#### Difficulty Adjustment
- **Target Block Time**: 20 seconds
- **Adjustment Period**: Every 144 blocks (approximately 48 minutes)
- **Algorithm**: Exponential moving average with bounds

```rust
pub fn calculate_new_difficulty(
    previous_difficulty: u64,
    actual_time: u64,
    target_time: u64,
) -> u64 {
    let ratio = actual_time as f64 / target_time as f64;
    let adjustment_factor = ratio.clamp(0.25, 4.0);
    
    (previous_difficulty as f64 * adjustment_factor) as u64
}
```

### Genesis Block Coordination

#### Bitcoin Synchronization
The genesis block is coordinated with Bitcoin blockchain:

```rust
const GENESIS_HEIGHT: u64 = 897767;
const CHAIN_INTERVAL_SECS: u64 = 20;

pub enum GenesisNodeType {
    Leader,   // Mines the genesis block
    Watcher,  // Waits for genesis block
}
```

#### Genesis Process
1. **Bitcoin Monitoring**: Watch for specific Bitcoin block
2. **Genesis Mining**: Leader node mines genesis block
3. **Genesis Broadcast**: Distribute genesis block to network
4. **Chain Start**: Begin normal block production

## Block Structure

### Block Header
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub version: u32,
    pub previous_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u64,
    pub difficulty: u64,
    pub nonce: u64,
    pub equix_solution: [u8; 16],
}
```

### Block Body
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub coinbase: CoinbaseTransaction,
}
```

### Block Validation
```rust
pub fn validate_block(block: &Block, chain_state: &ChainState) -> Result<(), BlockError> {
    // 1. Validate block header
    validate_block_header(&block.header, chain_state)?;
    
    // 2. Validate proof of work
    validate_proof_of_work(&block.header)?;
    
    // 3. Validate transactions
    for tx in &block.transactions {
        validate_transaction(tx, chain_state)?;
    }
    
    // 4. Validate merkle root
    let calculated_root = calculate_merkle_root(&block.transactions);
    if calculated_root != block.header.merkle_root {
        return Err(BlockError::InvalidMerkleRoot);
    }
    
    // 5. Validate coinbase
    validate_coinbase(&block.coinbase, &block.transactions)?;
    
    Ok(())
}
```

## Transaction Model

### UTXO (Unspent Transaction Output)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTXO {
    pub outpoint: OutPoint,
    pub output: TxOutput,
    pub height: u64,
    pub coinbase: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutPoint {
    pub txid: [u8; 32],
    pub index: u32,
}
```

### Transaction Structure
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub version: u32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u64,
    pub fee: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxInput {
    pub outpoint: OutPoint,
    pub script_sig: Script,
    pub sequence: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxOutput {
    pub value: u64,
    pub script_pubkey: Script,
}
```

### Transaction Validation
```rust
pub fn validate_transaction(
    tx: &Transaction,
    utxo_set: &UTXOSet,
) -> Result<(), TransactionError> {
    // 1. Basic format validation
    if tx.inputs.is_empty() || tx.outputs.is_empty() {
        return Err(TransactionError::EmptyInputsOrOutputs);
    }
    
    // 2. Input validation
    let mut total_input_value = 0u64;
    for input in &tx.inputs {
        let utxo = utxo_set.get(&input.outpoint)
            .ok_or(TransactionError::UTXONotFound)?;
        
        // Validate script signature
        validate_script(&input.script_sig, &utxo.output.script_pubkey)?;
        
        total_input_value += utxo.output.value;
    }
    
    // 3. Output validation
    let total_output_value: u64 = tx.outputs.iter()
        .map(|output| output.value)
        .sum();
    
    // 4. Fee validation
    if total_input_value < total_output_value + tx.fee {
        return Err(TransactionError::InsufficientFunds);
    }
    
    // 5. Double-spend check
    for input in &tx.inputs {
        if utxo_set.is_spent(&input.outpoint) {
            return Err(TransactionError::DoubleSpend);
        }
    }
    
    Ok(())
}
```

## Script System

### Script Types
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Script {
    PayToPubkey(PublicKey),
    PayToPubkeyHash([u8; 20]),
    PayToScriptHash([u8; 20]),
    Multisig { m: u8, pubkeys: Vec<PublicKey> },
    Timelock { time: u64, script: Box<Script> },
    Custom(Vec<u8>),
}
```

### Script Execution
```rust
pub fn execute_script(
    script_sig: &Script,
    script_pubkey: &Script,
    tx: &Transaction,
    input_index: usize,
) -> Result<bool, ScriptError> {
    match (script_sig, script_pubkey) {
        (Script::Custom(sig_bytes), Script::PayToPubkey(pubkey)) => {
            let signature = Signature::from_bytes(sig_bytes)?;
            let tx_hash = calculate_tx_hash(tx, input_index);
            Ok(pubkey.verify(&tx_hash, &signature))
        }
        
        (Script::Custom(sig_bytes), Script::PayToPubkeyHash(pubkey_hash)) => {
            // Extract pubkey and signature from script_sig
            let (signature, pubkey) = parse_p2pkh_script_sig(sig_bytes)?;
            
            // Verify pubkey hash
            let calculated_hash = hash160(&pubkey.to_bytes());
            if calculated_hash != *pubkey_hash {
                return Ok(false);
            }
            
            // Verify signature
            let tx_hash = calculate_tx_hash(tx, input_index);
            Ok(pubkey.verify(&tx_hash, &signature))
        }
        
        (script_sig, Script::Multisig { m, pubkeys }) => {
            execute_multisig_script(script_sig, *m, pubkeys, tx, input_index)
        }
        
        _ => Err(ScriptError::UnsupportedScriptType),
    }
}
```

## State Management

### Chain State
```rust
#[derive(Debug, Clone)]
pub struct ChainState {
    pub best_block_hash: [u8; 32],
    pub best_block_height: u64,
    pub total_work: U256,
    pub utxo_set: UTXOSet,
    pub difficulty: u64,
    pub last_adjustment_height: u64,
}
```

### UTXO Set Management
```rust
pub struct UTXOSet {
    utxos: HashMap<OutPoint, UTXO>,
    spent: HashSet<OutPoint>,
}

impl UTXOSet {
    pub fn apply_transaction(&mut self, tx: &Transaction, height: u64) {
        // Remove spent UTXOs
        for input in &tx.inputs {
            self.spent.insert(input.outpoint.clone());
            self.utxos.remove(&input.outpoint);
        }
        
        // Add new UTXOs
        let txid = tx.calculate_hash();
        for (index, output) in tx.outputs.iter().enumerate() {
            let outpoint = OutPoint {
                txid,
                index: index as u32,
            };
            let utxo = UTXO {
                outpoint: outpoint.clone(),
                output: output.clone(),
                height,
                coinbase: false,
            };
            self.utxos.insert(outpoint, utxo);
        }
    }
    
    pub fn revert_transaction(&mut self, tx: &Transaction) {
        // Remove UTXOs created by this transaction
        let txid = tx.calculate_hash();
        for index in 0..tx.outputs.len() {
            let outpoint = OutPoint {
                txid,
                index: index as u32,
            };
            self.utxos.remove(&outpoint);
        }
        
        // Restore spent UTXOs (requires additional data)
        for input in &tx.inputs {
            self.spent.remove(&input.outpoint);
            // Note: Would need to restore UTXO from storage
        }
    }
}
```

### Merkle Trees
```rust
pub fn calculate_merkle_root(transactions: &[Transaction]) -> [u8; 32] {
    if transactions.is_empty() {
        return [0; 32];
    }
    
    let mut hashes: Vec<[u8; 32]> = transactions
        .iter()
        .map(|tx| tx.calculate_hash())
        .collect();
    
    while hashes.len() > 1 {
        let mut next_level = Vec::new();
        
        for chunk in hashes.chunks(2) {
            let combined = if chunk.len() == 2 {
                [chunk[0], chunk[1]].concat()
            } else {
                [chunk[0], chunk[0]].concat()
            };
            
            let mut hasher = Blake3::new();
            hasher.update(&combined);
            next_level.push(hasher.finalize().into());
        }
        
        hashes = next_level;
    }
    
    hashes[0]
}
```

## Storage Layer

### Block Storage
```rust
pub trait BlockStorage {
    fn store_block(&mut self, block: &Block) -> Result<(), StorageError>;
    fn get_block(&self, hash: &[u8; 32]) -> Result<Option<Block>, StorageError>;
    fn get_block_header(&self, hash: &[u8; 32]) -> Result<Option<BlockHeader>, StorageError>;
    fn get_best_block(&self) -> Result<Option<Block>, StorageError>;
}

pub struct FileBlockStorage {
    data_dir: PathBuf,
    index: HashMap<[u8; 32], u64>, // hash -> file offset
}
```

### Transaction Pool
```rust
pub struct TransactionPool {
    transactions: HashMap<[u8; 32], Transaction>,
    by_fee_rate: BTreeMap<u64, HashSet<[u8; 32]>>, // fee_rate -> tx_hashes
    dependencies: HashMap<[u8; 32], HashSet<[u8; 32]>>, // tx -> dependent_txs
}

impl TransactionPool {
    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), PoolError> {
        let txid = tx.calculate_hash();
        let fee_rate = tx.fee * 1000 / tx.serialized_size() as u64;
        
        // Validate transaction
        self.validate_transaction(&tx)?;
        
        // Add to pool
        self.transactions.insert(txid, tx);
        self.by_fee_rate.entry(fee_rate).or_default().insert(txid);
        
        Ok(())
    }
    
    pub fn select_transactions(&self, max_size: usize) -> Vec<Transaction> {
        let mut selected = Vec::new();
        let mut total_size = 0;
        
        // Select transactions by fee rate (highest first)
        for (_, tx_hashes) in self.by_fee_rate.iter().rev() {
            for tx_hash in tx_hashes {
                if let Some(tx) = self.transactions.get(tx_hash) {
                    let tx_size = tx.serialized_size();
                    if total_size + tx_size <= max_size {
                        selected.push(tx.clone());
                        total_size += tx_size;
                    }
                }
            }
        }
        
        selected
    }
}
```

## Network Protocol

### Block Propagation
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockMessage {
    NewBlock(Block),
    BlockRequest([u8; 32]),
    BlockResponse(Option<Block>),
    BlockHeaders(Vec<BlockHeader>),
}

pub async fn handle_new_block(
    block: Block,
    chain_state: &mut ChainState,
    peer_manager: &mut PeerManager,
) -> Result<(), NetworkError> {
    // Validate block
    validate_block(&block, chain_state)?;
    
    // Apply block to chain state
    apply_block(&block, chain_state)?;
    
    // Propagate to peers
    let message = BlockMessage::NewBlock(block);
    peer_manager.broadcast(message).await?;
    
    Ok(())
}
```

### Chain Synchronization
```rust
pub async fn sync_chain(
    peer: &mut Peer,
    local_state: &ChainState,
) -> Result<Vec<Block>, SyncError> {
    // Request block headers from peer
    let headers_request = BlockMessage::BlockHeaders(vec![]);
    peer.send(headers_request).await?;
    
    // Receive headers
    let headers = match peer.receive().await? {
        BlockMessage::BlockHeaders(headers) => headers,
        _ => return Err(SyncError::UnexpectedMessage),
    };
    
    // Identify missing blocks
    let mut missing_blocks = Vec::new();
    for header in headers {
        if !local_state.has_block(&header.hash()) {
            missing_blocks.push(header.hash());
        }
    }
    
    // Request missing blocks
    let mut blocks = Vec::new();
    for block_hash in missing_blocks {
        let block_request = BlockMessage::BlockRequest(block_hash);
        peer.send(block_request).await?;
        
        match peer.receive().await? {
            BlockMessage::BlockResponse(Some(block)) => blocks.push(block),
            BlockMessage::BlockResponse(None) => {
                return Err(SyncError::BlockNotFound(block_hash));
            }
            _ => return Err(SyncError::UnexpectedMessage),
        }
    }
    
    Ok(blocks)
}
```

## Performance Optimizations

### Parallel Validation
```rust
pub fn validate_block_parallel(
    block: &Block,
    chain_state: &ChainState,
) -> Result<(), BlockError> {
    use rayon::prelude::*;
    
    // Validate transactions in parallel
    let validation_results: Result<Vec<_>, _> = block
        .transactions
        .par_iter()
        .map(|tx| validate_transaction(tx, &chain_state.utxo_set))
        .collect();
    
    validation_results?;
    
    // Validate other block components
    validate_block_header(&block.header, chain_state)?;
    validate_proof_of_work(&block.header)?;
    
    Ok(())
}
```

### Caching
```rust
pub struct BlockCache {
    blocks: LruCache<[u8; 32], Block>,
    headers: LruCache<[u8; 32], BlockHeader>,
    utxos: LruCache<OutPoint, UTXO>,
}

impl BlockCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            blocks: LruCache::new(capacity),
            headers: LruCache::new(capacity * 2),
            utxos: LruCache::new(capacity * 10),
        }
    }
}
```

## Error Handling

### Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum BlockchainError {
    #[error("Block validation failed: {0}")]
    BlockValidation(#[from] BlockError),
    
    #[error("Transaction validation failed: {0}")]
    TransactionValidation(#[from] TransactionError),
    
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
}

#[derive(Debug, thiserror::Error)]
pub enum BlockError {
    #[error("Invalid proof of work")]
    InvalidProofOfWork,
    
    #[error("Invalid merkle root")]
    InvalidMerkleRoot,
    
    #[error("Invalid timestamp")]
    InvalidTimestamp,
    
    #[error("Invalid difficulty")]
    InvalidDifficulty,
}
```

## Configuration

### Chain Parameters
```rust
pub struct ChainParams {
    pub target_block_time: u64,
    pub difficulty_adjustment_interval: u64,
    pub max_block_size: usize,
    pub coinbase_maturity: u64,
    pub genesis_block: Block,
}

pub const MAINNET_PARAMS: ChainParams = ChainParams {
    target_block_time: 20,
    difficulty_adjustment_interval: 144,
    max_block_size: 1_000_000,
    coinbase_maturity: 100,
    genesis_block: GENESIS_BLOCK,
};
```

### Runtime Configuration
```rust
pub struct BlockchainConfig {
    pub data_dir: PathBuf,
    pub max_connections: usize,
    pub enable_mining: bool,
    pub mining_threads: usize,
    pub mempool_size: usize,
}
``` 