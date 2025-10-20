use anyhow::Result;
use colored::*;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn run_cleanup(dry_run: bool) -> Result<()> {
    println!("{}", "═══════════════════════════════════════".bright_cyan());
    println!("{}", "       SYSTEM CLEANUP TOOL".bright_cyan().bold());
    println!("{}", "═══════════════════════════════════════".bright_cyan());
    println!();

    if dry_run {
        println!("{}", "🔍 DRY RUN MODE - No files will be deleted".bright_yellow());
        println!();
    }

    let mut total_size: u64 = 0;
    let mut total_files: usize = 0;

    // Define cleanup targets
    let cleanup_targets = get_cleanup_targets();

    println!("{}", "📁 Scanning for cleanable files...".bright_yellow());
    println!();

    for target in cleanup_targets {
        if target.path.exists() {
            println!("  Checking: {}", target.description.bright_cyan());
            
            let (size, count) = scan_directory(&target.path, &target.patterns)?;
            
            if count > 0 {
                println!("    Found: {} files ({}) {}", 
                    count.to_string().bright_green(),
                    format_bytes(size).bright_green(),
                    if target.safe { "✓".green() } else { "⚠️".yellow() }
                );
                total_size += size;
                total_files += count;

                if !dry_run && target.safe {
                    println!("    Cleaning...");
                    clean_directory(&target.path, &target.patterns)?;
                }
            } else {
                println!("    {}", "Already clean ✓".bright_green());
            }
            println!();
        }
    }

    println!("{}", "═══════════════════════════════════════".bright_cyan());
    println!("{}", "          CLEANUP SUMMARY".bright_cyan().bold());
    println!("{}", "═══════════════════════════════════════".bright_cyan());
    println!();
    println!("  Total Files Found:     {}", total_files.to_string().bright_yellow());
    println!("  Total Size:            {}", format_bytes(total_size).bright_yellow());
    
    if dry_run {
        println!();
        println!("{}", "💡 Run without --dry-run to actually clean these files".bright_cyan());
    } else {
        println!();
        println!("{}", "✓ Cleanup completed successfully!".bright_green());
    }
    
    println!("{}", "═══════════════════════════════════════".bright_cyan());

    Ok(())
}

struct CleanupTarget {
    path: PathBuf,
    description: String,
    patterns: Vec<String>,
    safe: bool,
}

fn get_cleanup_targets() -> Vec<CleanupTarget> {
    let mut targets = Vec::new();

    // Platform-specific temporary directories
    #[cfg(target_os = "linux")]
    {
        if let Ok(home) = std::env::var("HOME") {
            targets.push(CleanupTarget {
                path: PathBuf::from("/tmp"),
                description: "System temporary files".to_string(),
                patterns: vec!["*".to_string()],
                safe: true,
            });
            
            targets.push(CleanupTarget {
                path: PathBuf::from(format!("{}/.cache", home)),
                description: "User cache directory".to_string(),
                patterns: vec!["*".to_string()],
                safe: true,
            });

            targets.push(CleanupTarget {
                path: PathBuf::from(format!("{}/.local/share/Trash", home)),
                description: "Trash directory".to_string(),
                patterns: vec!["*".to_string()],
                safe: true,
            });
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            targets.push(CleanupTarget {
                path: PathBuf::from(format!("{}/.Trash", home)),
                description: "Trash directory".to_string(),
                patterns: vec!["*".to_string()],
                safe: true,
            });

            targets.push(CleanupTarget {
                path: PathBuf::from(format!("{}/Library/Caches", home)),
                description: "User caches".to_string(),
                patterns: vec!["*".to_string()],
                safe: true,
            });
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(temp) = std::env::var("TEMP") {
            targets.push(CleanupTarget {
                path: PathBuf::from(temp),
                description: "Temporary files".to_string(),
                patterns: vec!["*".to_string()],
                safe: true,
            });
        }

        if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
            targets.push(CleanupTarget {
                path: PathBuf::from(format!("{}\\Temp", appdata)),
                description: "Local temp files".to_string(),
                patterns: vec!["*".to_string()],
                safe: true,
            });
        }
    }

    targets
}

fn scan_directory(path: &PathBuf, _patterns: &[String]) -> Result<(u64, usize)> {
    let mut total_size: u64 = 0;
    let mut file_count: usize = 0;

    for entry in WalkDir::new(path)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                total_size += metadata.len();
                file_count += 1;
            }
        }
    }

    Ok((total_size, file_count))
}

fn clean_directory(path: &PathBuf, _patterns: &[String]) -> Result<()> {
    for entry in WalkDir::new(path)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let _ = fs::remove_file(entry.path());
        }
    }

    Ok(())
}

fn format_bytes(bytes: u64) -> String {
    use humansize::{format_size, BINARY};
    format_size(bytes, BINARY)
}
