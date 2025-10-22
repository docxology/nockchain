"""Docxology: Python bindings for Nockchain orchestrator.

This package provides Python bindings for the Nockchain orchestrator,
allowing Python applications to interact with Nockchain nodes, miners,
wallets, and gRPC APIs through a convenient interface.

Example:
    >>> from docxology import NodeConfig, start_node
    >>> config = NodeConfig()
    >>> node = await start_node(config)
    >>> await node.shutdown()
"""

from ._docxology import (
    NodeConfig,
    NodeHandle,
    MinerConfig,
    MinerHandle,
    WalletConfig,
    WalletManager,
    GrpcClient,
    start_node,
    start_miner,
    create_wallet,
    public_client,
)

__version__ = "0.1.0"
__all__ = [
    "NodeConfig",
    "NodeHandle",
    "MinerConfig",
    "MinerHandle",
    "WalletConfig",
    "WalletManager",
    "GrpcClient",
    "start_node",
    "start_miner",
    "create_wallet",
    "public_client",
]
