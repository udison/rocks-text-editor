use crate::app::App;

// TODO: Create a struct to store buffer thingies

pub fn write_buffer(app: &mut App, content: char) {
    app.text.push(content);
    app.modified = true;

    if content == '\n' {
        app.cursor.move_down();
    } else {
        app.cursor.move_right();
    }
}

pub fn write_buffer_str(app: &mut App, content: &str) {
    app.text.push_str(content);
    app.modified = true;

    if content == "\n" {
        app.cursor.move_down();
    } else {
        app.cursor.move_right();
    }
}

pub fn pop_buffer(app: &mut App) {
    app.text.pop();
    app.modified = true;
    app.cursor.move_left();
}
