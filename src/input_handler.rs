use std::io::Error;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{app::App, file_handler::save};

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
                // TODO: Reimplement save when piece table buffering is implemented
                //save(&app.current_file, app.buffer_handler.text.as_bytes())?
            }

            _ => {}
        }
    } else {
        match key.code {
            // Function keys
            KeyCode::Esc => app.quit(),

            // Move keys
            KeyCode::Up => app.buffer_handler.cursor.move_up(),
            KeyCode::Down => app.buffer_handler.cursor.move_down(),
            KeyCode::Left => app.buffer_handler.cursor.move_left(),
            KeyCode::Right => app.buffer_handler.cursor.move_right(),

            // Buffer changing keys
            KeyCode::Backspace => app.buffer_handler.pop(),
            KeyCode::Enter => app.buffer_handler.write("\n"),
            KeyCode::Tab => app.buffer_handler.write("    "),
            KeyCode::Char(char) => app.buffer_handler.write(char.to_string().as_str()),

            _ => {}
        }
    }

    Ok(())
}
