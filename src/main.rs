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

use std::fs;

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
    virtual_memory_usage: u64,

    // resident memory size in kilobytes
    resident_memory_usage: u64,

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
// calculate virtual memory usage percentage for a process based
// on its virtual memory usage
// and the total virtual memory used by all processes
fn calculate_virtual_memory_usage_percentage(process_memory: u64, total_used_memory: u64) -> f64 {
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
// calculate resident memory usage percentage for a process based
// on its resident memory usage
// and the total virtual memory used by all processes
fn calculate_resident_memory_usage_percentage(process_memory: u64, total_used_memory: u64) -> f64 {
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


fn parse_status_file(pid: u32) -> io::Result<(u64, u64)> {
    // Path to the file descriptor directory for the given process PID
    // let fd_dir_path = format!("/proc/{}/fd", pid);

    // // Try to read the directory and count the number of files
    // let fd_count = fs::read_dir(fd_dir_path)?
    //     .filter_map(Result::ok)
    //     .count(); // Count the number of entries (file descriptors)

    // You can extract voluntary_ctxt_switches, nonvoluntary_ctxt_switches, and owner_uid
    // from the /proc/[pid]/status file like in your previous code

    let status_path = format!("/proc/{}/status", pid);
    let file = fs::File::open(&status_path)?;
    let reader = io::BufReader::new(file);

    let mut voluntary_ctxt_switches = 0;
    let mut nonvoluntary_ctxt_switches = 0;
    //let mut owner_uid = 0;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("voluntary_ctxt_switches:") {
            // Extract voluntary context switches
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 1 {
                voluntary_ctxt_switches = parts[1].parse::<u64>().unwrap_or(0);
            }
        } else if line.starts_with("nonvoluntary_ctxt_switches:") {
            // Extract nonvoluntary context switches
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 1 {
                nonvoluntary_ctxt_switches = parts[1].parse::<u64>().unwrap_or(0);
            }
         } 
         //else if line.starts_with("Uid:") {
        //     // Extract process owner UID
        //     let parts: Vec<&str> = line.split_whitespace().collect();
        //     if parts.len() > 1 {
        //         owner_uid = parts[1].parse::<u32>().unwrap_or(0);
        //     }
        // }
    }

    Ok((voluntary_ctxt_switches, nonvoluntary_ctxt_switches))
}



// FUNCTION:
// retrieve information about all processes in the system
// returns a vector of ProcessUsage structs and the total 
// memory used by each process
fn get_processes() -> (Vec<ProcessUsage>, u64, u64) {

    // Linux has a default page size of 4069 bytes and we need this constant in calculation of resident memory
    const Page_Size: u64 = 4096;

    // initialize an empty vector 
    // to store process information
    let mut processes: Vec<ProcessUsage> = Vec::new();
    
    // initialize a variable to
    // accumulate the total virtual memory used by all processes
    let mut total_virtual_used_memory: u64 = 0;

    // initialize a variable to
    // accumulate the total resident memory used by all processes
    let mut total_resident_used_memory: u64 = 0;

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
                let virtual_memory_usage = stat.vsize / 1024;
                total_virtual_used_memory += virtual_memory_usage;


                // convert resident memory size 
                // from bytes to kilobytes
                let resident_memory_usage = (stat.rss*Page_Size) / 1024;
                total_resident_used_memory += resident_memory_usage;

                
		// add the process information 
		// to vector
                processes.push(ProcessUsage {
                    pid: stat.pid,
                    name: stat.comm.clone(),
                    cpu_usage,
                    virtual_memory_usage,
                    resident_memory_usage,
                });
            }
        }
    }
    
    //return the vector of processes
    // and the total memory used
    (processes, total_virtual_used_memory, total_resident_used_memory)
}

// FUNCTION:
// print each process in the vector with detailed stats 
fn print_sorted_processes(processes: Vec<ProcessUsage>, total_cpu_ticks: u64, total_virtual_used_memory: u64, total_resident_used_memory: u64,) {

    // iterate over each process in the vector
    for process in processes {
    
        // calculate CPU usage percentage for the process
        let cpu_percentage = calculate_cpu_usage_percentage(process.cpu_usage, total_cpu_ticks);
        
        // calculate virtual memory usage percentage for the process
        let virtual_memory_percentage = calculate_virtual_memory_usage_percentage(process.virtual_memory_usage, total_virtual_used_memory);
        
        // calculate resident memory usage percentage for the process
        let resident_memory_percentage = calculate_resident_memory_usage_percentage(process.resident_memory_usage, total_resident_used_memory);
        
	// print process details including:
	// 1. CPU usage in ticks and percentage
	// 2. Memory usage in KB and percentage
        println!(
            "PID: {}, Name: {}, CPU Usage: {} ticks ({:.2}%), Virtual Memory Usage: {} KB ({:.2}%, Resident Memory Usage: {} KB ({:.2}%)",
            process.pid, process.name, process.cpu_usage, cpu_percentage, process.virtual_memory_usage, virtual_memory_percentage, process.resident_memory_usage, resident_memory_percentage
        );
    }
}

// FUNCTION:
// sort processes by either CPU or memory usage and display them
fn print_all_processes_sorted(sort_by: &str) -> bool {

    // retrieve processes and the total memory used by all processes
    let (mut processes, total_virtual_used_memory, total_resident_used_memory) = get_processes();
    
    // retrieve total CPU ticks available on the system
    let total_cpu_ticks = get_total_cpu_ticks();
    
    // determine sorting method based on user input
    match sort_by {
    
    	// sort processes in descending order by CPU usage
        "cpu" => {
            processes.sort_by(|a, b| b.cpu_usage.cmp(&a.cpu_usage));
            print_sorted_processes(processes, total_cpu_ticks, total_virtual_used_memory, total_resident_used_memory);
            true
        }
        
        // sort processes in descending order by memory usage
        "memory" => {
            processes.sort_by(|a, b| b.resident_memory_usage.cmp(&a.resident_memory_usage));
            print_sorted_processes(processes, total_cpu_ticks, total_virtual_used_memory, total_resident_used_memory);
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
    let (_, total_virtual_used_memory, total_resident_used_memory) = get_processes();
    
    // retrieve the specific process by PID
    match Process::new(pid) {
        Ok(process) => {
            match process.stat() {
                Ok(stat) => {
                    
                    // retrieve stats for the process
                    let cpu_usage = stat.utime + stat.stime;
                    
                    // convert memory usage from bytes to kilobytes
                    let virtual_memory_usage= stat.vsize / 1024;

                    let resident_memory_usage = stat.rss / 1024;
                    
                    // calculate CPU usage percentage for the process
                    let cpu_percentage = calculate_cpu_usage_percentage(cpu_usage, total_cpu_ticks);
                    
                    // calculate the virtual memory usage percentage for the process
                    let virtual_memory_percentage = calculate_virtual_memory_usage_percentage(virtual_memory_usage, total_virtual_used_memory);

                    // calculate the resident memory usage percentage for the process
                    let resident_memory_percentage = calculate_resident_memory_usage_percentage(resident_memory_usage, total_resident_used_memory);
                    
		    // print detailed process information including
		    // 1. CPU
		    // 2. Memory usage
                    println!("PID: {}", stat.pid);
                    println!("Parent PID: {}", stat.ppid);
                    println!("Command: {}", stat.comm);
                    println!("State: {}", stat.state);
                    println!("CPU Usage: {} ticks ({:.2}%)", cpu_usage, cpu_percentage);
                    println!("Virtual Memory Usage: {} KB ({:.2}%)", virtual_memory_usage, virtual_memory_percentage);
                    println!("Resident Memory Usage: {} KB ({:.2}%)", resident_memory_usage, resident_memory_percentage);
                    match parse_status_file(pid.try_into().unwrap()) {
                        Ok((voluntary, nonvoluntary)) => {
                        //Ok((voluntary, nonvoluntary)) => {
                                    println!("Number of Nonpreemptive context switches: {}", voluntary);
                                    println!("Number of Preemptive context switches: {}", nonvoluntary);
                                    //println!("Process owner ID: {}", owner_id);
                                    //println!("Number of file descriptors currently accessing this process: {}", fd_count);
                                    }
                        Err(e) => {
                            eprintln!("Failed to parse status file for process {}: {:?}", pid, e);
                        },
                        }

                    println!("Number of threads used by the process: {}", stat.num_threads);
                    println!("Priority of the process: {}", stat.priority);
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
        println!("1. Enter 'CPU' or 'Memory' (Resident Memory) to view sorted processes based on usage:");
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

