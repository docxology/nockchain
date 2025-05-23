//! System monitoring and health checks for nockit
//! 
//! Provides real-time monitoring of nockchain operations, system health, and performance metrics.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::time::{sleep, Duration};

/// System health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub timestamp: DateTime<Utc>,
    pub overall_status: HealthStatus,
    pub nockchain_status: ServiceStatus,
    pub mining_status: ServiceStatus,
    pub network_status: ServiceStatus,
    pub wallet_status: ServiceStatus,
    pub system_metrics: SystemMetrics,
}

/// Health status levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

/// Service status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub status: HealthStatus,
    pub message: String,
    pub last_check: DateTime<Utc>,
    pub uptime_seconds: Option<u64>,
}

/// System performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub disk_usage_percent: f64,
    pub network_rx_bytes: u64,
    pub network_tx_bytes: u64,
    pub process_count: u32,
}

/// Run the monitoring dashboard
pub async fn run_monitor(interval: u64, format: &str, config_dir: &Path) -> Result<()> {
    println!("🔍 Starting nockchain monitoring (interval: {}s, format: {})", interval, format);
    println!("Press Ctrl+C to stop monitoring\n");
    
    loop {
        let health = collect_system_health(config_dir).await?;
        
        match format {
            "json" => print_json_status(&health)?,
            "compact" => print_compact_status(&health),
            _ => print_table_status(&health),
        }
        
        // Save health data
        save_health_data(&health, config_dir).await?;
        
        sleep(Duration::from_secs(interval)).await;
        
        // Clear screen for table format
        if format == "table" {
            print!("\x1B[2J\x1B[1;1H"); // Clear screen and move cursor to top
        }
    }
}

pub async fn collect_system_health(config_dir: &Path) -> Result<SystemHealth> {
    let timestamp = Utc::now();
    
    // Check nockchain service
    let nockchain_status = check_nockchain_service().await;
    
    // Check mining service
    let mining_status = check_mining_service(config_dir).await;
    
    // Check network connectivity
    let network_status = check_network_service(config_dir).await;
    
    // Check wallet service
    let wallet_status = check_wallet_service(config_dir).await;
    
    // Collect system metrics
    let system_metrics = collect_system_metrics().await;
    
    // Determine overall status
    let overall_status = determine_overall_status(&[
        &nockchain_status,
        &mining_status,
        &network_status,
        &wallet_status,
    ]);
    
    Ok(SystemHealth {
        timestamp,
        overall_status,
        nockchain_status,
        mining_status,
        network_status,
        wallet_status,
        system_metrics,
    })
}

async fn check_nockchain_service() -> ServiceStatus {
    // Check if nockchain process is running
    match std::process::Command::new("pgrep").arg("nockchain").output() {
        Ok(output) if output.status.success() && !output.stdout.is_empty() => {
            ServiceStatus {
                status: HealthStatus::Healthy,
                message: "Nockchain process running".to_string(),
                last_check: Utc::now(),
                uptime_seconds: None, // Could calculate from process start time
            }
        }
        _ => {
            ServiceStatus {
                status: HealthStatus::Critical,
                message: "Nockchain process not found".to_string(),
                last_check: Utc::now(),
                uptime_seconds: None,
            }
        }
    }
}

async fn check_mining_service(config_dir: &Path) -> ServiceStatus {
    match crate::mining::load_mining_process(config_dir).await {
        Ok(process) => {
            if let Some(pid) = process.pid {
                let is_running = crate::mining::check_process_running(pid);
                if is_running {
                    let uptime = (Utc::now() - process.start_time).num_seconds() as u64;
                    ServiceStatus {
                        status: HealthStatus::Healthy,
                        message: format!("Mining active (PID: {})", pid),
                        last_check: Utc::now(),
                        uptime_seconds: Some(uptime),
                    }
                } else {
                    ServiceStatus {
                        status: HealthStatus::Critical,
                        message: "Mining process stopped unexpectedly".to_string(),
                        last_check: Utc::now(),
                        uptime_seconds: None,
                    }
                }
            } else {
                ServiceStatus {
                    status: HealthStatus::Warning,
                    message: "Mining configured but not started".to_string(),
                    last_check: Utc::now(),
                    uptime_seconds: None,
                }
            }
        }
        Err(_) => {
            ServiceStatus {
                status: HealthStatus::Warning,
                message: "Mining not configured".to_string(),
                last_check: Utc::now(),
                uptime_seconds: None,
            }
        }
    }
}

async fn check_network_service(config_dir: &Path) -> ServiceStatus {
    match crate::network::get_node_network_status().await {
        Ok(status) => {
            let message = format!("Connected to {} peers", status.connected_peers);
            let health = if status.connected_peers > 0 {
                HealthStatus::Healthy
            } else {
                HealthStatus::Warning
            };
            
            ServiceStatus {
                status: health,
                message,
                last_check: Utc::now(),
                uptime_seconds: None,
            }
        }
        Err(_) => {
            ServiceStatus {
                status: HealthStatus::Critical,
                message: "Cannot connect to nockchain node".to_string(),
                last_check: Utc::now(),
                uptime_seconds: None,
            }
        }
    }
}

async fn check_wallet_service(config_dir: &Path) -> ServiceStatus {
    let config = match crate::config::NockitConfig::load_or_create(config_dir) {
        Ok(config) => config,
        Err(_) => {
            return ServiceStatus {
                status: HealthStatus::Warning,
                message: "Configuration not found".to_string(),
                last_check: Utc::now(),
                uptime_seconds: None,
            };
        }
    };
    
    if config.get_mining_pubkey().is_some() {
        ServiceStatus {
            status: HealthStatus::Healthy,
            message: "Wallet configured".to_string(),
            last_check: Utc::now(),
            uptime_seconds: None,
        }
    } else {
        ServiceStatus {
            status: HealthStatus::Warning,
            message: "No mining public key configured".to_string(),
            last_check: Utc::now(),
            uptime_seconds: None,
        }
    }
}

async fn collect_system_metrics() -> SystemMetrics {
    // This is a simplified implementation
    // In a real implementation, you'd use system monitoring libraries
    
    SystemMetrics {
        cpu_usage_percent: get_cpu_usage().await,
        memory_usage_percent: get_memory_usage().await,
        disk_usage_percent: get_disk_usage().await,
        network_rx_bytes: 0, // Would collect from /proc/net/dev or similar
        network_tx_bytes: 0,
        process_count: get_process_count().await,
    }
}

async fn get_cpu_usage() -> f64 {
    // Simplified CPU usage calculation
    // In practice, you'd read from /proc/stat or use a system monitoring library
    0.0
}

async fn get_memory_usage() -> f64 {
    // Simplified memory usage calculation
    // In practice, you'd read from /proc/meminfo or use a system monitoring library
    0.0
}

async fn get_disk_usage() -> f64 {
    // Simplified disk usage calculation
    // In practice, you'd use statvfs or similar system calls
    0.0
}

async fn get_process_count() -> u32 {
    // Count processes
    match std::process::Command::new("ps").arg("aux").output() {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.lines().count().saturating_sub(1) as u32 // Subtract header line
        }
        _ => 0,
    }
}

fn determine_overall_status(services: &[&ServiceStatus]) -> HealthStatus {
    let mut has_critical = false;
    let mut has_warning = false;
    
    for service in services {
        match service.status {
            HealthStatus::Critical => has_critical = true,
            HealthStatus::Warning => has_warning = true,
            HealthStatus::Healthy => {}
            HealthStatus::Unknown => has_warning = true,
        }
    }
    
    if has_critical {
        HealthStatus::Critical
    } else if has_warning {
        HealthStatus::Warning
    } else {
        HealthStatus::Healthy
    }
}

fn print_table_status(health: &SystemHealth) {
    println!("🔍 Nockchain System Monitor - {}", health.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("Overall Status: {}", format_health_status(&health.overall_status));
    println!();
    
    // Services table
    println!("┌─────────────────┬──────────────┬─────────────────────────────────────┐");
    println!("│ Service         │ Status       │ Details                             │");
    println!("├─────────────────┼──────────────┼─────────────────────────────────────┤");
    
    let services = [
        ("Nockchain", &health.nockchain_status),
        ("Mining", &health.mining_status),
        ("Network", &health.network_status),
        ("Wallet", &health.wallet_status),
    ];
    
    for (name, status) in &services {
        let status_str = format_health_status(&status.status);
        let details = if status.message.len() > 35 {
            format!("{}...", &status.message[..32])
        } else {
            status.message.clone()
        };
        
        println!("│ {:<15} │ {:<12} │ {:<35} │", name, status_str, details);
    }
    
    println!("└─────────────────┴──────────────┴─────────────────────────────────────┘");
    
    // System metrics
    println!();
    println!("System Metrics:");
    println!("  CPU Usage: {:.1}%", health.system_metrics.cpu_usage_percent);
    println!("  Memory Usage: {:.1}%", health.system_metrics.memory_usage_percent);
    println!("  Disk Usage: {:.1}%", health.system_metrics.disk_usage_percent);
    println!("  Process Count: {}", health.system_metrics.process_count);
    println!();
}

fn print_compact_status(health: &SystemHealth) {
    let status_char = match health.overall_status {
        HealthStatus::Healthy => "✅",
        HealthStatus::Warning => "⚠️",
        HealthStatus::Critical => "❌",
        HealthStatus::Unknown => "❓",
    };
    
    println!("{} {} | NC:{} MIN:{} NET:{} WAL:{} | CPU:{:.1}% MEM:{:.1}% PROC:{}",
        health.timestamp.format("%H:%M:%S"),
        status_char,
        format_compact_status(&health.nockchain_status.status),
        format_compact_status(&health.mining_status.status),
        format_compact_status(&health.network_status.status),
        format_compact_status(&health.wallet_status.status),
        health.system_metrics.cpu_usage_percent,
        health.system_metrics.memory_usage_percent,
        health.system_metrics.process_count,
    );
}

fn print_json_status(health: &SystemHealth) -> Result<()> {
    let json = serde_json::to_string_pretty(health)?;
    println!("{}", json);
    Ok(())
}

fn format_health_status(status: &HealthStatus) -> String {
    match status {
        HealthStatus::Healthy => "✅ Healthy".to_string(),
        HealthStatus::Warning => "⚠️  Warning".to_string(),
        HealthStatus::Critical => "❌ Critical".to_string(),
        HealthStatus::Unknown => "❓ Unknown".to_string(),
    }
}

fn format_compact_status(status: &HealthStatus) -> &'static str {
    match status {
        HealthStatus::Healthy => "✅",
        HealthStatus::Warning => "⚠️",
        HealthStatus::Critical => "❌",
        HealthStatus::Unknown => "❓",
    }
}

async fn save_health_data(health: &SystemHealth, config_dir: &Path) -> Result<()> {
    let health_dir = config_dir.join("health_data");
    tokio::fs::create_dir_all(&health_dir).await?;
    
    let timestamp = health.timestamp.format("%Y%m%d_%H%M%S");
    let health_file = health_dir.join(format!("health_{}.json", timestamp));
    
    let json = serde_json::to_string_pretty(health)?;
    tokio::fs::write(health_file, json).await?;
    
    // Also save as current health
    let current_health_file = config_dir.join("current_health.json");
    tokio::fs::write(current_health_file, serde_json::to_string_pretty(health)?).await?;
    
    Ok(())
} 