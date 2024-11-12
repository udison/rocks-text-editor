use std::io::Error;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::app::App;

pub fn handle_input(app: &mut App) -> Result<(), Error> {
    if let Event::Key(key) = event::read()? {
        match key.kind {
            KeyEventKind::Press => handle_key_press(app, key),
            _ => {}
        }
    }

    Ok(())
}

fn handle_key_press(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.quit(),

        _ => {}
    }
}