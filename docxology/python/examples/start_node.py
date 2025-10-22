#!/usr/bin/env python3
"""Example: Starting a Nockchain node with Python bindings.

This example demonstrates how to start a Nockchain node using the Python bindings
for the docxology orchestrator.
"""

import asyncio
import sys
import os

# Add the parent directory to the path so we can import docxology
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..'))

try:
    from docxology import NodeConfig, start_node
except ImportError as e:
    print(f"Could not import docxology: {e}")
    print("Make sure the Rust extension is built with 'maturin develop'")
    sys.exit(1)


async def main():
    """Start a Nockchain node with custom configuration."""
    print("Starting Nockchain node example...")

    # Create a custom configuration
    config = NodeConfig()

    # Set up data directory
    config.data_dir = "./node_data"

    # Configure network settings
    config.network.listen_addr = "/ip4/0.0.0.0/udp/0/quic-v1"

    # Configure logging
    config.logging.level = "info"
    config.logging.file_logging = False

    # Enable public API
    config.api.enable_public_api = True
    config.api.public_api_addr = "127.0.0.1:8080"

    print(f"Configuration: {config}")

    try:
        # Start the node
        node_handle = await start_node(config)

        print("Node started successfully!")
        print(f"Public API available at: http://{config.api.public_api_addr}")
        print("Press Ctrl+C to stop the node...")

        # Wait for shutdown signal
        try:
            await asyncio.Future()  # Run forever until interrupted
        except KeyboardInterrupt:
            print("\nShutting down node...")
            await node_handle.shutdown()
            print("Node stopped successfully!")

    except Exception as e:
        print(f"Failed to start node: {e}")
        print("This is expected in the current development state.")
        print("The node functionality would need full implementation of the underlying Nockchain components.")


if __name__ == "__main__":
    asyncio.run(main())
