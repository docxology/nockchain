//! Standalone log management tool for nockchain
//! 
//! A focused tool for log analysis, monitoring, and management.

use clap::{Parser, Subcommand};
use nockit::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "nocklog")]
#[command(about = "Nockchain Log Management Tool")]
#[command(version = nockit::VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Configuration directory
    #[arg(long, default_value = ".nockit")]
    config_dir: PathBuf,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Tail live logs
    Tail {
        /// Number of lines to show
        #[arg(short, long, default_value = "100")]
        lines: usize,
        /// Follow log updates
        #[arg(short, long)]
        follow: bool,
    },
    /// Search logs for patterns
    Search {
        /// Search pattern (regex supported)
        pattern: String,
        /// Log file to search
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    /// Analyze log patterns and statistics
    Analyze {
        /// Time period for analysis
        #[arg(short, long, default_value = "1h")]
        period: String,
    },
    /// Export logs in various formats
    Export {
        /// Output format (json, csv, txt)
        #[arg(short, long, default_value = "json")]
        format: String,
        /// Output file
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Clean old log files
    Clean {
        /// Days to keep logs
        #[arg(short, long, default_value = "7")]
        days: u32,
        /// Dry run (don't actually delete)
        #[arg(long)]
        dry_run: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    init_logging(log_level, &cli.config_dir.join(NOCKIT_LOG_DIR))?;
    
    match cli.command {
        Commands::Tail { lines, follow } => {
            logging::tail_logs(lines, follow, &cli.config_dir).await
        }
        Commands::Search { pattern, file } => {
            logging::search_logs(&pattern, file.as_ref().map(|p| p.as_path()), &cli.config_dir).await
        }
        Commands::Analyze { period } => {
            logging::analyze_logs(&period, &cli.config_dir).await
        }
        Commands::Export { format, output } => {
            logging::export_logs(&format, &output, &cli.config_dir).await
        }
        Commands::Clean { days, dry_run } => {
            clean_old_logs(days, dry_run, &cli.config_dir).await
        }
    }
}

async fn clean_old_logs(days: u32, dry_run: bool, config_dir: &PathBuf) -> Result<()> {
    use chrono::{Duration, Utc};
    use tokio::fs;
    
    let log_dir = config_dir.join(NOCKIT_LOG_DIR);
    let cutoff_time = Utc::now() - Duration::days(days as i64);
    
    if !log_dir.exists() {
        println!("Log directory does not exist: {}", log_dir.display());
        return Ok(());
    }
    
    println!("Cleaning logs older than {} days (cutoff: {})", days, cutoff_time);
    
    let mut entries = fs::read_dir(&log_dir).await?;
    let mut cleaned_count = 0;
    let mut total_size = 0u64;
    
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() {
            let metadata = fs::metadata(&path).await?;
            let modified = metadata.modified()?;
            let modified_chrono = chrono::DateTime::<Utc>::from(modified);
            
            if modified_chrono < cutoff_time {
                let size = metadata.len();
                total_size += size;
                
                if dry_run {
                    println!("Would delete: {} ({} bytes)", path.display(), size);
                } else {
                    match fs::remove_file(&path).await {
                        Ok(_) => {
                            println!("Deleted: {} ({} bytes)", path.display(), size);
                            cleaned_count += 1;
                        }
                        Err(e) => {
                            eprintln!("Failed to delete {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }
    }
    
    if dry_run {
        println!("Dry run completed. Would delete {} files ({} bytes total)", cleaned_count, total_size);
    } else {
        println!("Cleaned {} log files ({} bytes total)", cleaned_count, total_size);
    }
    
    Ok(())
} 