// data.rs

use procfs::process::*;
use std::fs::File;
use std::io::{self, BufRead};

pub struct ProcessUsage {
    pub pid: i32,
    pub name: String,
    pub cpu_usage: u64,
    pub memory_usage: u64,
}

pub fn get_processes() -> Vec<ProcessUsage> {
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
    processes
}

