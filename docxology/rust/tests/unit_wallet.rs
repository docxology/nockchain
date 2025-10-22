//! Unit tests for wallet module

use docxology::{wallet::{WalletManager, WalletConfig, KeyPair, KeySource, BalanceInfo, Transaction, SignedTransaction, TransactionOutput}, config::WalletConfig as ConfigWalletConfig};
use std::path::PathBuf;

#[tokio::test]
async fn test_wallet_creation() {
    let config = WalletConfig::default();
    let wallet = WalletManager::new(config).await;

    // This test will likely fail initially since we haven't fully implemented
    // the wallet functionality - this is expected in TDD
    assert!(wallet.is_ok());
}

#[tokio::test]
async fn test_wallet_keygen() {
    let config = WalletConfig::default();
    let wallet = WalletManager::new(config).await.unwrap();

    let keypair = wallet.keygen().await;

    // This test will fail initially - expected in TDD
    assert!(keypair.is_ok());

    let keypair = keypair.unwrap();
    assert!(!keypair.public_key.is_empty());
    assert!(!keypair.private_key.is_empty());
}

#[test]
fn test_keypair_creation() {
    let keypair = KeyPair {
        public_key: "test_public_key".to_string(),
        private_key: "test_private_key".to_string(),
        seed_phrase: Some("test seed phrase".to_string()),
    };

    assert_eq!(keypair.public_key, "test_public_key");
    assert_eq!(keypair.private_key, "test_private_key");
    assert_eq!(keypair.seed_phrase, Some("test seed phrase".to_string()));
}

#[test]
fn test_key_source_file() {
    let source = KeySource::File(PathBuf::from("test_keys.export"));

    match source {
        KeySource::File(path) => assert_eq!(path, PathBuf::from("test_keys.export")),
        _ => panic!("Expected file source"),
    }
}

#[test]
fn test_key_source_seed_phrase() {
    let source = KeySource::SeedPhrase {
        phrase: "word1 word2 word3 word4 word5 word6 word7 word8 word9 word10 word11 word12".to_string(),
        version: 0,
    };

    match source {
        KeySource::SeedPhrase { phrase, version } => {
            assert_eq!(version, 0);
            assert!(phrase.contains("word1"));
        }
        _ => panic!("Expected seed phrase source"),
    }
}

#[test]
fn test_balance_info() {
    let balance = BalanceInfo {
        address: "test_address".to_string(),
        balance: 1000,
        notes: vec![],
    };

    assert_eq!(balance.address, "test_address");
    assert_eq!(balance.balance, 1000);
    assert_eq!(balance.notes.len(), 0);
}

#[test]
fn test_transaction_creation() {
    let tx = Transaction {
        id: "test_tx_id".to_string(),
        inputs: vec![],
        outputs: vec![TransactionOutput {
            recipient: "test_recipient".to_string(),
            amount: 1000,
        }],
        fee: 10,
    };

    assert_eq!(tx.id, "test_tx_id");
    assert_eq!(tx.inputs.len(), 0);
    assert_eq!(tx.outputs.len(), 1);
    assert_eq!(tx.outputs[0].recipient, "test_recipient");
    assert_eq!(tx.outputs[0].amount, 1000);
    assert_eq!(tx.fee, 10);
}

#[test]
fn test_signed_transaction() {
    let tx = Transaction {
        id: "test_tx_id".to_string(),
        inputs: vec![],
        outputs: vec![],
        fee: 0,
    };

    let signed_tx = SignedTransaction {
        transaction: tx,
        signature: "test_signature".to_string(),
    };

    assert_eq!(signed_tx.transaction.id, "test_tx_id");
    assert_eq!(signed_tx.signature, "test_signature");
}

#[tokio::test]
async fn test_wallet_import_keys() {
    let config = WalletConfig::default();
    let wallet = WalletManager::new(config).await.unwrap();

    let source = KeySource::WatchOnly("test_pubkey".to_string());
    let result = wallet.import_keys(source).await;

    // This test will fail initially - expected in TDD
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_wallet_get_balance() {
    let config = WalletConfig::default();
    let wallet = WalletManager::new(config).await.unwrap();

    let balance = wallet.get_balance("test_address").await;

    // This test will fail initially - expected in TDD
    assert!(balance.is_ok());
}

#[tokio::test]
async fn test_wallet_build_transaction() {
    let config = WalletConfig::default();
    let wallet = WalletManager::new(config).await.unwrap();

    let tx = wallet.build_transaction("test_recipient", 1000).await;

    // This test will fail initially - expected in TDD
    assert!(tx.is_ok());

    let tx = tx.unwrap();
    assert_eq!(tx.outputs.len(), 1);
    assert_eq!(tx.outputs[0].recipient, "test_recipient");
    assert_eq!(tx.outputs[0].amount, 1000);
}

#[tokio::test]
async fn test_wallet_sign_transaction() {
    let config = WalletConfig::default();
    let wallet = WalletManager::new(config).await.unwrap();

    let tx = Transaction {
        id: "test_tx_id".to_string(),
        inputs: vec![],
        outputs: vec![],
        fee: 0,
    };

    let signed_tx = wallet.sign_transaction(&tx).await;

    // This test will fail initially - expected in TDD
    assert!(signed_tx.is_ok());

    let signed_tx = signed_tx.unwrap();
    assert_eq!(signed_tx.transaction.id, "test_tx_id");
    assert!(!signed_tx.signature.is_empty());
}
