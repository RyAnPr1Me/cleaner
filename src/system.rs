use anyhow::Result;
use colored::*;
use sysinfo::{System, Disks};

pub fn show_system_info() -> Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let disks = Disks::new_with_refreshed_list();

    println!("{}", "═══════════════════════════════════════".bright_cyan());
    println!("{}", "        SYSTEM INFORMATION".bright_cyan().bold());
    println!("{}", "═══════════════════════════════════════".bright_cyan());

    // OS Information
    println!();
    println!("{}", "🖥️  Operating System:".bright_yellow());
    println!("   Name:     {}", System::name().unwrap_or_else(|| "Unknown".to_string()));
    println!("   Version:  {}", System::os_version().unwrap_or_else(|| "Unknown".to_string()));
    println!("   Kernel:   {}", System::kernel_version().unwrap_or_else(|| "Unknown".to_string()));
    println!("   Hostname: {}", System::host_name().unwrap_or_else(|| "Unknown".to_string()));

    // CPU Information
    println!();
    println!("{}", "🔧 CPU Information:".bright_yellow());
    let cpus = sys.cpus();
    if !cpus.is_empty() {
        println!("   Model:    {}", cpus[0].brand());
        println!("   Cores:    {}", cpus.len());
        let avg_usage: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32;
        let usage_color = get_usage_color(avg_usage);
        println!("   Usage:    {}", format!("{:.1}%", avg_usage).color(usage_color));
    }

    // Memory Information
    println!();
    println!("{}", "💾 Memory Information:".bright_yellow());
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let mem_usage = (used_mem as f64 / total_mem as f64) * 100.0;
    let mem_color = get_usage_color(mem_usage as f32);
    
    println!("   Total:    {}", format_bytes(total_mem));
    println!("   Used:     {}", format_bytes(used_mem));
    println!("   Free:     {}", format_bytes(sys.available_memory()));
    println!("   Usage:    {}", format!("{:.1}%", mem_usage).color(mem_color));

    // Swap Information
    let total_swap = sys.total_swap();
    if total_swap > 0 {
        let used_swap = sys.used_swap();
        let swap_usage = (used_swap as f64 / total_swap as f64) * 100.0;
        println!();
        println!("{}", "💿 Swap Information:".bright_yellow());
        println!("   Total:    {}", format_bytes(total_swap));
        println!("   Used:     {}", format_bytes(used_swap));
        println!("   Usage:    {:.1}%", swap_usage);
    }

    // Disk Information
    println!();
    println!("{}", "💽 Disk Information:".bright_yellow());
    for disk in disks.list() {
        let total_space = disk.total_space();
        let available_space = disk.available_space();
        let used_space = total_space - available_space;
        let usage = if total_space > 0 {
            (used_space as f64 / total_space as f64) * 100.0
        } else {
            0.0
        };
        let disk_color = get_usage_color(usage as f32);
        
        println!("   Mount:    {}", disk.mount_point().display());
        println!("   Total:    {}", format_bytes(total_space));
        println!("   Used:     {}", format_bytes(used_space));
        println!("   Available: {}", format_bytes(available_space));
        println!("   Usage:    {}", format!("{:.1}%", usage).color(disk_color));
        println!();
    }

    // Process Information
    println!("{}", "⚙️  Process Information:".bright_yellow());
    let process_count = sys.processes().len();
    println!("   Total Processes: {}", process_count);

    // Top processes by memory
    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| b.memory().cmp(&a.memory()));
    
    println!();
    println!("   Top 5 Memory Consumers:");
    for (i, process) in processes.iter().take(5).enumerate() {
        println!("     {}. {} - {}", 
            i + 1, 
            process.name(),
            format_bytes(process.memory())
        );
    }

    println!("{}", "═══════════════════════════════════════".bright_cyan());

    Ok(())
}

fn format_bytes(bytes: u64) -> String {
    use humansize::{format_size, BINARY};
    format_size(bytes, BINARY)
}

fn get_usage_color(usage: f32) -> &'static str {
    if usage < 50.0 {
        "green"
    } else if usage < 80.0 {
        "yellow"
    } else {
        "red"
    }
}
