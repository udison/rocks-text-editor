mod app;
mod input_handler;

use app::App;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize and run the app
    let mut app = App::init()?;
    let res = app.run();
    
    if let Err(err) = res {
        println!("{err:?}");
    }

    app.restore_terminal()?;

    Ok(())
}
