use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use log::{info, warn};
use std::process;

mod system;
mod cleaner;
mod optimizer;
mod monitor;

#[derive(Parser)]
#[command(name = "cleaner")]
#[command(version = "0.1.0")]
#[command(about = "A universal Rust-based computer performance improver", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Monitor system performance and resources
    Monitor {
        /// Duration in seconds to monitor (0 for continuous)
        #[arg(short, long, default_value = "10")]
        duration: u64,
    },
    /// Clean temporary files and caches
    Clean {
        /// Dry run - don't actually delete files
        #[arg(short, long)]
        dry_run: bool,
    },
    /// Optimize system performance
    Optimize {
        /// Optimization level (1-3)
        #[arg(short, long, default_value = "2")]
        level: u8,
    },
    /// Show system information
    Info,
    /// Full system scan and report
    Scan,
}

fn main() {
    let cli = Cli::parse();

    // Initialize logger
    if cli.verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    println!("{}", "╔═══════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║     CLEANER - Performance Optimization Tool          ║".bright_cyan());
    println!("{}", "╚═══════════════════════════════════════════════════════╝".bright_cyan());
    println!();

    let result = match cli.command {
        Commands::Monitor { duration } => {
            info!("Starting system monitor for {} seconds", duration);
            monitor::run_monitor(duration)
        }
        Commands::Clean { dry_run } => {
            info!("Starting system cleanup (dry_run: {})", dry_run);
            cleaner::run_cleanup(dry_run)
        }
        Commands::Optimize { level } => {
            info!("Starting system optimization (level: {})", level);
            optimizer::run_optimization(level)
        }
        Commands::Info => {
            info!("Displaying system information");
            system::show_system_info()
        }
        Commands::Scan => {
            info!("Running full system scan");
            run_full_scan()
        }
    };

    match result {
        Ok(_) => {
            println!();
            println!("{}", "✓ Operation completed successfully!".bright_green());
            process::exit(0);
        }
        Err(e) => {
            eprintln!();
            eprintln!("{}", format!("✗ Error: {}", e).bright_red());
            warn!("Operation failed: {}", e);
            process::exit(1);
        }
    }
}

fn run_full_scan() -> Result<()> {
    println!("{}", "\n📊 Running Full System Scan...".bright_yellow());
    
    // Show system info
    system::show_system_info()?;
    
    // Monitor for a brief period
    println!();
    println!("{}", "📈 Monitoring System Performance...".bright_yellow());
    monitor::run_monitor(5)?;
    
    // Scan for cleanable files
    println!();
    println!("{}", "🧹 Scanning for Cleanable Files...".bright_yellow());
    cleaner::run_cleanup(true)?;
    
    // Show optimization recommendations
    println!();
    println!("{}", "💡 Optimization Recommendations:".bright_yellow());
    optimizer::show_recommendations()?;
    
    Ok(())
}
