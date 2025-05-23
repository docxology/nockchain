//! Environment setup and initialization for nockit
//! 
//! Provides comprehensive environment setup including Rust installation,
//! dependency management, and nockchain development environment preparation.

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tokio::fs;

/// System information for setup decisions
#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub distro: Option<String>,
    pub shell: String,
    pub home_dir: String,
}

/// Installation status for dependencies
#[derive(Debug, Clone)]
pub struct InstallationStatus {
    pub rust_installed: bool,
    pub cargo_installed: bool,
    pub git_installed: bool,
    pub build_tools_installed: bool,
    pub nockchain_binaries_available: bool,
}

/// Run comprehensive setup process
pub async fn run_setup(config_dir: &Path, force: bool, non_interactive: bool) -> Result<()> {
    println!("🚀 Starting comprehensive nockchain development environment setup");
    println!("================================================================");
    
    // Detect system information
    let system_info = detect_system_info()?;
    print_system_info(&system_info);
    
    // Check current installation status
    let mut status = check_installation_status().await?;
    print_installation_status(&status);
    
    // Install prerequisites if needed
    if !status.rust_installed || !status.cargo_installed || !status.build_tools_installed {
        if non_interactive || confirm_installation("Install missing prerequisites?")? {
            install_prerequisites(&system_info, &mut status).await?;
        }
    }
    
    // Setup Rust environment
    if status.rust_installed {
        setup_rust_environment(&system_info).await?;
    }
    
    // Create nockit configuration
    create_nockit_config(config_dir, force).await?;
    
    // Install nockchain binaries if needed
    if !status.nockchain_binaries_available {
        if non_interactive || confirm_installation("Install nockchain binaries?")? {
            install_nockchain_binaries(&system_info).await?;
        }
    }
    
    // Setup development environment
    setup_development_environment(config_dir).await?;
    
    // Create helper scripts
    create_helper_scripts(config_dir).await?;
    
    // Final verification
    verify_installation().await?;
    
    println!("\n✅ Setup completed successfully!");
    print_next_steps();
    
    Ok(())
}

/// Detect system information
pub fn detect_system_info() -> Result<SystemInfo> {
    let os = std::env::consts::OS.to_string();
    let arch = std::env::consts::ARCH.to_string();
    
    let distro = detect_linux_distro();
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());
    
    Ok(SystemInfo {
        os,
        arch,
        distro,
        shell,
        home_dir,
    })
}

/// Detect Linux distribution
fn detect_linux_distro() -> Option<String> {
    // Try to read /etc/os-release
    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("ID=") {
                return Some(line.replace("ID=", "").trim_matches('"').to_string());
            }
        }
    }
    
    // Fallback checks
    if std::path::Path::new("/etc/debian_version").exists() {
        Some("debian".to_string())
    } else if std::path::Path::new("/etc/redhat-release").exists() {
        Some("rhel".to_string())
    } else if std::path::Path::new("/etc/arch-release").exists() {
        Some("arch".to_string())
    } else {
        None
    }
}

/// Check current installation status
pub async fn check_installation_status() -> Result<InstallationStatus> {
    println!("🔍 Checking current installation status...");
    
    let rust_installed = check_command_available("rustc").await;
    let cargo_installed = check_command_available("cargo").await;
    let git_installed = check_command_available("git").await;
    let build_tools_installed = check_build_tools_available().await;
    let nockchain_binaries_available = check_nockchain_binaries().await;
    
    Ok(InstallationStatus {
        rust_installed,
        cargo_installed,
        git_installed,
        build_tools_installed,
        nockchain_binaries_available,
    })
}

/// Check if a command is available
pub async fn check_command_available(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Check if build tools are available
async fn check_build_tools_available() -> bool {
    let gcc_available = check_command_available("gcc").await;
    let make_available = check_command_available("make").await;
    let pkg_config_available = check_command_available("pkg-config").await;
    
    gcc_available && make_available && pkg_config_available
}

/// Check if nockchain binaries are available
async fn check_nockchain_binaries() -> bool {
    let nockchain_available = check_command_available("nockchain").await;
    let wallet_available = check_command_available("nockchain-wallet").await;
    
    nockchain_available && wallet_available
}

/// Install prerequisites based on system
pub async fn install_prerequisites(system_info: &SystemInfo, status: &mut InstallationStatus) -> Result<()> {
    println!("📦 Installing prerequisites...");
    
    match system_info.os.as_str() {
        "linux" => install_linux_prerequisites(system_info, status).await?,
        "macos" => install_macos_prerequisites(status).await?,
        "windows" => install_windows_prerequisites(status).await?,
        _ => {
            println!("⚠️  Unsupported OS: {}. Manual installation required.", system_info.os);
            return Ok(());
        }
    }
    
    Ok(())
}

/// Install Linux prerequisites
async fn install_linux_prerequisites(system_info: &SystemInfo, status: &mut InstallationStatus) -> Result<()> {
    let distro = system_info.distro.as_deref().unwrap_or("unknown");
    
    match distro {
        "ubuntu" | "debian" | "pop" => {
            install_debian_prerequisites(status).await?;
        }
        "fedora" | "rhel" | "centos" => {
            install_fedora_prerequisites(status).await?;
        }
        "arch" | "manjaro" => {
            install_arch_prerequisites(status).await?;
        }
        _ => {
            println!("⚠️  Unknown Linux distribution: {}. Attempting generic installation.", distro);
            install_generic_linux_prerequisites(status).await?;
        }
    }
    
    Ok(())
}

/// Install Debian/Ubuntu prerequisites
async fn install_debian_prerequisites(status: &mut InstallationStatus) -> Result<()> {
    println!("📦 Installing Debian/Ubuntu prerequisites...");
    
    // Update package list
    run_command_with_sudo(&["apt", "update"])?;
    
    // Install build essentials
    if !status.build_tools_installed {
        println!("Installing build tools...");
        run_command_with_sudo(&[
            "apt", "install", "-y",
            "build-essential",
            "pkg-config",
            "libssl-dev",
            "libclang-dev",
            "cmake",
            "curl",
            "wget",
        ])?;
        status.build_tools_installed = true;
    }
    
    // Install Git if needed
    if !status.git_installed {
        println!("Installing Git...");
        run_command_with_sudo(&["apt", "install", "-y", "git"])?;
        status.git_installed = true;
    }
    
    // Install Rust if needed
    if !status.rust_installed {
        install_rust().await?;
        status.rust_installed = true;
        status.cargo_installed = true;
    }
    
    Ok(())
}

/// Install Fedora/RHEL prerequisites
async fn install_fedora_prerequisites(status: &mut InstallationStatus) -> Result<()> {
    println!("📦 Installing Fedora/RHEL prerequisites...");
    
    // Install build tools
    if !status.build_tools_installed {
        println!("Installing build tools...");
        run_command_with_sudo(&[
            "dnf", "install", "-y",
            "gcc", "gcc-c++",
            "make", "cmake",
            "pkg-config",
            "openssl-devel",
            "clang-devel",
            "curl", "wget",
        ])?;
        status.build_tools_installed = true;
    }
    
    // Install Git if needed
    if !status.git_installed {
        println!("Installing Git...");
        run_command_with_sudo(&["dnf", "install", "-y", "git"])?;
        status.git_installed = true;
    }
    
    // Install Rust if needed
    if !status.rust_installed {
        install_rust().await?;
        status.rust_installed = true;
        status.cargo_installed = true;
    }
    
    Ok(())
}

/// Install Arch Linux prerequisites
async fn install_arch_prerequisites(status: &mut InstallationStatus) -> Result<()> {
    println!("📦 Installing Arch Linux prerequisites...");
    
    // Update package database
    run_command_with_sudo(&["pacman", "-Sy"])?;
    
    // Install build tools
    if !status.build_tools_installed {
        println!("Installing build tools...");
        run_command_with_sudo(&[
            "pacman", "-S", "--noconfirm",
            "base-devel",
            "pkg-config",
            "openssl",
            "clang",
            "cmake",
            "curl", "wget",
        ])?;
        status.build_tools_installed = true;
    }
    
    // Install Git if needed
    if !status.git_installed {
        println!("Installing Git...");
        run_command_with_sudo(&["pacman", "-S", "--noconfirm", "git"])?;
        status.git_installed = true;
    }
    
    // Install Rust if needed
    if !status.rust_installed {
        install_rust().await?;
        status.rust_installed = true;
        status.cargo_installed = true;
    }
    
    Ok(())
}

/// Install generic Linux prerequisites
async fn install_generic_linux_prerequisites(status: &mut InstallationStatus) -> Result<()> {
    println!("📦 Attempting generic Linux installation...");
    
    // Try to install Rust directly
    if !status.rust_installed {
        install_rust().await?;
        status.rust_installed = true;
        status.cargo_installed = true;
    }
    
    println!("⚠️  Please ensure the following are installed manually:");
    println!("   - GCC compiler");
    println!("   - Make");
    println!("   - pkg-config");
    println!("   - OpenSSL development headers");
    println!("   - Git");
    
    Ok(())
}

/// Install macOS prerequisites
async fn install_macos_prerequisites(status: &mut InstallationStatus) -> Result<()> {
    println!("📦 Installing macOS prerequisites...");
    
    // Check if Homebrew is available
    let homebrew_available = check_command_available("brew").await;
    
    if !homebrew_available {
        println!("Installing Homebrew...");
        install_homebrew().await?;
    }
    
    // Install Xcode command line tools
    if !status.build_tools_installed {
        println!("Installing Xcode command line tools...");
        run_command(&["xcode-select", "--install"])?;
        status.build_tools_installed = true;
    }
    
    // Install dependencies via Homebrew
    if !status.git_installed {
        run_command(&["brew", "install", "git"])?;
        status.git_installed = true;
    }
    
    // Install additional tools
    run_command(&["brew", "install", "pkg-config", "openssl", "cmake"])?;
    
    // Install Rust if needed
    if !status.rust_installed {
        install_rust().await?;
        status.rust_installed = true;
        status.cargo_installed = true;
    }
    
    Ok(())
}

/// Install Windows prerequisites
async fn install_windows_prerequisites(status: &mut InstallationStatus) -> Result<()> {
    println!("📦 Installing Windows prerequisites...");
    
    println!("⚠️  Windows setup requires manual installation:");
    println!("1. Install Visual Studio Build Tools or Visual Studio Community");
    println!("2. Install Git for Windows");
    println!("3. Install Rust using rustup-init.exe");
    println!("4. Restart your terminal after installation");
    
    // Try to install Rust if not available
    if !status.rust_installed {
        install_rust_windows().await?;
        status.rust_installed = true;
        status.cargo_installed = true;
    }
    
    Ok(())
}

/// Install Rust using rustup
pub async fn install_rust() -> Result<()> {
    println!("🦀 Installing Rust...");
    
    // Download and run rustup installer
    let install_script = if cfg!(target_os = "windows") {
        "https://win.rustup.rs/"
    } else {
        "https://sh.rustup.rs"
    };
    
    let output = Command::new("curl")
        .args(&["--proto", "=https", "--tlsv1.2", "-sSf", install_script])
        .output()
        .context("Failed to download rustup installer")?;
    
    if !output.status.success() {
        anyhow::bail!("Failed to download rustup installer");
    }
    
    // Run the installer
    let mut cmd = Command::new("sh");
    cmd.arg("-s").arg("--").arg("-y");
    
    let mut install_output = cmd
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = install_output.stdin.take() {
        use std::io::Write;
        stdin.write_all(&output.stdout)?;
    }
    
    let result = install_output.wait_with_output()?;
    
    if result.status.success() {
        println!("✅ Rust installed successfully");
        
        // Source the cargo environment
        source_cargo_env()?;
        
        // Install additional components
        install_rust_components().await?;
    } else {
        let stderr = String::from_utf8_lossy(&result.stderr);
        anyhow::bail!("Rust installation failed: {}", stderr);
    }
    
    Ok(())
}

/// Install Rust on Windows
async fn install_rust_windows() -> Result<()> {
    println!("🦀 Installing Rust on Windows...");
    println!("Please download and run rustup-init.exe from https://rustup.rs/");
    println!("Follow the installation prompts and restart your terminal.");
    Ok(())
}

/// Install Homebrew on macOS
async fn install_homebrew() -> Result<()> {
    println!("🍺 Installing Homebrew...");
    
    let install_script = r#"/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)""#;
    
    let output = Command::new("bash")
        .arg("-c")
        .arg(install_script)
        .output()
        .context("Failed to install Homebrew")?;
    
    if output.status.success() {
        println!("✅ Homebrew installed successfully");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Homebrew installation failed: {}", stderr);
    }
    
    Ok(())
}

/// Source cargo environment
fn source_cargo_env() -> Result<()> {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());
    let cargo_env = format!("{}/.cargo/env", home_dir);
    
    if std::path::Path::new(&cargo_env).exists() {
        // Add cargo to PATH for current session
        let cargo_bin = format!("{}/.cargo/bin", home_dir);
        if let Ok(current_path) = std::env::var("PATH") {
            let new_path = format!("{}:{}", cargo_bin, current_path);
            std::env::set_var("PATH", new_path);
        }
    }
    
    Ok(())
}

/// Install additional Rust components
pub async fn install_rust_components() -> Result<()> {
    println!("🔧 Installing additional Rust components...");
    
    // Install useful components
    let components = [
        "clippy",
        "rustfmt",
        "rust-src",
        "rust-analyzer",
    ];
    
    for component in &components {
        println!("Installing {}...", component);
        let output = Command::new("rustup")
            .args(&["component", "add", component])
            .output();
        
        match output {
            Ok(result) if result.status.success() => {
                println!("✅ {} installed", component);
            }
            _ => {
                println!("⚠️  Failed to install {}, continuing...", component);
            }
        }
    }
    
    // Install useful cargo tools
    let tools = [
        "cargo-audit",
        "cargo-outdated",
        "cargo-tree",
        "cargo-watch",
    ];
    
    for tool in &tools {
        println!("Installing {}...", tool);
        let output = Command::new("cargo")
            .args(&["install", tool])
            .output();
        
        match output {
            Ok(result) if result.status.success() => {
                println!("✅ {} installed", tool);
            }
            _ => {
                println!("⚠️  Failed to install {}, continuing...", tool);
            }
        }
    }
    
    Ok(())
}

/// Setup Rust environment
async fn setup_rust_environment(system_info: &SystemInfo) -> Result<()> {
    println!("🔧 Setting up Rust environment...");
    
    // Update Rust to latest stable
    run_command(&["rustup", "update", "stable"])?;
    
    // Set default toolchain
    run_command(&["rustup", "default", "stable"])?;
    
    // Add shell configuration
    add_rust_to_shell_config(system_info).await?;
    
    println!("✅ Rust environment configured");
    Ok(())
}

/// Add Rust to shell configuration
async fn add_rust_to_shell_config(system_info: &SystemInfo) -> Result<()> {
    let shell_config = match system_info.shell.as_str() {
        s if s.contains("bash") => format!("{}/.bashrc", system_info.home_dir),
        s if s.contains("zsh") => format!("{}/.zshrc", system_info.home_dir),
        s if s.contains("fish") => format!("{}/.config/fish/config.fish", system_info.home_dir),
        _ => format!("{}/.profile", system_info.home_dir),
    };
    
    let cargo_env_line = format!(". \"{}/.cargo/env\"", system_info.home_dir);
    
    // Check if already added
    if let Ok(content) = fs::read_to_string(&shell_config).await {
        if content.contains(&cargo_env_line) {
            return Ok(());
        }
    }
    
    // Add cargo environment to shell config
    let mut config_content = fs::read_to_string(&shell_config).await.unwrap_or_default();
    config_content.push_str(&format!("\n# Added by nockit setup\n{}\n", cargo_env_line));
    
    fs::write(&shell_config, config_content).await?;
    println!("✅ Added Rust to shell configuration: {}", shell_config);
    
    Ok(())
}

/// Install nockchain binaries
pub async fn install_nockchain_binaries(system_info: &SystemInfo) -> Result<()> {
    println!("⛓️  Installing nockchain binaries...");
    
    // Try to build from source if in nockchain repository
    if std::path::Path::new("../Cargo.toml").exists() {
        println!("Building nockchain from source...");
        
        // Build nockchain
        run_command(&["cargo", "build", "--release", "--bin", "nockchain"])?;
        run_command(&["cargo", "build", "--release", "--bin", "nockchain-wallet"])?;
        
        // Install binaries
        run_command(&["cargo", "install", "--path", "../crates/nockchain"])?;
        run_command(&["cargo", "install", "--path", "../crates/nockchain-wallet"])?;
        
        println!("✅ Nockchain binaries built and installed from source");
    } else {
        println!("⚠️  Nockchain source not found. Please clone the repository:");
        println!("   git clone https://github.com/nockchain/nockchain.git");
        println!("   cd nockchain");
        println!("   make install");
    }
    
    Ok(())
}

/// Create nockit configuration
pub async fn create_nockit_config(config_dir: &Path, force: bool) -> Result<()> {
    println!("⚙️  Creating nockit configuration...");
    
    // Create config directory
    fs::create_dir_all(config_dir).await?;
    
    // Create default configuration
    let config = crate::config::NockitConfig::load_or_create(config_dir)?;
    let config_file = config_dir.join("config.toml");
    
    if !config_file.exists() || force {
        config.save(&config_file)?;
        println!("✅ Configuration created: {}", config_file.display());
    }
    
    // Create environment file
    create_env_file(config_dir).await?;
    
    Ok(())
}

/// Create environment file
async fn create_env_file(config_dir: &Path) -> Result<()> {
    let env_file = config_dir.join(".env");
    
    if !env_file.exists() {
        let env_content = r#"# Nockchain Environment Configuration
# Generated by nockit setup

# Logging configuration
RUST_LOG=info,nockchain=debug
MINIMAL_LOG_FORMAT=false

# Mining configuration
# MINING_PUBKEY=your_public_key_here

# Network configuration
PEER_PORT=0
BIND_ADDRESS=/ip4/0.0.0.0/udp/0/quic-v1

# Development settings
RUST_BACKTRACE=1
"#;
        
        fs::write(&env_file, env_content).await?;
        println!("✅ Environment file created: {}", env_file.display());
    }
    
    Ok(())
}

/// Setup development environment
pub async fn setup_development_environment(config_dir: &Path) -> Result<()> {
    println!("🛠️  Setting up development environment...");
    
    // Create directory structure
    let dirs = [
        "logs",
        "backups",
        "scripts",
        "data",
        "benchmarks",
        "profiles",
    ];
    
    for dir in &dirs {
        let dir_path = config_dir.join(dir);
        fs::create_dir_all(&dir_path).await?;
        println!("✅ Created directory: {}", dir_path.display());
    }
    
    Ok(())
}

/// Create helper scripts
pub async fn create_helper_scripts(config_dir: &Path) -> Result<()> {
    println!("📜 Creating helper scripts...");
    
    let scripts_dir = config_dir.join("scripts");
    fs::create_dir_all(&scripts_dir).await?;
    
    // Create start mining script
    let start_mining_script = r#"#!/bin/bash
# Start nockchain mining
# Generated by nockit setup

set -e

CONFIG_DIR="$(dirname "$0")/.."
source "$CONFIG_DIR/.env"

if [ -z "$MINING_PUBKEY" ]; then
    echo "Error: MINING_PUBKEY not set in .env file"
    echo "Run: nockit wallet keygen"
    exit 1
fi

echo "Starting nockchain mining..."
echo "Public key: $MINING_PUBKEY"

nockchain mine --pubkey "$MINING_PUBKEY" \
    --bind-address "$BIND_ADDRESS" \
    --peer-port "$PEER_PORT"
"#;
    
    let start_script_path = scripts_dir.join("start_mining.sh");
    fs::write(&start_script_path, start_mining_script).await?;
    make_executable(&start_script_path).await?;
    
    // Create stop mining script
    let stop_mining_script = r#"#!/bin/bash
# Stop nockchain mining
# Generated by nockit setup

echo "Stopping nockchain mining..."
pkill -f "nockchain mine" || echo "No mining process found"
echo "Mining stopped"
"#;
    
    let stop_script_path = scripts_dir.join("stop_mining.sh");
    fs::write(&stop_script_path, stop_mining_script).await?;
    make_executable(&stop_script_path).await?;
    
    // Create status check script
    let status_script = r#"#!/bin/bash
# Check nockchain status
# Generated by nockit setup

echo "=== Nockchain Status ==="
echo

echo "Mining process:"
if pgrep -f "nockchain mine" > /dev/null; then
    echo "✅ Mining is running"
    pgrep -f "nockchain mine" | head -1 | xargs ps -p
else
    echo "❌ Mining is not running"
fi

echo
echo "Network status:"
nockit network status 2>/dev/null || echo "❌ Network check failed"

echo
echo "System health:"
nockit monitor --format compact 2>/dev/null || echo "❌ Health check failed"
"#;
    
    let status_script_path = scripts_dir.join("check_status.sh");
    fs::write(&status_script_path, status_script).await?;
    make_executable(&status_script_path).await?;
    
    println!("✅ Helper scripts created in: {}", scripts_dir.display());
    
    Ok(())
}

/// Make file executable
async fn make_executable(file_path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(file_path).await?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(file_path, perms).await?;
    }
    Ok(())
}

/// Verify installation
pub async fn verify_installation() -> Result<()> {
    println!("🔍 Verifying installation...");
    
    let checks = [
        ("rustc", "Rust compiler"),
        ("cargo", "Cargo package manager"),
        ("git", "Git version control"),
    ];
    
    for (command, description) in &checks {
        if check_command_available(command).await {
            let version = get_command_version(command).await;
            println!("✅ {}: {}", description, version);
        } else {
            println!("❌ {}: Not found", description);
        }
    }
    
    // Check nockit installation
    if check_command_available("nockit").await {
        println!("✅ Nockit: Available");
    } else {
        println!("⚠️  Nockit: Install with 'cargo install --path .'");
    }
    
    Ok(())
}

/// Get command version
async fn get_command_version(command: &str) -> String {
    let output = Command::new(command)
        .arg("--version")
        .output();
    
    match output {
        Ok(result) if result.status.success() => {
            String::from_utf8_lossy(&result.stdout).trim().to_string()
        }
        _ => "Unknown version".to_string(),
    }
}

/// Run command with sudo if needed
pub fn run_command_with_sudo(args: &[&str]) -> Result<()> {
    println!("Running: sudo {}", args.join(" "));
    
    let output = Command::new("sudo")
        .args(args)
        .output()
        .context("Failed to run command with sudo")?;
    
    if output.status.success() {
        println!("✅ Command completed successfully");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Command failed: {}", stderr);
    }
    
    Ok(())
}

/// Run command without sudo
pub fn run_command(args: &[&str]) -> Result<()> {
    println!("Running: {}", args.join(" "));
    
    let output = Command::new(args[0])
        .args(&args[1..])
        .output()
        .context("Failed to run command")?;
    
    if output.status.success() {
        println!("✅ Command completed successfully");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Command failed: {}", stderr);
    }
    
    Ok(())
}

/// Confirm installation with user
pub fn confirm_installation(message: &str) -> Result<bool> {
    use std::io::{self, Write};
    
    print!("{} [Y/n]: ", message);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let response = input.trim().to_lowercase();
    Ok(response.is_empty() || response == "y" || response == "yes")
}

/// Print system information
pub fn print_system_info(info: &SystemInfo) {
    println!("\n🖥️  System Information");
    println!("====================");
    println!("OS: {}", info.os);
    println!("Architecture: {}", info.arch);
    if let Some(distro) = &info.distro {
        println!("Distribution: {}", distro);
    }
    println!("Shell: {}", info.shell);
    println!("Home directory: {}", info.home_dir);
}

/// Print installation status
pub fn print_installation_status(status: &InstallationStatus) {
    println!("\n📋 Installation Status");
    println!("======================");
    println!("Rust: {}", if status.rust_installed { "✅ Installed" } else { "❌ Not installed" });
    println!("Cargo: {}", if status.cargo_installed { "✅ Installed" } else { "❌ Not installed" });
    println!("Git: {}", if status.git_installed { "✅ Installed" } else { "❌ Not installed" });
    println!("Build tools: {}", if status.build_tools_installed { "✅ Installed" } else { "❌ Not installed" });
    println!("Nockchain binaries: {}", if status.nockchain_binaries_available { "✅ Available" } else { "❌ Not available" });
}

/// Print next steps
fn print_next_steps() {
    println!("\n🎉 Next Steps");
    println!("=============");
    println!("1. Restart your terminal or run: source ~/.cargo/env");
    println!("2. Generate wallet keys: nockit wallet keygen");
    println!("3. Start mining: nockit mining start --pubkey <YOUR_PUBKEY>");
    println!("4. Monitor system: nockit monitor");
    println!("5. Check logs: nockit logs tail --follow");
    println!();
    println!("📚 Documentation: See README.md for detailed usage");
    println!("🆘 Support: Run 'nockit --help' for available commands");
} 