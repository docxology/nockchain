#!/usr/bin/env python3
"""Example: High-level convenience flows with Python bindings.

This example demonstrates the high-level convenience functions that combine
multiple operations into simple workflows using Python.
"""

import asyncio
import sys
import os

# Add the parent directory to the path so we can import docxology
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..'))

try:
    from docxology import (
        NodeConfig, MinerConfig, WalletConfig,
        setup_and_start_miner, get_balance, GrpcClient
    )
except ImportError as e:
    print(f"Could not import docxology: {e}")
    print("Make sure the Rust extension is built with 'maturin develop'")
    sys.exit(1)


async def main():
    """Demonstrate high-level flows."""
    print("Starting high-level flows example...")

    # Example 1: Set up and start a miner (complete workflow)
    print("\n=== Example 1: Setup and Start Miner ===")

    node_config = NodeConfig()
    node_config.data_dir = "./miner_node_data"

    miner_config = MinerConfig()
    miner_config.enabled = True
    miner_config.threads = 2

    try:
        node_handle, miner_handle, wallet = await setup_and_start_miner(node_config, miner_config)
        print("Miner setup completed successfully!")
        print(f"Node is running at: {node_handle.config().data_dir}")
        print(f"Miner is using {miner_handle.config().threads} threads")
        print(f"Wallet public key: {await wallet.get_public_key()}")

        # Let it run for a bit
        print("Mining for 10 seconds...")
        await asyncio.sleep(10)

        # Clean shutdown
        await miner_handle.shutdown()
        await node_handle.shutdown()
        print("Miner stopped successfully!")

    except Exception as e:
        print(f"Miner setup failed (expected): {e}")
        print("This is expected in the current development state.")

    # Example 2: Get balance using public API
    print("\n=== Example 2: Get Balance ===")

    test_address = "test_address_placeholder"

    try:
        balance = await get_balance(test_address, None)
        print(f"Balance for {test_address}: {balance} units")
    except Exception as e:
        print(f"Could not get balance (expected): {e}")
        print("This would work with a live Nockchain network.")

    # Example 3: Create and send transaction (would need wallet and node)
    print("\n=== Example 3: Create and Send Transaction ===")
    print("This example would demonstrate:")
    print("1. Creating a wallet")
    print("2. Building a transaction")
    print("3. Signing the transaction")
    print("4. Sending it via gRPC")
    print("5. Waiting for acceptance")
    print()
    print("The high-level flows would provide convenient functions for these operations.")
    print("In a real implementation, this would work end-to-end.")

    print("\nHigh-level flows example completed!")


if __name__ == "__main__":
    asyncio.run(main())
