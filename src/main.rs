// ***** THE Process Manager *****
// by:
//     *Abdelrahman Elaskary*
//     *Malak Zeerban*
//     *Mohamed Abdel-Hamid*
//     *Merna Elsaaran*

// procfs crate provides Rust bindings
// to read the /proc filesystem
// and parse files containing process
// information to retrieve process metrics
use procfs::process::*;

// provides methods for writing data to
// output streams
use std::io::{self, Write, BufRead};

// provides methods for file handling
use std::fs::File;

// STRUCT:
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

// FUNCTION:
// calculate CPU usage percentage for a process based on its CPU ticks
// and the total CPU ticks available on the system
fn calculate_cpu_usage_percentage(process_cpu_ticks: u64, total_cpu_ticks: u64) -> f64 {

    // check if total_cpu_ticks is 0
    // to prevent division by zero
    if total_cpu_ticks == 0 {
        0.0
    } else {
    
        // calculate CPU usage as a percentage
        // of total CPU ticks
        (process_cpu_ticks as f64 / total_cpu_ticks as f64) * 100.0
    }
}

// FUNCTION:
// calculate memory usage percentage for a process based
// on its memory usage
// and the total memory used by all processes
fn calculate_memory_usage_percentage(process_memory: u64, total_used_memory: u64) -> f64 {
    // check if total_used_memory is 0
    // to prevent division by zero
    if total_used_memory == 0 {
        0.0
    } else {
        // calculate memory usage as a percentage
        // of total memory used
        (process_memory as f64 / total_used_memory as f64) * 100.0
    }
}

// FUNCTION:
// retrieve information about all processes in the system
// returns a vector of ProcessUsage structs and the total 
// memory used by each process
fn get_processes() -> (Vec<ProcessUsage>, u64) {

    // initialize an empty vector 
    // to store process information
    let mut processes: Vec<ProcessUsage> = Vec::new();
    
    // initialize a variable to
    // accumulate the total memory used by all processes
    let mut total_used_memory: u64 = 0;

    // iterate over all processes
    // in the /proc filesystem
    for process_result in all_processes().unwrap() {
        // ensure process result is successful 
        // before proceeding
        if let Ok(process) = process_result {
        
            // retrieve process stats
            if let Ok(stat) = process.stat() {
            
                // calculate total CPU time 
                // for this process (user + system)
                let cpu_usage = stat.utime + stat.stime;
                
                // convert virtual memory size 
                // from bytes to kilobytes
                let memory_usage = stat.vsize / 1024;
                total_used_memory += memory_usage;
                
		// add the process information 
		// to vector
                processes.push(ProcessUsage {
                    pid: stat.pid,
                    name: stat.comm.clone(),
                    cpu_usage,
                    memory_usage,
                });
            }
        }
    }
    
    //return the vector of processes
    // and the total memory used
    (processes, total_used_memory)
}

// FUNCTION:
// print each process in the vector with detailed stats 
fn print_sorted_processes(processes: Vec<ProcessUsage>, total_cpu_ticks: u64, total_used_memory: u64) {

    // iterate over each process in the vector
    for process in processes {
    
        // calculate CPU usage percentage for the process
        let cpu_percentage = calculate_cpu_usage_percentage(process.cpu_usage, total_cpu_ticks);
        
        // calculate memory usage percentage for the process
        let memory_percentage = calculate_memory_usage_percentage(process.memory_usage, total_used_memory);
        
        
	// print process details including:
	// 1. CPU usage in ticks and percentage
	// 2. Memory usage in KB and percentage
        println!(
            "PID: {}, Name: {}, CPU Usage: {} ticks ({:.2}%), Memory Usage: {} KB ({:.2}%)",
            process.pid, process.name, process.cpu_usage, cpu_percentage, process.memory_usage, memory_percentage
        );
    }
}

// FUNCTION:
// sort processes by either CPU or memory usage and display them
fn print_all_processes_sorted(sort_by: &str) -> bool {

    // retrieve processes and the total memory used by all processes
    let (mut processes, total_used_memory) = get_processes();
    
    // retrieve total CPU ticks available on the system
    let total_cpu_ticks = get_total_cpu_ticks();
    
    // determine sorting method based on user input
    match sort_by {
    
    	// sort processes in descending order by CPU usage
        "cpu" => {
            processes.sort_by(|a, b| b.cpu_usage.cmp(&a.cpu_usage));
            print_sorted_processes(processes, total_cpu_ticks, total_used_memory);
            true
        }
        
        // sort processes in descending order by memory usage
        "memory" => {
            processes.sort_by(|a, b| b.memory_usage.cmp(&a.memory_usage));
            print_sorted_processes(processes, total_cpu_ticks, total_used_memory);
            true
        }
        
        // handle invalid sort options and display an error message
        _ => {
            eprintln!("Invalid sort option. Please use 'cpu' or 'memory'.");
            false
        }
    }
}

// FUNCTION:
// print detailed information for a specific process by PID
fn print_process_info(pid: i32) {
    
    // retrieve total CPU ticks to calculate CPU usage percentage
    let total_cpu_ticks = get_total_cpu_ticks();
    
    // retrieve only the total memory used by all processes
    let (_, total_used_memory) = get_processes();

    // retrieve the specific process by PID
    match Process::new(pid) {
        Ok(process) => {
            match process.stat() {
                Ok(stat) => {
                    
                    // retrieve stats for the process
                    let cpu_usage = stat.utime + stat.stime;
                    
                    // convert memory usage from bytes to kilobytes
                    let memory_usage = stat.vsize / 1024;
                    
                    // calculate CPU usage percentage for the process
                    let cpu_percentage = calculate_cpu_usage_percentage(cpu_usage, total_cpu_ticks);
                    
                    // calculate memory usage percentage for the process
                    let memory_percentage = calculate_memory_usage_percentage(memory_usage, total_used_memory);
                    
		    // print detailed process information including
		    // 1. CPU
		    // 2. Memory usage
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

// FUNCTION:
// read total CPU ticks from /proc/stat
fn get_total_cpu_ticks() -> u64 {
    
    // open /proc/stat for reading
    if let Ok(file) = File::open("/proc/stat") {
        let reader = io::BufReader::new(file);
        
        // read the first line, 
        // which starts with cpu 
        // and contains cumulative CPU times
        if let Some(Ok(line)) = reader.lines().next() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            
	    // check if the first token is cpu 
	    // and proceed to sum up CPU times
            if parts[0] == "cpu" {
            
                // parse each of the following values as u64 and sum them up
                // these values correspond to user, nice,
                // system, idle, iowait, irq, softirq, etc.
                return parts.iter().skip(1).filter_map(|v| v.parse::<u64>().ok()).sum();
            }
        }
    }
    
    // return 0 if we couldn't read or parse the file
    0
}

// MAIN:
// interact with user
fn main() {
    println!("===============================");
    println!("Welcome to THE Process Manager ");
    println!("===============================");
    
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

