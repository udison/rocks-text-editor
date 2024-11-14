use ratatui::{text::Text, widgets::Paragraph, Frame};

use crate::{app::App, ui::{app_layout, render_title_bar}};

// TODO: This could be just a function inside App
pub fn render(app: &App, frame: &mut Frame<'_>) {
    let app_layout = app_layout(frame);
    
    render_title_bar(frame, app, app_layout[0]);
    
    let test = Paragraph::new(Text::from("test"));
    frame.render_widget(test, app_layout[1]);
}