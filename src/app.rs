

use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, Error, Stdout};

use crate::input_handler::handle_input;

pub struct App {
    pub title: String,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    
    running: bool,
}

impl App {

    /// Setup the terminal's initial state and initializes the app
    pub fn init() -> Result<App, Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        
        Ok(App {
            title: String::from("Rocks Text Editor"),
            terminal: Terminal::new(backend)?,
            running: true
        })
    }

    /// Runs the application
    pub fn run(&mut self) -> Result<(), Error> {
        println!("App {} is running!", self.title);

        while self.running {
            handle_input(self)?;
        }

        Ok(())
    }

    /// Quits the application
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Restores the terminal back to the state it was before the App ran
    pub fn restore_terminal(&mut self) -> Result<(), Error> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;

        Ok(())
    }

}