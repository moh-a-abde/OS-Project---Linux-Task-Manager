// mod.rs

pub mod layout;
pub mod event;
pub mod render;

use crossterm::execute;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use std::io::{self, Stdout};

pub fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn main_loop(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| {
            let layout = layout::create_layout(f.size());
            render::render_layout(f, &layout);
        })?;

        if event::handle_events()? {
            break;
        }
    }
    Ok(())
}

