//! Integration tests for nockit toolkit
//! 
//! Tests the complete functionality of nockit including configuration,
//! logging, wallet operations, mining, network monitoring, and utilities.

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use serial_test::serial;
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::fs;

/// Test configuration management
#[tokio::test]
async fn test_config_management() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_dir = temp_dir.path();
    
    // Test default configuration creation
    let config = nockit::config::NockitConfig::load_or_create(config_dir)?;
    assert_eq!(config.logging.level, "info");
    assert_eq!(config.logging.format, "pretty");
    
    // Test configuration saving and loading
    let config_file = config_dir.join("config.toml");
    config.save(&config_file)?;
    assert!(config_file.exists());
    
    let loaded_config = nockit::config::NockitConfig::load(&config_file)?;
    assert_eq!(config.logging.level, loaded_config.logging.level);
    
    // Test mining pubkey setting
    let mut config = loaded_config;
    config.set_mining_pubkey("test_pubkey_123".to_string());
    assert_eq!(config.get_mining_pubkey(), Some("test_pubkey_123".to_string()));
    
    Ok(())
}

/// Test environment variable management
#[tokio::test]
async fn test_env_management() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let env_file = temp_dir.path().join(".env");
    
    // Create test .env file
    fs::write(&env_file, "TEST_VAR=test_value\nRUST_LOG=debug").await?;
    
    // Test loading environment file
    nockit::config::EnvManager::load_env_file(&env_file)?;
    
    // Test environment variable retrieval
    let env_vars = nockit::config::EnvManager::get_nockchain_env();
    assert!(!env_vars.is_empty());
    
    Ok(())
}

/// Test cryptographic utilities
#[tokio::test]
async fn test_crypto_utilities() -> Result<()> {
    // Test key pair generation
    let keypair = nockit::crypto::KeyPair::generate()?;
    assert!(!keypair.public_key.as_bytes().is_empty());
    
    // Test public key encoding/decoding
    let encoded = keypair.public_key.to_base58();
    let decoded = nockit::crypto::PublicKey::from_base58(&encoded)?;
    assert_eq!(keypair.public_key, decoded);
    
    // Test hashing
    let data = b"test data for hashing";
    let hash1 = nockit::crypto::hash_data(data);
    let hash2 = nockit::crypto::hash_data(data);
    assert_eq!(hash1, hash2);
    
    // Test hash hex encoding
    let hex = hash1.to_hex();
    let decoded_hash = nockit::crypto::Hash::from_hex(&hex)?;
    assert_eq!(hash1, decoded_hash);
    
    // Test signature creation and verification
    let message = b"test message for signing";
    let signature = keypair.private_key.sign(message)?;
    let is_valid = keypair.public_key.verify(message, &signature)?;
    assert!(is_valid);
    
    // Test nonce generation
    let nonce1 = nockit::crypto::generate_nonce()?;
    let nonce2 = nockit::crypto::generate_nonce()?;
    assert_ne!(nonce1, nonce2);
    
    Ok(())
}

/// Test logging functionality
#[tokio::test]
async fn test_logging_functionality() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let log_dir = temp_dir.path().join("logs");
    
    // Test log initialization
    nockit::logging::init_logging("debug", &log_dir)?;
    assert!(log_dir.exists());
    
    // Create test log entries
    let test_entries = vec![
        nockit::logging::LogEntry {
            timestamp: chrono::Utc::now(),
            level: "INFO".to_string(),
            target: "test".to_string(),
            message: "Test log message".to_string(),
            fields: std::collections::HashMap::new(),
        },
        nockit::logging::LogEntry {
            timestamp: chrono::Utc::now(),
            level: "ERROR".to_string(),
            target: "test".to_string(),
            message: "Test error message".to_string(),
            fields: std::collections::HashMap::new(),
        },
    ];
    
    // Test log export
    let export_file = temp_dir.path().join("test_export.json");
    let json = serde_json::to_string_pretty(&test_entries)?;
    fs::write(&export_file, json).await?;
    assert!(export_file.exists());
    
    Ok(())
}

/// Test wallet operations
#[tokio::test]
#[serial]
async fn test_wallet_operations() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_dir = temp_dir.path();
    
    // Test wallet info structure
    let wallet_info = nockit::wallet::WalletInfo {
        public_key: "test_pubkey_123".to_string(),
        address: "test_address_123".to_string(),
        balance: Some(1000),
        created_at: chrono::Utc::now(),
        last_updated: chrono::Utc::now(),
    };
    
    // Test wallet info serialization
    let json = serde_json::to_string_pretty(&wallet_info)?;
    assert!(json.contains("test_pubkey_123"));
    
    // Test key backup structure
    let key_backup = nockit::wallet::KeyBackup {
        public_key: "test_pubkey_123".to_string(),
        private_key_encrypted: "encrypted_private_key".to_string(),
        chain_code: "test_chain_code".to_string(),
        seed_phrase: vec!["word1".to_string(), "word2".to_string()],
        created_at: chrono::Utc::now(),
        backup_version: "1.0".to_string(),
    };
    
    let backup_json = serde_json::to_string_pretty(&key_backup)?;
    assert!(backup_json.contains("test_pubkey_123"));
    
    Ok(())
}

/// Test mining statistics
#[tokio::test]
async fn test_mining_statistics() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_dir = temp_dir.path();
    
    // Test mining stats structure
    let mining_stats = nockit::mining::MiningStats {
        start_time: chrono::Utc::now(),
        end_time: None,
        blocks_mined: 5,
        hash_rate: 1000.0,
        difficulty: 12345,
        rewards_earned: 500,
        uptime_seconds: 3600,
        errors: vec![],
    };
    
    // Test stats serialization
    let json = serde_json::to_string_pretty(&mining_stats)?;
    assert!(json.contains("blocks_mined"));
    assert!(json.contains("1000"));
    
    // Test mining process structure
    let mining_process = nockit::mining::MiningProcess {
        pid: Some(12345),
        pubkey: "test_mining_pubkey".to_string(),
        start_time: chrono::Utc::now(),
        status: nockit::mining::MiningStatus::Running,
        config_dir: config_dir.to_path_buf(),
    };
    
    let process_json = serde_json::to_string_pretty(&mining_process)?;
    assert!(process_json.contains("test_mining_pubkey"));
    
    Ok(())
}

/// Test network monitoring
#[tokio::test]
async fn test_network_monitoring() -> Result<()> {
    // Test network status structure
    let network_status = nockit::network::NetworkStatus {
        timestamp: chrono::Utc::now(),
        connected_peers: 3,
        total_connections: 5,
        network_id: Some("test_network".to_string()),
        local_peer_id: Some("local_peer_123".to_string()),
        listening_addresses: vec!["127.0.0.1:8080".to_string()],
        connectivity: nockit::network::ConnectivityStatus::Connected,
    };
    
    // Test status serialization
    let json = serde_json::to_string_pretty(&network_status)?;
    assert!(json.contains("connected_peers"));
    assert!(json.contains("test_network"));
    
    // Test peer info structure
    let peer_info = nockit::network::PeerInfo {
        peer_id: "peer_123".to_string(),
        address: "192.168.1.100:8080".to_string(),
        connection_time: chrono::Utc::now(),
        last_seen: chrono::Utc::now(),
        bytes_sent: 1024,
        bytes_received: 2048,
        latency_ms: Some(50),
        status: nockit::network::PeerStatus::Connected,
    };
    
    let peer_json = serde_json::to_string_pretty(&peer_info)?;
    assert!(peer_json.contains("peer_123"));
    
    Ok(())
}

/// Test system monitoring
#[tokio::test]
async fn test_system_monitoring() -> Result<()> {
    // Test system health structure
    let system_health = nockit::monitoring::SystemHealth {
        timestamp: chrono::Utc::now(),
        overall_status: nockit::monitoring::HealthStatus::Healthy,
        nockchain_status: nockit::monitoring::ServiceStatus {
            status: nockit::monitoring::HealthStatus::Healthy,
            message: "Service running".to_string(),
            last_check: chrono::Utc::now(),
            uptime_seconds: Some(3600),
        },
        mining_status: nockit::monitoring::ServiceStatus {
            status: nockit::monitoring::HealthStatus::Warning,
            message: "Mining not configured".to_string(),
            last_check: chrono::Utc::now(),
            uptime_seconds: None,
        },
        network_status: nockit::monitoring::ServiceStatus {
            status: nockit::monitoring::HealthStatus::Healthy,
            message: "Connected to 3 peers".to_string(),
            last_check: chrono::Utc::now(),
            uptime_seconds: None,
        },
        wallet_status: nockit::monitoring::ServiceStatus {
            status: nockit::monitoring::HealthStatus::Healthy,
            message: "Wallet configured".to_string(),
            last_check: chrono::Utc::now(),
            uptime_seconds: None,
        },
        system_metrics: nockit::monitoring::SystemMetrics {
            cpu_usage_percent: 25.5,
            memory_usage_percent: 60.2,
            disk_usage_percent: 45.8,
            network_rx_bytes: 1024000,
            network_tx_bytes: 512000,
            process_count: 150,
        },
    };
    
    // Test health serialization
    let json = serde_json::to_string_pretty(&system_health)?;
    assert!(json.contains("overall_status"));
    assert!(json.contains("system_metrics"));
    
    Ok(())
}

/// Test development utilities
#[tokio::test]
async fn test_dev_utilities() -> Result<()> {
    // Test system info collection
    let system_info = nockit::utils::get_system_info();
    assert!(!system_info.os.is_empty());
    assert!(!system_info.arch.is_empty());
    assert!(!system_info.nockit_version.is_empty());
    
    Ok(())
}

/// Test CLI commands
#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("nockit").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Nockchain Development Toolkit"));
}

#[test]
fn test_nocklog_help() {
    let mut cmd = Command::cargo_bin("nocklog").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Nockchain Log Management Tool"));
}

#[test]
fn test_nockmon_help() {
    let mut cmd = Command::cargo_bin("nockmon").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Nockchain System Monitor"));
}

#[test]
fn test_nocksetup_help() {
    let mut cmd = Command::cargo_bin("nocksetup").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Nockchain Development Environment Setup"));
}

/// Test error handling
#[tokio::test]
async fn test_error_handling() -> Result<()> {
    let temp_dir = TempDir::new()?;
    
    // Test loading non-existent config
    let non_existent = temp_dir.path().join("non_existent.toml");
    let result = nockit::config::NockitConfig::load(&non_existent);
    assert!(result.is_err());
    
    // Test invalid base58 decoding
    let result = nockit::crypto::PublicKey::from_base58("invalid_base58_!@#");
    assert!(result.is_err());
    
    // Test invalid hex decoding
    let result = nockit::crypto::Hash::from_hex("invalid_hex_string");
    assert!(result.is_err());
    
    Ok(())
}

/// Test concurrent operations
#[tokio::test]
async fn test_concurrent_operations() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_dir = temp_dir.path();
    
    // Test concurrent config operations
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let config_dir = config_dir.to_path_buf();
            tokio::spawn(async move {
                let mut config = nockit::config::NockitConfig::load_or_create(&config_dir).unwrap();
                config.set_mining_pubkey(format!("pubkey_{}", i));
                config.save(&config_dir.join(format!("config_{}.toml", i))).unwrap();
            })
        })
        .collect();
    
    // Wait for all tasks to complete
    for handle in handles {
        handle.await?;
    }
    
    // Verify all configs were created
    for i in 0..10 {
        let config_file = config_dir.join(format!("config_{}.toml", i));
        assert!(config_file.exists());
    }
    
    Ok(())
}

/// Test performance benchmarks
#[tokio::test]
async fn test_performance_benchmarks() -> Result<()> {
    use std::time::Instant;
    
    // Benchmark key generation
    let start = Instant::now();
    for _ in 0..100 {
        let _keypair = nockit::crypto::KeyPair::generate()?;
    }
    let key_gen_duration = start.elapsed();
    println!("Key generation (100 keys): {:?}", key_gen_duration);
    
    // Benchmark hashing
    let data = vec![0u8; 1024]; // 1KB of data
    let start = Instant::now();
    for _ in 0..1000 {
        let _hash = nockit::crypto::hash_data(&data);
    }
    let hash_duration = start.elapsed();
    println!("Hashing (1000 x 1KB): {:?}", hash_duration);
    
    // Benchmark config operations
    let temp_dir = TempDir::new()?;
    let config_dir = temp_dir.path();
    let start = Instant::now();
    for _ in 0..100 {
        let _config = nockit::config::NockitConfig::load_or_create(config_dir)?;
    }
    let config_duration = start.elapsed();
    println!("Config operations (100 loads): {:?}", config_duration);
    
    Ok(())
}

/// Test benchmarking module
#[tokio::test]
async fn test_benchmarking_module() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_dir = temp_dir.path();
    
    // Test performance profiler
    let mut profiler = nockit::bench::PerformanceProfiler::new();
    
    profiler.checkpoint("start");
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    
    profiler.checkpoint("crypto_work");
    for _ in 0..10 {
        let _keypair = nockit::crypto::KeyPair::generate()?;
    }
    
    profiler.checkpoint("end");
    profiler.sample_memory();
    
    let report = profiler.report();
    assert!(!report.checkpoint_durations.is_empty());
    assert!(report.total_duration.as_millis() > 0);
    
    // Test benchmark result creation
    let benchmark_result = nockit::bench::BenchmarkResult {
        name: "Test Benchmark".to_string(),
        description: "Test benchmark description".to_string(),
        timestamp: chrono::Utc::now(),
        duration: std::time::Duration::from_millis(100),
        iterations: 1000,
        operations_per_second: 10000.0,
        memory_usage_mb: 10.0,
        cpu_usage_percent: 50.0,
        metadata: std::collections::HashMap::from([
            ("test_key".to_string(), "test_value".to_string()),
        ]),
    };
    
    // Test benchmark suite creation
    let benchmark_suite = nockit::bench::BenchmarkSuite {
        name: "Test Suite".to_string(),
        version: "0.5.0".to_string(),
        timestamp: chrono::Utc::now(),
        results: vec![benchmark_result.clone()],
        summary: nockit::bench::BenchmarkSummary {
            total_tests: 1,
            total_duration: std::time::Duration::from_millis(100),
            average_ops_per_second: 10000.0,
            peak_memory_usage_mb: 10.0,
            average_cpu_usage_percent: 50.0,
        },
    };
    
    // Test serialization
    let json = serde_json::to_string_pretty(&benchmark_suite)?;
    assert!(json.contains("Test Suite"));
    assert!(json.contains("10000"));
    
    // Test benchmark comparison
    let comparison = nockit::bench::compare_benchmarks(&benchmark_suite, &benchmark_suite).await;
    assert_eq!(comparison.result_comparisons.len(), 1);
    assert_eq!(comparison.result_comparisons[0].performance_change_percent, 0.0);
    
    Ok(())
}

/// Test comprehensive example functionality
#[tokio::test]
async fn test_comprehensive_functionality() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_dir = temp_dir.path();
    
    // Test all major components work together
    
    // 1. Configuration
    let mut config = nockit::config::NockitConfig::load_or_create(config_dir)?;
    config.set_mining_pubkey("test_comprehensive_key".to_string());
    config.save(&config_dir.join("test_config.toml"))?;
    
    // 2. Cryptography
    let keypair = nockit::crypto::KeyPair::generate()?;
    let message = b"comprehensive test message";
    let signature = keypair.private_key.sign(message)?;
    assert!(keypair.public_key.verify(message, &signature)?);
    
    // 3. Wallet info
    let wallet_info = nockit::wallet::WalletInfo {
        public_key: keypair.public_key.to_base58(),
        address: "test_address".to_string(),
        balance: Some(1000),
        created_at: chrono::Utc::now(),
        last_updated: chrono::Utc::now(),
    };
    
    // 4. Mining stats
    let mining_stats = nockit::mining::MiningStats {
        start_time: chrono::Utc::now(),
        end_time: None,
        blocks_mined: 5,
        hash_rate: 1000.0,
        difficulty: 12345,
        rewards_earned: 250,
        uptime_seconds: 3600,
        errors: vec![],
    };
    
    // 5. Network status
    let network_status = nockit::network::NetworkStatus {
        timestamp: chrono::Utc::now(),
        connected_peers: 3,
        total_connections: 5,
        network_id: Some("test_network".to_string()),
        local_peer_id: Some("test_peer".to_string()),
        listening_addresses: vec!["127.0.0.1:8080".to_string()],
        connectivity: nockit::network::ConnectivityStatus::Connected,
    };
    
    // 6. System health
    let system_health = nockit::monitoring::SystemHealth {
        timestamp: chrono::Utc::now(),
        overall_status: nockit::monitoring::HealthStatus::Healthy,
        nockchain_status: nockit::monitoring::ServiceStatus {
            status: nockit::monitoring::HealthStatus::Healthy,
            message: "Test service running".to_string(),
            last_check: chrono::Utc::now(),
            uptime_seconds: Some(3600),
        },
        mining_status: nockit::monitoring::ServiceStatus {
            status: nockit::monitoring::HealthStatus::Healthy,
            message: "Test mining active".to_string(),
            last_check: chrono::Utc::now(),
            uptime_seconds: Some(1800),
        },
        network_status: nockit::monitoring::ServiceStatus {
            status: nockit::monitoring::HealthStatus::Healthy,
            message: "Test network connected".to_string(),
            last_check: chrono::Utc::now(),
            uptime_seconds: None,
        },
        wallet_status: nockit::monitoring::ServiceStatus {
            status: nockit::monitoring::HealthStatus::Healthy,
            message: "Test wallet configured".to_string(),
            last_check: chrono::Utc::now(),
            uptime_seconds: None,
        },
        system_metrics: nockit::monitoring::SystemMetrics {
            cpu_usage_percent: 25.0,
            memory_usage_percent: 50.0,
            disk_usage_percent: 30.0,
            network_rx_bytes: 1000000,
            network_tx_bytes: 500000,
            process_count: 100,
        },
    };
    
    // Test all components serialize correctly
    let config_json = serde_json::to_string_pretty(&config)?;
    let wallet_json = serde_json::to_string_pretty(&wallet_info)?;
    let mining_json = serde_json::to_string_pretty(&mining_stats)?;
    let network_json = serde_json::to_string_pretty(&network_status)?;
    let health_json = serde_json::to_string_pretty(&system_health)?;
    
    assert!(config_json.contains("test_comprehensive_key"));
    assert!(wallet_json.contains("test_address"));
    assert!(mining_json.contains("blocks_mined"));
    assert!(network_json.contains("test_network"));
    assert!(health_json.contains("overall_status"));
    
    // Test system info
    let system_info = nockit::utils::get_system_info();
    assert!(!system_info.os.is_empty());
    assert!(!system_info.nockit_version.is_empty());
    
    println!("✅ Comprehensive functionality test completed successfully");
    
    Ok(())
} 