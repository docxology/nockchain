# Nockchain Operations and Deployment Guide

## Overview

This guide covers the operational aspects of running Nockchain nodes, including deployment, monitoring, troubleshooting, and maintenance procedures.

## System Requirements

### Hardware Requirements

#### Mining Requirements
- **CPU**: 1 core, mining is single-threaded at this point.
- **RAM**: 64+ GB
- **Storage**: 200+ GB NVMe SSD
- **Network**: Stable, low-latency connection.

### Software Requirements

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install clang llvm-dev libclang-dev build-essential

# Arch Linux
sudo pacman -S clang llvm

# macOS
xcode-select --install
```

## Installation and Setup

### Quick Start

```bash
# Clone repository
git clone https://github.com/zorp-corp/nockchain.git
cd nockchain

# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Setup environment
cp .env_example .env

# Build and install
make install-hoonc
make build
make install-nockchain
make install-nockchain-wallet

# Add to PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

### Environment Configuration

#### `.env` File Configuration
```bash
# Logging configuration
RUST_LOG=info,nockchain=info,nockchain_libp2p_io=info,libp2p=info,libp2p_quic=info
MINIMAL_LOG_FORMAT=true

# Mining configuration
MINING_PUBKEY=<your-public-key>

# Optional: Custom ports
PEER_PORT=9000
```

#### Key Generation
```bash
# Generate new key pair
nockchain-wallet keygen

# Export keys for backup
nockchain-wallet export-keys

# Import keys from backup
nockchain-wallet import-keys --input keys.export
```

## Node Operations

### Running Nodes

#### Standard Node (Non-Mining)
```bash
# Basic node
nockchain

# With custom configuration
nockchain --bind /ip4/0.0.0.0/udp/9000/quic-v1

# Using script
./scripts/run_nockchain_node.sh
```

#### Mining Node
```bash
# Basic mining
nockchain --mining-pubkey <pubkey> --mine

# Advanced mining configuration
nockchain --mining-key-adv 1,1:<key1> --mine

# Using script
./scripts/run_nockchain_miner.sh
```

#### Genesis Operations
```bash
# Genesis leader (mines genesis block)
nockchain --genesis-leader --btc-node-url <url> --btc-username <user> --btc-password <pass>

# Genesis watcher (waits for genesis)
nockchain --genesis-watcher --btc-node-url <url> --btc-username <user> --btc-password <pass>

# Test network (fake genesis)
nockchain --fakenet --genesis-leader
```

### Network Configuration

#### Firewall Configuration
```bash
# Allow P2P port (default: random)
sudo ufw allow 9000/udp

# For specific bind address
sudo ufw allow from any to any port 9000 proto udp
```

#### NAT Configuration
```bash
# Port forwarding for NAT
nockchain --bind /ip4/<public-ip>/udp/<port>/quic-v1

# Example
nockchain --bind /ip4/203.0.113.1/udp/9000/quic-v1
```

#### Peer Management
```bash
# Connect to specific peers
nockchain --peer /ip4/203.0.113.1/udp/9000/quic-v1/p2p/<peer-id>

# Disable default peers
nockchain --no-default-peers

# Use allowed peers file
nockchain --allowed-peers-path peers.txt
```

### Multi-Instance Setup

```bash
# Create separate directories
mkdir node1 node2 node3

# Copy configuration
cp .env node1/
cp .env node2/
cp .env node3/

# Run instances with different ports
cd node1 && nockchain --bind /ip4/0.0.0.0/udp/9001/quic-v1 &
cd node2 && nockchain --bind /ip4/0.0.0.0/udp/9002/quic-v1 &
cd node3 && nockchain --bind /ip4/0.0.0.0/udp/9003/quic-v1 &
```

## Wallet Operations

### Basic Wallet Commands

```bash
# Check balance
nockchain-wallet --nockchain-socket ./nockchain.sock list-notes

# List notes by pubkey
nockchain-wallet --nockchain-socket ./nockchain.sock list-notes-by-pubkey <pubkey>

# Simple spend
nockchain-wallet --nockchain-socket ./nockchain.sock simple-spend \
  --names "note1,note2" \
  --recipients "addr1,addr2" \
  --gifts "100,200" \
  --fee 10

# Scan blockchain
nockchain-wallet --nockchain-socket ./nockchain.sock scan \
  --master-pubkey <pubkey> \
  --search-depth 100
```

### Key Management

```bash
# Derive child keys
nockchain-wallet derive-child --key-type pub --index 1
nockchain-wallet derive-child --key-type priv --index 1

# Show master keys
nockchain-wallet show-master-pubkey
nockchain-wallet show-master-privkey
nockchain-wallet show-seedphrase

# List all pubkeys
nockchain-wallet list-pubkeys
```

### Transaction Operations

```bash
# Create transaction from draft
nockchain-wallet make-tx --draft transaction.draft

# Sign transaction
nockchain-wallet sign-tx --draft transaction.draft --index 0
```

## Monitoring and Logging

### Log Configuration

#### Log Levels
```bash
# Minimal logging
RUST_LOG=error nockchain

# Standard logging
RUST_LOG=info nockchain

# Debug logging
RUST_LOG=debug nockchain

# Module-specific logging
RUST_LOG=nockchain=info,nockchain_libp2p_io=debug nockchain
```

#### Log Format
```bash
# Minimal format
MINIMAL_LOG_FORMAT=true nockchain

# Full format (default)
MINIMAL_LOG_FORMAT=false nockchain
```

### Monitoring Metrics

#### Key Metrics to Monitor
- **Block Height**: Current chain height
- **Peer Count**: Number of connected peers
- **Mining Status**: Mining activity and success rate
- **Memory Usage**: RAM consumption
- **Disk Usage**: Storage utilization
- **Network I/O**: Bandwidth usage

#### Log Analysis
```bash
# Monitor block production
tail -f nockchain.log | grep "block.*added to validated blocks"

# Monitor mining activity
tail -f nockchain.log | grep "mining-on"

# Monitor peer connections
tail -f nockchain.log | grep "peer"

# Monitor errors
tail -f nockchain.log | grep -i error
```

### Health Checks

```bash
#!/bin/bash
# health_check.sh

# Check if process is running
if ! pgrep -f nockchain > /dev/null; then
    echo "ERROR: Nockchain process not running"
    exit 1
fi

# Check socket connectivity
if [ -S "./nockchain.sock" ]; then
    echo "OK: Socket available"
else
    echo "WARNING: Socket not available"
fi

# Check disk space
DISK_USAGE=$(df . | tail -1 | awk '{print $5}' | sed 's/%//')
if [ "$DISK_USAGE" -gt 90 ]; then
    echo "WARNING: Disk usage at ${DISK_USAGE}%"
fi

echo "Health check completed"
```

## Troubleshooting

### Common Issues

#### Node Won't Start
```bash
# Check port availability
netstat -tulpn | grep :9000

# Check permissions
ls -la .data.nockchain

# Clean start
rm -rf .data.nockchain
nockchain --new
```

#### Peer Connection Issues
```bash
# Check firewall
sudo ufw status

# Test connectivity
telnet <peer-ip> <peer-port>

# Check NAT configuration
nockchain --bind /ip4/<public-ip>/udp/<port>/quic-v1
```

#### Mining Issues
```bash
# Verify mining key
nockchain-wallet show-master-pubkey

# Check mining flag
nockchain --mining-pubkey <pubkey> --mine

# Monitor mining logs
tail -f nockchain.log | grep mining

# Monitor mining activity with filtering
nockchain --mining-pubkey <pubkey> --mine | grep -aE "serf|panic|mining|validated|candidate"
```

##### Memory-Related Mining Issues

**Serf Panic Error Fix**
If you encounter "serf - panicked" errors during mining, especially on memory-constrained systems, try this fix:

```bash
# Fix for serf panic on memory-poor miners
sudo sysctl -w vm.overcommit_memory=1
```

This setting allows the kernel to overcommit memory, which can resolve memory allocation issues during mining operations. The setting tells the kernel to always say "yes" when a process asks for memory, which prevents allocation failures that can cause the serf (mining process) to panic.

**Making the Fix Permanent**
To make this setting persistent across reboots:

```bash
# Add to sysctl configuration
echo "vm.overcommit_memory=1" | sudo tee -a /etc/sysctl.conf

# Apply immediately
sudo sysctl -p
```

**Memory Overcommit Options**
- `0` (default): Heuristic overcommit handling
- `1`: Always overcommit (recommended for mining)
- `2`: Don't overcommit (strict accounting)

**Additional Memory Optimization for Mining**
```bash
# Check current memory settings
cat /proc/sys/vm/overcommit_memory
cat /proc/sys/vm/overcommit_ratio

# Monitor memory usage during mining
watch -n 1 'free -h && ps aux | grep nockchain | grep -v grep'

# Set swap usage preference (optional)
sudo sysctl -w vm.swappiness=10
```

#### Wallet Connection Issues
```bash
# Check socket path
ls -la nockchain.sock

# Verify socket permissions
chmod 660 nockchain.sock

# Test connection
nockchain-wallet --nockchain-socket ./nockchain.sock list-notes
```

### Performance Optimization

#### System Tuning
```bash
# Increase file descriptor limits
echo "* soft nofile 65536" >> /etc/security/limits.conf
echo "* hard nofile 65536" >> /etc/security/limits.conf

# Optimize network settings
echo "net.core.rmem_max = 134217728" >> /etc/sysctl.conf
echo "net.core.wmem_max = 134217728" >> /etc/sysctl.conf
sysctl -p
```

#### Memory Optimization
```bash
# Set memory limits
nockchain --max-system-memory-fraction 0.8

# Monitor memory usage
watch -n 1 'ps aux | grep nockchain'
```

#### Storage Optimization
```bash
# Use SSD for data directory
ln -s /mnt/ssd/nockchain-data .data.nockchain

# Monitor I/O
iotop -p $(pgrep nockchain)
```

## Backup and Recovery

### Data Backup

```bash
#!/bin/bash
# backup.sh

BACKUP_DIR="/backup/nockchain/$(date +%Y%m%d_%H%M%S)"
mkdir -p "$BACKUP_DIR"

# Backup blockchain data
cp -r .data.nockchain "$BACKUP_DIR/"

# Backup wallet keys
nockchain-wallet export-keys
cp keys.export "$BACKUP_DIR/"

# Backup configuration
cp .env "$BACKUP_DIR/"

echo "Backup completed: $BACKUP_DIR"
```

### Recovery Procedures

```bash
#!/bin/bash
# restore.sh

BACKUP_DIR="$1"

if [ -z "$BACKUP_DIR" ]; then
    echo "Usage: $0 <backup_directory>"
    exit 1
fi

# Stop nockchain
pkill nockchain

# Restore data
rm -rf .data.nockchain
cp -r "$BACKUP_DIR/.data.nockchain" .

# Restore keys
nockchain-wallet import-keys --input "$BACKUP_DIR/keys.export"

# Restore configuration
cp "$BACKUP_DIR/.env" .

echo "Recovery completed from: $BACKUP_DIR"
```

## Security Considerations

### Key Security
- Store private keys in secure, encrypted storage
- Use hardware security modules for production
- Implement key rotation procedures
- Regular backup verification

### Network Security
- Use VPN for sensitive operations
- Implement proper firewall rules
- Monitor for suspicious network activity
- Use allowed peers lists for restricted networks

### Operational Security
- Regular security updates
- Monitor system logs
- Implement access controls
- Use secure communication channels

## Production Deployment

### Systemd Service

```ini
# /etc/systemd/system/nockchain.service
[Unit]
Description=Nockchain Node
After=network.target

[Service]
Type=simple
User=nockchain
Group=nockchain
WorkingDirectory=/opt/nockchain
ExecStart=/usr/local/bin/nockchain
Restart=always
RestartSec=10
Environment=RUST_LOG=info
Environment=MINIMAL_LOG_FORMAT=true

[Install]
WantedBy=multi-user.target
```

```bash
# Enable and start service
sudo systemctl enable nockchain
sudo systemctl start nockchain
sudo systemctl status nockchain
```

### Docker Deployment

```dockerfile
# Dockerfile
FROM rust:1.75 as builder

WORKDIR /app
COPY . .
RUN make build

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/nockchain /usr/local/bin/
COPY --from=builder /app/target/release/nockchain-wallet /usr/local/bin/

EXPOSE 9000/udp
VOLUME ["/data"]

CMD ["nockchain"]
```

```yaml
# docker-compose.yml
version: '3.8'
services:
  nockchain:
    build: .
    ports:
      - "9000:9000/udp"
    volumes:
      - nockchain_data:/data
    environment:
      - RUST_LOG=info
      - MINIMAL_LOG_FORMAT=true
    restart: unless-stopped

volumes:
  nockchain_data:
```

### Load Balancing

```nginx
# nginx.conf for multiple nodes
upstream nockchain_nodes {
    server 127.0.0.1:9001;
    server 127.0.0.1:9002;
    server 127.0.0.1:9003;
}

server {
    listen 9000 udp;
    proxy_pass nockchain_nodes;
    proxy_timeout 1s;
    proxy_responses 1;
}
```

## Maintenance Procedures

### Regular Maintenance

```bash
#!/bin/bash
# maintenance.sh

# Update system packages
sudo apt update && sudo apt upgrade -y

# Check disk space
df -h

# Rotate logs
logrotate /etc/logrotate.d/nockchain

# Backup data
./backup.sh

# Check service status
systemctl status nockchain

# Verify connectivity
nockchain-wallet --nockchain-socket ./nockchain.sock list-notes
```

### Upgrade Procedures

```bash
#!/bin/bash
# upgrade.sh

# Stop service
sudo systemctl stop nockchain

# Backup current version
cp /usr/local/bin/nockchain /usr/local/bin/nockchain.backup

# Build new version
git pull
make build

# Install new version
sudo cp target/release/nockchain /usr/local/bin/
sudo cp target/release/nockchain-wallet /usr/local/bin/

# Start service
sudo systemctl start nockchain

# Verify upgrade
nockchain --version
```

## Performance Tuning

### CPU Optimization
- Use high-performance CPU governor
- Pin processes to specific cores
- Optimize compiler flags for target architecture

### Memory Optimization
- Tune garbage collection parameters
- Use memory-mapped files for large datasets
- Implement memory pooling

### Network Optimization
- Tune TCP/UDP buffer sizes
- Use high-performance network drivers
- Implement connection pooling

### Storage Optimization
- Use NVMe SSDs for best performance
- Implement proper I/O scheduling
- Use filesystem optimizations (ext4, xfs)

## Conclusion

This operations guide provides comprehensive coverage of Nockchain deployment, monitoring, and maintenance. Regular monitoring, proper backup procedures, and proactive maintenance are essential for reliable operation.

For additional support and updates, refer to the project documentation and community resources. 