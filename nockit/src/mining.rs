//! Mining operations and monitoring for nockit
//! 
//! Provides mining management, statistics tracking, and performance analysis.

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use tokio::fs;
use tokio::time::{sleep, Duration as TokioDuration};

/// Mining statistics structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningStats {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub blocks_mined: u64,
    pub hash_rate: f64,
    pub difficulty: u64,
    pub rewards_earned: u64,
    pub uptime_seconds: u64,
    pub errors: Vec<MiningError>,
}

/// Mining error tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningError {
    pub timestamp: DateTime<Utc>,
    pub error_type: String,
    pub message: String,
    pub severity: String,
}

/// Mining process information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningProcess {
    pub pid: Option<u32>,
    pub pubkey: String,
    pub start_time: DateTime<Utc>,
    pub status: MiningStatus,
    pub config_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MiningStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Error(String),
}

/// Start mining with monitoring
pub async fn start_mining(pubkey: &str, difficulty: Option<u64>, config_dir: &Path) -> Result<()> {
    println!("Starting mining operation...");
    
    // Check if mining is already running
    if is_mining_running(config_dir).await? {
        println!("⚠️  Mining is already running. Use 'nockit mining stop' to stop first.");
        return Ok(());
    }
    
    // Load configuration
    let mut config = crate::config::NockitConfig::load_or_create(config_dir)?;
    config.set_mining_pubkey(pubkey.to_string());
    config.save(&config_dir.join("config.toml"))?;
    
    // Set environment variables
    crate::config::EnvManager::set_nockchain_env(&config);
    
    // Create mining process info
    let mining_process = MiningProcess {
        pid: None,
        pubkey: pubkey.to_string(),
        start_time: Utc::now(),
        status: MiningStatus::Starting,
        config_dir: config_dir.to_path_buf(),
    };
    
    save_mining_process(&mining_process, config_dir).await?;
    
    // Build nockchain command
    let mut cmd = Command::new("nockchain");
    cmd.arg("--mining-pubkey").arg(pubkey);
    cmd.arg("--mine");
    
    if let Some(diff) = difficulty {
        cmd.arg("--difficulty").arg(diff.to_string());
    }
    
    // Set up logging
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    
    println!("Executing: nockchain --mining-pubkey {} --mine", pubkey);
    
    // Start the mining process
    let child = cmd.spawn()
        .context("Failed to start nockchain mining process. Make sure nockchain is installed and in PATH.")?;
    
    let pid = child.id();
    println!("✅ Mining process started with PID: {}", pid);
    
    // Update process info with PID
    let mut mining_process = mining_process;
    mining_process.pid = Some(pid);
    mining_process.status = MiningStatus::Running;
    save_mining_process(&mining_process, config_dir).await?;
    
    // Initialize mining stats
    let stats = MiningStats {
        start_time: Utc::now(),
        end_time: None,
        blocks_mined: 0,
        hash_rate: 0.0,
        difficulty: difficulty.unwrap_or(0),
        rewards_earned: 0,
        uptime_seconds: 0,
        errors: Vec::new(),
    };
    
    save_mining_stats(&stats, config_dir).await?;
    
    println!("Mining started successfully!");
    println!("Use 'nockit mining status' to check mining status");
    println!("Use 'nockit mining stop' to stop mining");
    
    Ok(())
}

/// Stop mining operations
pub async fn stop_mining(config_dir: &Path) -> Result<()> {
    println!("Stopping mining operation...");
    
    let mining_process = match load_mining_process(config_dir).await {
        Ok(process) => process,
        Err(_) => {
            println!("No active mining process found.");
            return Ok(());
        }
    };
    
    if let Some(pid) = mining_process.pid {
        // Try to terminate the process gracefully
        #[cfg(unix)]
        {
            use std::process;
            let output = Command::new("kill")
                .arg("-TERM")
                .arg(pid.to_string())
                .output();
            
            match output {
                Ok(output) if output.status.success() => {
                    println!("✅ Mining process (PID: {}) terminated gracefully", pid);
                }
                _ => {
                    println!("⚠️  Failed to terminate process gracefully, trying force kill...");
                    let _ = Command::new("kill")
                        .arg("-KILL")
                        .arg(pid.to_string())
                        .output();
                }
            }
        }
        
        #[cfg(windows)]
        {
            let _ = Command::new("taskkill")
                .arg("/PID")
                .arg(pid.to_string())
                .arg("/F")
                .output();
        }
    }
    
    // Update mining process status
    let mut updated_process = mining_process;
    updated_process.status = MiningStatus::Stopped;
    save_mining_process(&updated_process, config_dir).await?;
    
    // Update mining stats
    if let Ok(mut stats) = load_mining_stats(config_dir).await {
        stats.end_time = Some(Utc::now());
        stats.uptime_seconds = (Utc::now() - stats.start_time).num_seconds() as u64;
        save_mining_stats(&stats, config_dir).await?;
    }
    
    println!("Mining stopped successfully!");
    
    Ok(())
}

/// Check mining status and statistics
pub async fn check_status(config_dir: &Path) -> Result<()> {
    println!("=== Mining Status ===");
    
    match load_mining_process(config_dir).await {
        Ok(process) => {
            print_mining_process_status(&process);
            
            // Check if process is actually running
            if let Some(pid) = process.pid {
                let is_running = check_process_running(pid);
                if !is_running && matches!(process.status, MiningStatus::Running) {
                    println!("⚠️  Process appears to have stopped unexpectedly");
                    
                    // Update status
                    let mut updated_process = process;
                    updated_process.status = MiningStatus::Error("Process stopped unexpectedly".to_string());
                    save_mining_process(&updated_process, config_dir).await?;
                }
            }
        }
        Err(_) => {
            println!("No mining process information found.");
            println!("Use 'nockit mining start --pubkey <PUBKEY>' to start mining.");
            return Ok(());
        }
    }
    
    // Show mining statistics
    if let Ok(stats) = load_mining_stats(config_dir).await {
        print_mining_stats(&stats);
    }
    
    Ok(())
}

/// Analyze mining performance statistics
pub async fn analyze_stats(period: &str, config_dir: &Path) -> Result<()> {
    println!("=== Mining Performance Analysis ===");
    
    let duration = parse_time_period(period)?;
    let cutoff_time = Utc::now() - duration;
    
    // Load historical stats
    let stats_files = find_stats_files(config_dir).await?;
    let mut all_stats = Vec::new();
    
    for stats_file in stats_files {
        if let Ok(stats) = load_stats_from_file(&stats_file).await {
            if stats.start_time >= cutoff_time {
                all_stats.push(stats);
            }
        }
    }
    
    if all_stats.is_empty() {
        println!("No mining statistics found for the specified period.");
        return Ok(());
    }
    
    // Calculate aggregated statistics
    let total_blocks = all_stats.iter().map(|s| s.blocks_mined).sum::<u64>();
    let total_rewards = all_stats.iter().map(|s| s.rewards_earned).sum::<u64>();
    let total_uptime = all_stats.iter().map(|s| s.uptime_seconds).sum::<u64>();
    let avg_hash_rate = all_stats.iter().map(|s| s.hash_rate).sum::<f64>() / all_stats.len() as f64;
    
    println!("Period: {} (last {})", cutoff_time.format("%Y-%m-%d %H:%M:%S"), period);
    println!("Total mining sessions: {}", all_stats.len());
    println!("Total blocks mined: {}", total_blocks);
    println!("Total rewards earned: {}", total_rewards);
    println!("Total uptime: {} hours", total_uptime / 3600);
    println!("Average hash rate: {:.2} H/s", avg_hash_rate);
    
    if total_uptime > 0 {
        println!("Blocks per hour: {:.2}", (total_blocks as f64) / (total_uptime as f64 / 3600.0));
        println!("Rewards per hour: {:.2}", (total_rewards as f64) / (total_uptime as f64 / 3600.0));
    }
    
    // Show error summary
    let total_errors: usize = all_stats.iter().map(|s| s.errors.len()).sum();
    if total_errors > 0 {
        println!("\n=== Error Summary ===");
        println!("Total errors: {}", total_errors);
        
        let mut error_types = HashMap::new();
        for stats in &all_stats {
            for error in &stats.errors {
                *error_types.entry(&error.error_type).or_insert(0) += 1;
            }
        }
        
        for (error_type, count) in error_types {
            println!("  {}: {}", error_type, count);
        }
    }
    
    Ok(())
}

// Helper functions

async fn is_mining_running(config_dir: &Path) -> Result<bool> {
    match load_mining_process(config_dir).await {
        Ok(process) => {
            if let Some(pid) = process.pid {
                Ok(check_process_running(pid) && matches!(process.status, MiningStatus::Running))
            } else {
                Ok(false)
            }
        }
        Err(_) => Ok(false),
    }
}

pub fn check_process_running(pid: u32) -> bool {
    #[cfg(unix)]
    {
        use std::process::Command;
        Command::new("kill")
            .arg("-0")
            .arg(pid.to_string())
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
    
    #[cfg(windows)]
    {
        use std::process::Command;
        Command::new("tasklist")
            .arg("/FI")
            .arg(format!("PID eq {}", pid))
            .output()
            .map(|output| {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout.contains(&pid.to_string())
            })
            .unwrap_or(false)
    }
}

async fn save_mining_process(process: &MiningProcess, config_dir: &Path) -> Result<()> {
    let process_file = config_dir.join("mining_process.json");
    let json = serde_json::to_string_pretty(process)?;
    fs::write(process_file, json).await?;
    Ok(())
}

pub async fn load_mining_process(config_dir: &Path) -> Result<MiningProcess> {
    let process_file = config_dir.join("mining_process.json");
    let content = fs::read_to_string(process_file).await?;
    let process: MiningProcess = serde_json::from_str(&content)?;
    Ok(process)
}

async fn save_mining_stats(stats: &MiningStats, config_dir: &Path) -> Result<()> {
    let stats_dir = config_dir.join("mining_stats");
    fs::create_dir_all(&stats_dir).await?;
    
    let timestamp = stats.start_time.format("%Y%m%d_%H%M%S");
    let stats_file = stats_dir.join(format!("stats_{}.json", timestamp));
    
    let json = serde_json::to_string_pretty(stats)?;
    fs::write(stats_file, json).await?;
    
    // Also save as current stats
    let current_stats_file = config_dir.join("current_mining_stats.json");
    fs::write(current_stats_file, serde_json::to_string_pretty(stats)?).await?;
    
    Ok(())
}

async fn load_mining_stats(config_dir: &Path) -> Result<MiningStats> {
    let stats_file = config_dir.join("current_mining_stats.json");
    let content = fs::read_to_string(stats_file).await?;
    let stats: MiningStats = serde_json::from_str(&content)?;
    Ok(stats)
}

async fn find_stats_files(config_dir: &Path) -> Result<Vec<PathBuf>> {
    let stats_dir = config_dir.join("mining_stats");
    if !stats_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut entries = fs::read_dir(stats_dir).await?;
    let mut stats_files = Vec::new();
    
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
            stats_files.push(path);
        }
    }
    
    Ok(stats_files)
}

async fn load_stats_from_file(file: &Path) -> Result<MiningStats> {
    let content = fs::read_to_string(file).await?;
    let stats: MiningStats = serde_json::from_str(&content)?;
    Ok(stats)
}

fn parse_time_period(period: &str) -> Result<Duration> {
    match period {
        "1h" => Ok(Duration::hours(1)),
        "6h" => Ok(Duration::hours(6)),
        "12h" => Ok(Duration::hours(12)),
        "1d" => Ok(Duration::days(1)),
        "1w" => Ok(Duration::weeks(1)),
        "1m" => Ok(Duration::days(30)),
        _ => anyhow::bail!("Invalid time period: {}. Use 1h, 6h, 12h, 1d, 1w, or 1m", period),
    }
}

fn print_mining_process_status(process: &MiningProcess) {
    println!("Public key: {}", process.pubkey);
    println!("Start time: {}", process.start_time);
    println!("Status: {:?}", process.status);
    
    if let Some(pid) = process.pid {
        println!("Process ID: {}", pid);
        let is_running = check_process_running(pid);
        println!("Process running: {}", if is_running { "✅ Yes" } else { "❌ No" });
    }
    
    let uptime = Utc::now() - process.start_time;
    println!("Uptime: {} hours, {} minutes", 
             uptime.num_hours(), 
             uptime.num_minutes() % 60);
}

fn print_mining_stats(stats: &MiningStats) {
    println!("\n=== Mining Statistics ===");
    println!("Blocks mined: {}", stats.blocks_mined);
    println!("Hash rate: {:.2} H/s", stats.hash_rate);
    println!("Difficulty: {}", stats.difficulty);
    println!("Rewards earned: {}", stats.rewards_earned);
    println!("Uptime: {} seconds", stats.uptime_seconds);
    
    if !stats.errors.is_empty() {
        println!("\nRecent errors: {}", stats.errors.len());
        for (i, error) in stats.errors.iter().take(3).enumerate() {
            println!("  {}: {} - {}", i + 1, error.error_type, error.message);
        }
    }
} 