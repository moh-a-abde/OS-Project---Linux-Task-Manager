// procfs crate provides Rust bindings
// to read the /proc filesystem
// and parse files containing process
// information to retrieve process metrics
use procfs::process::*;
use std::io::{self, Write, BufRead};
use std::fs::File;

// store information about a process
struct ProcessUsage {
    // process ID
    pid: i32,

    // comm (command name of the process)
    name: String,

    // total CPU time in clock ticks (user time + system time)
    cpu_usage: u64,

    // virtual memory size in kilobytes
    memory_usage: u64,
}

// calculate CPU usage percentage for a process based on its CPU ticks
// and the total CPU ticks available on the system
fn calculate_cpu_usage_percentage(process_cpu_ticks: u64, total_cpu_ticks: u64) -> f64 {
    if total_cpu_ticks == 0 {
        0.0
    } else {
        (process_cpu_ticks as f64 / total_cpu_ticks as f64) * 100.0
    }
}

// calculate memory usage percentage for a process based on its memory usage
// and the total memory used by all processes
fn calculate_memory_usage_percentage(process_memory: u64, total_used_memory: u64) -> f64 {
    if total_used_memory == 0 {
        0.0
    } else {
        (process_memory as f64 / total_used_memory as f64) * 100.0
    }
}

// retrieve information about all processes in the system
// returns a vector of ProcessUsage structs and the total memory used
fn get_processes() -> (Vec<ProcessUsage>, u64) {
    let mut processes: Vec<ProcessUsage> = Vec::new();
    let mut total_used_memory: u64 = 0;

    for process_result in all_processes().unwrap() {
        if let Ok(process) = process_result {
            if let Ok(stat) = process.stat() {
                let cpu_usage = stat.utime + stat.stime;
                let memory_usage = stat.vsize / 1024;
                total_used_memory += memory_usage;

                processes.push(ProcessUsage {
                    pid: stat.pid,
                    name: stat.comm.clone(),
                    cpu_usage,
                    memory_usage,
                });
            }
        }
    }
    (processes, total_used_memory)
}

// print each process in the vector with detailed stats including CPU and memory percentages
fn print_sorted_processes(processes: Vec<ProcessUsage>, total_cpu_ticks: u64, total_used_memory: u64) {
    for process in processes {
        let cpu_percentage = calculate_cpu_usage_percentage(process.cpu_usage, total_cpu_ticks);
        let memory_percentage = calculate_memory_usage_percentage(process.memory_usage, total_used_memory);

        println!(
            "PID: {}, Name: {}, CPU Usage: {} ticks ({:.2}%), Memory Usage: {} KB ({:.2}%)",
            process.pid, process.name, process.cpu_usage, cpu_percentage, process.memory_usage, memory_percentage
        );
    }
}

// function to sort processes by either CPU or memory usage and display them
fn print_all_processes_sorted(sort_by: &str) -> bool {
    let (mut processes, total_used_memory) = get_processes();
    let total_cpu_ticks = get_total_cpu_ticks();

    match sort_by {
        "cpu" => {
            processes.sort_by(|a, b| b.cpu_usage.cmp(&a.cpu_usage));
            print_sorted_processes(processes, total_cpu_ticks, total_used_memory);
            true
        }
        "memory" => {
            processes.sort_by(|a, b| b.memory_usage.cmp(&a.memory_usage));
            print_sorted_processes(processes, total_cpu_ticks, total_used_memory);
            true
        }
        _ => {
            eprintln!("Invalid sort option. Please use 'cpu' or 'memory'.");
            false
        }
    }
}

// function to print detailed information for a specific process by PID
fn print_process_info(pid: i32) {
    let total_cpu_ticks = get_total_cpu_ticks();
    let (_, total_used_memory) = get_processes();

    match Process::new(pid) {
        Ok(process) => {
            match process.stat() {
                Ok(stat) => {
                    let cpu_usage = stat.utime + stat.stime;
                    let memory_usage = stat.vsize / 1024;
                    let cpu_percentage = calculate_cpu_usage_percentage(cpu_usage, total_cpu_ticks);
                    let memory_percentage = calculate_memory_usage_percentage(memory_usage, total_used_memory);

                    println!("PID: {}", stat.pid);
                    println!("Command: {}", stat.comm);
                    println!("State: {}", stat.state);
                    println!("CPU Usage: {} ticks ({:.2}%)", cpu_usage, cpu_percentage);
                    println!("Memory Usage: {} KB ({:.2}%)", memory_usage, memory_percentage);
                },
                Err(e) => eprintln!("Failed to get stat for process {}: {:?}", pid, e),
            }
        }
        Err(e) => eprintln!("Failed to find process with PID {}: {:?}", pid, e),
    }
}

// read the total CPU ticks from /proc/stat
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

// main function to interact with the user
fn main() {
    println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
    println!("Welcome to THE Process Manager ");
    println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
    
    loop {
        println!("1. Enter 'CPU' or 'Memory' to view sorted processes based on usage:");
        println!("2. Enter a specific process ID (PID) to view its information:");
        
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim().to_lowercase();

        if input == "cpu" || input == "memory" {
            if print_all_processes_sorted(&input) {
                break;
            }
        } else {
            match input.parse::<i32>() {
                Ok(pid) => {
                    print_process_info(pid);
                    break;
                }
                Err(_) => eprintln!("Invalid input: Please enter 'cpu', 'memory', or a valid PID number."),
            }
        }
    }
}

