// procfs crate provides Rust bindings
// to read the /proc filesystem
// and parse files containing process
// information to retrieve process metrics
use procfs::process::*;

// provides methods for writing data to
// output streams
use std::io::{self, Write};


// store information about a process
struct ProcessUsage {

    // process ID
    pid: i32,

    // comm
    name: String,

    // utime + stime - clock ticks

    cpu_usage: u64,

    // virtual memory size - bytes      
    memory_usage: u64,
}

// man proc
// /proc/[pid]/stat
fn get_processes() -> Vec<ProcessUsage> {
    
    // declare mutable variable
    let mut processes: Vec<ProcessUsage> = Vec::new();
   
    // iterate over all proccesses in /proc filesystem
    for process_result in all_processes().unwrap() {

        // ensure process result is successful 
        if let Ok(process) = process_result {
            // retrieve process stats
            if let Ok(stat) = process.stat() {

		// measured in clock ticks
		// total cpu time spent on this process -> utime(user) + stime(kernel)
                let cpu_usage = stat.utime + stat.stime;     // cumulative cpu time

		// converts the virtual memory size (vsize) from bytes to kilobytes
                let memory_usage = stat.vsize / 1024;        // convert memory usage
                
                // add process information to vector
                processes.push(ProcessUsage {
                    pid: stat.pid,
                    name: stat.comm.clone(),
                    cpu_usage,
                    memory_usage,
                });
            }
        }
    }
    // return vector with all process information
    processes
}

// print each proccess in the vector
// with detailed stats
fn print_sorted_processes(processes: Vec<ProcessUsage>) {
    for process in processes {
        println!(
            "PID: {}, Name: {}, CPU Usage (ticks): {}, Memory Usage: {} KB",
            process.pid, process.name, process.cpu_usage, process.memory_usage
        );
    }
}

// function to sort processes
// by either:
// 1. CPU
// 2. Memory Usage
// & display them
fn print_all_processes_sorted(sort_by: &str) -> bool {
    let mut processes = get_processes();

    match sort_by {
        "cpu" => {
            processes.sort_by(|a, b| b.cpu_usage.cmp(&a.cpu_usage));
            print_sorted_processes(processes);
            true
        },
        "memory" => {
            processes.sort_by(|a, b| b.memory_usage.cmp(&a.memory_usage));
            print_sorted_processes(processes);
            true
        },
        _ => {
            eprintln!("Invalid sort option. Please use 'cpu' or 'memory'.");
            false
        }
    }
}

// function to print detailed information
// for a specific proccess by PID
fn print_process_info(pid: i32) {
    match Process::new(pid) {
        Ok(process) => {

            // retrieve stats for given process
            match process.stat() {
                Ok(stat) => {
                    let cpu_usage = stat.utime + stat.stime;
                    let memory_usage = stat.vsize / 1024; 
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

// main.rs
fn main() {

    println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
    println!("Welcome to THE Process Manager ");
    println!("-_-_-_-_-_-_-_-_-_-_-_-_-_-_-_-");
    
    loop {
        println!("1. Enter 'CPU' or 'Memory' to view sorted processes:");
        println!("2. Enter a specific process ID (PID) to view its information: ");
        
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
