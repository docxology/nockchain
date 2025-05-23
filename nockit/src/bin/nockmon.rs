//! Standalone monitoring tool for nockchain
//! 
//! A focused tool for real-time system monitoring and health checks.

use clap::Parser;
use nockit::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "nockmon")]
#[command(about = "Nockchain System Monitor")]
#[command(version = nockit::VERSION)]
struct Cli {
    /// Update interval in seconds
    #[arg(short, long, default_value = "5")]
    interval: u64,
    
    /// Output format (json, table, compact)
    #[arg(short, long, default_value = "table")]
    format: String,
    
    /// Configuration directory
    #[arg(long, default_value = ".nockit")]
    config_dir: PathBuf,
    
    /// Run once and exit (don't loop)
    #[arg(long)]
    once: bool,
    
    /// Save output to file
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging (minimal for monitoring)
    init_logging("warn", &cli.config_dir.join(NOCKIT_LOG_DIR))?;
    
    if cli.once {
        // Run monitoring once and exit
        run_single_check(&cli).await?;
    } else {
        // Run continuous monitoring
        monitoring::run_monitor(cli.interval, &cli.format, &cli.config_dir).await?;
    }
    
    Ok(())
}

async fn run_single_check(cli: &Cli) -> Result<()> {
    let health = monitoring::collect_system_health(&cli.config_dir).await?;
    
    let output = match cli.format.as_str() {
        "json" => serde_json::to_string_pretty(&health)?,
        "compact" => format_compact_health(&health),
        _ => format_table_health(&health),
    };
    
    if let Some(output_file) = &cli.output {
        tokio::fs::write(output_file, &output).await?;
        println!("Health check saved to: {}", output_file.display());
    } else {
        println!("{}", output);
    }
    
    // Exit with appropriate code based on health status
    let exit_code = match health.overall_status {
        monitoring::HealthStatus::Healthy => 0,
        monitoring::HealthStatus::Warning => 1,
        monitoring::HealthStatus::Critical => 2,
        monitoring::HealthStatus::Unknown => 3,
    };
    
    std::process::exit(exit_code);
}

fn format_compact_health(health: &monitoring::SystemHealth) -> String {
    let status_char = match health.overall_status {
        monitoring::HealthStatus::Healthy => "✅",
        monitoring::HealthStatus::Warning => "⚠️",
        monitoring::HealthStatus::Critical => "❌",
        monitoring::HealthStatus::Unknown => "❓",
    };
    
    format!("{} {} | NC:{} MIN:{} NET:{} WAL:{} | CPU:{:.1}% MEM:{:.1}% PROC:{}",
        health.timestamp.format("%H:%M:%S"),
        status_char,
        format_service_status(&health.nockchain_status.status),
        format_service_status(&health.mining_status.status),
        format_service_status(&health.network_status.status),
        format_service_status(&health.wallet_status.status),
        health.system_metrics.cpu_usage_percent,
        health.system_metrics.memory_usage_percent,
        health.system_metrics.process_count,
    )
}

fn format_table_health(health: &monitoring::SystemHealth) -> String {
    let mut output = String::new();
    
    output.push_str(&format!("🔍 Nockchain System Monitor - {}\n", 
                            health.timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
    output.push_str(&format!("Overall Status: {}\n\n", 
                            format_health_status(&health.overall_status)));
    
    // Services table
    output.push_str("┌─────────────────┬──────────────┬─────────────────────────────────────┐\n");
    output.push_str("│ Service         │ Status       │ Details                             │\n");
    output.push_str("├─────────────────┼──────────────┼─────────────────────────────────────┤\n");
    
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
        
        output.push_str(&format!("│ {:<15} │ {:<12} │ {:<35} │\n", name, status_str, details));
    }
    
    output.push_str("└─────────────────┴──────────────┴─────────────────────────────────────┘\n\n");
    
    // System metrics
    output.push_str("System Metrics:\n");
    output.push_str(&format!("  CPU Usage: {:.1}%\n", health.system_metrics.cpu_usage_percent));
    output.push_str(&format!("  Memory Usage: {:.1}%\n", health.system_metrics.memory_usage_percent));
    output.push_str(&format!("  Disk Usage: {:.1}%\n", health.system_metrics.disk_usage_percent));
    output.push_str(&format!("  Process Count: {}\n", health.system_metrics.process_count));
    
    output
}

fn format_health_status(status: &monitoring::HealthStatus) -> String {
    match status {
        monitoring::HealthStatus::Healthy => "✅ Healthy".to_string(),
        monitoring::HealthStatus::Warning => "⚠️  Warning".to_string(),
        monitoring::HealthStatus::Critical => "❌ Critical".to_string(),
        monitoring::HealthStatus::Unknown => "❓ Unknown".to_string(),
    }
}

fn format_service_status(status: &monitoring::HealthStatus) -> &'static str {
    match status {
        monitoring::HealthStatus::Healthy => "✅",
        monitoring::HealthStatus::Warning => "⚠️",
        monitoring::HealthStatus::Critical => "❌",
        monitoring::HealthStatus::Unknown => "❓",
    }
} 