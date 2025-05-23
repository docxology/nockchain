# Nockchain Networking Documentation

## Overview

Nockchain implements a peer-to-peer networking layer built on libp2p with custom protocols for blockchain communication. The networking system handles peer discovery, block propagation, transaction gossip, and request/response patterns with built-in spam protection via proof-of-work.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    Nockchain P2P Network                       │
├─────────────────────────────────────────────────────────────────┤
│  Application Layer                                              │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │   Block Sync    │  │  Transaction    │  │   Peer Mgmt     │ │
│  │                 │  │     Gossip      │  │                 │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│  Protocol Layer                                                │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │  Request/Resp   │  │    Kademlia     │  │    Identify     │ │
│  │   (CBOR/PoW)    │  │      DHT        │  │                 │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
├─────────────────────────────────────────────────────────────────┤
│  Transport Layer                                               │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
│  │      QUIC       │  │       DNS       │  │   Connection    │ │
│  │                 │  │   Resolution    │  │    Limits       │ │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Network Behavior (`NockchainBehaviour`)

The main network behavior combines multiple libp2p protocols:

- **Identify**: Peer identification and address exchange
- **Ping**: Connection health monitoring
- **Kademlia DHT**: Peer discovery and routing
- **Request/Response**: Custom blockchain message protocol
- **Connection Management**: Limits and peer blocking
- **Peer Store**: Address and metadata persistence

### 2. Protocol Stack

#### Transport Layer
- **QUIC**: Primary transport protocol with TLS encryption
- **DNS Resolution**: Automatic DNS resolution for multiaddresses
- **Connection Timeouts**: Configurable timeouts for handshake and idle connections

#### Protocol Layer
- **Custom Request/Response**: CBOR-encoded messages with EquiX proof-of-work
- **Kademlia DHT**: Distributed hash table for peer discovery
- **Identify Protocol**: Peer information exchange

### 3. Message Types

#### NockchainRequest
```rust
pub enum NockchainRequest {
    /// Request with proof-of-work (anti-spam)
    Request {
        pow: equix::SolutionByteArray,
        nonce: u64,
        message: ByteBuf,
    },
    /// Gossip message (no PoW required)
    Gossip { 
        message: ByteBuf 
    },
}
```

#### NockchainResponse
```rust
pub enum NockchainResponse {
    /// Response with requested data
    Result { message: ByteBuf },
    /// Acknowledgment for gossip
    Ack,
}
```

## Network Configuration

### Connection Limits
```rust
// Maximum connections
const MAX_ESTABLISHED_CONNECTIONS: u32 = 64;
const MAX_ESTABLISHED_INCOMING_CONNECTIONS: u32 = 32;
const MAX_ESTABLISHED_OUTGOING_CONNECTIONS: u32 = 32;
const MAX_ESTABLISHED_CONNECTIONS_PER_PEER: u32 = 2;

// Pending connections
const MAX_PENDING_INCOMING_CONNECTIONS: u32 = 16;
const MAX_PENDING_OUTGOING_CONNECTIONS: u32 = 16;
```

### Timeouts and Intervals
```rust
// Connection management
const SWARM_IDLE_TIMEOUT: Duration = Duration::from_secs(30);
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(15);
const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(15);

// Protocol intervals
const KADEMLIA_BOOTSTRAP_INTERVAL: Duration = Duration::from_secs(300);
const IDENTIFY_INTERVAL: Duration = Duration::from_secs(300);
const REQUEST_RESPONSE_TIMEOUT: Duration = Duration::from_secs(20);
```

## Proof-of-Work Anti-Spam

### EquiX Integration
Nockchain uses EquiX proof-of-work to prevent spam in request/response patterns:

```rust
// PoW generation for requests
let mut nonce = 0u64;
let sol_bytes = loop {
    let nonce_buf = &mut pow_buf[0..size_of::<u64>()];
    nonce_buf.copy_from_slice(&nonce.to_le_bytes()[..]);
    
    if let Ok(sols) = builder.solve(&pow_buf[..]) {
        if !sols.is_empty() {
            break sols[0].to_bytes();
        }
    }
    nonce += 1;
};
```

### PoW Verification
```rust
// Verify PoW on received requests
let Ok(()) = request.verify_pow(equix_builder, &local_peer_id, &peer) else {
    warn!("bad libp2p powork from {peer}, blocking!");
    // Block the peer for invalid PoW
    return Ok(());
};
```

## Peer Discovery and Management

### Kademlia DHT
- **Bootstrap Interval**: 5 minutes
- **Peer Discovery**: Automatic via DHT routing
- **Address Storage**: Persistent peer address storage

### Backbone Nodes
```rust
// Testnet backbone (currently empty for testing)
const TESTNET_BACKBONE_NODES: &[&str] = &[];

// Production backbone
const REALNET_BACKBONE_NODES: &[&str] = &[
    "/dnsaddr/nockchain-backbone.zorp.io"
];
```

### Peer Blocking and Security
- **Automatic Blocking**: Peers sending invalid PoW are blocked
- **IP Logging**: Failed connections logged for fail2ban integration
- **Connection Limits**: Per-peer and global connection limits

## Message Tracking and Deduplication

### MessageTracker
The system tracks messages to prevent duplicates and manage peer relationships:

```rust
pub struct MessageTracker {
    block_id_to_peers: BTreeMap<String, BTreeSet<PeerId>>,
    peer_to_block_ids: BTreeMap<PeerId, BTreeSet<String>>,
    seen_blocks: BTreeSet<String>,
    seen_txs: BTreeSet<String>,
    block_cache: BTreeMap<String, Bytes>,
    tx_cache: BTreeMap<String, Bytes>,
}
```

### Tracking Operations
- **Track Block**: Associate block IDs with peers
- **Remove Peer**: Clean up when peers disconnect
- **Deduplication**: Prevent processing duplicate messages
- **Cache Management**: Store frequently accessed data

## Network Events and Handling

### Event Processing
```rust
pub enum NockchainEvent {
    Identify(identify::Event),
    Ping(ping::Event),
    Kad(kad::Event),
    RequestResponse(request_response::Event<NockchainRequest, NockchainResponse>),
    PeerStore(libp2p::peer_store::memory_store::Event),
}
```

### Swarm Actions
```rust
pub enum SwarmAction {
    SendResponse {
        channel: ResponseChannel<NockchainResponse>,
        response: NockchainResponse,
    },
    SendRequest {
        peer_id: PeerId,
        request: NockchainRequest,
    },
    BlockPeer {
        peer_id: PeerId,
    },
}
```

## Wire Protocol

### Wire Format
Messages use a structured wire format for routing:

```rust
pub enum Libp2pWire {
    Gossip(PeerId),
    Response(PeerId),
}

impl Wire for Libp2pWire {
    const VERSION: u64 = 1;
    const SOURCE: &'static str = "libp2p";
    
    fn to_wire(&self) -> WireRepr {
        let tags = vec![
            self.verb().into(), 
            "peer-id".into(), 
            self.peer_id().to_base58().into()
        ];
        WireRepr::new(Libp2pWire::SOURCE, Libp2pWire::VERSION, tags)
    }
}
```

### Effect Types
The system processes various effect types from the application layer:

- **Gossip**: Broadcast messages to all peers
- **Request**: Request specific data from peers
- **Track**: Track block/peer relationships
- **Seen**: Mark messages as seen
- **LiarPeer**: Handle dishonest peers
- **LiarBlockId**: Handle invalid block IDs

## Network Metrics

### Monitoring
The system includes comprehensive metrics via the `gnort` crate:

```rust
pub struct NockchainP2PMetrics {
    // Connection metrics
    connections_established: Counter,
    connections_closed: Counter,
    
    // Message metrics
    messages_sent: Counter,
    messages_received: Counter,
    
    // Error metrics
    pow_verification_failures: Counter,
    peer_blocks: Counter,
}
```

## Configuration and Deployment

### Environment Variables
```bash
# Logging configuration
RUST_LOG=info,nockchain_libp2p_io=info,libp2p=info

# Network binding
BIND_ADDR=/ip4/0.0.0.0/udp/0/quic-v1

# Peer configuration
INITIAL_PEERS=/ip4/1.2.3.4/udp/1234/quic-v1/p2p/12D3...
```

### Command Line Options
```bash
# Bind to specific address
nockchain --bind /ip4/0.0.0.0/udp/9000/quic-v1

# Connect to specific peers
nockchain --peer /ip4/1.2.3.4/udp/9000/quic-v1/p2p/12D3...

# Disable default backbone peers
nockchain --no-default-peers
```

## Security Considerations

### Threat Model
- **Spam Prevention**: EquiX PoW prevents request flooding
- **Peer Validation**: All messages validated before processing
- **Connection Limits**: Prevent resource exhaustion
- **Automatic Blocking**: Malicious peers automatically blocked

### Best Practices
1. **Firewall Configuration**: Open only necessary P2P ports
2. **NAT Configuration**: Proper port forwarding for connectivity
3. **Monitoring**: Track connection patterns and errors
4. **Resource Limits**: Configure appropriate connection limits

## Troubleshooting

### Common Issues

#### Connection Problems
```bash
# Check peer connectivity
nockchain --peer /ip4/peer.example.com/udp/9000/quic-v1/p2p/12D3...

# Verify port accessibility
nc -u peer.example.com 9000
```

#### NAT Traversal
```bash
# Configure external address
nockchain --bind /ip4/YOUR_PUBLIC_IP/udp/9000/quic-v1
```

#### Debugging
```bash
# Enable detailed logging
RUST_LOG=debug,nockchain_libp2p_io=trace nockchain

# Monitor specific modules
RUST_LOG=libp2p_quic=debug,libp2p_kad=debug nockchain
```

### Performance Tuning

#### Connection Optimization
- Adjust connection limits based on hardware
- Configure appropriate timeouts for network conditions
- Monitor connection churn and stability

#### Message Optimization
- Implement message batching for high throughput
- Optimize serialization for large messages
- Use compression for bandwidth efficiency

## Future Enhancements

### Planned Features
1. **Message Compression**: Reduce bandwidth usage
2. **Advanced Routing**: Implement custom routing strategies
3. **Network Sharding**: Support for network partitioning
4. **Enhanced Metrics**: More detailed performance monitoring
5. **Protocol Versioning**: Backward compatibility support

### Research Areas
1. **Gossip Optimization**: Improve message propagation efficiency
2. **NAT Traversal**: Enhanced connectivity in restricted networks
3. **Privacy Features**: Anonymous communication patterns
4. **Scalability**: Support for larger network sizes

## Integration Points

### NockApp Framework
The networking layer integrates with the NockApp framework through:

- **Driver Interface**: Implements `IODriverFn` for async operation
- **Effect Processing**: Handles networking effects from applications
- **Wire Protocol**: Uses structured wire format for message routing

### Blockchain Layer
Integration with blockchain components:

- **Block Propagation**: Efficient distribution of new blocks
- **Transaction Gossip**: Mempool synchronization across peers
- **Chain Synchronization**: Historical block retrieval
- **Consensus Participation**: Network-wide consensus coordination

This networking layer provides a robust, secure, and scalable foundation for Nockchain's peer-to-peer communication needs, with built-in spam protection and comprehensive monitoring capabilities. 