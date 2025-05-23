# Build System

## Overview

Nockchain uses a hybrid build system combining Rust's Cargo with Make for orchestrating complex build processes involving multiple languages and compilation stages.

## Workspace Structure

### Cargo Workspace
The project is organized as a Rust workspace with multiple crates:

```toml
[workspace]
members = [
    "crates/equix-latency",
    "crates/kernels", 
    "crates/hoonc",
    "crates/nockapp",
    "crates/nockchain-bitcoin-sync",
    "crates/nockchain-libp2p-io", 
    "crates/nockchain",
    "crates/nockvm/rust/ibig",
    "crates/nockvm/rust/murmur3",
    "crates/nockvm/rust/nockvm_macros",
    "crates/nockvm/rust/nockvm",
    "crates/nockchain-wallet",
    "crates/zkvm-jetpack",
]
```

### Crate Dependencies

#### Core Crates
- **nockchain**: Main blockchain implementation
- **nockvm**: Nock virtual machine
- **nockapp**: Application framework
- **kernels**: System kernels for different use cases

#### Specialized Crates
- **hoonc**: Hoon language compiler
- **nockchain-wallet**: Wallet implementation
- **nockchain-libp2p-io**: Networking layer
- **nockchain-bitcoin-sync**: Bitcoin integration
- **zkvm-jetpack**: Zero-knowledge proof system

#### Utility Crates
- **ibig**: Big integer arithmetic
- **murmur3**: Hash function implementation
- **nockvm_macros**: Procedural macros
- **equix-latency**: Mining algorithm benchmarks

## Build Profiles

### Development Profile
```toml
[profile.dev]
opt-level = 0
debug = 2
```
- No optimization for fast compilation
- Full debug information
- Suitable for development and debugging

### Release Profile
```toml
[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
debug = 1
```
- Maximum optimization
- Link-time optimization
- Single codegen unit for better optimization
- Minimal debug info

### Test Profile
```toml
[profile.test]
inherits = "release"
```
- Inherits release optimizations
- Ensures tests run with production-like performance

## Makefile Orchestration

### Environment Configuration
```makefile
# Load environment variables from .env file
include .env

# Set default env variables if not set in .env
export RUST_BACKTRACE ?= full
export RUST_LOG ?= info,nockchain=info,nockchain_libp2p_io=info,libp2p=info,libp2p_quic=info
export MINIMAL_LOG_FORMAT ?= true
export MINING_PUBKEY ?= <default-key>
```

### Primary Build Targets

#### Complete Build
```makefile
.PHONY: build
build: build-hoon-all build-rust
	$(call show_env_vars)
```
Builds both Hoon applications and Rust components.

#### Rust Build
```makefile
.PHONY: build-rust
build-rust:
	cargo build --release
```
Compiles all Rust crates in release mode.

#### Hoon Build
```makefile
.PHONY: build-hoon-all
build-hoon-all: nuke-assets update-hoonc ensure-dirs build-trivial $(HOON_TARGETS)
```
Compiles Hoon applications to JAM files.

### Installation Targets

#### Hoon Compiler
```makefile
.PHONY: install-hoonc
install-hoonc: nuke-hoonc-data
	cargo install --locked --force --path crates/hoonc --bin hoonc
```

#### Nockchain Node
```makefile
.PHONY: install-nockchain
install-nockchain: build-hoon-all
	cargo install --locked --force --path crates/nockchain --bin nockchain
```

#### Wallet
```makefile
.PHONY: install-nockchain-wallet
install-nockchain-wallet: build-hoon-all
	cargo install --locked --force --path crates/nockchain-wallet --bin nockchain-wallet
```

## Hoon Compilation Pipeline

### Hoon Source Files
```
hoon/
├── apps/
│   ├── dumbnet/
│   │   ├── outer.hoon    → assets/dumb.jam
│   │   └── miner.hoon    → assets/miner.jam
│   └── wallet/
│       └── wallet.hoon   → assets/wal.jam
└── common/               # Shared libraries
```

### Compilation Process
```makefile
# Build dumb.jam with hoonc
assets/dumb.jam: update-hoonc hoon/apps/dumbnet/outer.hoon $(HOON_SRCS)
	RUST_LOG=trace hoonc hoon/apps/dumbnet/outer.hoon hoon
	mv out.jam assets/dumb.jam
```

1. **Dependency Check**: Ensures hoonc is up to date
2. **Source Analysis**: Scans all Hoon source files
3. **Compilation**: Compiles Hoon to Nock bytecode
4. **JAM Encoding**: Serializes bytecode to binary format
5. **Asset Placement**: Moves compiled assets to target directory

## Dependencies

### Rust Dependencies

#### Core Dependencies
```toml
# Async runtime
tokio = { version = "1.32", features = ["full"] }

# Serialization
serde = "1.0.217"
serde_json = "1.0.104"
bincode = "2.0.0-rc.3"

# Cryptography
ed25519-dalek = "2.1.0"
x25519-dalek = "2.0.0"
aes-siv = "0.7.0"
sha2 = "0.10.8"
blake3 = "1.5.1"

# Networking
libp2p = { git = "https://github.com/libp2p/rust-libp2p.git" }
```

#### Specialized Dependencies
```toml
# Mining
equix = "0.2.2"

# Bitcoin integration
bitcoincore-rpc = "0.19.0"

# CLI
clap = "4.4.4"

# Tracing
tracing = "0.1.41"
tracing-subscriber = "0.3.18"
```

### System Dependencies

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install clang llvm-dev libclang-dev
```

#### Required Tools
- **Rust**: Latest stable toolchain via rustup
- **Clang**: C/C++ compiler for native dependencies
- **LLVM**: Required for some cryptographic libraries

## Rust Toolchain

### Toolchain Configuration
```toml
[toolchain]
channel = "nightly-2025-02-14"
components = ["miri"]
```

- **Nightly Channel**: Required for advanced features
- **Miri**: Memory safety checker for unsafe code
- **Specific Date**: Ensures reproducible builds

### Feature Flags

#### NockVM Features
```toml
[features]
default = ["mmap"]
malloc = []           # Use malloc instead of mmap
mmap = []            # Use memory mapping
check_all = []       # Enable all runtime checks
sham_hints = []      # Disable hint processing
```

#### Build-time Features
- **mmap**: Memory-mapped file I/O (default)
- **malloc**: Standard heap allocation
- **check_all**: Comprehensive runtime validation
- **sham_hints**: Performance optimization

## Build Scripts

### Custom Build Scripts
Some crates include custom build scripts for:

#### Code Generation
```rust
// build.rs
use vergen::{vergen, Config};

fn main() {
    let mut config = Config::default();
    *config.git_mut().sha_kind_mut() = vergen::ShaKind::Short;
    vergen(config).expect("Unable to generate version info");
}
```

#### Native Library Integration
```rust
// Linking native libraries
fn main() {
    cc::Build::new()
        .file("src/native/crypto.c")
        .compile("crypto");
}
```

## Testing Infrastructure

### Test Execution
```makefile
.PHONY: test
test:
	cargo test --release
```

### Test Categories
- **Unit Tests**: Individual component testing
- **Integration Tests**: Cross-component testing
- **Property Tests**: QuickCheck-style testing
- **Benchmark Tests**: Performance validation

### Test Configuration
```toml
[dev-dependencies]
criterion = { git = "https://github.com/vlovich/criterion.rs.git" }
quickcheck = "1.0.3"
quickcheck_macros = "1.0"
```

## Asset Management

### Asset Generation
```makefile
HOON_TARGETS=assets/dumb.jam assets/wal.jam assets/miner.jam

.PHONY: nuke-assets
nuke-assets:
	rm -f assets/*.jam
```

### Asset Dependencies
- **Hoon Source**: Application source code
- **Hoon Libraries**: Shared functionality
- **Compiler**: Up-to-date hoonc binary

## Environment Configuration

### Environment Variables
```bash
# Logging configuration
RUST_LOG=info,nockchain=info,nockchain_libp2p_io=info
MINIMAL_LOG_FORMAT=true

# Mining configuration
MINING_PUBKEY=<your-public-key>

# Development settings
RUST_BACKTRACE=full
```

### Configuration Files
- **.env**: Local environment variables
- **.env_example**: Template configuration
- **rust-toolchain.toml**: Rust version specification

## Continuous Integration

### Build Matrix
```yaml
# Example CI configuration
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest]
    rust: [nightly-2025-02-14]
```

### Build Steps
1. **Environment Setup**: Install dependencies
2. **Toolchain Installation**: Configure Rust
3. **Dependency Caching**: Cache Cargo dependencies
4. **Compilation**: Build all components
5. **Testing**: Run test suite
6. **Asset Verification**: Validate generated assets

## Optimization Strategies

### Compilation Optimization
- **LTO**: Link-time optimization for release builds
- **Codegen Units**: Single unit for maximum optimization
- **Target CPU**: Native CPU features when possible

### Dependency Optimization
- **Workspace Dependencies**: Shared version management
- **Feature Selection**: Minimal feature sets
- **Optional Dependencies**: Conditional compilation

### Build Caching
- **Cargo Cache**: Dependency and build caching
- **Asset Cache**: Compiled Hoon asset caching
- **Incremental Builds**: Minimal recompilation

## Troubleshooting

### Common Issues

#### Missing Dependencies
```bash
# Ubuntu/Debian
sudo apt install clang llvm-dev libclang-dev

# macOS
xcode-select --install
```

#### Toolchain Issues
```bash
# Update toolchain
rustup update nightly-2025-02-14
rustup component add miri
```

#### Build Failures
```bash
# Clean build
cargo clean
make nuke-assets
make build
```

### Debug Builds
```bash
# Enable debug logging
RUST_LOG=debug make build

# Build with debug symbols
cargo build --profile dev
``` 