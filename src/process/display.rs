// src/process/display.rs

use procfs::process::Process;

pub fn get_process_info(pid: i32) -> String {
    match Process::new(pid) {
        Ok(process) => {
            match process.stat() {
                Ok(stat) => {
                    format!(
                        "PID: {}\nCommand: {}\nState: {}\nCPU Usage: {} ticks\nMemory Usage: {} KB",
                        stat.pid,
                        stat.comm,
                        stat.state,
                        stat.utime + stat.stime,
                        stat.vsize / 1024
                    )
                },
                Err(e) => format!("Failed to get stat for process {}: {:?}", pid, e),
            }
        }
        Err(e) => format!("Failed to find process with PID {}: {:?}", pid, e),
    }
}

