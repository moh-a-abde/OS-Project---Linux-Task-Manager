use procfs::process::*;
use std::io::{self, Write};

fn print_process_stats(stat: &Stat) {
    println!("PID: {}", stat.pid);
    println!("Command: {}", stat.comm);
    println!("State: {}", stat.state);
    println!("CPU Usage: {:?}", stat.utime + stat.stime);
    println!("Memory Usage: {}", stat.vsize);
    println!("---");
}

fn print_all_processes() {
    for process_result in all_processes().unwrap() {
        if let Ok(process) = process_result {
            if let Ok(stat) = process.stat() {
                print_process_stats(&stat);
            } else {
                eprintln!("Failed to get stat for process {}", process.pid);
            }
        } else {
            eprintln!("Failed to get process information");
        }
    }
}

fn print_process_info(pid: i32) {
    match Process::new(pid) {
        Ok(process) => {
            match process.stat() {
                Ok(stat) => print_process_stats(&stat),
                Err(e) => eprintln!("Failed to get stat for process {}: {:?}", pid, e),
            }
        }
        Err(e) => eprintln!("Failed to find process with PID {}: {:?}", pid, e),
    }
}

fn main() {
    // Prompt the user for a process ID
    print!("Enter the process ID (PID): ");
    io::stdout().flush().unwrap(); // Flush to ensure the prompt is displayed

    // Read user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse input to an integer
    match input.trim().parse::<i32>() {
        Ok(pid) => print_process_info(pid),
        Err(_) => eprintln!("Invalid PID: Please enter a valid number."),
    }
}

