pub mod layout;
pub mod render;
pub mod event;

use tui::backend::CrosstermBackend;
use tui::Terminal;
use std::io::{self, Stdout};
use tui::layout::{Layout, Constraint, Direction};
use crate::process::data::{get_processes};
use crate::process::display::get_process_info;

use tui::widgets::{Paragraph, Block, Borders};
use tui::style::{Style, Color};
use sysinfo::{System, SystemExt};


pub fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn main_loop(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut system = System::new_all(); // Initialize the System struct for gathering system info
    let mut scroll_offset = 0;
    let mut input = String::new();
    let mut command_output = String::new();
    let mut processes = get_processes();
    let mut sort_by = String::from("none");

    loop {
        // Refresh system information
        system.refresh_all();

        // Sort processes based on the sort criterion
        if sort_by == "cpu" {
            processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal));
        } else if sort_by == "memory" {
            processes.sort_by(|a, b| b.memory_usage.partial_cmp(&a.memory_usage).unwrap_or(std::cmp::Ordering::Equal));
        }

        // Draw the TUI layout
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(tui::layout::Direction::Vertical)
                .constraints([
                    Constraint::Percentage(10), // System Info Header
                    Constraint::Percentage(50), // Processes Table
                    Constraint::Percentage(10), // Input
                    Constraint::Percentage(30), // Command Output
                ].as_ref())
                .split(f.size());

            render::render_system_info(f, chunks[0], &system); // Render the system info header
            render::render_layout(f, &chunks[1..], scroll_offset, &input, &command_output, &processes);
        })?;

        // Handle events (including user input)
        match event::handle_events(&mut input)? {
    event::EventAction::Quit => break,
    event::EventAction::ScrollDown => scroll_offset += 1,
    event::EventAction::ScrollUp => {
        if scroll_offset > 0 {
            scroll_offset -= 1;
        }
    }
    event::EventAction::ExecuteCommand(command) => {
        // Sorting or displaying process info based on command input
        if command == "cpu" || command == "memory" || command == "ppid" || command == "state" 
            || command == "start_time" || command == "priority" {
            
            // Sort by the specified field
            sort_by = command.clone();
            command_output = format!("Sorting processes by {}", command);

            // Apply sorting based on the current sort_by value
            match sort_by.as_str() {
                "cpu" => processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)),
                "memory" => processes.sort_by(|a, b| b.memory_usage.partial_cmp(&a.memory_usage).unwrap_or(std::cmp::Ordering::Equal)),
                "ppid" => processes.sort_by(|a, b| a.ppid.cmp(&b.ppid)),
                "state" => processes.sort_by(|a, b| a.state.cmp(&b.state)),
                "start_time" => processes.sort_by(|a, b| a.start_time.cmp(&b.start_time)),
                "priority" => processes.sort_by(|a, b| a.priority.cmp(&b.priority)),
                _ => {}
            }

        } else if let Ok(pid) = command.parse::<i32>() {
            // Display details for the process with the specified PID
            command_output = get_process_info(pid);
        } else {
            command_output = "Invalid command. Please enter 'cpu', 'memory', 'ppid', 'state', 'start_time', 'priority', or a valid PID.".to_string();
        }
        
        input.clear();
    }
    event::EventAction::None => {}
}
    }
    Ok(())
}

