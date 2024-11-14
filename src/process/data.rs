use procfs::process::*;
use std::fs::File;
use std::io::{self, BufRead};

pub struct ProcessUsage {
    pub pid: i32,
    pub name: String,
    pub cpu_usage: f64,      // Now stores CPU percentage as f64
    pub memory_usage: f64,   // Now stores memory percentage as f64
}

fn calculate_cpu_usage_percentage(process_cpu_ticks: u64, total_cpu_ticks: u64) -> f64 {
    if total_cpu_ticks == 0 {
        0.0
    } else {
        (process_cpu_ticks as f64 / total_cpu_ticks as f64) * 100.0
    }
}

fn calculate_memory_usage_percentage(process_memory: u64, total_used_memory: u64) -> f64 {
    if total_used_memory == 0 {
        0.0
    } else {
        (process_memory as f64 / total_used_memory as f64) * 100.0
    }
}

fn get_total_cpu_ticks() -> u64 {
    if let Ok(file) = File::open("/proc/stat") {
        let reader = io::BufReader::new(file);
        if let Some(Ok(line)) = reader.lines().next() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts[0] == "cpu" {
                return parts.iter().skip(1).filter_map(|v| v.parse::<u64>().ok()).sum();
            }
        }
    }
    0
}

pub fn get_processes() -> Vec<ProcessUsage> {
    let mut processes = Vec::new();
    let mut total_used_memory: u64 = 0;

    for process_result in all_processes().unwrap() {
        if let Ok(process) = process_result {
            if let Ok(stat) = process.stat() {
                let cpu_usage = stat.utime + stat.stime;
                let memory_usage = stat.vsize / 1024;  // Convert to KB
                total_used_memory += memory_usage;

                processes.push(ProcessUsage {
                    pid: stat.pid,
                    name: stat.comm.clone(),
                    cpu_usage: cpu_usage as f64,  // Temporarily store raw CPU ticks
                    memory_usage: memory_usage as f64,  // Temporarily store raw memory usage
                });
            }
        }
    }

    let total_cpu_ticks = get_total_cpu_ticks();

    // Calculate CPU and memory percentages
    for process in &mut processes {
        process.cpu_usage = calculate_cpu_usage_percentage(process.cpu_usage as u64, total_cpu_ticks);
        process.memory_usage = calculate_memory_usage_percentage(process.memory_usage as u64, total_used_memory);
    }

    processes
}

