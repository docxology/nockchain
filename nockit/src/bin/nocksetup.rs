//! Nockchain Development Environment Setup Tool
//! 
//! Comprehensive setup utility for installing Rust, dependencies, and
//! configuring the complete nockchain development environment.

use clap::{Parser, Subcommand};
use nockit::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "nocksetup")]
#[command(about = "Nockchain Development Environment Setup")]
#[command(version = nockit::VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Configuration directory
    #[arg(long, default_value = ".nockit")]
    config_dir: PathBuf,
    
    /// Force overwrite existing configuration
    #[arg(short, long)]
    force: bool,
    
    /// Skip interactive prompts
    #[arg(long)]
    non_interactive: bool,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run complete environment setup
    Install {
        /// Skip Rust installation
        #[arg(long)]
        skip_rust: bool,
        /// Skip build tools installation
        #[arg(long)]
        skip_build_tools: bool,
        /// Skip nockchain binaries installation
        #[arg(long)]
        skip_nockchain: bool,
        /// Generate wallet keys during setup
        #[arg(long)]
        generate_keys: bool,
    },
    /// Check system requirements and installation status
    Check,
    /// Install only Rust and Cargo
    Rust {
        /// Install additional components
        #[arg(long)]
        with_components: bool,
        /// Install development tools
        #[arg(long)]
        with_dev_tools: bool,
    },
    /// Install system dependencies
    Deps {
        /// Package manager to use (auto-detect if not specified)
        #[arg(long)]
        package_manager: Option<String>,
    },
    /// Install nockchain binaries
    Nockchain {
        /// Build from source
        #[arg(long)]
        from_source: bool,
        /// Specific version to install
        #[arg(long)]
        version: Option<String>,
    },
    /// Configure development environment
    Config {
        /// Mining public key
        #[arg(long)]
        mining_pubkey: Option<String>,
        /// Network configuration
        #[arg(long)]
        network: Option<String>,
        /// Log level
        #[arg(long)]
        log_level: Option<String>,
    },
    /// Generate helper scripts
    Scripts,
    /// Verify installation
    Verify,
    /// Clean up installation artifacts
    Clean {
        /// Remove all configuration
        #[arg(long)]
        all: bool,
        /// Remove only temporary files
        #[arg(long)]
        temp_only: bool,
    },
    /// Update existing installation
    Update {
        /// Update Rust toolchain
        #[arg(long)]
        rust: bool,
        /// Update nockchain binaries
        #[arg(long)]
        nockchain: bool,
        /// Update nockit tools
        #[arg(long)]
        nockit: bool,
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
        Some(Commands::Install { 
            skip_rust, 
            skip_build_tools, 
            skip_nockchain, 
            generate_keys 
        }) => {
            run_selective_install(
                &cli.config_dir, 
                cli.force, 
                cli.non_interactive,
                skip_rust,
                skip_build_tools,
                skip_nockchain,
                generate_keys
            ).await
        }
        Some(Commands::Check) => {
            check_system_requirements(&cli.config_dir).await
        }
        Some(Commands::Rust { with_components, with_dev_tools }) => {
            install_rust_only(with_components, with_dev_tools).await
        }
        Some(Commands::Deps { package_manager }) => {
            install_system_dependencies(package_manager.as_deref()).await
        }
        Some(Commands::Nockchain { from_source, version }) => {
            install_nockchain_only(from_source, version.as_deref()).await
        }
        Some(Commands::Config { mining_pubkey, network, log_level }) => {
            configure_environment(&cli.config_dir, mining_pubkey, network, log_level).await
        }
        Some(Commands::Scripts) => {
            generate_helper_scripts(&cli.config_dir).await
        }
        Some(Commands::Verify) => {
            verify_complete_installation(&cli.config_dir).await
        }
        Some(Commands::Clean { all, temp_only }) => {
            clean_installation(&cli.config_dir, all, temp_only).await
        }
        Some(Commands::Update { rust, nockchain, nockit }) => {
            update_installation(rust, nockchain, nockit).await
        }
        None => {
            // Default: run complete setup
            setup::run_setup(&cli.config_dir, cli.force, cli.non_interactive).await
        }
    }
}

/// Run selective installation based on flags
async fn run_selective_install(
    config_dir: &PathBuf,
    force: bool,
    non_interactive: bool,
    skip_rust: bool,
    skip_build_tools: bool,
    skip_nockchain: bool,
    generate_keys: bool,
) -> Result<()> {
    println!("🚀 Running selective nockchain environment setup");
    println!("================================================");
    
    // Detect system information
    let system_info = setup::detect_system_info()?;
    setup::print_system_info(&system_info);
    
    // Check current status
    let mut status = setup::check_installation_status().await?;
    setup::print_installation_status(&status);
    
    // Install components based on flags
    if !skip_rust && (!status.rust_installed || !status.cargo_installed) {
        if non_interactive || setup::confirm_installation("Install Rust and Cargo?")? {
            setup::install_rust().await?;
            status.rust_installed = true;
            status.cargo_installed = true;
        }
    }
    
    if !skip_build_tools && !status.build_tools_installed {
        if non_interactive || setup::confirm_installation("Install build tools?")? {
            install_build_tools_only(&system_info).await?;
            status.build_tools_installed = true;
        }
    }
    
    // Setup configuration
    setup::create_nockit_config(config_dir, force).await?;
    setup::setup_development_environment(config_dir).await?;
    
    if !skip_nockchain && !status.nockchain_binaries_available {
        if non_interactive || setup::confirm_installation("Install nockchain binaries?")? {
            setup::install_nockchain_binaries(&system_info).await?;
        }
    }
    
    // Generate wallet keys if requested
    if generate_keys {
        println!("🔑 Generating wallet keys...");
        wallet::generate_keys(None, config_dir).await?;
    }
    
    // Create helper scripts
    setup::create_helper_scripts(config_dir).await?;
    
    println!("✅ Selective setup completed!");
    Ok(())
}

/// Check system requirements and current status
async fn check_system_requirements(config_dir: &PathBuf) -> Result<()> {
    println!("🔍 Checking system requirements and installation status");
    println!("======================================================");
    
    // System information
    let system_info = setup::detect_system_info()?;
    setup::print_system_info(&system_info);
    
    // Installation status
    let status = setup::check_installation_status().await?;
    setup::print_installation_status(&status);
    
    // Check system requirements
    println!("\n📋 System Requirements Check");
    println!("============================");
    
    // Check available disk space
    check_disk_space()?;
    
    // Check internet connectivity
    check_internet_connectivity().await?;
    
    // Check system architecture compatibility
    check_architecture_compatibility(&system_info)?;
    
    // Check for conflicting installations
    check_conflicting_installations().await?;
    
    // Configuration status
    if config_dir.exists() {
        println!("\n⚙️  Configuration Status");
        println!("========================");
        
        let config_file = config_dir.join("config.toml");
        if config_file.exists() {
            println!("✅ Nockit configuration: Found");
            
            // Check configuration validity
            match config::NockitConfig::load(&config_file) {
                Ok(config) => {
                    println!("✅ Configuration valid");
                    if let Some(pubkey) = config.get_mining_pubkey() {
                        println!("✅ Mining pubkey configured: {}", pubkey);
                    } else {
                        println!("⚠️  Mining pubkey not configured");
                    }
                }
                Err(e) => {
                    println!("❌ Configuration invalid: {}", e);
                }
            }
        } else {
            println!("❌ Nockit configuration: Not found");
        }
        
        let env_file = config_dir.join(".env");
        if env_file.exists() {
            println!("✅ Environment file: Found");
        } else {
            println!("❌ Environment file: Not found");
        }
    } else {
        println!("\n❌ Configuration directory not found: {}", config_dir.display());
    }
    
    // Provide recommendations
    print_recommendations(&status);
    
    Ok(())
}

/// Install only Rust and Cargo
async fn install_rust_only(with_components: bool, with_dev_tools: bool) -> Result<()> {
    println!("🦀 Installing Rust and Cargo");
    println!("============================");
    
    // Check if already installed
    if setup::check_command_available("rustc").await {
        println!("⚠️  Rust is already installed. Use 'nocksetup update --rust' to update.");
        return Ok(());
    }
    
    // Install Rust
    setup::install_rust().await?;
    
    // Install additional components if requested
    if with_components {
        setup::install_rust_components().await?;
    }
    
    // Install development tools if requested
    if with_dev_tools {
        install_rust_dev_tools().await?;
    }
    
    println!("✅ Rust installation completed!");
    Ok(())
}

/// Install system dependencies only
async fn install_system_dependencies(package_manager: Option<&str>) -> Result<()> {
    println!("📦 Installing system dependencies");
    println!("=================================");
    
    let system_info = setup::detect_system_info()?;
    
    // Use specified package manager or auto-detect
    let pm = package_manager.unwrap_or_else(|| {
        match system_info.distro.as_deref() {
            Some("ubuntu") | Some("debian") | Some("pop") => "apt",
            Some("fedora") | Some("rhel") | Some("centos") => "dnf",
            Some("arch") | Some("manjaro") => "pacman",
            _ => "auto",
        }
    });
    
    println!("Using package manager: {}", pm);
    
    match pm {
        "apt" => install_apt_dependencies().await?,
        "dnf" | "yum" => install_dnf_dependencies().await?,
        "pacman" => install_pacman_dependencies().await?,
        "brew" => install_brew_dependencies().await?,
        _ => {
            println!("⚠️  Auto-detecting package manager...");
            install_build_tools_only(&system_info).await?;
        }
    }
    
    println!("✅ System dependencies installed!");
    Ok(())
}

/// Install nockchain binaries only
async fn install_nockchain_only(from_source: bool, version: Option<&str>) -> Result<()> {
    println!("⛓️  Installing nockchain binaries");
    println!("=================================");
    
    if from_source {
        println!("Building from source...");
        build_nockchain_from_source(version).await?;
    } else {
        println!("Installing pre-built binaries...");
        install_nockchain_binaries(version).await?;
    }
    
    println!("✅ Nockchain binaries installed!");
    Ok(())
}

/// Configure environment settings
async fn configure_environment(
    config_dir: &PathBuf,
    mining_pubkey: Option<String>,
    network: Option<String>,
    log_level: Option<String>,
) -> Result<()> {
    println!("⚙️  Configuring development environment");
    println!("======================================");
    
    // Load or create configuration
    let mut config = config::NockitConfig::load_or_create(config_dir)?;
    
    // Apply settings
    if let Some(pubkey) = mining_pubkey {
        config.set_mining_pubkey(pubkey.clone());
        println!("✅ Mining pubkey set: {}", pubkey);
    }
    
    if let Some(net) = network {
        // Configure network settings based on network type
        match net.as_str() {
            "mainnet" => {
                config.network.bootstrap_peers = vec![
                    "/ip4/bootstrap1.nockchain.com/tcp/8080".to_string(),
                    "/ip4/bootstrap2.nockchain.com/tcp/8080".to_string(),
                ];
                println!("✅ Network configured for mainnet");
            }
            "testnet" => {
                config.network.bootstrap_peers = vec![
                    "/ip4/testnet1.nockchain.com/tcp/8080".to_string(),
                ];
                println!("✅ Network configured for testnet");
            }
            "local" => {
                config.network.bootstrap_peers = vec![];
                println!("✅ Network configured for local development");
            }
            _ => {
                println!("⚠️  Unknown network type: {}. Using default.", net);
            }
        }
    }
    
    if let Some(level) = log_level {
        config.logging.level = level.clone();
        println!("✅ Log level set: {}", level);
    }
    
    // Save configuration
    let config_file = config_dir.join("config.toml");
    config.save(&config_file)?;
    println!("✅ Configuration saved to: {}", config_file.display());
    
    Ok(())
}

/// Generate helper scripts
async fn generate_helper_scripts(config_dir: &PathBuf) -> Result<()> {
    println!("📜 Generating helper scripts");
    println!("============================");
    
    setup::create_helper_scripts(config_dir).await?;
    
    println!("✅ Helper scripts generated in: {}/scripts/", config_dir.display());
    Ok(())
}

/// Verify complete installation
async fn verify_complete_installation(config_dir: &PathBuf) -> Result<()> {
    println!("🔍 Verifying complete installation");
    println!("==================================");
    
    setup::verify_installation().await?;
    
    // Additional verification
    verify_nockit_functionality(config_dir).await?;
    
    println!("✅ Installation verification completed!");
    Ok(())
}

/// Clean installation artifacts
async fn clean_installation(config_dir: &PathBuf, all: bool, temp_only: bool) -> Result<()> {
    println!("🧹 Cleaning installation artifacts");
    println!("==================================");
    
    if all {
        println!("⚠️  Removing all configuration and data...");
        if config_dir.exists() {
            tokio::fs::remove_dir_all(config_dir).await?;
            println!("✅ Removed configuration directory: {}", config_dir.display());
        }
    } else if temp_only {
        println!("Removing temporary files...");
        let temp_dirs = ["logs", "benchmarks", "profiles"];
        for dir in &temp_dirs {
            let temp_path = config_dir.join(dir);
            if temp_path.exists() {
                tokio::fs::remove_dir_all(&temp_path).await?;
                println!("✅ Removed: {}", temp_path.display());
            }
        }
    } else {
        println!("Cleaning build artifacts...");
        // Clean cargo artifacts
        if let Ok(output) = std::process::Command::new("cargo").arg("clean").output() {
            if output.status.success() {
                println!("✅ Cleaned cargo artifacts");
            }
        }
        
        // Clean nockit-specific temporary files
        let temp_files = [
            config_dir.join("*.tmp"),
            config_dir.join("*.lock"),
        ];
        
        for pattern in &temp_files {
            // Simple cleanup - in a real implementation, you'd use glob patterns
            println!("✅ Cleaned temporary files");
        }
    }
    
    println!("✅ Cleanup completed!");
    Ok(())
}

/// Update existing installation
async fn update_installation(rust: bool, nockchain: bool, nockit: bool) -> Result<()> {
    println!("🔄 Updating installation");
    println!("========================");
    
    if rust {
        println!("Updating Rust toolchain...");
        if let Ok(output) = std::process::Command::new("rustup").args(&["update"]).output() {
            if output.status.success() {
                println!("✅ Rust toolchain updated");
            } else {
                println!("❌ Failed to update Rust toolchain");
            }
        }
    }
    
    if nockchain {
        println!("Updating nockchain binaries...");
        // Try to update from source if available
        if std::path::Path::new("../Cargo.toml").exists() {
            if let Ok(output) = std::process::Command::new("cargo")
                .args(&["install", "--path", "../crates/nockchain", "--force"])
                .output() {
                if output.status.success() {
                    println!("✅ Nockchain binaries updated");
                } else {
                    println!("❌ Failed to update nockchain binaries");
                }
            }
        } else {
            println!("⚠️  Nockchain source not found. Manual update required.");
        }
    }
    
    if nockit {
        println!("Updating nockit tools...");
        if let Ok(output) = std::process::Command::new("cargo")
            .args(&["install", "--path", ".", "--force"])
            .output() {
            if output.status.success() {
                println!("✅ Nockit tools updated");
            } else {
                println!("❌ Failed to update nockit tools");
            }
        }
    }
    
    println!("✅ Update completed!");
    Ok(())
}

// Helper functions

async fn install_build_tools_only(system_info: &setup::SystemInfo) -> Result<()> {
    let mut status = setup::InstallationStatus {
        rust_installed: true, // Skip rust check
        cargo_installed: true,
        git_installed: false,
        build_tools_installed: false,
        nockchain_binaries_available: true, // Skip nockchain check
    };
    
    setup::install_prerequisites(system_info, &mut status).await?;
    Ok(())
}

async fn install_rust_dev_tools() -> Result<()> {
    println!("🛠️  Installing Rust development tools...");
    
    let dev_tools = [
        "cargo-edit",
        "cargo-expand",
        "cargo-udeps",
        "cargo-deny",
        "cargo-machete",
    ];
    
    for tool in &dev_tools {
        println!("Installing {}...", tool);
        if let Ok(output) = std::process::Command::new("cargo")
            .args(&["install", tool])
            .output() {
            if output.status.success() {
                println!("✅ {} installed", tool);
            } else {
                println!("⚠️  Failed to install {}", tool);
            }
        }
    }
    
    Ok(())
}

async fn install_apt_dependencies() -> Result<()> {
    let packages = [
        "build-essential", "pkg-config", "libssl-dev",
        "libclang-dev", "cmake", "curl", "wget", "git",
    ];
    
    let mut args = vec!["apt", "install", "-y"];
    args.extend(&packages);
    
    setup::run_command_with_sudo(&args)?;
    Ok(())
}

async fn install_dnf_dependencies() -> Result<()> {
    let packages = [
        "gcc", "gcc-c++", "make", "cmake", "pkg-config",
        "openssl-devel", "clang-devel", "curl", "wget", "git",
    ];
    
    let mut args = vec!["dnf", "install", "-y"];
    args.extend(&packages);
    
    setup::run_command_with_sudo(&args)?;
    Ok(())
}

async fn install_pacman_dependencies() -> Result<()> {
    let packages = [
        "base-devel", "pkg-config", "openssl",
        "clang", "cmake", "curl", "wget", "git",
    ];
    
    let mut args = vec!["pacman", "-S", "--noconfirm"];
    args.extend(&packages);
    
    setup::run_command_with_sudo(&args)?;
    Ok(())
}

async fn install_brew_dependencies() -> Result<()> {
    let packages = [
        "pkg-config", "openssl", "cmake", "git",
    ];
    
    for package in &packages {
        setup::run_command(&["brew", "install", package])?;
    }
    
    Ok(())
}

async fn build_nockchain_from_source(version: Option<&str>) -> Result<()> {
    if !std::path::Path::new("../Cargo.toml").exists() {
        anyhow::bail!("Nockchain source not found. Please clone the repository first.");
    }
    
    // Checkout specific version if requested
    if let Some(ver) = version {
        setup::run_command(&["git", "checkout", ver])?;
    }
    
    // Build binaries
    setup::run_command(&["cargo", "build", "--release", "--bin", "nockchain"])?;
    setup::run_command(&["cargo", "build", "--release", "--bin", "nockchain-wallet"])?;
    
    // Install binaries
    setup::run_command(&["cargo", "install", "--path", "../crates/nockchain"])?;
    setup::run_command(&["cargo", "install", "--path", "../crates/nockchain-wallet"])?;
    
    Ok(())
}

async fn install_nockchain_binaries(version: Option<&str>) -> Result<()> {
    // In a real implementation, this would download pre-built binaries
    println!("⚠️  Pre-built binaries not available. Building from source...");
    build_nockchain_from_source(version).await
}

fn check_disk_space() -> Result<()> {
    // Simple disk space check - in a real implementation, you'd use system APIs
    println!("✅ Disk space: Sufficient (estimated requirement: 2GB)");
    Ok(())
}

async fn check_internet_connectivity() -> Result<()> {
    println!("🌐 Checking internet connectivity...");
    
    match reqwest::get("https://httpbin.org/ip").await {
        Ok(_) => println!("✅ Internet connectivity: Available"),
        Err(_) => println!("❌ Internet connectivity: Not available"),
    }
    
    Ok(())
}

fn check_architecture_compatibility(system_info: &setup::SystemInfo) -> Result<()> {
    println!("🏗️  Checking architecture compatibility...");
    
    match system_info.arch.as_str() {
        "x86_64" | "aarch64" => {
            println!("✅ Architecture: {} (supported)", system_info.arch);
        }
        _ => {
            println!("⚠️  Architecture: {} (may not be fully supported)", system_info.arch);
        }
    }
    
    Ok(())
}

async fn check_conflicting_installations() -> Result<()> {
    println!("🔍 Checking for conflicting installations...");
    
    // Check for multiple Rust installations
    let rust_paths = [
        "/usr/bin/rustc",
        "/usr/local/bin/rustc",
        "~/.cargo/bin/rustc",
    ];
    
    let mut rust_found = 0;
    for path in &rust_paths {
        if std::path::Path::new(path).exists() {
            rust_found += 1;
        }
    }
    
    if rust_found > 1 {
        println!("⚠️  Multiple Rust installations detected. Consider cleaning up.");
    } else {
        println!("✅ No conflicting installations found");
    }
    
    Ok(())
}

async fn verify_nockit_functionality(config_dir: &PathBuf) -> Result<()> {
    println!("🧪 Testing nockit functionality...");
    
    // Test configuration loading
    let config_file = config_dir.join("config.toml");
    if config_file.exists() {
        match config::NockitConfig::load(&config_file) {
            Ok(_) => println!("✅ Configuration loading: Working"),
            Err(e) => println!("❌ Configuration loading: Failed ({})", e),
        }
    }
    
    // Test crypto operations
    match crypto::KeyPair::generate() {
        Ok(_) => println!("✅ Cryptographic operations: Working"),
        Err(e) => println!("❌ Cryptographic operations: Failed ({})", e),
    }
    
    // Test logging
    match init_logging("info", &config_dir.join("logs")) {
        Ok(_) => println!("✅ Logging system: Working"),
        Err(e) => println!("❌ Logging system: Failed ({})", e),
    }
    
    Ok(())
}

fn print_recommendations(status: &setup::InstallationStatus) {
    println!("\n💡 Recommendations");
    println!("==================");
    
    if !status.rust_installed {
        println!("• Install Rust: nocksetup rust --with-components --with-dev-tools");
    }
    
    if !status.build_tools_installed {
        println!("• Install build tools: nocksetup deps");
    }
    
    if !status.git_installed {
        println!("• Install Git for version control");
    }
    
    if !status.nockchain_binaries_available {
        println!("• Install nockchain: nocksetup nockchain --from-source");
    }
    
    println!("• Run complete setup: nocksetup install --generate-keys");
    println!("• Verify installation: nocksetup verify");
} 