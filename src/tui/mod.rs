pub mod layout;
pub mod render;
pub mod event;

use tui::backend::CrosstermBackend;
use tui::Terminal;
use std::io::{self, Stdout};
use tui::layout::{Layout, Constraint};
use crate::process::data::{get_processes, ProcessUsage};
use crate::process::display::get_process_info;


pub fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn main_loop(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut scroll_offset = 0;
    let mut input = String::new();
    let mut command_output = String::new();
    let mut processes = get_processes();
    let mut sort_by = String::from("none");

    loop {
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
                    Constraint::Percentage(60),
                    Constraint::Percentage(10),
                    Constraint::Percentage(30),
                ].as_ref())
                .split(f.size());

            render::render_layout(f, &chunks, scroll_offset, &input, &command_output, &processes);
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
                if command == "cpu" || command == "memory" {
                    sort_by = command.clone();
                    command_output = format!("Sorting processes by {}", command);
                } else if let Ok(pid) = command.parse::<i32>() {
                    // Display details for the process with the specified PID
                    command_output = get_process_info(pid);
                } else {
                    command_output = "Invalid command. Please enter 'cpu', 'memory', or a valid PID.".to_string();
                }
                input.clear();
            }
            event::EventAction::None => {}
        }
    }
    Ok(())
}


