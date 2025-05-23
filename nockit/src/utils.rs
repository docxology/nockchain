//! Development utilities and helper functions for nockit
//! 
//! Provides development environment management, testing utilities, and build helpers.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tokio::fs;

/// Initialize a new development environment
pub async fn init_dev_environment(name: &str, config_dir: &Path) -> Result<()> {
    println!("🚀 Initializing development environment: {}", name);
    
    let project_dir = std::env::current_dir()?.join(name);
    
    if project_dir.exists() {
        anyhow::bail!("Directory '{}' already exists", name);
    }
    
    // Create project directory structure
    fs::create_dir_all(&project_dir).await?;
    fs::create_dir_all(project_dir.join("src")).await?;
    fs::create_dir_all(project_dir.join("tests")).await?;
    fs::create_dir_all(project_dir.join("examples")).await?;
    fs::create_dir_all(project_dir.join("docs")).await?;
    
    // Create Cargo.toml
    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
# Nockchain dependencies
nockchain = {{ path = "../crates/nockchain" }}
nockchain-wallet = {{ path = "../crates/nockchain-wallet" }}
nockvm = {{ path = "../crates/nockvm/rust/nockvm" }}

# Common dependencies
anyhow = "1.0"
tokio = {{ version = "1.32", features = ["full"] }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
tracing = "0.1"

[dev-dependencies]
tempfile = "3.3"
"#, name);
    
    fs::write(project_dir.join("Cargo.toml"), cargo_toml).await?;
    
    // Create main.rs
    let main_rs = r#"use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    println!("Hello, Nockchain!");
    
    Ok(())
}
"#;
    
    fs::write(project_dir.join("src").join("main.rs"), main_rs).await?;
    
    // Create lib.rs
    let lib_rs = r#"//! Nockchain development project
//! 
//! This is a template project for nockchain development.

pub mod utils;

pub use utils::*;

/// Project version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
"#;
    
    fs::write(project_dir.join("src").join("lib.rs"), lib_rs).await?;
    
    // Create utils.rs
    let utils_rs = r#"//! Utility functions for this project

use anyhow::Result;

/// Example utility function
pub fn hello_nockchain() -> String {
    "Hello from Nockchain!".to_string()
}

/// Example async function
pub async fn async_hello() -> Result<String> {
    Ok("Async hello from Nockchain!".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_nockchain() {
        assert_eq!(hello_nockchain(), "Hello from Nockchain!");
    }

    #[tokio::test]
    async fn test_async_hello() {
        let result = async_hello().await.unwrap();
        assert_eq!(result, "Async hello from Nockchain!");
    }
}
"#;
    
    fs::write(project_dir.join("src").join("utils.rs"), utils_rs).await?;
    
    // Create README.md
    let readme = format!(r#"# {}

A Nockchain development project.

## Getting Started

```bash
# Build the project
cargo build

# Run the project
cargo run

# Run tests
cargo test

# Run with nockit
nockit dev test
```

## Development

This project is set up for nockchain development with:

- Nockchain dependencies pre-configured
- Example code structure
- Test setup
- Documentation template

## Resources

- [Nockchain Documentation](../docs/)
- [Nockit Toolkit](../nockit/)
"#, name);
    
    fs::write(project_dir.join("README.md"), readme).await?;
    
    // Create .gitignore
    let gitignore = r#"# Rust
/target/
Cargo.lock

# IDE
.vscode/
.idea/

# OS
.DS_Store
Thumbs.db

# Nockchain
.data.*
.nockchain-*
*.jam
"#;
    
    fs::write(project_dir.join(".gitignore"), gitignore).await?;
    
    println!("✅ Development environment '{}' created successfully!", name);
    println!("📁 Project directory: {}", project_dir.display());
    println!("\nNext steps:");
    println!("1. cd {}", name);
    println!("2. cargo build");
    println!("3. cargo run");
    
    Ok(())
}

/// Run development tests
pub async fn run_tests(suite: Option<&str>, config_dir: &Path) -> Result<()> {
    println!("🧪 Running development tests...");
    
    let mut cmd = Command::new("cargo");
    cmd.arg("test");
    
    if let Some(suite) = suite {
        cmd.arg("--test").arg(suite);
        println!("Running test suite: {}", suite);
    } else {
        println!("Running all tests");
    }
    
    // Add verbose output
    cmd.arg("--").arg("--nocapture");
    
    let output = cmd.output()
        .context("Failed to execute cargo test")?;
    
    if output.status.success() {
        println!("✅ Tests completed successfully!");
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("❌ Tests failed:");
        println!("{}", stderr);
        anyhow::bail!("Test execution failed");
    }
    
    Ok(())
}

/// Build development project
pub async fn build_project(target: &str, config_dir: &Path) -> Result<()> {
    println!("🔨 Building project (target: {})...", target);
    
    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    
    match target {
        "release" => {
            cmd.arg("--release");
            println!("Building in release mode");
        }
        "debug" => {
            println!("Building in debug mode");
        }
        _ => {
            anyhow::bail!("Invalid build target: {}. Use 'debug' or 'release'", target);
        }
    }
    
    let output = cmd.output()
        .context("Failed to execute cargo build")?;
    
    if output.status.success() {
        println!("✅ Build completed successfully!");
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.trim().is_empty() {
            println!("{}", stdout);
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("❌ Build failed:");
        println!("{}", stderr);
        anyhow::bail!("Build execution failed");
    }
    
    Ok(())
}

/// Clean development artifacts
pub async fn clean_artifacts(config_dir: &Path) -> Result<()> {
    println!("🧹 Cleaning development artifacts...");
    
    // Clean Cargo artifacts
    let output = Command::new("cargo")
        .arg("clean")
        .output()
        .context("Failed to execute cargo clean")?;
    
    if output.status.success() {
        println!("✅ Cargo artifacts cleaned");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("⚠️  Cargo clean failed: {}", stderr);
    }
    
    // Clean nockit artifacts
    let artifacts_to_clean = [
        "target",
        ".data.nockchain",
        ".data.hoonc",
        "assets/*.jam",
        "*.jam",
    ];
    
    for pattern in &artifacts_to_clean {
        if let Err(e) = clean_pattern(pattern).await {
            println!("⚠️  Failed to clean {}: {}", pattern, e);
        } else {
            println!("✅ Cleaned {}", pattern);
        }
    }
    
    // Clean nockit-specific directories
    let nockit_dirs = [
        config_dir.join("logs"),
        config_dir.join("mining_stats"),
        config_dir.join("traffic_stats"),
        config_dir.join("health_data"),
    ];
    
    for dir in &nockit_dirs {
        if dir.exists() {
            match fs::remove_dir_all(dir).await {
                Ok(_) => println!("✅ Cleaned {}", dir.display()),
                Err(e) => println!("⚠️  Failed to clean {}: {}", dir.display(), e),
            }
        }
    }
    
    println!("🧹 Cleanup completed!");
    
    Ok(())
}

/// Generate project documentation
pub async fn generate_docs(config_dir: &Path) -> Result<()> {
    println!("📚 Generating project documentation...");
    
    let output = Command::new("cargo")
        .arg("doc")
        .arg("--no-deps")
        .arg("--open")
        .output()
        .context("Failed to execute cargo doc")?;
    
    if output.status.success() {
        println!("✅ Documentation generated successfully!");
        println!("📖 Documentation should open in your browser");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("❌ Documentation generation failed:");
        println!("{}", stderr);
        anyhow::bail!("Documentation generation failed");
    }
    
    Ok(())
}

/// Run code formatting
pub async fn format_code() -> Result<()> {
    println!("🎨 Formatting code...");
    
    let output = Command::new("cargo")
        .arg("fmt")
        .output()
        .context("Failed to execute cargo fmt")?;
    
    if output.status.success() {
        println!("✅ Code formatted successfully!");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("❌ Code formatting failed:");
        println!("{}", stderr);
        anyhow::bail!("Code formatting failed");
    }
    
    Ok(())
}

/// Run code linting
pub async fn lint_code() -> Result<()> {
    println!("🔍 Linting code...");
    
    let output = Command::new("cargo")
        .arg("clippy")
        .arg("--all-targets")
        .arg("--all-features")
        .arg("--")
        .arg("-D")
        .arg("warnings")
        .output()
        .context("Failed to execute cargo clippy")?;
    
    if output.status.success() {
        println!("✅ Code linting completed successfully!");
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.trim().is_empty() {
            println!("{}", stdout);
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("❌ Code linting found issues:");
        println!("{}", stderr);
        anyhow::bail!("Code linting failed");
    }
    
    Ok(())
}

/// Check project dependencies for security issues
pub async fn audit_dependencies() -> Result<()> {
    println!("🔒 Auditing dependencies for security issues...");
    
    // Check if cargo-audit is installed
    let audit_check = Command::new("cargo")
        .arg("audit")
        .arg("--version")
        .output();
    
    match audit_check {
        Ok(output) if output.status.success() => {
            // Run the audit
            let output = Command::new("cargo")
                .arg("audit")
                .output()
                .context("Failed to execute cargo audit")?;
            
            if output.status.success() {
                println!("✅ Dependency audit completed - no issues found!");
            } else {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("⚠️  Dependency audit found issues:");
                println!("{}", stdout);
            }
        }
        _ => {
            println!("⚠️  cargo-audit not installed. Install with:");
            println!("   cargo install cargo-audit");
        }
    }
    
    Ok(())
}

// Helper functions

async fn clean_pattern(pattern: &str) -> Result<()> {
    // Simple pattern matching for common cases
    if pattern.contains("*") {
        // Use shell expansion for glob patterns
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("rm -rf {}", pattern))
            .output();
        
        match output {
            Ok(output) if output.status.success() => Ok(()),
            _ => Ok(()), // Ignore errors for glob patterns that might not match
        }
    } else {
        // Direct file/directory removal
        let path = std::path::Path::new(pattern);
        if path.exists() {
            if path.is_dir() {
                fs::remove_dir_all(path).await?;
            } else {
                fs::remove_file(path).await?;
            }
        }
        Ok(())
    }
}

/// Get system information for debugging
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        rust_version: get_rust_version(),
        cargo_version: get_cargo_version(),
        nockit_version: crate::VERSION.to_string(),
    }
}

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub rust_version: String,
    pub cargo_version: String,
    pub nockit_version: String,
}

fn get_rust_version() -> String {
    Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(|| "unknown".to_string())
        .trim()
        .to_string()
}

fn get_cargo_version() -> String {
    Command::new("cargo")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or_else(|| "unknown".to_string())
        .trim()
        .to_string()
}

/// Run comprehensive benchmark suite
pub async fn run_benchmarks(output: Option<&Path>, config_dir: &Path) -> Result<()> {
    println!("🚀 Running comprehensive benchmark suite...");
    
    let suite = crate::bench::run_benchmark_suite(config_dir).await?;
    
    if let Some(output_file) = output {
        let json = serde_json::to_string_pretty(&suite)?;
        fs::write(output_file, json).await?;
        println!("📊 Benchmark results saved to: {}", output_file.display());
    }
    
    Ok(())
}

/// Compare benchmark results between two runs
pub async fn compare_benchmarks(current: &Path, previous: &Path, config_dir: &Path) -> Result<()> {
    println!("📊 Comparing benchmark results...");
    
    // Load benchmark results
    let current_content = fs::read_to_string(current).await?;
    let current_suite: crate::bench::BenchmarkSuite = serde_json::from_str(&current_content)?;
    
    let previous_content = fs::read_to_string(previous).await?;
    let previous_suite: crate::bench::BenchmarkSuite = serde_json::from_str(&previous_content)?;
    
    // Compare results
    let comparison = crate::bench::compare_benchmarks(&current_suite, &previous_suite).await;
    
    // Print comparison results
    print_benchmark_comparison(&comparison);
    
    // Save comparison results
    let comparison_file = config_dir.join("benchmark_comparison.json");
    let json = serde_json::to_string_pretty(&comparison)?;
    fs::write(&comparison_file, json).await?;
    println!("📊 Comparison results saved to: {}", comparison_file.display());
    
    Ok(())
}

/// Profile performance of specific operations
pub async fn profile_operation(operation: &str, iterations: u64, config_dir: &Path) -> Result<()> {
    println!("🔍 Profiling operation: {} ({} iterations)", operation, iterations);
    
    let mut profiler = crate::bench::PerformanceProfiler::new();
    
    match operation {
        "crypto" => {
            for i in 0..iterations {
                if i % 100 == 0 {
                    profiler.checkpoint(&format!("iteration_{}", i));
                    profiler.sample_memory();
                }
                
                let _keypair = crate::crypto::KeyPair::generate()?;
                let data = format!("test data {}", i);
                let _hash = crate::crypto::hash_data(data.as_bytes());
            }
        }
        "config" => {
            for i in 0..iterations {
                if i % 10 == 0 {
                    profiler.checkpoint(&format!("iteration_{}", i));
                    profiler.sample_memory();
                }
                
                let mut config = crate::config::NockitConfig::load_or_create(config_dir)?;
                config.set_mining_pubkey(format!("profile_key_{}", i));
            }
        }
        "serialization" => {
            for i in 0..iterations {
                if i % 100 == 0 {
                    profiler.checkpoint(&format!("iteration_{}", i));
                    profiler.sample_memory();
                }
                
                let wallet_info = crate::wallet::WalletInfo {
                    public_key: format!("pubkey_{}", i),
                    private_key: None,
                    address: format!("address_{}", i),
                    chain_code: None,
                    seed_phrase: None,
                    balance: Some(i),
                    created_at: chrono::Utc::now(),
                    last_updated: chrono::Utc::now(),
                    wallet_type: "benchmark".to_string(),
                };
                
                let _json = serde_json::to_string(&wallet_info)?;
            }
        }
        _ => {
            anyhow::bail!("Unknown operation: {}. Available: crypto, config, serialization", operation);
        }
    }
    
    let report = profiler.report();
    print_profiling_report(&report);
    
    // Save profiling report
    let report_file = config_dir.join(format!("profile_{}_{}.json", operation, chrono::Utc::now().format("%Y%m%d_%H%M%S")));
    let json = serde_json::to_string_pretty(&report)?;
    fs::write(&report_file, json).await?;
    println!("📊 Profiling report saved to: {}", report_file.display());
    
    Ok(())
}

/// Print benchmark comparison results
fn print_benchmark_comparison(comparison: &crate::bench::BenchmarkComparison) {
    println!("\n📊 Benchmark Comparison");
    println!("======================");
    println!("Current version: {}", comparison.current_version);
    println!("Previous version: {}", comparison.previous_version);
    println!("Comparison timestamp: {}", comparison.comparison_timestamp);
    
    println!("\n📈 Performance Changes");
    println!("=====================");
    
    for result in &comparison.result_comparisons {
        let change_indicator = if result.improved { "📈" } else { "📉" };
        let change_color = if result.improved { "+" } else { "" };
        
        println!("{} {}: {:.2} ops/sec → {:.2} ops/sec ({}{}%)",
                 change_indicator,
                 result.name,
                 result.previous_ops_per_second,
                 result.current_ops_per_second,
                 change_color,
                 result.performance_change_percent);
    }
}

/// Print profiling report
fn print_profiling_report(report: &crate::bench::ProfilingReport) {
    println!("\n🔍 Profiling Report");
    println!("==================");
    println!("Total duration: {:?}", report.total_duration);
    println!("Peak memory usage: {:.2} MB", report.peak_memory);
    
    println!("\n⏱️ Checkpoint Timings");
    println!("====================");
    
    for (name, duration) in &report.checkpoint_durations {
        println!("• {}: {:?}", name, duration);
    }
    
    if !report.memory_samples.is_empty() {
        println!("\n💾 Memory Usage Samples");
        println!("======================");
        println!("Samples: {}", report.memory_samples.len());
        println!("Average: {:.2} MB", report.memory_samples.iter().sum::<f64>() / report.memory_samples.len() as f64);
        println!("Peak: {:.2} MB", report.peak_memory);
    }
} 