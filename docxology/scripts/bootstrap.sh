#!/bin/bash
# Bootstrap script for Docxology development environment
# This script sets up all necessary dependencies for developing with Docxology

set -e

echo "🚀 Bootstrapping Docxology development environment..."

# Check if we're in the right directory
if [[ ! -f "../Cargo.toml" || ! -f "README.md" ]]; then
    echo "❌ Error: Please run this script from the docxology directory"
    exit 1
fi

# Function to detect OS
detect_os() {
    case "$(uname -s)" in
        Linux*)     echo "linux" ;;
        Darwin*)    echo "macos" ;;
        CYGWIN*|MINGW*|MSYS*) echo "windows" ;;
        *)          echo "unknown" ;;
    esac
}

OS=$(detect_os)
echo "📋 Detected OS: $OS"

# Install Rust if not present
if ! command -v rustup &> /dev/null; then
    echo "🔧 Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
else
    echo "✅ Rust toolchain already installed"
fi

# Update Rust toolchain
echo "🔄 Updating Rust toolchain..."
rustup update stable

# Install additional Rust components
echo "🔧 Installing Rust components..."
rustup component add rustfmt clippy

# Install system dependencies based on OS
case $OS in
    "linux")
        echo "🔧 Installing Linux dependencies..."

        # Update package manager
        if command -v apt-get &> /dev/null; then
            sudo apt-get update
            sudo apt-get install -y \
                build-essential \
                pkg-config \
                libssl-dev \
                clang \
                llvm-dev \
                libclang-dev \
                protobuf-compiler \
                python3-dev \
                python3-pip
        elif command -v yum &> /dev/null; then
            sudo yum groupinstall -y "Development Tools"
            sudo yum install -y \
                openssl-devel \
                clang \
                llvm-devel \
                clang-devel \
                protobuf-compiler \
                python3-devel \
                python3-pip
        fi
        ;;

    "macos")
        echo "🔧 Installing macOS dependencies..."

        # Install Homebrew if not present
        if ! command -v brew &> /dev/null; then
            /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        fi

        # Install dependencies
        brew install \
            openssl \
            protobuf \
            python3
        ;;

    "windows")
        echo "🔧 Installing Windows dependencies..."

        # Install Chocolatey if not present
        if ! command -v choco &> /dev/null; then
            echo "Please install Chocolatey from https://chocolatey.org/"
            echo "Then run: choco install rust python3 protoc"
            exit 1
        fi

        choco install -y \
            rust \
            python3 \
            protoc
        ;;
esac

# Install Python dependencies for documentation and Python bindings
echo "🐍 Installing Python dependencies..."
python3 -m pip install --upgrade pip

# Install documentation tools
echo "📚 Installing documentation tools..."
python3 -m pip install mdbook mdbook-mermaid mdbook-plantuml

# Install Python development tools
python3 -m pip install maturin pytest black mypy

# Install Rust dependencies for Python bindings
echo "🦀 Installing Rust Python binding dependencies..."
cargo install cbindgen

# Build the project to verify everything works
echo "🔨 Building Docxology..."
cd ../
cargo check --workspace

echo "✅ Bootstrap completed successfully!"
echo ""
echo "🎉 Next steps:"
echo "  1. Run 'cargo build' to build the project"
echo "  2. Run 'cargo test' to run tests"
echo "  3. Check out examples in 'docxology/rust/examples/'"
echo "  4. Read the documentation in 'docxology/docs/book/'"
echo ""
echo "📖 For more information, see the README.md"
