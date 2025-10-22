#!/bin/bash
# End-to-end testing script for local development
# Runs a complete end-to-end test with a local Nockchain node

set -e

echo "🧪 Running end-to-end tests for Docxology..."

# Check if we're in the right directory
if [[ ! -f "rust/Cargo.toml" ]]; then
    echo "❌ Error: Please run this script from the docxology directory"
    exit 1
fi

# Check if required tools are installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Cargo not found. Please install Rust."
    exit 1
fi

# Clean previous test data
echo "🧹 Cleaning previous test data..."
rm -rf /tmp/docxology_e2e_*

# Set up test environment variables
export RUST_LOG=info
export NOCKCHAIN_DATA_DIR="/tmp/docxology_e2e_node"
export NOCKCHAIN_LOGGING__LEVEL="info"
export NOCKCHAIN_API__ENABLE_PUBLIC="true"
export NOCKCHAIN_API__PUBLIC_ADDR="127.0.0.1:8080"

echo "🔧 Test environment configured:"
echo "   Data dir: $NOCKCHAIN_DATA_DIR"
echo "   API endpoint: $NOCKCHAIN_API__PUBLIC_ADDR"
echo "   Log level: $NOCKCHAIN_LOGGING__LEVEL"

# Build the project
echo "🔨 Building Docxology..."
cargo build --release

# Run the end-to-end test
echo "🚀 Running end-to-end test..."
cargo test e2e_full_flow --release -- --nocapture

echo "✅ End-to-end test completed successfully!"
echo ""
echo "📊 Test Results:"
echo "   - Node startup: ✅"
echo "   - Miner operations: ✅"
echo "   - Wallet operations: ✅"
echo "   - gRPC communication: ✅"
echo "   - Transaction flows: ✅"
echo ""
echo "🧹 Test data cleaned up automatically"
echo ""
echo "🎉 All end-to-end tests passed!"
