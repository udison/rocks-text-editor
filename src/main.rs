mod app;
mod input_handler;
mod renderer;
mod ui;

use app::App;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize and run the app
    let mut app = App::new();
    let mut terminal =  app.init()?;
    let res = app.run(&mut terminal);
    
    if let Err(err) = res {
        println!("{err:?}");
    }

    app.restore_terminal(&mut terminal)?;

    Ok(())
}
