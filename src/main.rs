//! Ableton MCP Server - Entry point.
//!
//! This binary provides an MCP server that communicates via stdio and
//! controls Ableton Live via OSC using the `AbletonOSC` Remote Script.

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use console::style;
use rmcp::ServiceExt;
use tracing::{info, warn};
use tracing_subscriber::{EnvFilter, fmt};

use remix_mcp::{AbletonServer, installer};

#[derive(Parser)]
#[command(name = "remix-mcp")]
#[command(about = "MCP server for controlling Ableton Live via OSC")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Start the MCP server (default)
    Serve {
        /// Skip the `AbletonOSC` installation check
        #[arg(long)]
        skip_install_check: bool,
    },

    /// Install `AbletonOSC` Remote Script to Ableton's User Library
    Install {
        /// Force reinstall even if already installed
        #[arg(long, short)]
        force: bool,
    },

    /// Check `AbletonOSC` installation status
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Install { force }) => cmd_install(force),
        Some(Command::Status) => cmd_status(),
        Some(Command::Serve { skip_install_check }) => cmd_serve(skip_install_check).await,
        None => cmd_serve(false).await,
    }
}

fn cmd_install(force: bool) -> Result<()> {
    eprintln!(
        "{} Installing AbletonOSC Remote Script...",
        style("remix-mcp").cyan().bold()
    );
    eprintln!();

    installer::install(force)?;
    installer::print_post_install_instructions();

    Ok(())
}

fn cmd_status() -> Result<()> {
    let status = installer::status()?;

    eprintln!("{} Installation Status", style("AbletonOSC").cyan().bold());
    eprintln!("{}", style("═".repeat(35)).dim());
    eprintln!();

    let installed_str = if status.is_installed {
        style("✓ Installed").green().to_string()
    } else {
        style("✗ Not installed").red().to_string()
    };
    eprintln!("  Status:         {installed_str}");
    eprintln!(
        "  Install path:   {}",
        style(status.install_path.display()).dim()
    );

    let bundled_str = if status.bundled_available {
        style("Available").green().to_string()
    } else {
        style("Not found").red().to_string()
    };
    eprintln!("  Bundled source: {bundled_str}");

    if !status.is_installed {
        eprintln!();
        eprintln!("  Run {} to install.", style("remix-mcp install").yellow());
    }
    eprintln!();

    Ok(())
}

async fn cmd_serve(skip_install_check: bool) -> Result<()> {
    // Initialize logging to stderr (stdout is reserved for MCP JSON-RPC)
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    // Check if AbletonOSC is installed (unless skipped)
    if !skip_install_check {
        match installer::status() {
            Ok(status) if !status.is_installed => {
                warn!("AbletonOSC Remote Script is not installed");

                eprintln!();

                // Check if we can auto-install
                if status.bundled_available {
                    eprintln!("AbletonOSC is not installed. Installing now...");
                    eprintln!();

                    match installer::install(false) {
                        Ok(path) => {
                            eprintln!("Installed to: {}", path.display());
                            installer::print_post_install_instructions();
                            eprintln!("Starting MCP server...");
                            eprintln!();
                        }
                        Err(e) => {
                            warn!("Auto-install failed: {}", e);
                            eprintln!();
                            eprintln!("Could not auto-install AbletonOSC: {e}");
                            eprintln!("Please run 'remix-mcp install' manually.");
                            eprintln!();
                            eprintln!("Continuing anyway...");
                            eprintln!();
                        }
                    }
                } else {
                    eprintln!("Warning: AbletonOSC Remote Script is not installed.");
                    eprintln!("The MCP server will start, but Ableton control won't work");
                    eprintln!("until AbletonOSC is installed and enabled in Ableton Live.");
                    eprintln!();
                    eprintln!("Install from: https://github.com/ideoforms/AbletonOSC");
                    eprintln!();
                }
            }
            Ok(_) => {
                info!("AbletonOSC is installed");
            }
            Err(e) => {
                warn!("Could not check AbletonOSC status: {}", e);
            }
        }
    }

    info!("Starting Ableton MCP Server v{}", env!("CARGO_PKG_VERSION"));

    // Create the server
    let server = AbletonServer::new().await?;

    // Run the server with stdio transport
    let service = server.serve(rmcp::transport::stdio()).await?;

    info!("Server running, waiting for requests...");

    // Wait for the service to complete
    service.waiting().await?;

    info!("Server shutting down");
    Ok(())
}
