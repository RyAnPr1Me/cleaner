# Technical Architecture

## Overview

Cleaner is a modular Rust application designed for cross-platform system performance monitoring and optimization.

## Project Structure

```
cleaner/
├── src/
│   ├── main.rs           # CLI entry point and command routing
│   ├── system.rs         # System information module
│   ├── monitor.rs        # Real-time monitoring module
│   ├── cleaner.rs        # File cleanup module
│   └── optimizer.rs      # Performance optimization module
├── Cargo.toml            # Project configuration and dependencies
├── README.md             # User documentation
├── LICENSE               # MIT License
├── CONTRIBUTING.md       # Contribution guidelines
└── .gitignore           # Git ignore patterns

```

## Module Description

### main.rs
- **Purpose**: Application entry point and CLI interface
- **Key Features**:
  - Command-line argument parsing using `clap`
  - Subcommand routing (monitor, clean, optimize, info, scan)
  - Global options (verbose logging)
  - Error handling and exit codes
  - Beautiful CLI header display

### system.rs
- **Purpose**: Display comprehensive system information
- **Key Features**:
  - OS detection (name, version, kernel, hostname)
  - CPU information (model, cores, usage)
  - Memory metrics (total, used, free, percentage)
  - Swap information
  - Disk information for all mounted volumes
  - Process listing with top memory consumers
  - Color-coded output based on resource usage

### monitor.rs
- **Purpose**: Real-time system performance monitoring
- **Key Features**:
  - Continuous or timed monitoring
  - Live CPU and memory usage updates
  - Progress bar with elapsed time
  - Performance assessment
  - Inline metric display
  - Summary generation

### cleaner.rs
- **Purpose**: Safe system cleanup operations
- **Key Features**:
  - Platform-specific cleanup targets
  - Dry-run mode for safety
  - Temporary file scanning
  - Cache clearing
  - Directory traversal with depth limits
  - Size calculation and reporting
  - Safe deletion practices

### optimizer.rs
- **Purpose**: System performance optimization
- **Key Features**:
  - Multi-level optimization (1-3)
  - System state analysis
  - Process analysis (high memory, idle, duplicates)
  - Memory management recommendations
  - CPU affinity optimization hints
  - Disk I/O optimization
  - Intelligent recommendations

## Dependencies

### Core Dependencies
- **sysinfo** (0.30): Cross-platform system information library
- **clap** (4.5): Modern CLI argument parser with derive macros
- **colored** (2.1): Terminal color output
- **indicatif** (0.17): Progress bars and spinners
- **walkdir** (2.5): Efficient directory traversal
- **humansize** (2.1): Human-readable byte formatting
- **chrono** (0.4): Date and time handling
- **anyhow** (1.0): Ergonomic error handling
- **log** (0.4): Logging facade
- **env_logger** (0.11): Logger implementation

## Design Patterns

### Safety First
- Dry-run mode for destructive operations
- Safe defaults
- Clear warnings for high-impact operations
- Graceful error handling

### Cross-Platform
- Platform-specific code using Rust's conditional compilation
- Abstract interfaces for OS-specific operations
- Tested on Linux, macOS, and Windows

### Performance
- Release builds with LTO (Link Time Optimization)
- Optimized for size and speed
- Minimal dependencies
- Efficient algorithms

### User Experience
- Color-coded output for quick scanning
- Progress indicators for long operations
- Clear error messages
- Comprehensive help text
- Emoji icons for visual appeal

## Build Configuration

### Release Profile
```toml
[profile.release]
opt-level = 3        # Maximum optimization
lto = true           # Link Time Optimization
codegen-units = 1    # Better optimization
strip = true         # Strip symbols
```

This configuration produces a compact, fast binary (~2.2MB).

## Error Handling

The application uses `anyhow::Result` for error propagation, providing:
- Rich error context
- Stack traces in debug mode
- User-friendly error messages
- Proper exit codes (0 for success, 1 for errors)

## Logging

Two logging levels:
- **Normal**: Info level, shows key operations
- **Verbose** (`-v`): Debug level, shows detailed information

## Future Enhancements

Potential areas for expansion:
- GUI interface
- Configuration file support
- Scheduled optimization
- Process management features
- Network monitoring
- Service/daemon mode
- Benchmark mode
- Export reports (JSON, CSV, HTML)
- Plugin system
- Remote monitoring
