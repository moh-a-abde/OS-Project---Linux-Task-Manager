// app.rs

pub mod tui;
pub mod process;

use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut terminal = tui::init_terminal()?;

    tui::main_loop(&mut terminal)?;

    disable_raw_mode()?;
    Ok(())
}

