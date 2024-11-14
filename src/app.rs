// app.rs

use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crate::tui;
use crate::process;
use crate::tui::main_loop;
use crate::tui::init_terminal;


pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut terminal = init_terminal()?;

    main_loop(&mut terminal)?;

    disable_raw_mode()?;
    Ok(())
}

