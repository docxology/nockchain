#!/bin/bash
# Build script for Python bindings
# Builds the Rust crate with Python bindings and installs the Python package

set -e

echo "🐍 Building Python bindings for Docxology..."

# Check if we're in the right directory
if [[ ! -f "rust/Cargo.toml" || ! -f "python/pyproject.toml" ]]; then
    echo "❌ Error: Please run this script from the docxology directory"
    exit 1
fi

# Check if maturin is installed
if ! command -v maturin &> /dev/null; then
    echo "🔧 Installing maturin..."
    python3 -m pip install maturin
fi

# Check if we're in a virtual environment
if [[ "$VIRTUAL_ENV" != "" ]]; then
    echo "📦 Using virtual environment: $VIRTUAL_ENV"
else
    echo "⚠️  Warning: Not in a virtual environment"
    echo "   Consider using 'python3 -m venv venv && source venv/bin/activate'"
fi

# Build the Rust library with Python bindings
echo "🔨 Building Rust library with Python bindings..."
cd rust
cargo build --release --features python

# Build the Python package
echo "📦 Building Python package..."
cd ../python
maturin develop --release

echo "✅ Python bindings built and installed successfully!"
echo ""
echo "🎉 You can now use Docxology from Python:"
echo "   import docxology"
echo "   from docxology import NodeConfig, start_node"
echo ""
echo "📖 Check out the Python examples in 'examples/'"
echo ""
echo "🧪 Run Python tests with: python -m pytest tests/"
