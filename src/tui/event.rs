use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub enum EventAction {
    Quit,
    ScrollUp,
    ScrollDown,
    None,
}

pub fn handle_events() -> Result<EventAction, Box<dyn std::error::Error>> {
    if event::poll(Duration::from_millis(200))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(EventAction::Quit),
                KeyCode::Down | KeyCode::Char('j') => return Ok(EventAction::ScrollDown),
                KeyCode::Up | KeyCode::Char('k') => return Ok(EventAction::ScrollUp),
                _ => (),
            }
        }
    }
    Ok(EventAction::None)
}

