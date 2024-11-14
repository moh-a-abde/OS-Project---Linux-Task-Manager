use procfs::process::*;
use std::io::{self, Write};
use std::cmp::Ordering;

struct ProcessUsage {
    pid: i32,
    name: String,
    cpu_usage: u64,       // CPU usage in cumulative ticks
    memory_usage: u64,    // Memory usage in kilobytes
}

fn get_processes() -> Vec<ProcessUsage> {
    let mut processes: Vec<ProcessUsage> = Vec::new();

    for process_result in all_processes().unwrap() {
        if let Ok(process) = process_result {
            if let Ok(stat) = process.stat() {
                let cpu_usage = stat.utime + stat.stime;     // Cumulative CPU time in ticks
                let memory_usage = stat.vsize / 1024;        // Convert memory usage to KB

                processes.push(ProcessUsage {
                    pid: stat.pid,
                    name: stat.comm.clone(),
                    cpu_usage,
                    memory_usage,
                });
            }
        }
    }
    processes
}

fn print_sorted_processes(processes: Vec<ProcessUsage>) {
    for process in processes {
        println!(
            "PID: {}, Name: {}, CPU Usage (ticks): {}, Memory Usage: {} KB",
            process.pid, process.name, process.cpu_usage, process.memory_usage
        );
    }
}

fn print_all_processes_sorted(sort_by: &str) {
    let mut processes = get_processes();

    match sort_by {
        "cpu" => processes.sort_by(|a, b| b.cpu_usage.cmp(&a.cpu_usage)),
        "memory" => processes.sort_by(|a, b| b.memory_usage.cmp(&a.memory_usage)),
        _ => {
            eprintln!("Invalid sort option. Please use 'cpu' or 'memory'.");
            return;
        }
    }

    print_sorted_processes(processes);
}

fn print_process_info(pid: i32) {
    match Process::new(pid) {
        Ok(process) => {
            match process.stat() {
                Ok(stat) => {
                    let cpu_usage = stat.utime + stat.stime; // Cumulative CPU time in ticks
                    let memory_usage = stat.vsize / 1024;    // Convert memory usage to KB
                    println!("PID: {}", stat.pid);
                    println!("Command: {}", stat.comm);
                    println!("State: {}", stat.state);
                    println!("CPU Usage (ticks): {}", cpu_usage);
                    println!("Memory Usage: {} KB", memory_usage);
                },
                Err(e) => eprintln!("Failed to get stat for process {}: {:?}", pid, e),
            }
        }
        Err(e) => eprintln!("Failed to find process with PID {}: {:?}", pid, e),
    }
}

fn main() {
    // Prompt the user to choose between viewing a single process or sorted processes
    print!("Enter 'cpu' or 'memory' to view sorted processes, or a specific process ID (PID): ");
    io::stdout().flush().unwrap();

    // Read user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    if input == "cpu" || input == "memory" {
        print_all_processes_sorted(input);
    } else {
        // Parse input as PID and display information for the specific process
        match input.parse::<i32>() {
            Ok(pid) => print_process_info(pid),
            Err(_) => eprintln!("Invalid input: Please enter 'cpu', 'memory', or a valid PID number."),
        }
    }
}

