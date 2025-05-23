#!/usr/bin/env python3
"""
Nockchain Peer Configuration Helper

This utility script reads the nockchain peer configuration and provides
various output formats for use with nockchain tools and scripts.

Features:
- Parse TOML peer configuration
- Generate command-line arguments for nockchain tools
- Filter peers by region, reliability, or provider
- Export peer lists in various formats
- Validate peer connectivity

Usage:
    python3 peer_helper.py [--format FORMAT] [--region REGION] [--reliability LEVEL]

Examples:
    python3 peer_helper.py --format args
    python3 peer_helper.py --format json --region EU
    python3 peer_helper.py --format list --reliability high
"""

import argparse
import json
import sys
from pathlib import Path
from typing import Dict, List, Optional, Any

try:
    import tomllib  # Python 3.11+
except ImportError:
    try:
        import tomli as tomllib  # type: ignore # Fallback for older Python versions
    except ImportError:
        print("Error: tomllib/tomli not available. Install with: pip install tomli")
        sys.exit(1)

class PeerConfigHelper:
    """Helper class for managing nockchain peer configurations."""
    
    def __init__(self, config_path: Path):
        self.config_path = config_path
        self.config = self._load_config()
    
    def _load_config(self) -> Dict[str, Any]:
        """Load and parse the TOML configuration file."""
        if not self.config_path.exists():
            raise FileNotFoundError(f"Configuration file not found: {self.config_path}")
        
        try:
            with open(self.config_path, 'rb') as f:
                return tomllib.load(f)
        except Exception as e:
            raise ValueError(f"Failed to parse configuration file: {e}")
    
    def get_all_peers(self) -> List[Dict[str, Any]]:
        """Get all peer configurations."""
        return self.config.get('peers', [])
    
    def filter_peers(self, 
                    region: Optional[str] = None,
                    reliability: Optional[str] = None,
                    provider: Optional[str] = None) -> List[Dict[str, Any]]:
        """Filter peers based on specified criteria."""
        peers = self.get_all_peers()
        
        if region:
            peers = [p for p in peers if p.get('region', '').upper() == region.upper()]
        
        if reliability:
            peers = [p for p in peers if p.get('reliability', '').lower() == reliability.lower()]
        
        if provider:
            peers = [p for p in peers if p.get('provider', '').upper() == provider.upper()]
        
        return peers
    
    def get_regional_peers(self, region: str) -> List[str]:
        """Get peer addresses for a specific region from the regions configuration."""
        regions = self.config.get('regions', {})
        region_key = region.lower()
        
        if region_key in regions:
            return regions[region_key].get('peers', [])
        
        return []
    
    def format_as_args(self, peers: List[Dict[str, Any]]) -> str:
        """Format peer list as command-line arguments."""
        args: List[str] = []
        for peer in peers:
            args.append(f"--peer {peer['address']}")
        return " \\\n".join(args)
    
    def format_as_list(self, peers: List[Dict[str, Any]]) -> str:
        """Format peer list as simple text list."""
        return "\n".join(peer['address'] for peer in peers)
    
    def format_as_json(self, peers: List[Dict[str, Any]]) -> str:
        """Format peer list as JSON."""
        return json.dumps(peers, indent=2)
    
    def format_as_table(self, peers: List[Dict[str, Any]]) -> str:
        """Format peer list as a readable table."""
        if not peers:
            return "No peers found matching criteria."
        
        lines: List[str] = []
        header = f"{'ADDRESS':50} {'REGION':8} {'RELIABILITY':12} {'PROVIDER':10}"
        separator = "-" * 85
        lines.append(header)
        lines.append(separator)
        
        for peer in peers:
            address = peer['address']
            region = peer.get('region', 'Unknown')
            reliability = peer.get('reliability', 'Unknown')
            provider = peer.get('provider', 'N/A')
            
            row = f"{address:50} {region:8} {reliability:12} {provider:10}"
            lines.append(row)
        
        return "\n".join(lines)
    
    def get_recommendations(self) -> Dict[str, Any]:
        """Get peer selection recommendations."""
        return self.config.get('recommendations', {})
    
    def get_network_info(self) -> Dict[str, Any]:
        """Get network configuration information."""
        return self.config.get('network', {})
    
    def validate_config(self) -> List[str]:
        """Validate the configuration and return any issues found."""
        issues = []
        
        # Check required sections
        if 'peers' not in self.config:
            issues.append("Missing 'peers' section")
        
        if 'network' not in self.config:
            issues.append("Missing 'network' section")
        
        # Validate peer entries
        peers = self.get_all_peers()
        for i, peer in enumerate(peers):
            if 'address' not in peer:
                issues.append(f"Peer {i}: missing 'address' field")
            
            # Basic address format validation
            address = peer.get('address', '')
            if not address.startswith('/ip4/'):
                issues.append(f"Peer {i}: invalid address format: {address}")
        
        return issues

def parse_arguments() -> argparse.Namespace:
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description="Nockchain peer configuration helper",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Output Formats:
  args    - Command-line arguments (--peer ADDRESS)
  list    - Simple address list (one per line)
  json    - JSON format with full peer information
  table   - Human-readable table format

Examples:
  python3 peer_helper.py --format args
  python3 peer_helper.py --format table --region EU
  python3 peer_helper.py --format json --reliability high
  python3 peer_helper.py --format list --provider GCP
        """
    )
    
    parser.add_argument(
        "--config",
        type=Path,
        default=Path(__file__).parent / "nockchain_peers.toml",
        help="Path to peer configuration file"
    )
    
    parser.add_argument(
        "--format",
        choices=["args", "list", "json", "table"],
        default="table",
        help="Output format (default: table)"
    )
    
    parser.add_argument(
        "--region",
        help="Filter by region (EU, US, etc.)"
    )
    
    parser.add_argument(
        "--reliability",
        choices=["high", "medium", "low"],
        help="Filter by reliability level"
    )
    
    parser.add_argument(
        "--provider",
        help="Filter by provider (GCP, AWS, etc.)"
    )
    
    parser.add_argument(
        "--regional",
        help="Use regional peer group (eu, us, global)"
    )
    
    parser.add_argument(
        "--validate",
        action="store_true",
        help="Validate configuration and exit"
    )
    
    parser.add_argument(
        "--info",
        action="store_true",
        help="Show network information and recommendations"
    )
    
    return parser.parse_args()

def main() -> None:
    """Main entry point."""
    args = parse_arguments()
    
    try:
        helper = PeerConfigHelper(args.config)
        
        # Validation mode
        if args.validate:
            issues = helper.validate_config()
            if issues:
                print("Configuration issues found:")
                for issue in issues:
                    print(f"  - {issue}")
                sys.exit(1)
            else:
                print("Configuration is valid.")
                sys.exit(0)
        
        # Info mode
        if args.info:
            network_info = helper.get_network_info()
            recommendations = helper.get_recommendations()
            
            print("Network Information:")
            for key, value in network_info.items():
                print(f"  {key}: {value}")
            
            print("\nRecommendations:")
            for key, value in recommendations.items():
                print(f"  {key}: {value}")
            
            sys.exit(0)
        
        # Get peers based on filters
        if args.regional:
            # Use regional peer group
            peer_addresses = helper.get_regional_peers(args.regional)
            peers = [{"address": addr} for addr in peer_addresses]
        else:
            # Use filtered peers
            peers = helper.filter_peers(
                region=args.region,
                reliability=args.reliability,
                provider=args.provider
            )
        
        # Format and output
        if args.format == "args":
            print(helper.format_as_args(peers))
        elif args.format == "list":
            print(helper.format_as_list(peers))
        elif args.format == "json":
            print(helper.format_as_json(peers))
        elif args.format == "table":
            print(helper.format_as_table(peers))
        
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main() 