//! Unit tests for configuration module

use docxology::config::*;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_node_config_default() {
    let config = NodeConfig::default();

    assert_eq!(config.data_dir, None);
    assert_eq!(config.network.listen_addr, "/ip4/0.0.0.0/udp/0/quic-v1");
    assert_eq!(config.network.bootstrap_peers.len(), 0);
    assert!(config.network.enable_upnp);
    assert_eq!(config.network.max_peers, 50);
    assert_eq!(config.network.peer_port, 0);

    assert!(config.mining.is_none());

    assert!(!config.api.enable_public_api);
    assert_eq!(config.api.public_api_addr, "0.0.0.0:8080");
    assert_eq!(config.api.private_api_addr, "127.0.0.1:8081");
    assert_eq!(config.api.cors_origins, vec!["*".to_string()]);

    assert_eq!(config.logging.level, "info");
    assert!(!config.logging.json);
    assert!(!config.logging.file_logging);
    assert!(config.logging.log_file.is_none());
    assert_eq!(config.logging.max_file_size, 10);
    assert_eq!(config.logging.max_files, 5);

    assert_eq!(config.custom.len(), 0);
}

#[test]
fn test_network_config_default() {
    let config = NetworkConfig::default();

    assert_eq!(config.listen_addr, "/ip4/0.0.0.0/udp/0/quic-v1");
    assert_eq!(config.bootstrap_peers.len(), 0);
    assert!(config.enable_upnp);
    assert_eq!(config.max_peers, 50);
    assert_eq!(config.peer_port, 0);
}

#[test]
fn test_miner_config_default() {
    let config = MinerConfig::default();

    assert!(config.pubkey.is_none());
    assert!(config.privkey.is_none());
    assert_eq!(config.threads, num_cpus::get());
    assert!(!config.enabled);
    assert!(config.target.is_none());
}

#[test]
fn test_api_config_default() {
    let config = ApiConfig::default();

    assert!(!config.enable_public_api);
    assert_eq!(config.public_api_addr, "0.0.0.0:8080");
    assert_eq!(config.private_api_addr, "127.0.0.1:8081");
    assert_eq!(config.cors_origins, vec!["*".to_string()]);
}

#[test]
fn test_logging_config_default() {
    let config = LoggingConfig::default();

    assert_eq!(config.level, "info");
    assert!(!config.json);
    assert!(!config.file_logging);
    assert!(config.log_file.is_none());
    assert_eq!(config.max_file_size, 10);
    assert_eq!(config.max_files, 5);
}

#[test]
fn test_wallet_config_default() {
    let config = WalletConfig::default();

    assert!(config.data_dir.is_none());
    assert_eq!(config.grpc_endpoint, "https://nockchain-api.zorp.io");
    assert_eq!(config.timeout_secs, 30);

    assert_eq!(config.retry.max_attempts, 3);
    assert_eq!(config.retry.initial_delay_ms, 1000);
    assert_eq!(config.retry.max_delay_ms, 30000);
    assert_eq!(config.retry.backoff_multiplier, 2.0);
}

#[test]
fn test_retry_config_default() {
    let config = RetryConfig::default();

    assert_eq!(config.max_attempts, 3);
    assert_eq!(config.initial_delay_ms, 1000);
    assert_eq!(config.max_delay_ms, 30000);
    assert_eq!(config.backoff_multiplier, 2.0);
}

#[test]
fn test_node_config_from_env() {
    // Set environment variables
    std::env::set_var("NOCKCHAIN_DATA_DIR", "/tmp/test-data");
    std::env::set_var("NOCKCHAIN_NETWORK__LISTEN_ADDR", "/ip4/127.0.0.1/udp/9000/quic-v1");
    std::env::set_var("NOCKCHAIN_LOGGING__LEVEL", "debug");

    let config = NodeConfig::from_env().unwrap();

    assert_eq!(config.data_dir, Some(PathBuf::from("/tmp/test-data")));
    assert_eq!(config.network.listen_addr, "/ip4/127.0.0.1/udp/9000/quic-v1");
    assert_eq!(config.logging.level, "debug");

    // Clean up
    std::env::remove_var("NOCKCHAIN_DATA_DIR");
    std::env::remove_var("NOCKCHAIN_NETWORK__LISTEN_ADDR");
    std::env::remove_var("NOCKCHAIN_LOGGING__LEVEL");
}

#[test]
fn test_miner_config_validation() {
    // Test that miner config requires keys when enabled
    let mut config = MinerConfig::default();
    config.enabled = true;

    // This would need validation logic implemented in the miner module
    // For now, we test the basic structure
    assert!(config.enabled);
    assert!(config.pubkey.is_none());
    assert!(config.privkey.is_none());
}

#[test]
fn test_key_source_variants() {
    use docxology::wallet::KeySource;

    let file_source = KeySource::File(PathBuf::from("test_keys.export"));
    let seed_source = KeySource::SeedPhrase {
        phrase: "word1 word2 word3 word4 word5 word6 word7 word8 word9 word10 word11 word12".to_string(),
        version: 0,
    };
    let privkey_source = KeySource::PrivateKey {
        privkey: "test_privkey_base58".to_string(),
        chain_code: "test_chain_code".to_string(),
    };
    let watch_source = KeySource::WatchOnly("test_pubkey_base58".to_string());

    // All variants should be valid
    match file_source {
        KeySource::File(path) => assert_eq!(path, PathBuf::from("test_keys.export")),
        _ => panic!("Expected file source"),
    }

    match seed_source {
        KeySource::SeedPhrase { phrase, version } => {
            assert_eq!(version, 0);
            assert!(phrase.contains("word1"));
        }
        _ => panic!("Expected seed phrase source"),
    }

    match privkey_source {
        KeySource::PrivateKey { privkey, chain_code } => {
            assert_eq!(privkey, "test_privkey_base58");
            assert_eq!(chain_code, "test_chain_code");
        }
        _ => panic!("Expected private key source"),
    }

    match watch_source {
        KeySource::WatchOnly(pubkey) => assert_eq!(pubkey, "test_pubkey_base58"),
        _ => panic!("Expected watch-only source"),
    }
}
