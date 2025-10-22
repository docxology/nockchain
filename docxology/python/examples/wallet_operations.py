#!/usr/bin/env python3
"""Example: Wallet operations with Python bindings.

This example demonstrates how to perform wallet operations using the Python bindings
for the docxology orchestrator.
"""

import asyncio
import sys
import os

# Add the parent directory to the path so we can import docxology
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '..'))

try:
    from docxology import WalletConfig, create_wallet, WalletManager
except ImportError as e:
    print(f"Could not import docxology: {e}")
    print("Make sure the Rust extension is built with 'maturin develop'")
    sys.exit(1)


async def main():
    """Demonstrate wallet operations."""
    print("Starting wallet operations example...")

    # Create a wallet configuration
    config = WalletConfig()
    config.data_dir = "./wallet_data"

    try:
        # Create a wallet
        wallet = await create_wallet(config)
        print("Wallet created successfully!")

        # Generate a new key pair
        print("Generating new key pair...")
        try:
            keypair = await wallet.keygen()
            print("Generated new key pair:"            print(f"  Public key: {keypair.public_key}")
            print(f"  Private key: {keypair.private_key}")
            if keypair.seed_phrase:
                print(f"  Seed phrase: {keypair.seed_phrase}")
        except Exception as e:
            print(f"Key generation failed (expected): {e}")

        # Check balance (will likely fail without a running node)
        print("\nChecking balance...")
        try:
            balance = await wallet.get_balance("test_address")
            print(f"Balance: {balance.balance} units")
        except Exception as e:
            print(f"Could not check balance (expected): {e}")

        # Build a sample transaction
        print("\nBuilding sample transaction...")
        try:
            tx = await wallet.build_transaction("recipient_address", 1000)
            print("Transaction built:"            print(f"  ID: {tx.id}")
            print(f"  Outputs: {tx.outputs[0].amount} units to {tx.outputs[0].recipient}")
            print(f"  Fee: {tx.fee} units")

            # Sign the transaction
            signed_tx = await wallet.sign_transaction(tx)
            print("Transaction signed successfully!")
            print(f"  Signature: {signed_tx.signature}")
        except Exception as e:
            print(f"Could not build/sign transaction (expected): {e}")

        # Demonstrate importing keys
        print("\nImporting watch-only key...")
        try:
            await wallet.import_keys("watch_only", "test_pubkey_base58")
            print("Successfully imported watch-only key")
        except Exception as e:
            print(f"Could not import key (expected): {e}")

    except Exception as e:
        print(f"Wallet operations failed (expected): {e}")
        print("This is expected in the current development state.")
        print("Wallet functionality would need full implementation of the underlying components.")


if __name__ == "__main__":
    asyncio.run(main())
