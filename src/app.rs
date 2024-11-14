use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, Error, Stdout};

use crate::input_handler::handle_input;
use crate::renderer;

pub struct App {
    pub title: String,
    pub version: String,
    pub text: String, // yes i know... relax
    
    running: bool,
}

impl App {

    /// Instantiate the app
    pub fn new() -> App {
        App {
            title: String::from("Rocks Text Editor"),
            version: String::from("v0.0.1"),
            running: true,
            text: String::from("")
        }
    }

    /// Setup and return the terminal
    pub fn init() -> Result<Terminal<CrosstermBackend<Stdout>>, Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        
        Terminal::new(backend)
    }

    /// Runs the application
    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Error> {
        while self.running {
            handle_input(self)?;
            
            terminal.draw(|frame| renderer::render(self, frame))?;
        }

        Ok(())
    }

    /// Quits the application
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Restores the terminal back to the state it was before the App ran
    pub fn restore_terminal(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Error> {
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

}