use crate::app::App;

pub fn write_buffer(app: &mut App, content: char) {
    app.text.push(content);
    app.modified = true;
}
