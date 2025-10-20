# Cleaner 🚀

A universal Rust-based computer performance improver - an advanced system optimization tool designed to monitor, clean, and optimize your computer's performance.

## Features ✨

- **Real-time System Monitoring** 📊
  - CPU usage tracking
  - Memory consumption analysis
  - Disk space monitoring
  - Process management insights

- **Intelligent Cleanup** 🧹
  - Temporary file removal
  - Cache clearing
  - Safe file deletion with dry-run mode
  - **Runs entirely in user space - no elevated privileges required**
  - Cross-platform support (Linux, macOS, Windows)

- **Performance Optimization** ⚡
  - Multi-level optimization (Basic, Moderate, Aggressive)
  - Memory analysis and recommendations
  - Process analysis
  - System resource recommendations
  - **All operations safe for user space - no system modifications**

- **Comprehensive System Information** 💻
  - OS details
  - Hardware specifications
  - Resource utilization
  - Top memory consumers

## User Space Operation 🔒

**Cleaner is designed to run completely in user space without requiring elevated privileges:**

- ✅ **No sudo/administrator required** - All operations work with standard user permissions
- ✅ **Safe by design** - Only accesses user-owned directories and files
- ✅ **Read-only system analysis** - System monitoring and optimization use read-only operations
- ✅ **User directories only** - Cleanup targets only user-accessible temporary files and caches

### Platform-Specific User Directories

**Linux:**
- `$TMPDIR` or `~/.tmp` - User temporary files
- `~/.cache` - User cache directory
- `~/.local/share/Trash/files` - User trash

**macOS:**
- `$TMPDIR` - User temporary files
- `~/.Trash` - User trash
- `~/Library/Caches` - User caches
- `~/Library/Logs` - User logs

**Windows:**
- `%TEMP%` - User temporary files
- `%TMP%` - Alternative temp directory
- `%LOCALAPPDATA%\Temp` - Local AppData temp
- `%LOCALAPPDATA%\Microsoft\Windows\INetCache` - Internet cache
- User's Recycle Bin

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/RyAnPr1Me/cleaner.git
cd cleaner

# Build the project
cargo build --release

# The binary will be available at target/release/cleaner
```

## Usage

### Display Help

```bash
cleaner --help
```

### System Information

Display detailed system information including CPU, memory, disk, and running processes:

```bash
cleaner info
```

### Monitor System Performance

Monitor system resources in real-time:

```bash
# Monitor for 10 seconds (default)
cleaner monitor

# Monitor for 30 seconds
cleaner monitor -d 30

# Monitor continuously (Ctrl+C to stop)
cleaner monitor -d 0
```

### Clean Temporary Files

Safely remove temporary files and caches:

```bash
# Dry run - see what would be cleaned without deleting
cleaner clean --dry-run

# Actually perform the cleanup
cleaner clean
```

### Optimize System Performance

Apply performance optimizations:

```bash
# Basic optimization (level 1)
cleaner optimize -l 1

# Moderate optimization (level 2) - recommended
cleaner optimize -l 2

# Aggressive optimization (level 3)
cleaner optimize -l 3
```

### Full System Scan

Run a comprehensive system scan with information, monitoring, cleanup preview, and recommendations:

```bash
cleaner scan
```

### Verbose Logging

Enable verbose logging for any command:

```bash
cleaner -v info
cleaner --verbose monitor
```

## Commands Reference

| Command | Description | Options |
|---------|-------------|---------|
| `info` | Show detailed system information | - |
| `monitor` | Real-time system performance monitoring | `-d, --duration <SECONDS>` (default: 10) |
| `clean` | Clean temporary files and caches | `--dry-run` (preview without deleting) |
| `optimize` | Optimize system performance | `-l, --level <1-3>` (default: 2) |
| `scan` | Full system scan and report | - |

## Optimization Levels

All optimization levels run in user space and perform analysis only - no system modifications require elevated privileges.

### Level 1 - Basic (User Space)
- Memory usage analysis
- Basic process analysis
- Memory recommendations
- Safe, read-only operations

### Level 2 - Moderate (Recommended, User Space)
- All Level 1 optimizations
- Process priority analysis
- CPU affinity analysis
- Disk I/O analysis
- Idle process identification

### Level 3 - Aggressive (User Space)
- All Level 1 & 2 optimizations
- Advanced memory analysis
- Process consolidation recommendations
- Duplicate process detection
- System service analysis

**Note:** All optimization operations are non-invasive and run with standard user permissions. The tool provides recommendations and analysis without requiring elevated privileges.

## Platform Support

**Fully tested and verified on:**
- ✅ **Linux** (Ubuntu, Debian, Fedora, Arch, etc.) - Works with standard user permissions
- ✅ **macOS** - Compatible with all modern versions
- ✅ **Windows** - Windows 10, 11, and Server editions

**Cross-platform features:**
- Platform-specific cleanup targets
- Automatic detection of user directories
- Environment variable resolution
- Graceful permission handling

## Requirements

- Rust 1.70 or later
- Cargo (comes with Rust)

## Dependencies

This project uses the following high-quality Rust crates:
- `sysinfo` - System information gathering
- `clap` - Command-line argument parsing
- `colored` - Terminal color output
- `indicatif` - Progress bars and spinners
- `walkdir` - Directory traversal
- `humansize` - Human-readable file sizes
- `chrono` - Date and time handling
- `anyhow` - Error handling
- `log` & `env_logger` - Logging

## Safety

The cleaner tool is designed with safety and user-space operation in mind:
- **No elevated privileges required** - Runs entirely in user space
- **Dry-run mode** for previewing changes before cleanup
- **Only targets user-accessible directories** - No system directories
- **No system-critical files** are touched
- **Cross-platform compatibility** with graceful permission handling
- **Safe deletion practices** with error handling
- **Read-only system analysis** - Monitoring and optimization don't modify system state
- Safe deletion practices
- Cross-platform compatibility checks

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see LICENSE file for details

## Author

RyAnPr1Me

## Disclaimer

While this tool is designed to be safe, always:
- Back up important data before running system optimizations
- Test with `--dry-run` first when cleaning files
- Understand what each optimization level does
- Use at your own risk

## Screenshots

```
╔═══════════════════════════════════════════════════════╗
║     CLEANER - Performance Optimization Tool          ║
╚═══════════════════════════════════════════════════════╝
```

The tool provides beautiful, colored terminal output with:
- Clear visual hierarchy
- Color-coded warnings and success messages
- Progress bars for long-running operations
- Emoji icons for better readability
- Detailed performance metrics