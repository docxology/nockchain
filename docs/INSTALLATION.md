# Nockchain Installation Guide

This guide provides detailed instructions for installing and setting up Nockchain on various operating systems, along with troubleshooting tips for common issues.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Installation Steps](#installation-steps)
   - [Linux](#linux)
   - [macOS](#macos)
   - [Windows](#windows)
3. [Verification](#verification)
4. [Running Nockchain](#running-nockchain)
5. [Troubleshooting](#troubleshooting)
6. [Updating](#updating)
7. [Docker Installation](#docker-installation)

## Prerequisites

Before installing Nockchain, ensure your system meets the following requirements:

### Hardware Requirements

- **CPU**: 64-bit processor, 2+ cores recommended
- **RAM**: 4GB minimum, 8GB+ recommended
- **Disk Space**: 10GB minimum free space, SSD recommended
- **Network**: Stable internet connection

### Software Requirements

- **Rust Toolchain**: 1.70.0 or newer
- **Git**: For cloning the repository
- **Build Tools**: Appropriate for your operating system (see OS-specific sections)

## Installation Steps

### Linux

#### Ubuntu/Debian

1. **Install build dependencies**

   ```bash
   sudo apt update
   sudo apt install -y build-essential pkg-config libssl-dev git curl
   ```

2. **Install Rust using rustup**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

3. **Clone the repository**

   ```bash
   git clone https://github.com/tetra/nockchain.git
   cd nockchain
   ```

4. **Install the Hoon compiler**

   ```bash
   make install-choo
   ```

5. **Build Nockchain**

   ```bash
   make build-hoon-all
   make build
   ```

#### Fedora/RHEL/CentOS

1. **Install build dependencies**

   ```bash
   sudo dnf install -y gcc gcc-c++ make openssl-devel git curl
   ```

2. Follow steps 2-5 from Ubuntu/Debian installation.

### macOS

1. **Install Homebrew (if not already installed)**

   ```bash
   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
   ```

2. **Install build dependencies**

   ```bash
   brew install openssl pkg-config git
   ```

3. **Install Rust using rustup**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

4. **Clone the repository**

   ```bash
   git clone https://github.com/tetra/nockchain.git
   cd nockchain
   ```

5. **Install the Hoon compiler**

   ```bash
   make install-choo
   ```

6. **Build Nockchain**

   ```bash
   make build-hoon-all
   make build
   ```

### Windows

#### Using Windows Subsystem for Linux (Recommended)

1. **Install WSL 2** following the [official guide](https://docs.microsoft.com/en-us/windows/wsl/install)

2. **Launch your WSL distribution** (e.g., Ubuntu) and follow the Linux installation steps.

#### Native Windows Installation

1. **Install build tools**
   - Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
   - During installation, select "C++ build tools" and ensure the Windows 10 SDK is included

2. **Install Rust using rustup**
   - Download and run the [rustup installer](https://win.rustup.rs/)
   - Choose the default installation options

3. **Install Git**
   - Download and install [Git for Windows](https://git-scm.com/download/win)

4. **Clone the repository**
   ```
   git clone https://github.com/tetra/nockchain.git
   cd nockchain
   ```

5. **Install the Hoon compiler**
   ```
   make install-choo
   ```

6. **Build Nockchain**
   ```
   make build-hoon-all
   make build
   ```

## Verification

After installation, verify that Nockchain has been installed correctly:

```bash
./target/release/nockchain --version
```

You should see output showing the version number of Nockchain.

## Running Nockchain

### Starting a Leader Node

To run a node that creates and publishes the genesis block:

```bash
make run-nockchain-leader
```

### Starting a Follower Node

To run a node that connects to the network and syncs with existing blocks:

```bash
make run-nockchain-follower
```

### Running Tests

To verify the installation by running the test suite:

```bash
make test
```

## Troubleshooting

### Common Issues and Solutions

#### Rust Installation Problems

**Issue**: `rustup` command not found after installation.

**Solution**: Restart your terminal or run `source $HOME/.cargo/env` to update your PATH.

---

**Issue**: Rust compilation errors.

**Solution**: Ensure you have the latest Rust version:
```bash
rustup update
```

#### Build Failures

**Issue**: `make build` fails with dependency errors.

**Solution**: Update your package manager and reinstall dependencies. On Ubuntu/Debian:
```bash
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev
```

---

**Issue**: `make build-hoon-all` fails.

**Solution**: Ensure the Hoon compiler is properly installed:
```bash
make install-choo
which choo  # Should output a path
```

#### Runtime Errors

**Issue**: "Address already in use" error when starting Nockchain.

**Solution**: Check for and stop any existing Nockchain processes:
```bash
pkill nockchain
# or on Windows
taskkill /IM nockchain.exe /F
```

---

**Issue**: Node fails to connect to the network.

**Solution**: Check your network configuration and firewall settings. Ensure the required ports are open (default is 8765).

#### Operating System Specific Issues

**macOS Issue**: OpenSSL linking errors.

**Solution**: Set OpenSSL paths explicitly:
```bash
export OPENSSL_INCLUDE_DIR=$(brew --prefix openssl)/include
export OPENSSL_LIB_DIR=$(brew --prefix openssl)/lib
```

---

**Windows Issue**: "Make not recognized" error.

**Solution**: Install Make for Windows:
```
choco install make  # Using Chocolatey package manager
```
Or use WSL (recommended).

### Diagnostic Commands

To gather information for troubleshooting:

```bash
# Check system information
uname -a  # Linux/macOS
systeminfo  # Windows

# Check Rust version
rustc --version
cargo --version

# Check OpenSSL version
openssl version

# Verify Hoon compiler
choo --version

# Show environment variables
env | grep RUST
```

## Updating

To update Nockchain to the latest version:

1. **Pull the latest changes**

   ```bash
   git pull origin main
   ```

2. **Rebuild the project**

   ```bash
   make build-hoon-all
   make build
   ```

3. **Restart your node** with the updated binary.

## Docker Installation

For a containerized installation:

1. **Install Docker** following the [official guide](https://docs.docker.com/get-docker/)

2. **Build the Docker image**

   ```bash
   docker build -t nockchain .
   ```

3. **Run a leader node**

   ```bash
   docker run -p 8765:8765 --name nockchain-leader nockchain make run-nockchain-leader
   ```

4. **Run a follower node**

   ```bash
   docker run -p 8766:8765 --name nockchain-follower nockchain make run-nockchain-follower
   ```

### Docker Compose

For a multi-node setup using Docker Compose:

1. Create a `docker-compose.yml` file:

```yaml
version: '3'
services:
  leader:
    build: .
    ports:
      - "8765:8765"
    command: make run-nockchain-leader
    volumes:
      - leader-data:/data

  follower:
    build: .
    ports:
      - "8766:8765"
    command: make run-nockchain-follower
    depends_on:
      - leader
    volumes:
      - follower-data:/data

volumes:
  leader-data:
  follower-data:
```

2. Start the services:

```bash
docker-compose up
```

## Further Reading

- [Main README](../README.md)
- [Contributing Guide](./CONTRIBUTING.md)
- [Technical Roadmap](./ROADMAP.md)
- [Documentation Home](./README.md) 