use std::io::Error;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{app::App, buffer_handler::write_buffer, file_manipulation::save};

pub fn handle_input(app: &mut App) -> Result<(), Error> {
    if let Event::Key(key) = event::read()? {
        match key.kind {
            KeyEventKind::Press => handle_key_press(app, key)?,

            _ => {}
        }
    }

    Ok(())
}

fn handle_key_press(app: &mut App, key: KeyEvent) -> Result<(), Error> {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('s') => {
                app.modified = false;
                save(&app.current_file, app.text.as_bytes())?
            }

            _ => {}
        }
    } else {
        match key.code {
            KeyCode::Esc => {
                app.quit();
            }
            KeyCode::Char(char) => write_buffer(app, char),
            KeyCode::Backspace => {
                app.text.pop();
                app.modified = true;
            }
            KeyCode::Enter => write_buffer(app, '\n'),

            _ => {}
        }
    }

    Ok(())
}
