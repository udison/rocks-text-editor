use std::io::Error;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{app::App, buffer_handler::write_buffer, file_handler::save};
use crate::buffer_handler::{pop_buffer, write_buffer_str};

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
            // Function keys
            KeyCode::Esc => app.quit(),

            // Move keys
            KeyCode::Up => app.cursor.move_up(),
            KeyCode::Down => app.cursor.move_down(),
            KeyCode::Left => app.cursor.move_left(),
            KeyCode::Right => app.cursor.move_right(),

            // Buffer changing keys
            KeyCode::Backspace => pop_buffer(app),
            KeyCode::Enter => write_buffer(app, '\n'),
            KeyCode::Tab => write_buffer_str(app, "    "),
            KeyCode::Char(char) => write_buffer(app, char),

            _ => {}
        }
    }

    Ok(())
}
