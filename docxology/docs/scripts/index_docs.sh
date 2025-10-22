#!/bin/bash
# Documentation indexing script for DocuMCP
# Indexes all documentation sources for semantic search

set -e

echo "🔍 Indexing documentation for semantic search..."

# Check if we're in the right directory
if [[ ! -f "rag/config.json" ]]; then
    echo "❌ Error: Please run this script from the docs/rag directory"
    exit 1
fi

# Check if DocuMCP is installed
if ! command -v documcp &> /dev/null; then
    echo "🔧 Installing DocuMCP..."
    npm install -g @documcp/cli
fi

# Generate fresh documentation first
echo "📚 Generating fresh documentation..."
cd ../..
./scripts/generate_docs.sh

# Go back to the rag directory
cd docs/rag

# Start the indexing process
echo "🚀 Starting documentation indexing..."
documcp index --config config.json

echo "✅ Documentation indexed successfully!"
echo ""
echo "🔍 Semantic search now available at:"
echo "   - API: http://localhost:3000/api/search"
echo "   - Web UI: http://localhost:3000 (if web interface enabled)"
echo ""
echo "📖 Example queries:"
echo "   - 'How do I start a Nockchain node?'"
echo "   - 'Show me wallet operations examples'"
echo "   - 'What are the gRPC API endpoints?'"
echo "   - 'How to configure mining?'"
echo ""
echo "🛠️  To start the search server:"
echo "   documcp serve --config config.json"
