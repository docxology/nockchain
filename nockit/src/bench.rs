//! Benchmarking and performance testing utilities for nockit
//! 
//! Provides comprehensive performance testing, profiling, and optimization
//! tools for nockchain operations and nockit functionality.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, Instant};
use tokio::fs;

/// Benchmark result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub duration: Duration,
    pub iterations: u64,
    pub operations_per_second: f64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub metadata: HashMap<String, String>,
}

/// Benchmark suite for comprehensive testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSuite {
    pub name: String,
    pub version: String,
    pub timestamp: DateTime<Utc>,
    pub results: Vec<BenchmarkResult>,
    pub summary: BenchmarkSummary,
}

/// Summary statistics for benchmark suite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    pub total_tests: usize,
    pub total_duration: Duration,
    pub average_ops_per_second: f64,
    pub peak_memory_usage_mb: f64,
    pub average_cpu_usage_percent: f64,
}

/// Performance profiler for detailed analysis
pub struct PerformanceProfiler {
    start_time: Instant,
    checkpoints: Vec<(String, Instant)>,
    memory_samples: Vec<f64>,
}

impl PerformanceProfiler {
    /// Create a new performance profiler
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            checkpoints: Vec::new(),
            memory_samples: Vec::new(),
        }
    }
    
    /// Add a checkpoint for timing analysis
    pub fn checkpoint(&mut self, name: &str) {
        self.checkpoints.push((name.to_string(), Instant::now()));
    }
    
    /// Sample current memory usage
    pub fn sample_memory(&mut self) {
        // In a real implementation, this would use system APIs to get actual memory usage
        // For now, we'll use a placeholder
        self.memory_samples.push(0.0);
    }
    
    /// Generate profiling report
    pub fn report(&self) -> ProfilingReport {
        let total_duration = self.start_time.elapsed();
        let mut checkpoint_durations = Vec::new();
        
        let mut last_time = self.start_time;
        for (name, time) in &self.checkpoints {
            let duration = time.duration_since(last_time);
            checkpoint_durations.push((name.clone(), duration));
            last_time = *time;
        }
        
        ProfilingReport {
            total_duration,
            checkpoint_durations,
            memory_samples: self.memory_samples.clone(),
            peak_memory: self.memory_samples.iter().fold(0.0f64, |a, &b| a.max(b)),
        }
    }
}

/// Profiling report structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingReport {
    pub total_duration: Duration,
    pub checkpoint_durations: Vec<(String, Duration)>,
    pub memory_samples: Vec<f64>,
    pub peak_memory: f64,
}

/// Run comprehensive benchmark suite
pub async fn run_benchmark_suite(config_dir: &Path) -> Result<BenchmarkSuite> {
    println!("🚀 Running comprehensive nockit benchmark suite...");
    
    let mut results = Vec::new();
    let start_time = Instant::now();
    
    // Benchmark cryptographic operations
    results.push(benchmark_crypto_operations().await?);
    
    // Benchmark configuration operations
    results.push(benchmark_config_operations(config_dir).await?);
    
    // Benchmark logging operations
    results.push(benchmark_logging_operations(config_dir).await?);
    
    // Benchmark serialization operations
    results.push(benchmark_serialization_operations().await?);
    
    // Benchmark file I/O operations
    results.push(benchmark_file_operations(config_dir).await?);
    
    // Benchmark network operations
    results.push(benchmark_network_operations().await?);
    
    let total_duration = start_time.elapsed();
    
    // Calculate summary statistics
    let summary = calculate_summary(&results, total_duration);
    
    let suite = BenchmarkSuite {
        name: "Nockit Comprehensive Benchmark".to_string(),
        version: crate::VERSION.to_string(),
        timestamp: Utc::now(),
        results,
        summary,
    };
    
    // Save benchmark results
    save_benchmark_results(&suite, config_dir).await?;
    
    println!("✅ Benchmark suite completed in {:?}", total_duration);
    print_benchmark_summary(&suite);
    
    Ok(suite)
}

/// Benchmark cryptographic operations
async fn benchmark_crypto_operations() -> Result<BenchmarkResult> {
    println!("🔐 Benchmarking cryptographic operations...");
    
    let iterations = 1000u64;
    let start = Instant::now();
    
    for _ in 0..iterations {
        // Key generation
        let _keypair = crate::crypto::KeyPair::generate()?;
        
        // Hashing
        let data = b"benchmark data for hashing";
        let _hash = crate::crypto::hash_data(data);
        
        // Signature operations
        let keypair = crate::crypto::KeyPair::generate()?;
        let message = b"benchmark message";
        let signature = keypair.private_key.sign(message)?;
        let _verified = keypair.public_key.verify(message, &signature)?;
    }
    
    let duration = start.elapsed();
    let ops_per_second = (iterations * 4) as f64 / duration.as_secs_f64(); // 4 ops per iteration
    
    Ok(BenchmarkResult {
        name: "Cryptographic Operations".to_string(),
        description: "Key generation, hashing, signing, and verification".to_string(),
        timestamp: Utc::now(),
        duration,
        iterations: iterations * 4,
        operations_per_second: ops_per_second,
        memory_usage_mb: 0.0, // Placeholder
        cpu_usage_percent: 0.0, // Placeholder
        metadata: HashMap::from([
            ("operations".to_string(), "keygen,hash,sign,verify".to_string()),
            ("algorithm".to_string(), "BLAKE3".to_string()),
        ]),
    })
}

/// Benchmark configuration operations
async fn benchmark_config_operations(config_dir: &Path) -> Result<BenchmarkResult> {
    println!("⚙️ Benchmarking configuration operations...");
    
    let iterations = 100u64;
    let start = Instant::now();
    
    for i in 0..iterations {
        // Load/create configuration
        let mut config = crate::config::NockitConfig::load_or_create(config_dir)?;
        
        // Modify configuration
        config.set_mining_pubkey(format!("benchmark_pubkey_{}", i));
        
        // Save configuration
        let config_file = config_dir.join(format!("bench_config_{}.toml", i));
        config.save(&config_file)?;
        
        // Load configuration back
        let _loaded_config = crate::config::NockitConfig::load(&config_file)?;
        
        // Clean up
        let _ = std::fs::remove_file(config_file);
    }
    
    let duration = start.elapsed();
    let ops_per_second = (iterations * 3) as f64 / duration.as_secs_f64(); // 3 ops per iteration
    
    Ok(BenchmarkResult {
        name: "Configuration Operations".to_string(),
        description: "Load, modify, save, and reload configuration files".to_string(),
        timestamp: Utc::now(),
        duration,
        iterations: iterations * 3,
        operations_per_second: ops_per_second,
        memory_usage_mb: 0.0,
        cpu_usage_percent: 0.0,
        metadata: HashMap::from([
            ("format".to_string(), "TOML".to_string()),
            ("operations".to_string(), "load,save,modify".to_string()),
        ]),
    })
}

/// Benchmark logging operations
async fn benchmark_logging_operations(config_dir: &Path) -> Result<BenchmarkResult> {
    println!("📝 Benchmarking logging operations...");
    
    let iterations = 1000u64;
    let log_dir = config_dir.join("bench_logs");
    fs::create_dir_all(&log_dir).await?;
    
    let start = Instant::now();
    
    // Create test log entries
    let mut log_entries = Vec::new();
    for i in 0..iterations {
        log_entries.push(crate::logging::LogEntry {
            timestamp: Utc::now(),
            level: if i % 10 == 0 { "ERROR" } else { "INFO" }.to_string(),
            target: "benchmark".to_string(),
            message: format!("Benchmark log message {}", i),
            fields: HashMap::new(),
        });
    }
    
    // Benchmark serialization
    let json = serde_json::to_string_pretty(&log_entries)?;
    
    // Benchmark file writing
    let log_file = log_dir.join("benchmark.json");
    fs::write(&log_file, json).await?;
    
    // Benchmark file reading
    let _content = fs::read_to_string(&log_file).await?;
    
    let duration = start.elapsed();
    let ops_per_second = iterations as f64 / duration.as_secs_f64();
    
    // Clean up
    let _ = fs::remove_dir_all(log_dir).await;
    
    Ok(BenchmarkResult {
        name: "Logging Operations".to_string(),
        description: "Log entry creation, serialization, and file I/O".to_string(),
        timestamp: Utc::now(),
        duration,
        iterations,
        operations_per_second: ops_per_second,
        memory_usage_mb: 0.0,
        cpu_usage_percent: 0.0,
        metadata: HashMap::from([
            ("format".to_string(), "JSON".to_string()),
            ("entries".to_string(), iterations.to_string()),
        ]),
    })
}

/// Benchmark serialization operations
async fn benchmark_serialization_operations() -> Result<BenchmarkResult> {
    println!("📦 Benchmarking serialization operations...");
    
    let iterations = 1000u64;
    let start = Instant::now();
    
    for _ in 0..iterations {
        // Create test data structures
        let wallet_info = crate::wallet::WalletInfo {
            public_key: "benchmark_pubkey".to_string(),
            private_key: None,
            address: "benchmark_address".to_string(),
            chain_code: None,
            seed_phrase: None,
            balance: Some(1000),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            wallet_type: "benchmark".to_string(),
        };
        
        let mining_stats = crate::mining::MiningStats {
            start_time: Utc::now(),
            end_time: None,
            blocks_mined: 10,
            hash_rate: 1000.0,
            difficulty: 12345,
            rewards_earned: 500,
            uptime_seconds: 3600,
            errors: Vec::new(),
        };
        
        // Serialize to JSON
        let _wallet_json = serde_json::to_string(&wallet_info)?;
        let _stats_json = serde_json::to_string(&mining_stats)?;
        
        // Serialize to pretty JSON
        let _wallet_pretty = serde_json::to_string_pretty(&wallet_info)?;
        let _stats_pretty = serde_json::to_string_pretty(&mining_stats)?;
    }
    
    let duration = start.elapsed();
    let ops_per_second = (iterations * 4) as f64 / duration.as_secs_f64(); // 4 serializations per iteration
    
    Ok(BenchmarkResult {
        name: "Serialization Operations".to_string(),
        description: "JSON serialization of data structures".to_string(),
        timestamp: Utc::now(),
        duration,
        iterations: iterations * 4,
        operations_per_second: ops_per_second,
        memory_usage_mb: 0.0,
        cpu_usage_percent: 0.0,
        metadata: HashMap::from([
            ("format".to_string(), "JSON".to_string()),
            ("structures".to_string(), "WalletInfo,MiningStats".to_string()),
        ]),
    })
}

/// Benchmark file I/O operations
async fn benchmark_file_operations(config_dir: &Path) -> Result<BenchmarkResult> {
    println!("💾 Benchmarking file I/O operations...");
    
    let iterations = 100u64;
    let bench_dir = config_dir.join("bench_files");
    fs::create_dir_all(&bench_dir).await?;
    
    let start = Instant::now();
    
    for i in 0..iterations {
        let file_path = bench_dir.join(format!("bench_file_{}.txt", i));
        let data = format!("Benchmark data for file {}", i).repeat(100); // ~2KB per file
        
        // Write file
        fs::write(&file_path, &data).await?;
        
        // Read file
        let _content = fs::read_to_string(&file_path).await?;
        
        // Append to file
        fs::write(&file_path, format!("{}\nAppended data", data)).await?;
        
        // Delete file
        fs::remove_file(&file_path).await?;
    }
    
    let duration = start.elapsed();
    let ops_per_second = (iterations * 4) as f64 / duration.as_secs_f64(); // 4 ops per iteration
    
    // Clean up
    let _ = fs::remove_dir_all(bench_dir).await;
    
    Ok(BenchmarkResult {
        name: "File I/O Operations".to_string(),
        description: "File creation, reading, writing, and deletion".to_string(),
        timestamp: Utc::now(),
        duration,
        iterations: iterations * 4,
        operations_per_second: ops_per_second,
        memory_usage_mb: 0.0,
        cpu_usage_percent: 0.0,
        metadata: HashMap::from([
            ("file_size".to_string(), "~2KB".to_string()),
            ("operations".to_string(), "write,read,append,delete".to_string()),
        ]),
    })
}

/// Benchmark network operations
async fn benchmark_network_operations() -> Result<BenchmarkResult> {
    println!("🌐 Benchmarking network operations...");
    
    let iterations = 10u64; // Fewer iterations for network operations
    let start = Instant::now();
    
    for _ in 0..iterations {
        // DNS resolution benchmark
        match hickory_resolver::TokioAsyncResolver::tokio_from_system_conf() {
            Ok(resolver) => {
                let _ = tokio::time::timeout(
                    std::time::Duration::from_secs(2),
                    resolver.lookup_ip("google.com")
                ).await;
            }
            Err(_) => {} // Skip if resolver setup fails
        }
        
        // HTTP request benchmark (if available)
        let _ = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            reqwest::get("https://httpbin.org/ip")
        ).await;
    }
    
    let duration = start.elapsed();
    let ops_per_second = (iterations * 2) as f64 / duration.as_secs_f64(); // 2 ops per iteration
    
    Ok(BenchmarkResult {
        name: "Network Operations".to_string(),
        description: "DNS resolution and HTTP requests".to_string(),
        timestamp: Utc::now(),
        duration,
        iterations: iterations * 2,
        operations_per_second: ops_per_second,
        memory_usage_mb: 0.0,
        cpu_usage_percent: 0.0,
        metadata: HashMap::from([
            ("operations".to_string(), "dns,http".to_string()),
            ("timeout".to_string(), "5s".to_string()),
        ]),
    })
}

/// Calculate summary statistics for benchmark suite
fn calculate_summary(results: &[BenchmarkResult], total_duration: Duration) -> BenchmarkSummary {
    let total_tests = results.len();
    let average_ops_per_second = results.iter()
        .map(|r| r.operations_per_second)
        .sum::<f64>() / total_tests as f64;
    let peak_memory_usage_mb = results.iter()
        .map(|r| r.memory_usage_mb)
        .fold(0.0f64, |a, b| a.max(b));
    let average_cpu_usage_percent = results.iter()
        .map(|r| r.cpu_usage_percent)
        .sum::<f64>() / total_tests as f64;
    
    BenchmarkSummary {
        total_tests,
        total_duration,
        average_ops_per_second,
        peak_memory_usage_mb,
        average_cpu_usage_percent,
    }
}

/// Save benchmark results to file
async fn save_benchmark_results(suite: &BenchmarkSuite, config_dir: &Path) -> Result<()> {
    let bench_dir = config_dir.join("benchmarks");
    fs::create_dir_all(&bench_dir).await?;
    
    let timestamp = suite.timestamp.format("%Y%m%d_%H%M%S");
    let results_file = bench_dir.join(format!("benchmark_results_{}.json", timestamp));
    
    let json = serde_json::to_string_pretty(suite)?;
    fs::write(&results_file, json).await?;
    
    println!("📊 Benchmark results saved to: {}", results_file.display());
    Ok(())
}

/// Print benchmark summary
fn print_benchmark_summary(suite: &BenchmarkSuite) {
    println!("\n📊 Benchmark Summary");
    println!("==================");
    println!("Suite: {}", suite.name);
    println!("Version: {}", suite.version);
    println!("Timestamp: {}", suite.timestamp);
    println!("Total tests: {}", suite.summary.total_tests);
    println!("Total duration: {:?}", suite.summary.total_duration);
    println!("Average ops/sec: {:.2}", suite.summary.average_ops_per_second);
    
    println!("\n📈 Individual Results");
    println!("====================");
    for result in &suite.results {
        println!("• {}: {:.2} ops/sec ({:?})", 
                 result.name, 
                 result.operations_per_second, 
                 result.duration);
    }
}

/// Compare benchmark results between runs
pub async fn compare_benchmarks(
    current: &BenchmarkSuite, 
    previous: &BenchmarkSuite
) -> BenchmarkComparison {
    let mut comparisons = Vec::new();
    
    for current_result in &current.results {
        if let Some(previous_result) = previous.results.iter()
            .find(|r| r.name == current_result.name) {
            
            let performance_change = (current_result.operations_per_second - previous_result.operations_per_second) 
                / previous_result.operations_per_second * 100.0;
            
            comparisons.push(BenchmarkResultComparison {
                name: current_result.name.clone(),
                current_ops_per_second: current_result.operations_per_second,
                previous_ops_per_second: previous_result.operations_per_second,
                performance_change_percent: performance_change,
                improved: performance_change > 0.0,
            });
        }
    }
    
    BenchmarkComparison {
        current_version: current.version.clone(),
        previous_version: previous.version.clone(),
        comparison_timestamp: Utc::now(),
        result_comparisons: comparisons,
    }
}

/// Benchmark comparison structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkComparison {
    pub current_version: String,
    pub previous_version: String,
    pub comparison_timestamp: DateTime<Utc>,
    pub result_comparisons: Vec<BenchmarkResultComparison>,
}

/// Individual benchmark result comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResultComparison {
    pub name: String,
    pub current_ops_per_second: f64,
    pub previous_ops_per_second: f64,
    pub performance_change_percent: f64,
    pub improved: bool,
} 