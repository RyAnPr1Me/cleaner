use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use sysinfo::System;
use std::thread;
use std::time::Duration;

pub fn run_monitor(duration: u64) -> Result<()> {
    let mut sys = System::new_all();
    
    println!("{}", "═══════════════════════════════════════".bright_cyan());
    println!("{}", "      REAL-TIME SYSTEM MONITOR".bright_cyan().bold());
    println!("{}", "═══════════════════════════════════════".bright_cyan());
    println!();

    if duration == 0 {
        println!("{}", "Monitoring continuously (Press Ctrl+C to stop)...".bright_yellow());
        loop {
            display_metrics(&mut sys);
            thread::sleep(Duration::from_secs(2));
        }
    } else {
        let pb = ProgressBar::new(duration);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}s {msg}")
                .unwrap()
                .progress_chars("#>-")
        );

        for i in 0..duration {
            display_metrics(&mut sys);
            pb.set_position(i + 1);
            pb.set_message(format!("Monitoring..."));
            thread::sleep(Duration::from_secs(1));
        }

        pb.finish_with_message("Monitoring complete");
    }

    println!();
    print_summary(&sys);

    Ok(())
}

fn display_metrics(sys: &mut System) {
    sys.refresh_all();

    // CPU Usage
    let cpus = sys.cpus();
    let avg_cpu: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32;
    
    // Memory Usage
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let mem_usage = (used_mem as f64 / total_mem as f64) * 100.0;

    // Print inline metrics
    print!("\r{} CPU: {} | Memory: {} ({}) / {} ({}%)   ",
        "📊".to_string(),
        format!("{:5.1}%", avg_cpu).color(get_usage_color(avg_cpu)),
        format_bytes(used_mem),
        format!("{:.1}%", mem_usage).color(get_usage_color(mem_usage as f32)),
        format_bytes(total_mem),
        format!("{:.1}", mem_usage)
    );
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}

fn print_summary(sys: &System) {
    println!();
    println!("{}", "═══════════════════════════════════════".bright_cyan());
    println!("{}", "         MONITORING SUMMARY".bright_cyan().bold());
    println!("{}", "═══════════════════════════════════════".bright_cyan());
    
    let cpus = sys.cpus();
    let avg_cpu: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32;
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let mem_usage = (used_mem as f64 / total_mem as f64) * 100.0;

    println!();
    println!("  Average CPU Usage: {}", format!("{:.1}%", avg_cpu).color(get_usage_color(avg_cpu)));
    println!("  Memory Usage:      {}", format!("{:.1}%", mem_usage).color(get_usage_color(mem_usage as f32)));
    println!("  Active Processes:  {}", sys.processes().len());
    
    // Performance Assessment
    println!();
    println!("{}", "  Performance Assessment:".bright_yellow());
    
    if avg_cpu > 80.0 || mem_usage > 80.0 {
        println!("  {}", "⚠️  High resource usage detected!".bright_red());
        println!("  {}", "Consider running optimization or closing unused applications.".yellow());
    } else if avg_cpu > 50.0 || mem_usage > 50.0 {
        println!("  {}", "⚡ Moderate resource usage.".bright_yellow());
        println!("  {}", "System is performing adequately.".yellow());
    } else {
        println!("  {}", "✓ Excellent! System resources are optimal.".bright_green());
    }
    
    println!("{}", "═══════════════════════════════════════".bright_cyan());
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
