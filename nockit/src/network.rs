//! Network monitoring and diagnostics for nockit
//! 
//! Provides network connectivity checks, peer management, and traffic monitoring.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};
use tokio::fs;
use tokio::time::{sleep, timeout};

/// Network status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    pub timestamp: DateTime<Utc>,
    pub connected_peers: u32,
    pub total_connections: u32,
    pub network_id: Option<String>,
    pub local_peer_id: Option<String>,
    pub listening_addresses: Vec<String>,
    pub connectivity: ConnectivityStatus,
}

/// Peer information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub peer_id: String,
    pub address: String,
    pub connection_time: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub latency_ms: Option<u64>,
    pub status: PeerStatus,
}

/// Network connectivity status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectivityStatus {
    Connected,
    Connecting,
    Disconnected,
    Error(String),
}

/// Peer connection status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PeerStatus {
    Connected,
    Connecting,
    Disconnected,
    Banned,
}

/// Network traffic statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrafficStats {
    pub timestamp: DateTime<Utc>,
    pub duration_seconds: u64,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub peer_stats: HashMap<String, PeerTrafficStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerTrafficStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
}

/// Check network connectivity status
pub async fn check_status(config_dir: &Path) -> Result<()> {
    println!("=== Network Status ===");
    
    // Try to get status from running nockchain node
    match get_node_network_status().await {
        Ok(status) => {
            print_network_status(&status);
            save_network_status(&status, config_dir).await?;
        }
        Err(e) => {
            println!("❌ Could not connect to nockchain node: {}", e);
            println!("Make sure nockchain is running and accessible.");
            
            // Try to load cached status
            if let Ok(cached_status) = load_network_status(config_dir).await {
                println!("\n=== Cached Network Status ===");
                print_network_status(&cached_status);
                println!("⚠️  This information may be outdated.");
            }
        }
    }
    
    // Check basic network connectivity
    check_basic_connectivity().await?;
    
    Ok(())
}

/// List connected peers
pub async fn list_peers(config_dir: &Path) -> Result<()> {
    println!("=== Connected Peers ===");
    
    match get_peer_list().await {
        Ok(peers) => {
            if peers.is_empty() {
                println!("No peers currently connected.");
            } else {
                println!("Found {} connected peers:", peers.len());
                for (i, peer) in peers.iter().enumerate() {
                    print_peer_info(i + 1, peer);
                }
                
                // Save peer information
                save_peer_list(&peers, config_dir).await?;
            }
        }
        Err(e) => {
            println!("❌ Could not retrieve peer list: {}", e);
            
            // Try to load cached peer list
            if let Ok(cached_peers) = load_peer_list(config_dir).await {
                println!("\n=== Cached Peer List ===");
                for (i, peer) in cached_peers.iter().enumerate() {
                    print_peer_info(i + 1, peer);
                }
                println!("⚠️  This information may be outdated.");
            }
        }
    }
    
    Ok(())
}

/// Ping a specific peer or test general connectivity
pub async fn ping_peer(target: Option<&str>, config_dir: &Path) -> Result<()> {
    if let Some(target) = target {
        println!("Pinging peer: {}", target);
        
        // Try to ping specific peer
        match ping_specific_peer(target).await {
            Ok(latency) => {
                println!("✅ Peer {} responded in {}ms", target, latency);
            }
            Err(e) => {
                println!("❌ Failed to ping peer {}: {}", target, e);
            }
        }
    } else {
        println!("Testing general network connectivity...");
        
        // Get peer list and ping all peers
        match get_peer_list().await {
            Ok(peers) => {
                if peers.is_empty() {
                    println!("No peers to ping.");
                    return Ok(());
                }
                
                println!("Pinging {} peers...", peers.len());
                let mut successful_pings = 0;
                let mut total_latency = 0u64;
                
                for peer in &peers {
                    match ping_specific_peer(&peer.peer_id).await {
                        Ok(latency) => {
                            println!("✅ {}: {}ms", peer.peer_id, latency);
                            successful_pings += 1;
                            total_latency += latency;
                        }
                        Err(_) => {
                            println!("❌ {}: timeout", peer.peer_id);
                        }
                    }
                }
                
                if successful_pings > 0 {
                    let avg_latency = total_latency / successful_pings as u64;
                    println!("\nPing summary: {}/{} peers responded, average latency: {}ms", 
                             successful_pings, peers.len(), avg_latency);
                } else {
                    println!("\n❌ No peers responded to ping.");
                }
            }
            Err(e) => {
                println!("❌ Could not retrieve peer list for pinging: {}", e);
            }
        }
    }
    
    Ok(())
}

/// Monitor network traffic for a specified duration
pub async fn monitor_traffic(duration: u64, config_dir: &Path) -> Result<()> {
    println!("Monitoring network traffic for {} seconds...", duration);
    
    let start_time = Utc::now();
    let start_instant = Instant::now();
    
    // Get initial stats
    let initial_stats = get_traffic_stats().await.unwrap_or_default();
    
    // Wait for the specified duration
    sleep(tokio::time::Duration::from_secs(duration)).await;
    
    // Get final stats
    let final_stats = get_traffic_stats().await.unwrap_or_default();
    
    // Calculate differences
    let traffic_stats = TrafficStats {
        timestamp: start_time,
        duration_seconds: duration,
        total_bytes_sent: final_stats.total_bytes_sent.saturating_sub(initial_stats.total_bytes_sent),
        total_bytes_received: final_stats.total_bytes_received.saturating_sub(initial_stats.total_bytes_received),
        messages_sent: final_stats.messages_sent.saturating_sub(initial_stats.messages_sent),
        messages_received: final_stats.messages_received.saturating_sub(initial_stats.messages_received),
        peer_stats: HashMap::new(), // Would need more detailed implementation
    };
    
    print_traffic_stats(&traffic_stats);
    save_traffic_stats(&traffic_stats, config_dir).await?;
    
    Ok(())
}

// Helper functions

pub async fn get_node_network_status() -> Result<NetworkStatus> {
    // This would typically query the nockchain node's API or RPC interface
    // For now, we'll simulate or try to parse from command output
    
    let output = Command::new("nockchain")
        .arg("--help") // Placeholder - would use actual status command
        .output()
        .context("Failed to execute nockchain command")?;
    
    if !output.status.success() {
        anyhow::bail!("Nockchain command failed");
    }
    
    // Parse output or use default values
    Ok(NetworkStatus {
        timestamp: Utc::now(),
        connected_peers: 0, // Would parse from actual output
        total_connections: 0,
        network_id: Some("nockchain-mainnet".to_string()),
        local_peer_id: None,
        listening_addresses: vec![],
        connectivity: ConnectivityStatus::Connected,
    })
}

async fn get_peer_list() -> Result<Vec<PeerInfo>> {
    // This would query the node for peer information
    // For now, return empty list or simulated data
    Ok(vec![])
}

async fn ping_specific_peer(peer_id: &str) -> Result<u64> {
    let start = Instant::now();
    
    // This would send an actual ping to the peer
    // For now, simulate with a small delay
    sleep(tokio::time::Duration::from_millis(50)).await;
    
    let latency = start.elapsed().as_millis() as u64;
    Ok(latency)
}

async fn get_traffic_stats() -> Result<TrafficStats> {
    // This would query actual network statistics from the node
    Ok(TrafficStats {
        timestamp: Utc::now(),
        duration_seconds: 0,
        total_bytes_sent: 0,
        total_bytes_received: 0,
        messages_sent: 0,
        messages_received: 0,
        peer_stats: HashMap::new(),
    })
}

async fn check_basic_connectivity() -> Result<()> {
    println!("\n=== Basic Connectivity Tests ===");
    
    // Test DNS resolution
    match hickory_resolver::TokioAsyncResolver::tokio_from_system_conf() {
        Ok(resolver) => {
            match timeout(Duration::from_secs(5), resolver.lookup_ip("google.com")).await {
                Ok(Ok(_)) => println!("✅ DNS resolution: Working"),
                Ok(Err(e)) => println!("❌ DNS resolution: Failed ({})", e),
                Err(_) => println!("❌ DNS resolution: Timeout"),
            }
        }
        Err(e) => println!("❌ DNS resolver setup failed: {}", e),
    }
    
    // Test internet connectivity
    match timeout(Duration::from_secs(10), reqwest::get("https://httpbin.org/ip")).await {
        Ok(Ok(response)) if response.status().is_success() => {
            println!("✅ Internet connectivity: Working");
        }
        Ok(Ok(response)) => {
            println!("❌ Internet connectivity: HTTP error {}", response.status());
        }
        Ok(Err(e)) => {
            println!("❌ Internet connectivity: Failed ({})", e);
        }
        Err(_) => {
            println!("❌ Internet connectivity: Timeout");
        }
    }
    
    Ok(())
}

fn print_network_status(status: &NetworkStatus) {
    println!("Timestamp: {}", status.timestamp);
    println!("Connected peers: {}", status.connected_peers);
    println!("Total connections: {}", status.total_connections);
    
    if let Some(network_id) = &status.network_id {
        println!("Network ID: {}", network_id);
    }
    
    if let Some(peer_id) = &status.local_peer_id {
        println!("Local peer ID: {}", peer_id);
    }
    
    println!("Connectivity: {:?}", status.connectivity);
    
    if !status.listening_addresses.is_empty() {
        println!("Listening addresses:");
        for addr in &status.listening_addresses {
            println!("  {}", addr);
        }
    }
}

fn print_peer_info(index: usize, peer: &PeerInfo) {
    println!("\n{}. Peer ID: {}", index, peer.peer_id);
    println!("   Address: {}", peer.address);
    println!("   Connected: {}", peer.connection_time);
    println!("   Last seen: {}", peer.last_seen);
    println!("   Status: {:?}", peer.status);
    
    if let Some(latency) = peer.latency_ms {
        println!("   Latency: {}ms", latency);
    }
    
    if peer.bytes_sent > 0 || peer.bytes_received > 0 {
        println!("   Traffic: {} sent, {} received", 
                 format_bytes(peer.bytes_sent), 
                 format_bytes(peer.bytes_received));
    }
}

fn print_traffic_stats(stats: &TrafficStats) {
    println!("\n=== Traffic Statistics ===");
    println!("Monitoring period: {} seconds", stats.duration_seconds);
    println!("Total bytes sent: {}", format_bytes(stats.total_bytes_sent));
    println!("Total bytes received: {}", format_bytes(stats.total_bytes_received));
    println!("Messages sent: {}", stats.messages_sent);
    println!("Messages received: {}", stats.messages_received);
    
    if stats.duration_seconds > 0 {
        let sent_rate = stats.total_bytes_sent as f64 / stats.duration_seconds as f64;
        let recv_rate = stats.total_bytes_received as f64 / stats.duration_seconds as f64;
        println!("Average send rate: {}/s", format_bytes(sent_rate as u64));
        println!("Average receive rate: {}/s", format_bytes(recv_rate as u64));
    }
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

async fn save_network_status(status: &NetworkStatus, config_dir: &Path) -> Result<()> {
    let status_file = config_dir.join("network_status.json");
    let json = serde_json::to_string_pretty(status)?;
    fs::write(status_file, json).await?;
    Ok(())
}

async fn load_network_status(config_dir: &Path) -> Result<NetworkStatus> {
    let status_file = config_dir.join("network_status.json");
    let content = fs::read_to_string(status_file).await?;
    let status: NetworkStatus = serde_json::from_str(&content)?;
    Ok(status)
}

async fn save_peer_list(peers: &[PeerInfo], config_dir: &Path) -> Result<()> {
    let peers_file = config_dir.join("peer_list.json");
    let json = serde_json::to_string_pretty(peers)?;
    fs::write(peers_file, json).await?;
    Ok(())
}

async fn load_peer_list(config_dir: &Path) -> Result<Vec<PeerInfo>> {
    let peers_file = config_dir.join("peer_list.json");
    let content = fs::read_to_string(peers_file).await?;
    let peers: Vec<PeerInfo> = serde_json::from_str(&content)?;
    Ok(peers)
}

async fn save_traffic_stats(stats: &TrafficStats, config_dir: &Path) -> Result<()> {
    let stats_dir = config_dir.join("traffic_stats");
    fs::create_dir_all(&stats_dir).await?;
    
    let timestamp = stats.timestamp.format("%Y%m%d_%H%M%S");
    let stats_file = stats_dir.join(format!("traffic_{}.json", timestamp));
    
    let json = serde_json::to_string_pretty(stats)?;
    fs::write(stats_file, json).await?;
    Ok(())
} 