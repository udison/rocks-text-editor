use ratatui::Frame;

use crate::{app::App, ui::{app_layout, render_text_editor, render_title_bar}};

pub fn render(app: &App, frame: &mut Frame<'_>) {
    let app_layout = app_layout(frame);

    render_title_bar(frame, app, app_layout[0]);
    render_text_editor(frame, app, app_layout[1]);
}