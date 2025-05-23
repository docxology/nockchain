//! Wallet management functionality for nockit
//! 
//! Provides wallet operations including key generation, backup, restore, and status checking.
//! Uses real nockchain-wallet commands with comprehensive logging and parsing.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::fs;
use tracing::{debug, error, info, warn};

/// Wallet information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    pub public_key: String,
    pub private_key: Option<String>,
    pub address: String,
    pub chain_code: Option<String>,
    pub seed_phrase: Option<Vec<String>>,
    pub balance: Option<u64>,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub wallet_type: String, // "real" or "demo"
}

/// Key backup structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBackup {
    pub public_key: String,
    pub private_key_encrypted: String,
    pub chain_code: String,
    pub seed_phrase: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub backup_version: String,
}

/// Real wallet command execution result
#[derive(Debug, Clone)]
pub struct WalletCommandResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

/// Generate new wallet keys using real nockchain-wallet keygen
pub async fn generate_keys(output: Option<&Path>, config_dir: &Path) -> Result<()> {
    info!("🔑 Generating new wallet keys using nockchain-wallet keygen...");
    
    // Execute real nockchain-wallet keygen command
    let result = execute_wallet_command(&["keygen"], None).await?;
    
    if !result.success {
        error!("Key generation failed: {}", result.stderr);
        anyhow::bail!("nockchain-wallet keygen failed: {}", result.stderr);
    }
    
    info!("✅ Real cryptographic keys generated successfully");
    debug!("Keygen output: {}", result.stdout);
    
    // Parse the real output to extract key information
    let key_info = parse_keygen_output(&result.stdout)?;
    info!("📝 Parsed public key: {}", key_info.public_key);
    
    // Save wallet information to nockit config
    let wallet_info = WalletInfo {
        public_key: key_info.public_key.clone(),
        private_key: key_info.private_key.clone(),
        address: derive_address_from_pubkey(&key_info.public_key),
        chain_code: key_info.chain_code.clone(),
        seed_phrase: key_info.seed_phrase.clone(),
        balance: None,
        created_at: Utc::now(),
        last_updated: Utc::now(),
        wallet_type: "real".to_string(),
    };
    
    save_wallet_info(&wallet_info, config_dir).await?;
    
    // Update nockit configuration with the new public key
    let mut config = crate::config::NockitConfig::load_or_create(config_dir)?;
    config.set_mining_pubkey(key_info.public_key.clone());
    config.save(&config_dir.join("config.toml"))?;
    
    if let Some(output_path) = output {
        save_key_info_to_file(&key_info, output_path).await?;
        info!("Key information saved to: {}", output_path.display());
    }
    
    println!("✅ New wallet keys generated successfully!");
    println!("📝 Public key: {}", key_info.public_key);
    println!("📍 Address: {}", wallet_info.address);
    if let Some(ref seed) = key_info.seed_phrase {
        println!("🌱 Seed phrase: {} words generated", seed.len());
    }
    println!("⚠️  Please backup your seed phrase and private key securely!");
    
    Ok(())
}

/// Check wallet status and balance using real nockchain-wallet commands
pub async fn check_status(pubkey: Option<&str>, config_dir: &Path, socket_path: Option<&Path>) -> Result<()> {
    let config = crate::config::NockitConfig::load_or_create(config_dir)?;
    
    let target_pubkey = if let Some(pk) = pubkey {
        pk.to_string()
    } else if let Some(pk) = config.get_mining_pubkey() {
        pk
    } else {
        anyhow::bail!("No public key specified. Use --pubkey or set MINING_PUBKEY in config.");
    };
    
    info!("🔍 Checking wallet status for: {}", target_pubkey);
    
    // Load saved wallet info if available
    if let Ok(wallet_info) = load_wallet_info(config_dir).await {
        if wallet_info.public_key == target_pubkey {
            print_wallet_status(&wallet_info);
        }
    }
    
    // Show master public key using real command
    match show_master_pubkey().await {
        Ok(result) => {
            if result.success {
                info!("✅ Master public key verified");
                println!("🔑 Master public key: {}", result.stdout.trim());
            } else {
                warn!("Could not retrieve master public key: {}", result.stderr);
            }
        }
        Err(e) => {
            warn!("Master public key check failed: {}", e);
        }
    }
    
    // Try to get current balance and notes using real commands
    if let Some(socket) = socket_path {
        match list_notes(Some(socket)).await {
            Ok(result) => {
                if result.success {
                    info!("✅ Successfully retrieved wallet notes");
                    println!("\n💰 Wallet Notes (UTXOs):");
                    println!("{}", result.stdout);
                } else {
                    warn!("Could not retrieve notes: {}", result.stderr);
                }
            }
            Err(e) => {
                warn!("Notes retrieval failed: {}", e);
            }
        }
        
        // Try to get notes by specific pubkey
        match list_notes_by_pubkey(&target_pubkey, Some(socket)).await {
            Ok(result) => {
                if result.success {
                    info!("✅ Successfully retrieved notes for pubkey");
                    println!("\n📋 Notes for {}: ", target_pubkey);
                    println!("{}", result.stdout);
                } else {
                    debug!("No specific notes found for pubkey: {}", result.stderr);
                }
            }
            Err(e) => {
                debug!("Pubkey-specific notes query failed: {}", e);
            }
        }
    } else {
        println!("ℹ️  To check balance and notes, provide --nockchain-socket path");
    }
    
    // Show seed phrase if available
    match show_seedphrase().await {
        Ok(result) => {
            if result.success {
                println!("\n🌱 Seed phrase: {}", result.stdout.trim());
            }
        }
        Err(_) => {
            debug!("Seed phrase not available or protected");
        }
    }
    
    Ok(())
}

/// Backup wallet keys using real nockchain-wallet export-keys
pub async fn backup_keys(output: Option<&Path>, config_dir: &Path) -> Result<()> {
    info!("💾 Creating wallet backup using nockchain-wallet export-keys...");
    
    let config = crate::config::NockitConfig::load_or_create(config_dir)?;
    let backup_dir = output.map(|p| p.to_path_buf())
        .unwrap_or_else(|| config_dir.join(&config.wallet.backup_dir));
    
    fs::create_dir_all(&backup_dir).await
        .with_context(|| format!("Failed to create backup directory: {}", backup_dir.display()))?;
    
    // Use real nockchain-wallet export-keys command
    let result = execute_wallet_command(&["export-keys"], None).await?;
    
    if !result.success {
        error!("Key export failed: {}", result.stderr);
        anyhow::bail!("nockchain-wallet export-keys failed: {}", result.stderr);
    }
    
    info!("✅ Keys exported successfully");
    
    // The export-keys command creates a keys.export file in current directory
    // Move it to our backup directory with timestamp
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let source_file = PathBuf::from("keys.export");
    let backup_file = backup_dir.join(format!("wallet_backup_{}.export", timestamp));
    
    if source_file.exists() {
        fs::rename(&source_file, &backup_file).await
            .context("Failed to move backup file")?;
        info!("Backup file moved to: {}", backup_file.display());
    } else {
        warn!("keys.export file not found, backup may have failed");
    }
    
    println!("✅ Wallet backup created: {}", backup_file.display());
    
    // Create additional metadata file
    let metadata_file = backup_dir.join(format!("wallet_backup_{}.json", timestamp));
    let backup_metadata = serde_json::json!({
        "backup_created": Utc::now(),
        "backup_file": backup_file.file_name(),
        "nockit_version": crate::VERSION,
        "backup_type": "nockchain_wallet_export",
        "command_used": "nockchain-wallet export-keys"
    });
    
    fs::write(&metadata_file, serde_json::to_string_pretty(&backup_metadata)?)
        .await
        .context("Failed to write backup metadata")?;
    
    println!("📋 Backup metadata saved: {}", metadata_file.display());
    println!("⚠️  Store this backup in a secure location!");
    
    Ok(())
}

/// Restore wallet from backup using real nockchain-wallet import-keys
pub async fn restore_keys(input: &Path, config_dir: &Path) -> Result<()> {
    if !input.exists() {
        anyhow::bail!("Backup file not found: {}", input.display());
    }
    
    info!("🔄 Restoring wallet from backup: {}", input.display());
    
    // Use real nockchain-wallet import-keys command
    let result = execute_wallet_command(&["import-keys", "--input", input.to_str().unwrap()], None).await?;
    
    if !result.success {
        error!("Key import failed: {}", result.stderr);
        anyhow::bail!("nockchain-wallet import-keys failed: {}", result.stderr);
    }
    
    info!("✅ Keys imported successfully");
    println!("Restore output:");
    println!("{}", result.stdout);
    
    // Try to get the master public key after import
    match show_master_pubkey().await {
        Ok(pubkey_result) => {
            if pubkey_result.success {
                let public_key = pubkey_result.stdout.trim().to_string();
                
                // Update nockit config with restored key
                let mut config = crate::config::NockitConfig::load_or_create(config_dir)?;
                config.set_mining_pubkey(public_key.clone());
                config.save(&config_dir.join("config.toml"))?;
                
                // Create wallet info for restored wallet
                let wallet_info = WalletInfo {
                    public_key: public_key.clone(),
                    private_key: None, // Don't store private key in config
                    address: derive_address_from_pubkey(&public_key),
                    chain_code: None,
                    seed_phrase: None,
                    balance: None,
                    created_at: Utc::now(), // Restoration time
                    last_updated: Utc::now(),
                    wallet_type: "real_restored".to_string(),
                };
                
                save_wallet_info(&wallet_info, config_dir).await?;
                info!("Updated nockit config with restored public key: {}", public_key);
            }
        }
        Err(e) => {
            warn!("Could not retrieve public key after restore: {}", e);
        }
    }
    
    println!("✅ Wallet restored successfully!");
    
    Ok(())
}

// Real nockchain-wallet command execution functions

/// Execute a nockchain-wallet command with proper error handling and logging
async fn execute_wallet_command(args: &[&str], socket_path: Option<&Path>) -> Result<WalletCommandResult> {
    let mut cmd = Command::new("nockchain-wallet");
    
    // Add socket path if provided
    if let Some(socket) = socket_path {
        cmd.args(&["--nockchain-socket", socket.to_str().unwrap()]);
    }
    
    cmd.args(args);
    
    debug!("Executing: nockchain-wallet {}", args.join(" "));
    
    let output = cmd.output()
        .with_context(|| format!("Failed to execute nockchain-wallet {}. Make sure nockchain-wallet is installed and in PATH.", args.join(" ")))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let success = output.status.success();
    let exit_code = output.status.code();
    
    if !success {
        debug!("Command failed with exit code: {:?}", exit_code);
        debug!("Stderr: {}", stderr);
    }
    
    Ok(WalletCommandResult {
        success,
        stdout,
        stderr,
        exit_code,
    })
}

/// Show master public key using real nockchain-wallet command
pub async fn show_master_pubkey() -> Result<WalletCommandResult> {
    execute_wallet_command(&["show-master-pubkey"], None).await
}

/// Show seed phrase using real nockchain-wallet command
pub async fn show_seedphrase() -> Result<WalletCommandResult> {
    execute_wallet_command(&["show-seedphrase"], None).await
}

/// Show master private key using real nockchain-wallet command
pub async fn show_master_privkey() -> Result<WalletCommandResult> {
    execute_wallet_command(&["show-master-privkey"], None).await
}

/// List all notes using real nockchain-wallet command
pub async fn list_notes(socket_path: Option<&Path>) -> Result<WalletCommandResult> {
    execute_wallet_command(&["list-notes"], socket_path).await
}

/// List notes by public key using real nockchain-wallet command
pub async fn list_notes_by_pubkey(pubkey: &str, socket_path: Option<&Path>) -> Result<WalletCommandResult> {
    execute_wallet_command(&["list-notes-by-pubkey", "--pubkey", pubkey], socket_path).await
}

/// Update wallet balance using real nockchain-wallet command
pub async fn update_balance(socket_path: Option<&Path>) -> Result<WalletCommandResult> {
    execute_wallet_command(&["update-balance"], socket_path).await
}

/// Derive child key using real nockchain-wallet command
pub async fn derive_child_key(key_type: &str, index: u64) -> Result<WalletCommandResult> {
    execute_wallet_command(&["derive-child", "--key-type", key_type, "--index", &index.to_string()], None).await
}

/// Generate master private key from seed phrase using real nockchain-wallet command
pub async fn gen_master_privkey(seedphrase: &str) -> Result<WalletCommandResult> {
    execute_wallet_command(&["gen-master-privkey", "--seedphrase", seedphrase], None).await
}

/// Generate master public key from private key using real nockchain-wallet command
pub async fn gen_master_pubkey(master_privkey: &str) -> Result<WalletCommandResult> {
    execute_wallet_command(&["gen-master-pubkey", "--master-privkey", master_privkey], None).await
}

/// Scan blockchain using real nockchain-wallet command
pub async fn scan_blockchain(master_pubkey: &str, search_depth: Option<u64>, include_timelocks: bool, include_multisig: bool, socket_path: Option<&Path>) -> Result<WalletCommandResult> {
    let mut args = vec!["scan", "--master-pubkey", master_pubkey];
    
    let depth_string;
    if let Some(depth) = search_depth {
        depth_string = depth.to_string();
        args.extend(&["--search-depth", &depth_string]);
    }
    
    if include_timelocks {
        args.push("--include-timelocks");
    }
    
    if include_multisig {
        args.push("--include-multisig");
    }
    
    execute_wallet_command(&args, socket_path).await
}

// Helper functions

#[derive(Debug)]
struct KeyInfo {
    public_key: String,
    private_key: Option<String>,
    chain_code: Option<String>,
    seed_phrase: Option<Vec<String>>,
}

/// Parse real nockchain-wallet keygen output
fn parse_keygen_output(output: &str) -> Result<KeyInfo> {
    let mut public_key = None;
    let mut private_key = None;
    let mut chain_code = None;
    let mut seed_phrase = None;
    
    debug!("Parsing keygen output: {}", output);
    
    let lines: Vec<&str> = output.lines().collect();
    debug!("Total lines to parse: {}", lines.len());
    
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();
        debug!("Line {}: '{}'", i, line);
        
        // Look for "New Public Key" followed by quoted key (possibly multi-line)
        if line == "New Public Key" && i + 1 < lines.len() {
            debug!("Found 'New Public Key' at line {}, checking next line", i);
            if let Some(key) = extract_multiline_quoted_value(&lines, i + 1) {
                debug!("Extracted public key: {}", key);
                public_key = Some(key);
            } else {
                debug!("Failed to extract quoted value from line {}: '{}'", i + 1, lines[i + 1]);
            }
        }
        // Look for "New Private Key" followed by quoted key
        else if line == "New Private Key" && i + 1 < lines.len() {
            debug!("Found 'New Private Key' at line {}, checking next line", i);
            if let Some(key) = extract_multiline_quoted_value(&lines, i + 1) {
                debug!("Extracted private key: {}", key);
                private_key = Some(key);
            } else {
                debug!("Failed to extract quoted value from line {}: '{}'", i + 1, lines[i + 1]);
            }
        }
        // Look for "Chain Code" followed by quoted code
        else if line == "Chain Code" && i + 1 < lines.len() {
            debug!("Found 'Chain Code' at line {}, checking next line", i);
            if let Some(code) = extract_multiline_quoted_value(&lines, i + 1) {
                debug!("Extracted chain code: {}", code);
                chain_code = Some(code);
            } else {
                debug!("Failed to extract quoted value from line {}: '{}'", i + 1, lines[i + 1]);
            }
        }
        // Look for "Seed Phrase" followed by quoted phrase (possibly multi-line)
        else if line == "Seed Phrase" && i + 1 < lines.len() {
            debug!("Found 'Seed Phrase' at line {}, checking next line", i);
            if let Some(phrase_str) = extract_multiline_quoted_value(&lines, i + 1) {
                let words: Vec<String> = phrase_str
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                
                if words.len() >= 12 { // Valid BIP39 seed phrase
                    debug!("Extracted seed phrase: {} words", words.len());
                    seed_phrase = Some(words);
                } else {
                    debug!("Invalid seed phrase length: {} words", words.len());
                }
            } else {
                debug!("Failed to extract quoted seed phrase from line {}: '{}'", i + 1, lines[i + 1]);
            }
        }
        
        i += 1;
    }
    
    debug!("Parsing results - public_key: {:?}, private_key: {:?}, chain_code: {:?}, seed_phrase: {:?}", 
           public_key.as_ref().map(|k| &k[..20.min(k.len())]), 
           private_key.as_ref().map(|k| &k[..20.min(k.len())]), 
           chain_code.as_ref().map(|k| &k[..20.min(k.len())]), 
           seed_phrase.as_ref().map(|s| s.len()));
    
    let public_key = public_key.context("Could not find public key in nockchain-wallet output")?;
    
    info!("Successfully parsed keygen output - public key: {}", public_key);
    
    Ok(KeyInfo {
        public_key,
        private_key,
        chain_code,
        seed_phrase,
    })
}

/// Extract quoted value that may span multiple lines
fn extract_multiline_quoted_value(lines: &[&str], start_idx: usize) -> Option<String> {
    if start_idx >= lines.len() {
        return None;
    }
    
    let first_line = lines[start_idx].trim();
    
    // Handle single quotes
    if first_line.starts_with('\'') {
        if first_line.ends_with('\'') && first_line.len() > 2 {
            // Single line case
            return Some(first_line[1..first_line.len()-1].to_string());
        } else {
            // Multi-line case - collect until closing quote
            let mut result = String::new();
            result.push_str(&first_line[1..]); // Remove opening quote
            
            for i in (start_idx + 1)..lines.len() {
                let line = lines[i].trim();
                if line.ends_with('\'') {
                    // Found closing quote
                    result.push(' ');
                    result.push_str(&line[..line.len()-1]); // Remove closing quote
                    return Some(result);
                } else {
                    result.push(' ');
                    result.push_str(line);
                }
            }
        }
    }
    
    // Handle double quotes
    if first_line.starts_with('"') {
        if first_line.ends_with('"') && first_line.len() > 2 {
            // Single line case
            return Some(first_line[1..first_line.len()-1].to_string());
        } else {
            // Multi-line case - collect until closing quote
            let mut result = String::new();
            result.push_str(&first_line[1..]); // Remove opening quote
            
            for i in (start_idx + 1)..lines.len() {
                let line = lines[i].trim();
                if line.ends_with('"') {
                    // Found closing quote
                    result.push_str(&line[..line.len()-1]); // Remove closing quote
                    return Some(result);
                } else {
                    result.push_str(line);
                }
            }
        }
    }
    
    None
}

/// Derive address from public key (simplified implementation)
fn derive_address_from_pubkey(pubkey: &str) -> String {
    // For now, use a simple derivation - in production this would use proper address derivation
    format!("addr_{}", &pubkey[..8])
}

async fn save_wallet_info(wallet_info: &WalletInfo, config_dir: &Path) -> Result<()> {
    let wallet_file = config_dir.join("wallet_info.json");
    let json = serde_json::to_string_pretty(wallet_info)?;
    fs::write(wallet_file, json).await?;
    info!("Wallet info saved to: {}", config_dir.join("wallet_info.json").display());
    Ok(())
}

async fn load_wallet_info(config_dir: &Path) -> Result<WalletInfo> {
    let wallet_file = config_dir.join("wallet_info.json");
    let content = fs::read_to_string(wallet_file).await?;
    let wallet_info: WalletInfo = serde_json::from_str(&content)?;
    Ok(wallet_info)
}

async fn save_key_info_to_file(key_info: &KeyInfo, output_path: &Path) -> Result<()> {
    let info = serde_json::json!({
        "public_key": key_info.public_key,
        "private_key": key_info.private_key,
        "chain_code": key_info.chain_code,
        "seed_phrase": key_info.seed_phrase,
        "generated_at": Utc::now(),
        "nockit_version": crate::VERSION,
        "wallet_type": "real"
    });
    
    let json = serde_json::to_string_pretty(&info)?;
    fs::write(output_path, json).await?;
    Ok(())
}

fn print_wallet_status(wallet_info: &WalletInfo) {
    println!("\n=== Wallet Status ===");
    println!("🔑 Public key: {}", wallet_info.public_key);
    println!("📍 Address: {}", wallet_info.address);
    println!("🏷️  Wallet type: {}", wallet_info.wallet_type);
    if let Some(balance) = wallet_info.balance {
        println!("💰 Cached balance: {} units", balance);
    }
    if let Some(ref seed) = wallet_info.seed_phrase {
        println!("🌱 Seed phrase: {} words", seed.len());
    }
    println!("📅 Created: {}", wallet_info.created_at);
    println!("🔄 Last updated: {}", wallet_info.last_updated);
} 