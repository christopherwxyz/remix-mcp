//! `AbletonOSC` Remote Script installer.
//!
//! Handles detection and installation of the `AbletonOSC` Remote Script
//! to Ableton Live's User Library.

use std::fs;
use std::path::PathBuf;

use color_eyre::eyre::{Context, ContextCompat, Result, bail};
use console::{Emoji, style};
use indicatif::{ProgressBar, ProgressStyle};

/// The name of the Remote Script folder.
const REMOTE_SCRIPT_NAME: &str = "AbletonOSC";

/// Gets the path to the bundled `AbletonOSC` source.
///
/// This looks for the submodule relative to the executable.
pub fn bundled_source_path() -> Result<PathBuf> {
    // Try relative to executable first (for installed binaries)
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let bundled = exe_dir.join(REMOTE_SCRIPT_NAME);
            if bundled.exists() && bundled.join("__init__.py").exists() {
                return Ok(bundled);
            }

            // Check one level up (for cargo run scenarios)
            if let Some(parent) = exe_dir.parent() {
                // Could be in target/debug or target/release
                if let Some(grandparent) = parent.parent() {
                    let bundled = grandparent.join(REMOTE_SCRIPT_NAME);
                    if bundled.exists() && bundled.join("__init__.py").exists() {
                        return Ok(bundled);
                    }
                }
            }
        }
    }

    // Try current working directory
    let cwd = std::env::current_dir()?;
    let bundled = cwd.join(REMOTE_SCRIPT_NAME);
    if bundled.exists() && bundled.join("__init__.py").exists() {
        return Ok(bundled);
    }

    // Try CARGO_MANIFEST_DIR for development
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let bundled = PathBuf::from(manifest_dir).join(REMOTE_SCRIPT_NAME);
        if bundled.exists() && bundled.join("__init__.py").exists() {
            return Ok(bundled);
        }
    }

    bail!(
        "Could not find bundled AbletonOSC. Ensure the submodule is initialized: \
         git submodule update --init"
    )
}

/// Gets the Ableton User Library Remote Scripts path for the current OS.
pub fn remote_scripts_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;

    #[cfg(target_os = "macos")]
    let path = home.join("Music/Ableton/User Library/Remote Scripts");

    #[cfg(target_os = "windows")]
    let path = home.join("Documents/Ableton/User Library/Remote Scripts");

    #[cfg(target_os = "linux")]
    let path = {
        // Linux is not officially supported by Ableton, but some users run it via Wine
        // Check common locations
        let wine_path = home
            .join(".wine/drive_c/users")
            .join(whoami::username())
            .join("Documents/Ableton/User Library/Remote Scripts");
        if wine_path.exists() {
            wine_path
        } else {
            // Fallback to a standard location
            home.join(".ableton/remote-scripts")
        }
    };

    Ok(path)
}

/// Gets the installation destination path.
pub fn install_destination() -> Result<PathBuf> {
    Ok(remote_scripts_path()?.join(REMOTE_SCRIPT_NAME))
}

/// Checks if `AbletonOSC` is already installed.
pub fn is_installed() -> Result<bool> {
    let dest = install_destination()?;
    Ok(dest.exists() && dest.join("__init__.py").exists())
}

// Emoji constants for terminal output
static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍 ", "");
static PACKAGE: Emoji<'_, '_> = Emoji("📦 ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "");
static FOLDER: Emoji<'_, '_> = Emoji("📁 ", "");
static TRASH: Emoji<'_, '_> = Emoji("🗑️  ", "");

/// Installs `AbletonOSC` to the Remote Scripts folder.
///
/// Returns the installation path on success.
pub fn install(force: bool) -> Result<PathBuf> {
    eprintln!(
        "{}{} Locating bundled AbletonOSC...",
        LOOKING_GLASS,
        style("Step 1/3").bold().dim()
    );
    let source = bundled_source_path()?;

    let dest = install_destination()?;

    // Check if already installed
    if dest.exists() {
        if force {
            eprintln!(
                "{}{} Removing existing installation...",
                TRASH,
                style("Step 2/3").bold().dim()
            );
            fs::remove_dir_all(&dest).with_context(|| {
                format!(
                    "Failed to remove existing installation at {}",
                    dest.display()
                )
            })?;
        } else {
            bail!(
                "AbletonOSC is already installed at {}. Use --force to reinstall.",
                dest.display()
            );
        }
    } else {
        eprintln!(
            "{}{} Preparing installation directory...",
            FOLDER,
            style("Step 2/3").bold().dim()
        );
    }

    // Ensure parent directory exists
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "Failed to create Remote Scripts directory at {}",
                parent.display()
            )
        })?;
    }

    eprintln!(
        "{}{} Installing AbletonOSC...",
        PACKAGE,
        style("Step 3/3").bold().dim()
    );

    // Count files for progress bar
    let file_count = count_files(&source)?;
    let pb = ProgressBar::new(file_count);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓░"),
    );

    // Copy the directory recursively with progress
    copy_dir_recursive_with_progress(&source, &dest, &pb)
        .with_context(|| format!("Failed to copy AbletonOSC to {}", dest.display()))?;

    pb.finish_and_clear();
    eprintln!(
        "{} {} installed to {}",
        SPARKLE,
        style("AbletonOSC").green().bold(),
        style(dest.display()).cyan()
    );

    Ok(dest)
}

/// Counts files in a directory recursively (excluding hidden/skipped).
fn count_files(dir: &PathBuf) -> Result<u64> {
    let mut count = 0;
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        // Skip hidden files and excluded directories
        let name = entry.file_name();
        if name.to_string_lossy().starts_with('.')
            || name == "tests"
            || name == "client"
            || name == ".github"
        {
            continue;
        }

        if file_type.is_dir() {
            count += count_files(&entry.path())?;
        } else {
            count += 1;
        }
    }
    Ok(count)
}

/// Recursively copies a directory with progress reporting.
fn copy_dir_recursive_with_progress(src: &PathBuf, dst: &PathBuf, pb: &ProgressBar) -> Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        // Skip hidden files and directories (like .git)
        if entry.file_name().to_string_lossy().starts_with('.') {
            continue;
        }

        // Skip test and client directories (not needed for Remote Script)
        let name = entry.file_name();
        if name == "tests" || name == "client" || name == ".github" {
            continue;
        }

        if file_type.is_dir() {
            copy_dir_recursive_with_progress(&src_path, &dst_path, pb)?;
        } else {
            pb.set_message(name.to_string_lossy().to_string());
            fs::copy(&src_path, &dst_path)?;
            pb.inc(1);
        }
    }

    Ok(())
}

/// Returns installation status information.
pub struct InstallStatus {
    pub is_installed: bool,
    pub install_path: PathBuf,
    pub bundled_available: bool,
}

/// Gets the current installation status.
pub fn status() -> Result<InstallStatus> {
    let install_path = install_destination()?;
    let is_installed = install_path.exists() && install_path.join("__init__.py").exists();
    let bundled_available = bundled_source_path().is_ok();

    Ok(InstallStatus {
        is_installed,
        install_path,
        bundled_available,
    })
}

/// Prints post-installation instructions.
pub fn print_post_install_instructions() {
    eprintln!();
    eprintln!(
        "{} To finish setup:",
        style("Installation complete!").green().bold()
    );
    eprintln!();
    eprintln!(
        "  {}  Restart Ableton Live (if running)",
        style("1.").cyan().bold()
    );
    eprintln!(
        "  {}  Open {} ({} on macOS, {} on Windows)",
        style("2.").cyan().bold(),
        style("Preferences").yellow(),
        style("Cmd+,").dim(),
        style("Ctrl+,").dim()
    );
    eprintln!(
        "  {}  Go to {} tab",
        style("3.").cyan().bold(),
        style("Link/Tempo/MIDI").yellow()
    );
    eprintln!(
        "  {}  Under Control Surface, select {}",
        style("4.").cyan().bold(),
        style("'AbletonOSC'").green()
    );
    eprintln!();
    eprintln!(
        "You should see: {}",
        style("'AbletonOSC: Listening for OSC on port 11000'").green()
    );
    eprintln!();
}
