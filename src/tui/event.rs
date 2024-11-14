// event.rs

use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub fn handle_events() -> Result<bool, Box<dyn std::error::Error>> {
    if event::poll(Duration::from_millis(200))? {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

