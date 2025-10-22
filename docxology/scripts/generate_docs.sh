#!/bin/bash
# Documentation generation script
# Generates Rust documentation and builds the mdBook

set -e

echo "📚 Generating documentation for Docxology..."

# Check if we're in the right directory
if [[ ! -f "rust/Cargo.toml" || ! -f "docs/book/book.toml" ]]; then
    echo "❌ Error: Please run this script from the docxology directory"
    exit 1
fi

# Generate Rust documentation
echo "📖 Generating Rust API documentation..."
cd rust
cargo doc --no-deps --document-private-items

# Copy generated docs to the docs directory
echo "📁 Copying documentation to docs/generated/..."
rm -rf ../docs/generated/doc
cp -r target/doc ../docs/generated/

# Build the mdBook
echo "📚 Building mdBook documentation..."
cd ../docs/book
mdbook build

echo "✅ Documentation generated successfully!"
echo ""
echo "📖 Documentation available at:"
echo "   - Rust API docs: docs/generated/doc/docxology/"
echo "   - User guide: docs/generated/"
echo ""
echo "🌐 Serve locally with: cd docs/generated && python3 -m http.server 8000"
echo ""
echo "🔍 For semantic search, configure DocuMCP in docs/rag/"
