use clap::{Parser, Subcommand};
use nockit::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "nockit")]
#[command(about = "Nockchain Development Toolkit")]
#[command(version = nockit::VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Configuration directory
    #[arg(long, default_value = ".nockit")]
    config_dir: PathBuf,
    
    /// Path to nockchain socket for blockchain operations
    #[arg(long)]
    nockchain_socket: Option<PathBuf>,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Setup and initialize nockchain environment
    Setup {
        /// Force overwrite existing configuration
        #[arg(long)]
        force: bool,
        /// Skip interactive prompts
        #[arg(long)]
        non_interactive: bool,
    },
    /// Wallet management operations
    Wallet {
        #[command(subcommand)]
        action: WalletCommands,
    },
    /// Mining operations and monitoring
    Mining {
        #[command(subcommand)]
        action: MiningCommands,
    },
    /// Network monitoring and diagnostics
    Network {
        #[command(subcommand)]
        action: NetworkCommands,
    },
    /// Log management and analysis
    Logs {
        #[command(subcommand)]
        action: LogCommands,
    },
    /// System monitoring and health checks
    Monitor {
        /// Update interval in seconds
        #[arg(short, long, default_value = "5")]
        interval: u64,
        /// Output format (json, table, compact)
        #[arg(short, long, default_value = "table")]
        format: String,
    },
    /// Development utilities
    Dev {
        #[command(subcommand)]
        action: DevCommands,
    },
    /// Performance benchmarking
    Bench {
        #[command(subcommand)]
        action: BenchCommands,
    },
}

#[derive(Subcommand)]
enum WalletCommands {
    /// Generate new wallet keys using real nockchain-wallet
    Keygen {
        /// Output file for keys
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Check wallet balance and status using real nockchain-wallet
    Status {
        /// Public key to check
        #[arg(short, long)]
        pubkey: Option<String>,
    },
    /// Backup wallet keys using real nockchain-wallet export-keys
    Backup {
        /// Output directory for backup
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Restore wallet from backup using real nockchain-wallet import-keys
    Restore {
        /// Backup file to restore from
        #[arg(short, long)]
        input: PathBuf,
    },
    /// Show master public key using real nockchain-wallet
    ShowPubkey,
    /// Show seed phrase using real nockchain-wallet
    ShowSeed,
    /// List wallet notes (UTXOs) using real nockchain-wallet
    ListNotes {
        /// Filter by specific public key
        #[arg(short, long)]
        pubkey: Option<String>,
    },
    /// Update wallet balance using real nockchain-wallet
    UpdateBalance,
    /// Derive child key using real nockchain-wallet
    DeriveChild {
        /// Key type (pub or priv)
        #[arg(short, long, default_value = "pub")]
        key_type: String,
        /// Child key index (0-255)
        #[arg(short, long)]
        index: u64,
    },
    /// Scan blockchain for wallet transactions
    Scan {
        /// Master public key to scan for
        #[arg(short, long)]
        master_pubkey: String,
        /// Search depth (default: 100)
        #[arg(short, long, default_value = "100")]
        search_depth: u64,
        /// Include timelocks in scan
        #[arg(long)]
        include_timelocks: bool,
        /// Include multisig in scan
        #[arg(long)]
        include_multisig: bool,
    },
}

#[derive(Subcommand)]
enum MiningCommands {
    /// Start mining with monitoring
    Start {
        /// Mining public key
        #[arg(short, long)]
        pubkey: String,
        /// Mining difficulty target
        #[arg(short, long)]
        difficulty: Option<u64>,
    },
    /// Stop mining operations
    Stop,
    /// Check mining status and statistics
    Status,
    /// Analyze mining performance
    Stats {
        /// Time period for analysis (1h, 1d, 1w)
        #[arg(short, long, default_value = "1h")]
        period: String,
    },
}

#[derive(Subcommand)]
enum NetworkCommands {
    /// Check network connectivity
    Status,
    /// List connected peers
    Peers,
    /// Test network latency
    Ping {
        /// Target peer ID or address
        target: Option<String>,
    },
    /// Monitor network traffic
    Traffic {
        /// Monitor duration in seconds
        #[arg(short, long, default_value = "60")]
        duration: u64,
    },
}

#[derive(Subcommand)]
enum LogCommands {
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
}

#[derive(Subcommand)]
enum DevCommands {
    /// Initialize development environment
    Init {
        /// Project name
        name: String,
    },
    /// Run development tests
    Test {
        /// Test suite to run
        #[arg(short, long)]
        suite: Option<String>,
    },
    /// Build development assets
    Build {
        /// Build target (debug, release)
        #[arg(short, long, default_value = "debug")]
        target: String,
    },
    /// Clean development artifacts
    Clean,
}

#[derive(Subcommand)]
enum BenchCommands {
    /// Run comprehensive benchmark suite
    Run {
        /// Save results to file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Compare benchmark results
    Compare {
        /// Current benchmark results file
        #[arg(short, long)]
        current: PathBuf,
        /// Previous benchmark results file
        #[arg(short, long)]
        previous: PathBuf,
    },
    /// Profile performance of specific operations
    Profile {
        /// Operation to profile
        operation: String,
        /// Number of iterations
        #[arg(short, long, default_value = "1000")]
        iterations: u64,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    init_logging(log_level, &cli.config_dir.join(NOCKIT_LOG_DIR))?;
    
    // Ensure config directory exists
    std::fs::create_dir_all(&cli.config_dir)?;
    
    match cli.command {
        Commands::Setup { force, non_interactive } => {
            setup::run_setup(&cli.config_dir, force, non_interactive).await
        }
        Commands::Wallet { action } => {
            handle_wallet_command(action, &cli.config_dir, cli.nockchain_socket.as_ref()).await
        }
        Commands::Mining { action } => {
            handle_mining_command(action, &cli.config_dir).await
        }
        Commands::Network { action } => {
            handle_network_command(action, &cli.config_dir).await
        }
        Commands::Logs { action } => {
            handle_log_command(action, &cli.config_dir).await
        }
        Commands::Monitor { interval, format } => {
            monitoring::run_monitor(interval, &format, &cli.config_dir).await
        }
        Commands::Dev { action } => {
            handle_dev_command(action, &cli.config_dir).await
        }
        Commands::Bench { action } => {
            handle_bench_command(action, &cli.config_dir).await
        }
    }
}

async fn handle_wallet_command(action: WalletCommands, config_dir: &PathBuf, socket_path: Option<&PathBuf>) -> Result<()> {
    match action {
        WalletCommands::Keygen { output } => {
            wallet::generate_keys(output.as_ref().map(|p| p.as_path()), config_dir).await
        }
        WalletCommands::Status { pubkey } => {
            wallet::check_status(
                pubkey.as_ref().map(|s| s.as_str()), 
                config_dir, 
                socket_path.map(|p| p.as_path())
            ).await
        }
        WalletCommands::Backup { output } => {
            wallet::backup_keys(output.as_ref().map(|p| p.as_path()), config_dir).await
        }
        WalletCommands::Restore { input } => {
            wallet::restore_keys(&input, config_dir).await
        }
        WalletCommands::ShowPubkey => {
            match wallet::show_master_pubkey().await {
                Ok(result) => {
                    if result.success {
                        println!("🔑 Master Public Key: {}", result.stdout.trim());
                    } else {
                        eprintln!("❌ Failed to retrieve public key: {}", result.stderr);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                }
            }
            Ok(())
        }
        WalletCommands::ShowSeed => {
            match wallet::show_seedphrase().await {
                Ok(result) => {
                    if result.success {
                        println!("🌱 Seed Phrase: {}", result.stdout.trim());
                        println!("⚠️  Keep this seed phrase secure and private!");
                    } else {
                        eprintln!("❌ Failed to retrieve seed phrase: {}", result.stderr);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                }
            }
            Ok(())
        }
        WalletCommands::ListNotes { pubkey } => {
            let socket = socket_path.map(|p| p.as_path());
            
            if socket.is_none() {
                eprintln!("⚠️  --nockchain-socket required for listing notes");
                return Ok(());
            }
            
            match pubkey {
                Some(pk) => {
                    match wallet::list_notes_by_pubkey(&pk, socket).await {
                        Ok(result) => {
                            if result.success {
                                println!("💰 Notes for {}: ", pk);
                                println!("{}", result.stdout);
                            } else {
                                eprintln!("❌ Failed to retrieve notes: {}", result.stderr);
                            }
                        }
                        Err(e) => {
                            eprintln!("❌ Error: {}", e);
                        }
                    }
                }
                None => {
                    match wallet::list_notes(socket).await {
                        Ok(result) => {
                            if result.success {
                                println!("💰 All Wallet Notes:");
                                println!("{}", result.stdout);
                            } else {
                                eprintln!("❌ Failed to retrieve notes: {}", result.stderr);
                            }
                        }
                        Err(e) => {
                            eprintln!("❌ Error: {}", e);
                        }
                    }
                }
            }
            Ok(())
        }
        WalletCommands::UpdateBalance => {
            let socket = socket_path.map(|p| p.as_path());
            
            if socket.is_none() {
                eprintln!("⚠️  --nockchain-socket required for updating balance");
                return Ok(());
            }
            
            match wallet::update_balance(socket).await {
                Ok(result) => {
                    if result.success {
                        println!("✅ Balance updated successfully");
                        println!("{}", result.stdout);
                    } else {
                        eprintln!("❌ Failed to update balance: {}", result.stderr);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                }
            }
            Ok(())
        }
        WalletCommands::DeriveChild { key_type, index } => {
            match wallet::derive_child_key(&key_type, index).await {
                Ok(result) => {
                    if result.success {
                        println!("🔑 Derived {} key at index {}: ", key_type, index);
                        println!("{}", result.stdout);
                    } else {
                        eprintln!("❌ Failed to derive child key: {}", result.stderr);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                }
            }
            Ok(())
        }
        WalletCommands::Scan { master_pubkey, search_depth, include_timelocks, include_multisig } => {
            let socket = socket_path.map(|p| p.as_path());
            
            if socket.is_none() {
                eprintln!("⚠️  --nockchain-socket required for blockchain scanning");
                return Ok(());
            }
            
            println!("🔍 Scanning blockchain for transactions...");
            println!("   Master pubkey: {}", master_pubkey);
            println!("   Search depth: {}", search_depth);
            println!("   Include timelocks: {}", include_timelocks);
            println!("   Include multisig: {}", include_multisig);
            
            match wallet::scan_blockchain(&master_pubkey, Some(search_depth), include_timelocks, include_multisig, socket).await {
                Ok(result) => {
                    if result.success {
                        println!("✅ Blockchain scan completed:");
                        println!("{}", result.stdout);
                    } else {
                        eprintln!("❌ Blockchain scan failed: {}", result.stderr);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                }
            }
            Ok(())
        }
    }
}

async fn handle_mining_command(action: MiningCommands, config_dir: &PathBuf) -> Result<()> {
    match action {
        MiningCommands::Start { pubkey, difficulty } => {
            mining::start_mining(&pubkey, difficulty, config_dir).await
        }
        MiningCommands::Stop => {
            mining::stop_mining(config_dir).await
        }
        MiningCommands::Status => {
            mining::check_status(config_dir).await
        }
        MiningCommands::Stats { period } => {
            mining::analyze_stats(&period, config_dir).await
        }
    }
}

async fn handle_network_command(action: NetworkCommands, config_dir: &PathBuf) -> Result<()> {
    match action {
        NetworkCommands::Status => {
            network::check_status(config_dir).await
        }
        NetworkCommands::Peers => {
            network::list_peers(config_dir).await
        }
        NetworkCommands::Ping { target } => {
            network::ping_peer(target.as_ref().map(|s| s.as_str()), config_dir).await
        }
        NetworkCommands::Traffic { duration } => {
            network::monitor_traffic(duration, config_dir).await
        }
    }
}

async fn handle_log_command(action: LogCommands, config_dir: &PathBuf) -> Result<()> {
    match action {
        LogCommands::Tail { lines, follow } => {
            logging::tail_logs(lines, follow, config_dir).await
        }
        LogCommands::Search { pattern, file } => {
            logging::search_logs(&pattern, file.as_ref().map(|p| p.as_path()), config_dir).await
        }
        LogCommands::Analyze { period } => {
            logging::analyze_logs(&period, config_dir).await
        }
        LogCommands::Export { format, output } => {
            logging::export_logs(&format, &output, config_dir).await
        }
    }
}

async fn handle_dev_command(action: DevCommands, config_dir: &PathBuf) -> Result<()> {
    match action {
        DevCommands::Init { name } => {
            utils::init_dev_environment(&name, config_dir).await
        }
        DevCommands::Test { suite } => {
            utils::run_tests(suite.as_ref().map(|s| s.as_str()), config_dir).await
        }
        DevCommands::Build { target } => {
            utils::build_project(&target, config_dir).await
        }
        DevCommands::Clean => {
            utils::clean_artifacts(config_dir).await
        }
    }
}

async fn handle_bench_command(action: BenchCommands, config_dir: &PathBuf) -> Result<()> {
    match action {
        BenchCommands::Run { output } => {
            utils::run_benchmarks(output.as_ref().map(|p| p.as_path()), config_dir).await
        }
        BenchCommands::Compare { current, previous } => {
            utils::compare_benchmarks(&current, &previous, config_dir).await
        }
        BenchCommands::Profile { operation, iterations } => {
            utils::profile_operation(&operation, iterations, config_dir).await
        }
    }
} 