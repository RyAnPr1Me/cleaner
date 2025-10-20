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

    // Platform-specific temporary directories (user space only)
    #[cfg(target_os = "linux")]
    {
        if let Ok(home) = std::env::var("HOME") {
            // User's temporary directory - safe to access without root
            if let Ok(tmpdir) = std::env::var("TMPDIR") {
                targets.push(CleanupTarget {
                    path: PathBuf::from(tmpdir),
                    description: "User temporary files".to_string(),
                    patterns: vec!["*".to_string()],
                    safe: true,
                });
            }
            
            // User cache directory
            targets.push(CleanupTarget {
                path: PathBuf::from(format!("{}/.cache", home)),
                description: "User cache directory".to_string(),
                patterns: vec!["*".to_string()],
                safe: true,
            });

            // User trash directory
            targets.push(CleanupTarget {
                path: PathBuf::from(format!("{}/.local/share/Trash/files", home)),
                description: "Trash directory".to_string(),
                patterns: vec!["*".to_string()],
                safe: true,
            });

            // User's .tmp directory if it exists
            targets.push(CleanupTarget {
                path: PathBuf::from(format!("{}/.tmp", home)),
                description: "User .tmp directory".to_string(),
                patterns: vec!["*".to_string()],
                safe: true,
            });
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            // User's temporary directory
            if let Ok(tmpdir) = std::env::var("TMPDIR") {
                targets.push(CleanupTarget {
                    path: PathBuf::from(tmpdir),
                    description: "User temporary files".to_string(),
                    patterns: vec!["*".to_string()],
                    safe: true,
                });
            }

            // User trash directory
            targets.push(CleanupTarget {
                path: PathBuf::from(format!("{}/.Trash", home)),
                description: "Trash directory".to_string(),
                patterns: vec!["*".to_string()],
                safe: true,
            });

            // User caches
            targets.push(CleanupTarget {
                path: PathBuf::from(format!("{}/Library/Caches", home)),
                description: "User caches".to_string(),
                patterns: vec!["*".to_string()],
                safe: true,
            });

            // User logs
            targets.push(CleanupTarget {
                path: PathBuf::from(format!("{}/Library/Logs", home)),
                description: "User logs".to_string(),
                patterns: vec!["*".to_string()],
                safe: true,
            });
        }
    }

    #[cfg(target_os = "windows")]
    {
        // User's temporary directory
        if let Ok(temp) = std::env::var("TEMP") {
            targets.push(CleanupTarget {
                path: PathBuf::from(temp),
                description: "User temporary files".to_string(),
                patterns: vec!["*".to_string()],
                safe: true,
            });
        }

        // Alternative user temp directory
        if let Ok(tmp) = std::env::var("TMP") {
            let tmp_path = PathBuf::from(&tmp);
            // Only add if it's different from TEMP
            if !targets.iter().any(|t| t.path == tmp_path) {
                targets.push(CleanupTarget {
                    path: tmp_path,
                    description: "User TMP files".to_string(),
                    patterns: vec!["*".to_string()],
                    safe: true,
                });
            }
        }

        // Local AppData temp
        if let Ok(appdata) = std::env::var("LOCALAPPDATA") {
            targets.push(CleanupTarget {
                path: PathBuf::from(format!("{}\\Temp", appdata)),
                description: "Local AppData temp files".to_string(),
                patterns: vec!["*".to_string()],
                safe: true,
            });

            // Windows update cache (user accessible)
            targets.push(CleanupTarget {
                path: PathBuf::from(format!("{}\\Microsoft\\Windows\\INetCache", appdata)),
                description: "Internet cache".to_string(),
                patterns: vec!["*".to_string()],
                safe: true,
            });
        }

        // Recycle bin for current user
        if let Ok(userprofile) = std::env::var("USERPROFILE") {
            targets.push(CleanupTarget {
                path: PathBuf::from(format!("{}\\$Recycle.Bin", userprofile)),
                description: "Recycle Bin".to_string(),
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

    // Check if we have read permissions before trying to scan
    if let Err(_) = std::fs::read_dir(path) {
        // If we can't read the directory, return 0 instead of failing
        return Ok((0, 0));
    }

    for entry in WalkDir::new(path)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok()) // Skip entries we can't access
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
    // Check if we have write permissions before trying to clean
    if let Err(_) = std::fs::read_dir(path) {
        // If we can't read the directory, skip it
        return Ok(());
    }

    for entry in WalkDir::new(path)
        .max_depth(3)
        .into_iter()
        .filter_map(|e| e.ok()) // Skip entries we can't access
    {
        if entry.file_type().is_file() {
            // Silently skip files we can't delete (permission errors)
            let _ = fs::remove_file(entry.path());
        }
    }

    Ok(())
}

fn format_bytes(bytes: u64) -> String {
    use humansize::{format_size, BINARY};
    format_size(bytes, BINARY)
}
