// Declare submodules for `tui`
pub mod layout;
pub mod render;
pub mod event;

// Remove the unused `execute` import
use tui::backend::CrosstermBackend;
use tui::Terminal;
use std::io::{self, Stdout};

// Initialize the terminal
pub fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn main_loop(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut scroll_offset = 0;

    loop {
        terminal.draw(|f| {
            let layout = layout::create_layout(f.size());
            render::render_layout(f, &layout, scroll_offset);
        })?;

        match event::handle_events()? {
            event::EventAction::Quit => break,
            event::EventAction::ScrollDown => scroll_offset += 1,
            event::EventAction::ScrollUp => {
                if scroll_offset > 0 {
                    scroll_offset -= 1;
                }
            }
            event::EventAction::None => {}
        }
    }
    Ok(())
}

