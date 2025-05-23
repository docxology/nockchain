#!/usr/bin/env python3
"""
Nockchain Mining Statistics Analyzer

This script analyzes nockchain wallet data to extract and rank mining statistics
by parsing coinbase block entries and computing miner rankings.

Features:
- Socket-based communication with nockchain wallet
- Coinbase block detection and parsing
- Miner ranking and statistics
- Configurable timeout and debug logging
- Comprehensive error handling

Usage:
    python3 mining_stats.py [--socket PATH] [--timeout SECONDS] [--debug]

Requirements:
    - nockchain-wallet binary in PATH
    - Active nockchain socket connection
    - Python 3.6+ with standard library
"""

import re
import subprocess
import sys
import argparse
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Tuple, Optional

# ── DEFAULT CONFIGURATION ───────────────────────────────────────────────
DEFAULT_SOCKET = Path.cwd() / ".socket" / "nockchain_npc.sock"
DEFAULT_TIMEOUT = 30
DEFAULT_DEBUG = False

class MiningStatsAnalyzer:
    """Analyzes nockchain mining statistics from wallet data."""
    
    def __init__(self, socket_path: Path, timeout: int = DEFAULT_TIMEOUT, debug: bool = DEFAULT_DEBUG):
        self.socket_path = socket_path
        self.timeout = timeout
        self.debug = debug
        
    def log(self, *args) -> None:
        """Debug logging output."""
        if self.debug:
            print("→ DEBUG:", *args)
    
    def verify_socket(self) -> None:
        """Verify that the nockchain socket exists and is accessible."""
        if not self.socket_path.exists():
            sys.exit(f"Error: socket not found at {self.socket_path}")
        
        if not self.socket_path.is_socket():
            sys.exit(f"Error: {self.socket_path} is not a valid socket")
    
    def fetch_wallet_data(self) -> str:
        """Fetch raw wallet data using nockchain-wallet list-notes command."""
        self.log("Running list-notes command...")
        
        try:
            proc = subprocess.run(
                ["nockchain-wallet", "--nockchain-socket", str(self.socket_path), "list-notes"],
                capture_output=True, 
                text=True, 
                timeout=self.timeout
            )
        except subprocess.TimeoutExpired:
            sys.exit(f"Error: list-notes command timed out after {self.timeout} seconds")
        except FileNotFoundError:
            sys.exit("Error: nockchain-wallet binary not found in PATH")
        
        if proc.returncode != 0:
            error_msg = proc.stderr.strip() if proc.stderr else "Unknown error"
            sys.exit(f"Error: list-notes failed with return code {proc.returncode} ({error_msg})")
        
        raw_data = proc.stdout
        self.log(f"Retrieved {len(raw_data)} bytes of wallet data")
        return raw_data
    
    def extract_coinbase_entries(self, raw_data: str) -> List[str]:
        """Extract coinbase block entries from raw wallet data."""
        # Pattern matches: is-coinbase=%.y followed by pks=<|...signature...|>
        pattern = r"is-coinbase=%\.y[\s\S]*?pks=<\|([\s\S]*?)\|>"
        matches = re.findall(pattern, raw_data)
        
        self.log(f"Found {len(matches)} raw coinbase signature entries")
        return matches
    
    def process_coinbase_signatures(self, signatures: List[str]) -> Dict[str, int]:
        """Process coinbase signatures and count occurrences per wallet."""
        signature_counts = {}
        
        for signature in signatures:
            # Normalize signature by removing whitespace and newlines
            normalized_sig = re.sub(r"\s+", "", signature)
            signature_counts[normalized_sig] = signature_counts.get(normalized_sig, 0) + 1
        
        self.log(f"Processed signatures for {len(signature_counts)} unique wallets")
        return signature_counts
    
    def calculate_mined_blocks(self, signature_counts: Dict[str, int]) -> Dict[str, int]:
        """Calculate complete mined blocks per wallet.
        
        Each complete block generates two signature entries, so we divide by 2
        and only count wallets with at least 2 signatures (complete blocks).
        """
        blocks_mined = {}
        
        for wallet, count in signature_counts.items():
            if count >= 2:  # Only count complete blocks
                blocks_mined[wallet] = count // 2
        
        total_blocks = sum(blocks_mined.values())
        self.log(f"Calculated {total_blocks} total complete coinbase blocks")
        
        return blocks_mined
    
    def format_wallet_address(self, wallet: str, max_length: int = 36) -> str:
        """Format wallet address for display with truncation."""
        if len(wallet) <= max_length:
            return wallet
        
        # Show first 16 and last 16 characters with ellipsis
        prefix_len = min(16, (max_length - 3) // 2)
        suffix_len = min(16, max_length - 3 - prefix_len)
        
        return f"{wallet[:prefix_len]}...{wallet[-suffix_len:]}"
    
    def print_mining_rankings(self, blocks_mined: Dict[str, int]) -> None:
        """Print formatted mining rankings table."""
        if not blocks_mined:
            print("No complete coinbase blocks found.")
            return
        
        total_blocks = sum(blocks_mined.values())
        
        print(f"\nMiner Rankings (out of {total_blocks} full coinbase blocks):")
        print(f"{'#':>4}  {'WALLET':36} {'BLOCKS':>6} {'%':>6}")
        print("-" * 60)
        
        # Sort by blocks mined (descending) and enumerate for ranking
        sorted_miners = sorted(blocks_mined.items(), key=lambda x: x[1], reverse=True)
        
        for rank, (wallet, block_count) in enumerate(sorted_miners, start=1):
            formatted_wallet = self.format_wallet_address(wallet)
            percentage = (block_count / total_blocks) * 100
            
            print(f"{f'#{rank}':>4}  {formatted_wallet:36} {block_count:6d} {percentage:5.1f}%")
        
        print(f"\nSummary:")
        print(f"  Total miners: {len(blocks_mined)}")
        print(f"  Total blocks: {total_blocks}")
        print(f"  Average blocks per miner: {total_blocks / len(blocks_mined):.1f}")
        print(f"  Last updated: {datetime.now():%Y-%m-%d %H:%M:%S}")
    
    def analyze_mining_stats(self) -> Dict[str, int]:
        """Main analysis workflow."""
        # Step 1: Verify socket connection
        self.verify_socket()
        
        # Step 2: Fetch wallet data
        raw_data = self.fetch_wallet_data()
        
        # Step 3: Extract coinbase entries
        coinbase_signatures = self.extract_coinbase_entries(raw_data)
        
        if not coinbase_signatures:
            print("No coinbase blocks found in wallet data.")
            return {}
        
        # Step 4: Process signatures and count per wallet
        signature_counts = self.process_coinbase_signatures(coinbase_signatures)
        
        # Step 5: Calculate complete blocks mined
        blocks_mined = self.calculate_mined_blocks(signature_counts)
        
        return blocks_mined

def parse_arguments() -> argparse.Namespace:
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description="Analyze nockchain mining statistics from wallet data",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python3 mining_stats.py
  python3 mining_stats.py --socket /custom/path/to/socket
  python3 mining_stats.py --timeout 60 --debug
  python3 mining_stats.py --help
        """
    )
    
    parser.add_argument(
        "--socket", 
        type=Path,
        default=DEFAULT_SOCKET,
        help=f"Path to nockchain socket (default: {DEFAULT_SOCKET})"
    )
    
    parser.add_argument(
        "--timeout",
        type=int,
        default=DEFAULT_TIMEOUT,
        help=f"Timeout for wallet commands in seconds (default: {DEFAULT_TIMEOUT})"
    )
    
    parser.add_argument(
        "--debug",
        action="store_true",
        default=DEFAULT_DEBUG,
        help="Enable debug logging output"
    )
    
    parser.add_argument(
        "--version",
        action="version",
        version="Nockchain Mining Stats Analyzer v1.0"
    )
    
    return parser.parse_args()

def main() -> None:
    """Main entry point."""
    args = parse_arguments()
    
    print("⛏️  Nockchain Mining Statistics Analyzer")
    print("=" * 45)
    
    # Initialize analyzer with configuration
    analyzer = MiningStatsAnalyzer(
        socket_path=args.socket,
        timeout=args.timeout,
        debug=args.debug
    )
    
    try:
        # Run analysis
        blocks_mined = analyzer.analyze_mining_stats()
        
        # Display results
        analyzer.print_mining_rankings(blocks_mined)
        
    except KeyboardInterrupt:
        print("\n\nAnalysis interrupted by user.")
        sys.exit(1)
    except Exception as e:
        print(f"\nUnexpected error: {e}")
        if args.debug:
            import traceback
            traceback.print_exc()
        sys.exit(1)

if __name__ == "__main__":
    main() 