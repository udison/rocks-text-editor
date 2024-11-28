mod app;
mod buffer_handler;
mod file_handler;
mod input_handler;
mod renderer;
mod ui;
mod cursor;

use app::App;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize and run the app
    let mut app = App::new();
    let mut terminal = app.init()?;
    let res = app.run(&mut terminal);

    if let Err(err) = res {
        println!("{err:?}");
    }

    app.restore_terminal(&mut terminal)?;

    Ok(())
}
