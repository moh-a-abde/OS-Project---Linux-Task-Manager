# THE Process Manager

A Rust-based command-line application that monitors and displays detailed process metrics. This tool retrieves CPU and memory usage data for processes running on the system and allows sorting by either CPU or memory usage.

### Authors
- Abdelrahman Elaskary
- Malak Zeerban
- Mohamed Abdel-Hamid
- Merna Elsaaran

## Features
1. **Retrieve and Display Process Data**: Reads process information from the `/proc` filesystem using the `procfs` crate.
2. **CPU and Memory Usage Calculations**: Calculates and displays CPU and memory usage as a percentage of total available resources.
3. **Sorting**: Sort processes by either CPU or memory usage for easy analysis.
4. **Detailed Process Information**: Provides detailed information for individual processes based on their PID.

## Prerequisites
- **Rust**: Ensure you have Rust installed on your machine.
- **Linux**: This tool relies on the `/proc` filesystem, which is available on Linux-based systems.

## Structure
'app.rs' and 'main.rs' for the primary application setup.
'process/data.rs' and 'process/display.rs' for handling process-related data.
'tui/render.rs' and 'tui/mod.rs' for TUI layout and rendering.
