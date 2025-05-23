#!/bin/bash
# Nockchain Development Environment Setup Script
# Comprehensive installation of Rust, dependencies, and nockit toolkit

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
NOCKIT_VERSION="0.5.0"
RUST_MIN_VERSION="1.70.0"
CONFIG_DIR="${HOME}/.nockit"

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

log_step() {
    echo -e "${PURPLE}[STEP]${NC} $1"
}

# Print banner
print_banner() {
    echo -e "${CYAN}"
    cat << 'EOF'
    ╔═══════════════════════════════════════════════════════════════╗
    ║                                                               ║
    ║    🚀 Nockchain Development Environment Setup v0.5.0         ║
    ║                                                               ║
    ║    Comprehensive installation of Rust, dependencies,         ║
    ║    and the complete nockchain development toolkit            ║
    ║                                                               ║
    ╚═══════════════════════════════════════════════════════════════╝
EOF
    echo -e "${NC}"
}

# Detect operating system
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if [ -f /etc/os-release ]; then
            . /etc/os-release
            OS=$ID
            DISTRO=$ID
        else
            OS="linux"
            DISTRO="unknown"
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
        DISTRO="macos"
    elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
        OS="windows"
        DISTRO="windows"
    else
        OS="unknown"
        DISTRO="unknown"
    fi
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check if running as root
check_root() {
    if [[ $EUID -eq 0 ]]; then
        log_error "This script should not be run as root"
        log_info "Please run as a regular user. The script will prompt for sudo when needed."
        exit 1
    fi
}

# Check system requirements
check_requirements() {
    log_step "Checking system requirements..."
    
    # Check available disk space (require at least 2GB)
    available_space=$(df / | awk 'NR==2 {print $4}')
    required_space=2097152  # 2GB in KB
    
    if [ "$available_space" -lt "$required_space" ]; then
        log_error "Insufficient disk space. Required: 2GB, Available: $(($available_space / 1024 / 1024))GB"
        exit 1
    fi
    
    log_success "Disk space check passed"
    
    # Check internet connectivity
    if ! curl -s --head https://httpbin.org/ip > /dev/null; then
        log_error "Internet connectivity required for installation"
        exit 1
    fi
    
    log_success "Internet connectivity check passed"
}

# Install system dependencies
install_system_deps() {
    log_step "Installing system dependencies..."
    
    case "$DISTRO" in
        ubuntu|debian|pop)
            log_info "Installing dependencies for Ubuntu/Debian..."
            sudo apt update
            sudo apt install -y \
                build-essential \
                pkg-config \
                libssl-dev \
                libclang-dev \
                cmake \
                curl \
                wget \
                git \
                unzip
            ;;
        fedora|rhel|centos)
            log_info "Installing dependencies for Fedora/RHEL/CentOS..."
            sudo dnf install -y \
                gcc \
                gcc-c++ \
                make \
                cmake \
                pkg-config \
                openssl-devel \
                clang-devel \
                curl \
                wget \
                git \
                unzip
            ;;
        arch|manjaro)
            log_info "Installing dependencies for Arch Linux..."
            sudo pacman -Sy --noconfirm \
                base-devel \
                pkg-config \
                openssl \
                clang \
                cmake \
                curl \
                wget \
                git \
                unzip
            ;;
        macos)
            log_info "Installing dependencies for macOS..."
            
            # Install Homebrew if not present
            if ! command_exists brew; then
                log_info "Installing Homebrew..."
                /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
            fi
            
            # Install Xcode command line tools
            if ! xcode-select -p &> /dev/null; then
                log_info "Installing Xcode command line tools..."
                xcode-select --install
                log_warning "Please complete the Xcode installation and re-run this script"
                exit 1
            fi
            
            brew install pkg-config openssl cmake git
            ;;
        *)
            log_warning "Unknown distribution: $DISTRO"
            log_info "Please install the following manually:"
            log_info "  - GCC/Clang compiler"
            log_info "  - Make and CMake"
            log_info "  - pkg-config"
            log_info "  - OpenSSL development headers"
            log_info "  - Git"
            ;;
    esac
    
    log_success "System dependencies installed"
}

# Install Rust
install_rust() {
    log_step "Installing Rust..."
    
    if command_exists rustc; then
        local rust_version=$(rustc --version | cut -d' ' -f2)
        log_info "Rust is already installed: $rust_version"
        
        # Check if version is sufficient
        if [ "$(printf '%s\n' "$RUST_MIN_VERSION" "$rust_version" | sort -V | head -n1)" = "$RUST_MIN_VERSION" ]; then
            log_success "Rust version is sufficient"
            return 0
        else
            log_warning "Rust version is too old. Updating..."
        fi
    fi
    
    # Install or update Rust
    log_info "Downloading and installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    
    # Source cargo environment
    source "$HOME/.cargo/env"
    
    # Update to latest stable
    rustup update stable
    rustup default stable
    
    # Install useful components
    log_info "Installing Rust components..."
    rustup component add clippy rustfmt rust-src rust-analyzer
    
    log_success "Rust installed successfully"
}

# Install development tools
install_dev_tools() {
    log_step "Installing development tools..."
    
    # Ensure cargo is available
    source "$HOME/.cargo/env"
    
    local tools=(
        "cargo-audit"
        "cargo-outdated"
        "cargo-tree"
        "cargo-watch"
        "cargo-edit"
        "cargo-expand"
    )
    
    for tool in "${tools[@]}"; do
        log_info "Installing $tool..."
        if cargo install "$tool"; then
            log_success "$tool installed"
        else
            log_warning "Failed to install $tool, continuing..."
        fi
    done
}

# Clone nockchain repository
clone_nockchain() {
    log_step "Setting up nockchain repository..."
    
    local repo_dir="$HOME/nockchain"
    
    if [ -d "$repo_dir" ]; then
        log_info "Nockchain repository already exists at $repo_dir"
        cd "$repo_dir"
        
        # Update repository
        log_info "Updating repository..."
        git pull origin main || git pull origin master || log_warning "Failed to update repository"
    else
        log_info "Cloning nockchain repository..."
        git clone https://github.com/nockchain/nockchain.git "$repo_dir"
        cd "$repo_dir"
    fi
    
    log_success "Nockchain repository ready"
}

# Build and install nockit
build_nockit() {
    log_step "Building and installing nockit..."
    
    cd "$HOME/nockchain/nockit"
    
    # Build nockit
    log_info "Building nockit toolkit..."
    cargo build --release
    
    # Install nockit binaries
    log_info "Installing nockit binaries..."
    cargo install --path .
    
    log_success "Nockit installed successfully"
}

# Build nockchain binaries
build_nockchain() {
    log_step "Building nockchain binaries..."
    
    cd "$HOME/nockchain"
    
    # Build nockchain
    log_info "Building nockchain..."
    cargo build --release --bin nockchain
    
    # Build nockchain-wallet
    log_info "Building nockchain-wallet..."
    cargo build --release --bin nockchain-wallet
    
    # Install binaries
    log_info "Installing nockchain binaries..."
    cargo install --path crates/nockchain
    cargo install --path crates/nockchain-wallet
    
    log_success "Nockchain binaries installed successfully"
}

# Setup configuration
setup_config() {
    log_step "Setting up configuration..."
    
    # Ensure cargo is available
    source "$HOME/.cargo/env"
    
    # Create configuration directory
    mkdir -p "$CONFIG_DIR"
    
    # Run nockit setup
    log_info "Running nockit setup..."
    if command_exists nockit; then
        nockit setup --force --non-interactive
    else
        log_warning "nockit command not found, skipping configuration setup"
    fi
    
    log_success "Configuration setup completed"
}

# Add to shell configuration
setup_shell() {
    log_step "Setting up shell configuration..."
    
    local shell_config=""
    
    # Detect shell and config file
    case "$SHELL" in
        */bash)
            shell_config="$HOME/.bashrc"
            ;;
        */zsh)
            shell_config="$HOME/.zshrc"
            ;;
        */fish)
            shell_config="$HOME/.config/fish/config.fish"
            ;;
        *)
            shell_config="$HOME/.profile"
            ;;
    esac
    
    # Add cargo to PATH if not already present
    local cargo_env_line='. "$HOME/.cargo/env"'
    
    if [ -f "$shell_config" ] && ! grep -q "$cargo_env_line" "$shell_config"; then
        echo "" >> "$shell_config"
        echo "# Added by nockchain setup" >> "$shell_config"
        echo "$cargo_env_line" >> "$shell_config"
        log_success "Added Rust to shell configuration: $shell_config"
    fi
    
    # Add nockit configuration
    local nockit_env_line="export NOCKIT_CONFIG_DIR=\"$CONFIG_DIR\""
    
    if [ -f "$shell_config" ] && ! grep -q "NOCKIT_CONFIG_DIR" "$shell_config"; then
        echo "$nockit_env_line" >> "$shell_config"
        log_success "Added nockit configuration to shell"
    fi
}

# Verify installation
verify_installation() {
    log_step "Verifying installation..."
    
    # Source cargo environment
    source "$HOME/.cargo/env"
    
    local errors=0
    
    # Check Rust
    if command_exists rustc; then
        local rust_version=$(rustc --version)
        log_success "Rust: $rust_version"
    else
        log_error "Rust not found"
        ((errors++))
    fi
    
    # Check Cargo
    if command_exists cargo; then
        local cargo_version=$(cargo --version)
        log_success "Cargo: $cargo_version"
    else
        log_error "Cargo not found"
        ((errors++))
    fi
    
    # Check Git
    if command_exists git; then
        local git_version=$(git --version)
        log_success "Git: $git_version"
    else
        log_error "Git not found"
        ((errors++))
    fi
    
    # Check nockit
    if command_exists nockit; then
        local nockit_version=$(nockit --version)
        log_success "Nockit: $nockit_version"
    else
        log_error "Nockit not found"
        ((errors++))
    fi
    
    # Check nockchain binaries
    if command_exists nockchain; then
        log_success "Nockchain binary: Available"
    else
        log_warning "Nockchain binary not found (optional)"
    fi
    
    if command_exists nockchain-wallet; then
        log_success "Nockchain wallet: Available"
    else
        log_warning "Nockchain wallet not found (optional)"
    fi
    
    if [ $errors -eq 0 ]; then
        log_success "All core components verified successfully!"
    else
        log_error "$errors errors found during verification"
        return 1
    fi
}

# Print next steps
print_next_steps() {
    echo -e "${CYAN}"
    cat << 'EOF'
    ╔═══════════════════════════════════════════════════════════════╗
    ║                                                               ║
    ║    🎉 Installation completed successfully!                    ║
    ║                                                               ║
    ╚═══════════════════════════════════════════════════════════════╝
EOF
    echo -e "${NC}"
    
    log_info "Next steps:"
    echo "1. Restart your terminal or run: source ~/.cargo/env"
    echo "2. Generate wallet keys: nockit wallet keygen"
    echo "3. Start mining: nockit mining start --pubkey <YOUR_PUBKEY>"
    echo "4. Monitor system: nockit monitor"
    echo "5. Check logs: nockit logs tail --follow"
    echo ""
    log_info "Useful commands:"
    echo "• nockit --help                 - Show all available commands"
    echo "• nocksetup check              - Verify installation status"
    echo "• nocksetup verify             - Run comprehensive verification"
    echo "• nockmon                      - Real-time system monitoring"
    echo "• nocklog tail -f              - Follow live logs"
    echo ""
    log_info "Configuration directory: $CONFIG_DIR"
    log_info "Helper scripts: $CONFIG_DIR/scripts/"
    echo ""
    log_success "Happy nockchain development! 🚀"
}

# Handle script interruption
cleanup() {
    log_warning "Installation interrupted"
    exit 1
}

# Parse command line arguments
parse_args() {
    SKIP_DEPS=false
    SKIP_RUST=false
    SKIP_NOCKCHAIN=false
    GENERATE_KEYS=false
    NON_INTERACTIVE=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --skip-deps)
                SKIP_DEPS=true
                shift
                ;;
            --skip-rust)
                SKIP_RUST=true
                shift
                ;;
            --skip-nockchain)
                SKIP_NOCKCHAIN=true
                shift
                ;;
            --generate-keys)
                GENERATE_KEYS=true
                shift
                ;;
            --non-interactive)
                NON_INTERACTIVE=true
                shift
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
}

# Show help
show_help() {
    cat << EOF
Nockchain Development Environment Setup Script

Usage: $0 [OPTIONS]

Options:
    --skip-deps         Skip system dependencies installation
    --skip-rust         Skip Rust installation
    --skip-nockchain    Skip nockchain binaries installation
    --generate-keys     Generate wallet keys after setup
    --non-interactive   Run without interactive prompts
    --help, -h          Show this help message

Examples:
    $0                              # Full installation
    $0 --skip-deps                  # Skip system dependencies
    $0 --generate-keys              # Include key generation
    $0 --non-interactive            # Automated installation

EOF
}

# Main installation function
main() {
    # Set up signal handlers
    trap cleanup SIGINT SIGTERM
    
    # Parse arguments
    parse_args "$@"
    
    # Print banner
    print_banner
    
    # Initial checks
    check_root
    detect_os
    log_info "Detected OS: $OS ($DISTRO)"
    
    check_requirements
    
    # Installation steps
    if [ "$SKIP_DEPS" = false ]; then
        install_system_deps
    else
        log_info "Skipping system dependencies installation"
    fi
    
    if [ "$SKIP_RUST" = false ]; then
        install_rust
        install_dev_tools
    else
        log_info "Skipping Rust installation"
    fi
    
    clone_nockchain
    build_nockit
    
    if [ "$SKIP_NOCKCHAIN" = false ]; then
        build_nockchain
    else
        log_info "Skipping nockchain binaries installation"
    fi
    
    setup_config
    setup_shell
    
    # Generate keys if requested
    if [ "$GENERATE_KEYS" = true ]; then
        log_step "Generating wallet keys..."
        source "$HOME/.cargo/env"
        if command_exists nockit; then
            nockit wallet keygen
            log_success "Wallet keys generated"
        else
            log_warning "nockit not available for key generation"
        fi
    fi
    
    # Verify installation
    if verify_installation; then
        print_next_steps
    else
        log_error "Installation verification failed"
        exit 1
    fi
}

# Run main function with all arguments
main "$@" 