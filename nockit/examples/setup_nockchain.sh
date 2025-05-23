#!/bin/bash

# Nockchain Development Environment Setup Script
# This script automates the complete setup process for nockchain development
# including system dependencies, Rust installation, and project building

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running on supported OS
check_os() {
    log_info "Checking operating system compatibility..."
    
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        log_success "Linux detected - proceeding with setup"
        
        # Check for Ubuntu/Debian
        if command -v apt &> /dev/null; then
            PACKAGE_MANAGER="apt"
            log_info "Using apt package manager"
        else
            log_error "This script currently supports Ubuntu/Debian systems with apt"
            exit 1
        fi
    else
        log_error "This script currently supports Linux systems only"
        exit 1
    fi
}

# Update system packages
update_system() {
    log_info "Updating system packages..."
    sudo apt update
    log_success "System packages updated"
}

# Install system dependencies
install_system_deps() {
    log_info "Installing system dependencies (clang, llvm-dev, libclang-dev)..."
    
    # Check if dependencies are already installed
    if command -v clang &> /dev/null && command -v llvm-config &> /dev/null; then
        log_success "System dependencies already installed"
        return 0
    fi
    
    sudo apt install -y clang llvm-dev libclang-dev
    log_success "System dependencies installed"
}

# Install Rust
install_rust() {
    log_info "Checking Rust installation..."
    
    # Check if Rust is already installed
    if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
        log_success "Rust is already installed"
        rustc --version
        cargo --version
        return 0
    fi
    
    log_info "Installing Rust via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    
    # Source the Rust environment
    source "$HOME/.cargo/env"
    
    log_success "Rust installed successfully"
    rustc --version
    cargo --version
}

# Setup environment file
setup_env_file() {
    log_info "Setting up environment file..."
    
    if [[ ! -f ".env" ]]; then
        if [[ -f ".env_example" ]]; then
            cp .env_example .env
            log_success "Environment file created from .env_example"
        else
            log_warning ".env_example not found, creating basic .env file"
            cat > .env << EOF
# Nockchain Environment Configuration
MINING_PUBKEY=
RUST_LOG=info
EOF
        fi
    else
        log_success "Environment file already exists"
    fi
}

# Install hoonc
install_hoonc() {
    log_info "Installing hoonc (Hoon compiler)..."
    
    # Check if hoonc is already installed
    if command -v hoonc &> /dev/null; then
        log_success "hoonc is already installed"
        return 0
    fi
    
    # Clean up any existing hoonc data
    rm -rf .data.hoonc
    rm -rf ~/.nockapp/hoonc
    
    # Install hoonc
    make install-hoonc
    
    log_success "hoonc installed successfully"
}

# Build the project
build_project() {
    log_info "Building nockchain project..."
    
    # This will compile Hoon files and build Rust binaries
    make build
    
    log_success "Project built successfully"
}

# Install nockchain binaries
install_binaries() {
    log_info "Installing nockchain binaries..."
    
    # Install wallet
    if ! command -v nockchain-wallet &> /dev/null; then
        log_info "Installing nockchain-wallet..."
        make install-nockchain-wallet
        log_success "nockchain-wallet installed"
    else
        log_success "nockchain-wallet already installed"
    fi
    
    # Install nockchain
    if ! command -v nockchain &> /dev/null; then
        log_info "Installing nockchain..."
        make install-nockchain
        log_success "nockchain installed"
    else
        log_success "nockchain already installed"
    fi
    
    # Update PATH
    export PATH="$HOME/.cargo/bin:$PATH"
    log_success "PATH updated to include Cargo binaries"
}

# Verify installation
verify_installation() {
    log_info "Verifying installation..."
    
    # Check Rust
    if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
        log_success "✓ Rust: $(rustc --version)"
    else
        log_error "✗ Rust installation failed"
        return 1
    fi
    
    # Check system dependencies
    if command -v clang &> /dev/null; then
        log_success "✓ Clang: $(clang --version | head -n1)"
    else
        log_error "✗ Clang not found"
        return 1
    fi
    
    # Check hoonc
    if command -v hoonc &> /dev/null; then
        log_success "✓ hoonc: Available"
    else
        log_error "✗ hoonc not found"
        return 1
    fi
    
    # Check nockchain binaries
    if command -v nockchain-wallet &> /dev/null; then
        log_success "✓ nockchain-wallet: Available"
    else
        log_warning "⚠ nockchain-wallet not found in PATH"
    fi
    
    if command -v nockchain &> /dev/null; then
        log_success "✓ nockchain: Available"
    else
        log_warning "⚠ nockchain not found in PATH"
    fi
    
    log_success "Installation verification completed"
}

# Generate keys (optional)
generate_keys() {
    log_info "Key generation is optional. You can generate keys later with:"
    echo "  nockchain-wallet keygen"
    echo ""
    echo "To set up mining, copy the public key to your .env file:"
    echo "  MINING_PUBKEY=<your-public-key>"
}

# Main setup function
main() {
    echo "🚀 Nockchain Development Environment Setup"
    echo "=========================================="
    echo ""
    
    # Check if we're in the right directory
    if [[ ! -f "Cargo.toml" ]] || [[ ! -d "crates/nockchain" ]]; then
        log_error "This script must be run from the nockchain project root directory"
        exit 1
    fi
    
    # Run setup steps
    check_os
    update_system
    install_system_deps
    install_rust
    
    # Source Rust environment for current session
    source "$HOME/.cargo/env" 2>/dev/null || true
    
    setup_env_file
    install_hoonc
    build_project
    install_binaries
    verify_installation
    
    echo ""
    log_success "🎉 Nockchain development environment setup completed!"
    echo ""
    echo "Next steps:"
    echo "1. Generate keys: nockchain-wallet keygen"
    echo "2. Update .env with your mining pubkey"
    echo "3. Run a node: sh ./scripts/run_nockchain_node.sh"
    echo "4. Run a miner: sh ./scripts/run_nockchain_miner.sh"
    echo ""
    echo "For more information, see the README.md file."
    
    generate_keys
}

# Handle script arguments
case "${1:-}" in
    --help|-h)
        echo "Nockchain Development Environment Setup Script"
        echo ""
        echo "Usage: $0 [options]"
        echo ""
        echo "Options:"
        echo "  --help, -h     Show this help message"
        echo "  --verify       Only run verification checks"
        echo "  --deps-only    Only install system dependencies"
        echo ""
        echo "This script will:"
        echo "1. Update system packages"
        echo "2. Install system dependencies (clang, llvm-dev, libclang-dev)"
        echo "3. Install Rust via rustup"
        echo "4. Set up environment file"
        echo "5. Install hoonc (Hoon compiler)"
        echo "6. Build the nockchain project"
        echo "7. Install nockchain binaries"
        echo "8. Verify the installation"
        exit 0
        ;;
    --verify)
        verify_installation
        exit 0
        ;;
    --deps-only)
        check_os
        update_system
        install_system_deps
        install_rust
        log_success "Dependencies installation completed"
        exit 0
        ;;
    "")
        main
        ;;
    *)
        log_error "Unknown option: $1"
        echo "Use --help for usage information"
        exit 1
        ;;
esac 