use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use std::fs::File;
use std::io::{self, BufWriter, Error, Stdout, Write};
use std::env::args;
use std::path::{PathBuf};
use std::fs;
use crate::input_handler::handle_input;
use crate::renderer;

pub struct App {
    pub title: String,
    pub version: String,
    pub current_file: PathBuf,
    pub text: String, // yes i know... relax
    
    running: bool,
    current_file_path: String,
}

impl App {

    /// Instantiate the app
    pub fn new() -> App {
        let mut text = String::from("");
        let mut current_file_path = String::from("");
        let current_file: PathBuf = if args().count() > 1  {
            let path = PathBuf::from(args().nth(1).unwrap().as_str());

            if path.is_file() {
                current_file_path = String::from(path.to_str().unwrap());
                text = fs::read_to_string(current_file_path.clone()).unwrap()
            }

            path
        } else {
            PathBuf::new()
        };

        App {
            title: String::from("Rocks Text Editor"),
            version: String::from("v0.0.1"),
            current_file,
            text,

            running: true,
            current_file_path
        }
    }

    /// Setup and return the terminal
    pub fn init(&self) -> Result<Terminal<CrosstermBackend<Stdout>>, Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
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
            LeaveAlternateScreen
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

    /// Saves a file containing the buffered text to the desired path
    pub fn save(&self) -> Result<(), Error> {
        let file = File::create("output.txt")?;
        let mut wbuf = BufWriter::new(file);
    
        wbuf.write_all(self.text.as_bytes())
    }

    pub fn get_current_file_path(&self) -> String {
        self.current_file_path.clone()
    }

}