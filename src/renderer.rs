use ratatui::Frame;

use crate::{app::App, ui::{app_layout, render_text_editor, render_title_bar}};
use crate::ui::render_footer;

pub fn render(app: &App, frame: &mut Frame<'_>) {
    let app_layout = app_layout(frame);

    render_title_bar(frame, app, app_layout[0]);
    render_text_editor(frame, app, app_layout[1]);
    render_footer(frame, app, app_layout[2]);
}