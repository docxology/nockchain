//! Advanced logging functionality for nockit
//! 
//! Provides log initialization, parsing, analysis, and monitoring capabilities.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::time::{sleep, Duration};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Log entry structure for parsing and analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub target: String,
    pub message: String,
    pub fields: HashMap<String, String>,
}

/// Log statistics for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogStats {
    pub total_entries: usize,
    pub level_counts: HashMap<String, usize>,
    pub target_counts: HashMap<String, usize>,
    pub error_patterns: Vec<String>,
    pub time_range: (DateTime<Utc>, DateTime<Utc>),
}

/// Initialize logging with file rotation and structured output
pub fn init_logging(level: &str, log_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(log_dir)
        .with_context(|| format!("Failed to create log directory: {}", log_dir.display()))?;
    
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level));
    
    let file_appender = tracing_appender::rolling::daily(log_dir, "nockit.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            fmt::layer()
                .with_writer(std::io::stdout)
                .with_ansi(true)
                .compact()
        )
        .with(
            fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .json()
        )
        .init();
    
    Ok(())
}

/// Tail logs with optional following
pub async fn tail_logs(lines: usize, follow: bool, config_dir: &Path) -> Result<()> {
    let log_dir = config_dir.join(crate::NOCKIT_LOG_DIR);
    let log_files = find_log_files(&log_dir).await?;
    
    if log_files.is_empty() {
        println!("No log files found in {}", log_dir.display());
        return Ok(());
    }
    
    // Get the most recent log file
    let latest_log = log_files.into_iter().max_by_key(|p| {
        std::fs::metadata(p).and_then(|m| m.modified()).unwrap_or(std::time::UNIX_EPOCH)
    }).unwrap();
    
    println!("Tailing log file: {}", latest_log.display());
    
    if follow {
        tail_follow(&latest_log, lines).await?;
    } else {
        tail_static(&latest_log, lines).await?;
    }
    
    Ok(())
}

/// Search logs for patterns
pub async fn search_logs(pattern: &str, file: Option<&Path>, config_dir: &Path) -> Result<()> {
    let regex = regex::Regex::new(pattern)
        .with_context(|| format!("Invalid regex pattern: {}", pattern))?;
    
    let log_files = if let Some(file) = file {
        vec![file.to_path_buf()]
    } else {
        let log_dir = config_dir.join(crate::NOCKIT_LOG_DIR);
        find_log_files(&log_dir).await?
    };
    
    let mut total_matches = 0;
    
    for log_file in log_files {
        println!("\n=== Searching {} ===", log_file.display());
        let matches = search_file(&log_file, &regex).await?;
        total_matches += matches;
    }
    
    println!("\nTotal matches found: {}", total_matches);
    Ok(())
}

/// Analyze logs for patterns and statistics
pub async fn analyze_logs(period: &str, config_dir: &Path) -> Result<()> {
    let log_dir = config_dir.join(crate::NOCKIT_LOG_DIR);
    let log_files = find_log_files(&log_dir).await?;
    
    if log_files.is_empty() {
        println!("No log files found for analysis");
        return Ok(());
    }
    
    let duration = parse_time_period(period)?;
    let cutoff_time = Utc::now() - duration;
    
    let mut stats = LogStats {
        total_entries: 0,
        level_counts: HashMap::new(),
        target_counts: HashMap::new(),
        error_patterns: Vec::new(),
        time_range: (Utc::now(), cutoff_time),
    };
    
    for log_file in log_files {
        analyze_file(&log_file, &mut stats, cutoff_time).await?;
    }
    
    print_analysis_results(&stats);
    Ok(())
}

/// Export logs in various formats
pub async fn export_logs(format: &str, output: &Path, config_dir: &Path) -> Result<()> {
    let log_dir = config_dir.join(crate::NOCKIT_LOG_DIR);
    let log_files = find_log_files(&log_dir).await?;
    
    let mut all_entries = Vec::new();
    
    for log_file in log_files {
        let entries = parse_log_file(&log_file).await?;
        all_entries.extend(entries);
    }
    
    // Sort by timestamp
    all_entries.sort_by_key(|entry| entry.timestamp);
    
    match format {
        "json" => export_json(&all_entries, output).await?,
        "csv" => export_csv(&all_entries, output).await?,
        "txt" => export_text(&all_entries, output).await?,
        _ => anyhow::bail!("Unsupported export format: {}", format),
    }
    
    println!("Exported {} log entries to {}", all_entries.len(), output.display());
    Ok(())
}

// Helper functions

async fn find_log_files(log_dir: &Path) -> Result<Vec<PathBuf>> {
    if !log_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut entries = fs::read_dir(log_dir).await?;
    let mut log_files = Vec::new();
    
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "log") {
            log_files.push(path);
        }
    }
    
    Ok(log_files)
}

async fn tail_static(file: &Path, lines: usize) -> Result<()> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let all_lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;
    
    let start = if all_lines.len() > lines {
        all_lines.len() - lines
    } else {
        0
    };
    
    for line in &all_lines[start..] {
        println!("{}", line);
    }
    
    Ok(())
}

async fn tail_follow(file: &Path, initial_lines: usize) -> Result<()> {
    // Show initial lines
    tail_static(file, initial_lines).await?;
    
    let mut file = File::open(file)?;
    file.seek(SeekFrom::End(0))?;
    let mut reader = BufReader::new(file);
    
    println!("\n--- Following log file (Ctrl+C to exit) ---");
    
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => {
                // No new data, wait and try again
                sleep(Duration::from_millis(100)).await;
                continue;
            }
            Ok(_) => {
                print!("{}", line);
            }
            Err(e) => {
                eprintln!("Error reading log file: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}

async fn search_file(file: &Path, regex: &regex::Regex) -> Result<usize> {
    let content = fs::read_to_string(file).await?;
    let mut matches = 0;
    
    for (line_num, line) in content.lines().enumerate() {
        if regex.is_match(line) {
            println!("{}:{}: {}", file.display(), line_num + 1, line);
            matches += 1;
        }
    }
    
    Ok(matches)
}

async fn analyze_file(file: &Path, stats: &mut LogStats, cutoff_time: DateTime<Utc>) -> Result<()> {
    let entries = parse_log_file(file).await?;
    
    for entry in entries {
        if entry.timestamp >= cutoff_time {
            stats.total_entries += 1;
            
            *stats.level_counts.entry(entry.level.clone()).or_insert(0) += 1;
            *stats.target_counts.entry(entry.target.clone()).or_insert(0) += 1;
            
            if entry.level == "ERROR" {
                stats.error_patterns.push(entry.message.clone());
            }
            
            if entry.timestamp < stats.time_range.0 {
                stats.time_range.0 = entry.timestamp;
            }
            if entry.timestamp > stats.time_range.1 {
                stats.time_range.1 = entry.timestamp;
            }
        }
    }
    
    Ok(())
}

async fn parse_log_file(file: &Path) -> Result<Vec<LogEntry>> {
    let content = fs::read_to_string(file).await?;
    let mut entries = Vec::new();
    
    for line in content.lines() {
        if let Ok(entry) = parse_log_line(line) {
            entries.push(entry);
        }
    }
    
    Ok(entries)
}

fn parse_log_line(line: &str) -> Result<LogEntry> {
    // Try to parse as JSON first (structured logs)
    if let Ok(json_entry) = serde_json::from_str::<serde_json::Value>(line) {
        return parse_json_log_entry(&json_entry);
    }
    
    // Fall back to parsing text logs
    parse_text_log_entry(line)
}

fn parse_json_log_entry(json: &serde_json::Value) -> Result<LogEntry> {
    let timestamp = json["timestamp"]
        .as_str()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(Utc::now);
    
    let level = json["level"].as_str().unwrap_or("INFO").to_string();
    let target = json["target"].as_str().unwrap_or("unknown").to_string();
    let message = json["message"].as_str().unwrap_or("").to_string();
    
    let mut fields = HashMap::new();
    if let Some(obj) = json.as_object() {
        for (key, value) in obj {
            if !["timestamp", "level", "target", "message"].contains(&key.as_str()) {
                fields.insert(key.clone(), value.to_string());
            }
        }
    }
    
    Ok(LogEntry {
        timestamp,
        level,
        target,
        message,
        fields,
    })
}

fn parse_text_log_entry(line: &str) -> Result<LogEntry> {
    // Simple text log parsing - adjust regex based on your log format
    let re = regex::Regex::new(r"^(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+Z)\s+(\w+)\s+(\S+):\s+(.+)$")?;
    
    if let Some(captures) = re.captures(line) {
        let timestamp = DateTime::parse_from_rfc3339(&captures[1])?
            .with_timezone(&Utc);
        let level = captures[2].to_string();
        let target = captures[3].to_string();
        let message = captures[4].to_string();
        
        Ok(LogEntry {
            timestamp,
            level,
            target,
            message,
            fields: HashMap::new(),
        })
    } else {
        // Fallback for unparseable lines
        Ok(LogEntry {
            timestamp: Utc::now(),
            level: "UNKNOWN".to_string(),
            target: "unknown".to_string(),
            message: line.to_string(),
            fields: HashMap::new(),
        })
    }
}

fn parse_time_period(period: &str) -> Result<chrono::Duration> {
    match period {
        "1h" => Ok(chrono::Duration::hours(1)),
        "6h" => Ok(chrono::Duration::hours(6)),
        "12h" => Ok(chrono::Duration::hours(12)),
        "1d" => Ok(chrono::Duration::days(1)),
        "1w" => Ok(chrono::Duration::weeks(1)),
        "1m" => Ok(chrono::Duration::days(30)),
        _ => anyhow::bail!("Invalid time period: {}. Use 1h, 6h, 12h, 1d, 1w, or 1m", period),
    }
}

fn print_analysis_results(stats: &LogStats) {
    println!("\n=== Log Analysis Results ===");
    println!("Total entries: {}", stats.total_entries);
    println!("Time range: {} to {}", stats.time_range.0, stats.time_range.1);
    
    println!("\nLog levels:");
    for (level, count) in &stats.level_counts {
        println!("  {}: {}", level, count);
    }
    
    println!("\nTop targets:");
    let mut targets: Vec<_> = stats.target_counts.iter().collect();
    targets.sort_by(|a, b| b.1.cmp(a.1));
    for (target, count) in targets.iter().take(10) {
        println!("  {}: {}", target, count);
    }
    
    if !stats.error_patterns.is_empty() {
        println!("\nRecent error patterns:");
        for (i, error) in stats.error_patterns.iter().take(5).enumerate() {
            println!("  {}: {}", i + 1, error);
        }
    }
}

async fn export_json(entries: &[LogEntry], output: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(entries)?;
    fs::write(output, json).await?;
    Ok(())
}

async fn export_csv(entries: &[LogEntry], output: &Path) -> Result<()> {
    let mut csv = String::from("timestamp,level,target,message\n");
    
    for entry in entries {
        csv.push_str(&format!(
            "{},{},{},\"{}\"\n",
            entry.timestamp,
            entry.level,
            entry.target,
            entry.message.replace('"', "\"\"")
        ));
    }
    
    fs::write(output, csv).await?;
    Ok(())
}

async fn export_text(entries: &[LogEntry], output: &Path) -> Result<()> {
    let mut text = String::new();
    
    for entry in entries {
        text.push_str(&format!(
            "{} {} {}: {}\n",
            entry.timestamp,
            entry.level,
            entry.target,
            entry.message
        ));
    }
    
    fs::write(output, text).await?;
    Ok(())
} 